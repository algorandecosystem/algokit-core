//! Publisher tooling for `seed_vault_wasm`: generate a publisher key, sign
//! module manifests, sign revocation lists. Host-only (`std`); the TA only
//! builds the library.
//!
//! Keep real publisher seeds offline (ideally in an HSM): anyone holding one
//! can sign modules every TA trusting its public key will run.
//!
//! ```text
//! seed_vault_wasm_sign keygen <seed.hex> <public.key>
//! seed_vault_wasm_sign dev-seed <seed.hex>
//! seed_vault_wasm_sign pubkey <seed.hex> <public.key>
//! seed_vault_wasm_sign sign <seed.hex> <key_id> <module.wasm> <name> <version> <caps> <out.svwm>
//!     caps: comma-separated `account-entropy`, `human-sign`, or `none`
//! seed_vault_wasm_sign revoke <seed.hex> <key_id> <sequence> <out.svwr> [key:<id>|program:<hex>|module:<path>]...
//! seed_vault_wasm_sign verify <public.key> <key_id> <signed file> [module.wasm]
//! ```
//!
//! `seed.hex`: the 32-byte Falcon-1024 keygen seed, as hex. `public.key`:
//! the raw 1793-byte public key (what the TA builds in). Signed files are
//! `seed_vault_wasm::Signed` encodings, sent as-is in `LOAD_FINISH`/`REVOKE`.

use std::io::Read;
use std::process::ExitCode;
use std::{env, fs};

use algokit_crypto::falcon1024::PublicKey;
use seed_vault_wasm::manifest::{Capabilities, Manifest, PublisherKey, Revocations, Signed};
use seed_vault_wasm::policy::{dev_publisher_seed, publisher_keypair};
use seed_vault_wasm::ProgramId;

type Result<T> = std::result::Result<T, String>;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<()> {
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["keygen", seed_out, public_out] => {
            let mut seed = [0u8; 32];
            fs::File::open("/dev/urandom")
                .and_then(|mut f| f.read_exact(&mut seed))
                .map_err(|e| format!("reading /dev/urandom: {e}"))?;
            write_seed(seed_out, &seed)?;
            write_public(public_out, &seed)
        }
        ["dev-seed", seed_out] => write_seed(seed_out, &dev_publisher_seed()),
        ["pubkey", seed_file, public_out] => write_public(public_out, &read_seed(seed_file)?),
        ["sign", seed_file, key_id, module, name, version, caps, out] => {
            let manifest = Manifest {
                key_id: parse(key_id, "key_id")?,
                program_id: ProgramId::of(&read(module)?),
                name: name.to_string(),
                version: parse(version, "version")?,
                capabilities: parse_caps(caps)?,
            };
            let body = manifest.encode();
            if Manifest::decode(&body).as_ref() != Some(&manifest) {
                return Err(format!("invalid name {name:?}: use 1-64 of [a-z0-9._-]"));
            }
            let (_, private_key) = publisher_keypair(&read_seed(seed_file)?);
            write(out, &Signed::sign(body, &private_key).encode())
        }
        ["revoke", seed_file, key_id, sequence, out, entries @ ..] => {
            let mut list = Revocations {
                key_id: parse(key_id, "key_id")?,
                sequence: parse(sequence, "sequence")?,
                ..Revocations::default()
            };
            for entry in entries {
                match entry.split_once(':') {
                    Some(("key", id)) => list.revoked_keys.push(parse(id, "key")?),
                    Some(("program", hex)) => list.revoked_programs.push(ProgramId(unhex32(hex)?)),
                    Some(("module", path)) => {
                        list.revoked_programs.push(ProgramId::of(&read(path)?))
                    }
                    _ => return Err(format!("bad revocation entry {entry:?}")),
                }
            }
            let body = list.encode();
            if Revocations::decode(&body).as_ref() != Some(&list) {
                return Err("too many revocations (at most 64 of each kind)".into());
            }
            let (_, private_key) = publisher_keypair(&read_seed(seed_file)?);
            write(out, &Signed::sign(body, &private_key).encode())
        }
        ["verify", public_file, key_id, signed_file, module @ ..] => {
            let key = PublisherKey {
                id: parse(key_id, "key_id")?,
                public_key: read_public(public_file)?,
            };
            let signed = Signed::decode(&read(signed_file)?).ok_or("not a signed file")?;
            if let Ok(manifest) = Manifest::verify(&signed, std::slice::from_ref(&key)) {
                println!("valid manifest: {manifest:?}");
                if let [module] = module {
                    if ProgramId::of(&read(module)?) != manifest.program_id {
                        return Err(format!("manifest is not for {module}"));
                    }
                    println!("matches {module}");
                }
                return Ok(());
            }
            let list = Revocations::verify(&signed, std::slice::from_ref(&key))
                .map_err(|e| format!("not validly signed by this key: {e:?}"))?;
            println!("valid revocation list: {list:?}");
            Ok(())
        }
        _ => Err("usage: see the doc comment at the top of seed_vault_wasm_sign.rs".into()),
    }
}

fn parse_caps(caps: &str) -> Result<Capabilities> {
    let mut out = Capabilities::default();
    for cap in caps.split(',') {
        match cap {
            "none" => {}
            "account-entropy" => out.account_entropy = true,
            "human-sign" => out.human_sign = true,
            _ => return Err(format!("unknown capability {cap:?}")),
        }
    }
    Ok(out)
}

fn parse<T: std::str::FromStr>(text: &str, what: &str) -> Result<T> {
    text.parse().map_err(|_| format!("bad {what} {text:?}"))
}

fn read(path: &str) -> Result<Vec<u8>> {
    fs::read(path).map_err(|e| format!("reading {path}: {e}"))
}

fn write(path: &str, bytes: &[u8]) -> Result<()> {
    fs::write(path, bytes).map_err(|e| format!("writing {path}: {e}"))
}

fn unhex32(text: &str) -> Result<[u8; 32]> {
    let text = text.trim();
    if text.len() != 64 {
        return Err(format!("expected 64 hex digits, got {}", text.len()));
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).map_err(|_| "bad hex")?;
    }
    Ok(out)
}

fn read_seed(path: &str) -> Result<[u8; 32]> {
    unhex32(&String::from_utf8(read(path)?).map_err(|_| "seed file isn't text")?)
}

fn write_seed(path: &str, seed: &[u8; 32]) -> Result<()> {
    use std::fmt::Write;
    let mut hex = String::new();
    for byte in seed {
        let _ = write!(hex, "{byte:02x}");
    }
    write(path, format!("{hex}\n").as_bytes())
}

fn read_public(path: &str) -> Result<PublicKey> {
    let bytes = read(path)?;
    let mut key = PublicKey::default();
    if bytes.len() != key.0.len() {
        return Err(format!(
            "{path}: expected a {}-byte public key",
            key.0.len()
        ));
    }
    key.0.copy_from_slice(&bytes);
    Ok(key)
}

fn write_public(path: &str, seed: &[u8; 32]) -> Result<()> {
    let (public_key, _) = publisher_keypair(seed);
    write(path, &public_key.0)
}

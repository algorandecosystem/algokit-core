use std::collections::HashMap;

use crate::{Package, get_repo_root, run};
use color_eyre::eyre::Result;

pub fn build(package: &Package) -> Result<()> {
    let gradle_root_dir = get_repo_root()
        .join("packages")
        .join("android")
        .join(package.to_string());

    let so_file_output_dir = gradle_root_dir.join("src").join("main").join("jniLibs");
    let kotlin_out_dir = gradle_root_dir.join("src").join("main").join("kotlin");

    // Build for Android targets
    let rustflags =
        "-C link-arg=-Wl,-z,max-page-size=16384 -C link-arg=-Wl,-z,common-page-size=16384";
    let cargo_build_cmd = format!(
        "cargo ndk -o {} --manifest-path {} -t armeabi-v7a -t arm64-v8a -t x86_64 build --release",
        so_file_output_dir.display(),
        package.crate_manifest().display()
    );

    let env = HashMap::from([("RUSTFLAGS".to_string(), rustflags.to_string())]);
    run(&cargo_build_cmd, None, Some(env))?;

    // `cargo ndk -o` copies every `.so` in the shared `target/<triple>/release`
    // dir, including other FFI crates' libraries left over from earlier
    // builds. Keep only this package's own library.
    let own_lib = format!("lib{}.so", package.crate_name());
    for abi_dir in std::fs::read_dir(&so_file_output_dir)? {
        let abi_dir = abi_dir?.path();
        if !abi_dir.is_dir() {
            continue;
        }
        for lib in std::fs::read_dir(&abi_dir)? {
            let lib = lib?.path();
            if lib.file_name().is_some_and(|name| name != own_lib.as_str()) {
                println!("Removing unrelated library {}", lib.display());
                std::fs::remove_file(&lib)?;
            }
        }
    }

    // Build for host platform (for running unit tests)
    println!("Building for host platform to enable unit tests...");
    let host_build_cmd = format!(
        "cargo build --manifest-path {} --release",
        package.crate_manifest().display()
    );
    run(&host_build_cmd, None, None)?;

    // Copy host library to test resources so JNA can find it
    let test_resources_dir = gradle_root_dir.join("src").join("test").join("resources");

    std::fs::create_dir_all(&test_resources_dir)?;

    let host_dylib = package.dylib(None);
    let test_lib_dest = test_resources_dir.join(host_dylib.file_name().unwrap());

    std::fs::copy(&host_dylib, &test_lib_dest)?;
    println!(
        "Copied host library from {} to {} for unit tests",
        host_dylib.display(),
        test_lib_dest.display()
    );

    if kotlin_out_dir.exists() {
        std::fs::remove_dir_all(&kotlin_out_dir)?;
    }

    run(
        &format!(
            "cargo run -p uniffi-bindgen generate --library {} --language kotlin --out-dir {}",
            package.dylib(Some("aarch64-linux-android")).display(),
            kotlin_out_dir.display()
        ),
        None,
        None,
    )?;

    run(
        "sh -c './gradlew assembleRelease'",
        Some(&gradle_root_dir),
        None,
    )?;

    Ok(())
}

use crate::{Package, get_repo_root, run};
use color_eyre::eyre::Result;

pub fn build(package: &Package) -> Result<()> {
    let so_file_output_dir = get_repo_root()
        .join("packages")
        .join("android")
        .join(package.to_string())
        .join(package.to_string().replace("algokit_", ""))
        .join("src")
        .join("main")
        .join("jniLibs");

    let kotlin_out_dir = get_repo_root()
        .join("packages")
        .join("android")
        .join(package.to_string())
        .join(package.to_string().replace("algokit_", ""))
        .join("src")
        .join("main")
        .join("kotlin");

    let cargo_build_cmd = format!(
        "cargo ndk -o {} --manifest-path {} -t armeabi-v7a -t arm64-v8a -t x86_64 build --release",
        so_file_output_dir.display(),
        package.crate_manifest().display()
    );

    run(&cargo_build_cmd, None, None)?;

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

    Ok(())
}

use crate::{Package, get_repo_root, run};
use color_eyre::eyre::{Result, eyre};

/// Returns the resource-path segment (e.g. "darwin-aarch64") that JNA uses to locate
/// natively-bundled libraries on the classpath for the current host platform. This
/// mirrors `com.sun.jna.Platform.RESOURCE_PREFIX`, and lets us ship a self-contained
/// JAR that JNA will automatically extract the native library from at runtime,
/// without consumers needing to set `jna.library.path` themselves.
fn jna_resource_prefix() -> Result<&'static str> {
    let prefix = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "darwin-aarch64",
        ("macos", "x86_64") => "darwin-x86-64",
        ("linux", "aarch64") => "linux-aarch64",
        ("linux", "x86_64") => "linux-x86-64",
        ("windows", "x86_64") => "win32-x86-64",
        ("windows", "aarch64") => "win32-aarch64",
        (os, arch) => return Err(eyre!("Unsupported host platform for JVM build: {os}-{arch}")),
    };

    Ok(prefix)
}

pub fn build(package: &Package) -> Result<()> {
    let gradle_root_dir = get_repo_root()
        .join("packages")
        .join("kotlin")
        .join(package.to_string());

    let kotlin_out_dir = gradle_root_dir.join("src").join("main").join("kotlin");
    let resources_dir = gradle_root_dir
        .join("src")
        .join("main")
        .join("resources")
        .join(jna_resource_prefix()?);
    let test_resources_dir = gradle_root_dir.join("src").join("test").join("resources");

    // Build the native library for the host platform. Unlike Android there's no
    // NDK/cross-compilation step here - a desktop/server JVM app just loads whatever
    // native library matches the machine it's running on.
    println!("Building native library for host platform...");
    let host_build_cmd = format!(
        "cargo build --manifest-path {} --release",
        package.crate_manifest().display()
    );
    run(&host_build_cmd, None, None)?;

    // Generate the Kotlin bindings. This is the exact same JNA-based Kotlin that
    // powers the Android package - it has no Android-specific dependencies, so it
    // works unmodified on a desktop/server JVM.
    if kotlin_out_dir.exists() {
        std::fs::remove_dir_all(&kotlin_out_dir)?;
    }

    run(
        &format!(
            "cargo run -p uniffi-bindgen generate --library {} --language kotlin --out-dir {}",
            package.dylib(None).display(),
            kotlin_out_dir.display()
        ),
        None,
        None,
    )?;

    // Bundle the host native library inside the JAR's resources, under the path
    // JNA looks for automatically.
    std::fs::create_dir_all(&resources_dir)?;
    let host_dylib = package.dylib(None);
    std::fs::copy(
        &host_dylib,
        resources_dir.join(host_dylib.file_name().unwrap()),
    )?;

    // Also copy it to test resources so unit tests can load it via JNA.
    std::fs::create_dir_all(&test_resources_dir)?;
    std::fs::copy(
        &host_dylib,
        test_resources_dir.join(host_dylib.file_name().unwrap()),
    )?;

    run("sh -c './gradlew jar'", Some(&gradle_root_dir), None)?;

    println!(
        "Built JAR for {} containing the native library for this host platform only ({}). \
         For a multi-platform release, run this build on each target OS/arch and merge the \
         resulting `src/main/resources/*` directories before publishing.",
        package,
        jna_resource_prefix()?
    );

    Ok(())
}

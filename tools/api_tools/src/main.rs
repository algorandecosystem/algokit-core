use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Output;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use duct::cmd;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about = "API development tools", long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Test the OAS generator
    #[command(name = "test-oas")]
    TestOas,
    /// Format the OAS generator code
    #[command(name = "format-oas")]
    FormatOas,
    /// Lint and type-check the OAS generator
    #[command(name = "lint-oas")]
    LintOas,
    /// Format generated Rust code
    #[command(name = "format-algod")]
    FormatAlgod,
    /// Format generated indexer Rust code
    #[command(name = "format-indexer")]
    FormatIndexer,
    /// Generate algod API client
    #[command(name = "generate-algod")]
    GenerateAlgod,
    /// Generate indexer API client
    #[command(name = "generate-indexer")]
    GenerateIndexer,
    /// Generate both algod and indexer API clients
    #[command(name = "generate-all")]
    GenerateAll,
    /// Generate TypeScript algod client
    #[command(name = "generate-ts-algod")]
    GenerateTsAlgod,
    /// Generate TypeScript indexer client
    #[command(name = "generate-ts-indexer")]
    GenerateTsIndexer,
    /// Generate both TypeScript clients (algod and indexer)
    #[command(name = "generate-ts-all")]
    GenerateTsAll,
    /// Convert OpenAPI specifications (both algod and indexer)
    #[command(name = "convert-openapi")]
    ConvertOpenapi,
    /// Convert algod OpenAPI specification only
    #[command(name = "convert-algod")]
    ConvertAlgod,
    /// Convert indexer OpenAPI specification only
    #[command(name = "convert-indexer")]
    ConvertIndexer,
}

fn get_repo_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = Path::new(manifest_dir)
        .parent() // tools/
        .unwrap()
        .parent() // repo root
        .unwrap();

    PathBuf::from(repo_root)
}

fn run(
    command_str: &str,
    dir: Option<&Path>,
    env_vars: Option<HashMap<String, String>>,
) -> Result<Output> {
    let parsed_command: Vec<String> = shlex::Shlex::new(command_str).collect();

    let working_dir = get_repo_root().join(dir.unwrap_or(Path::new("")));
    let mut command = cmd(&parsed_command[0], &parsed_command[1..])
        .dir(&working_dir)
        .stderr_to_stdout();

    if let Some(env_vars) = env_vars {
        for (key, value) in &env_vars {
            command = command.env(key, value);
        }
    }

    Ok(command.run()?)
}

fn execute_command(command: &Commands) -> Result<()> {
    fn clean_ts_package_with_preserve(rel_dir: &str, preserve: &[&str]) -> Result<()> {
        let root = get_repo_root();
        let pkg_dir = root.join(rel_dir);
        if !pkg_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&pkg_dir)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if preserve.iter().any(|p| *p == name_str) {
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                // Remove entire directory tree
                fs::remove_dir_all(&path)?;
            } else {
                // Remove file
                fs::remove_file(&path)?;
            }
        }
        Ok(())
    }
    fn clean_ts_package(rel_dir: &str) -> Result<()> {
        let default_preserve: &[&str] = &["tests", "node_modules"];
        clean_ts_package_with_preserve(rel_dir, default_preserve)
    }
    match command {
        Commands::TestOas => {
            run("uv run pytest", Some(Path::new("api/oas_generator")), None)?;
        }
        Commands::FormatOas => {
            run(
                "uv run ruff format",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
        }
        Commands::LintOas => {
            run(
                "uv run ruff check",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            run(
                "uv run mypy rust_oas_generator",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
        }
        Commands::FormatAlgod => {
            run(
                "cargo fmt --manifest-path Cargo.toml -p algod_client",
                None,
                None,
            )?;
        }
        Commands::FormatIndexer => {
            run(
                "cargo fmt --manifest-path Cargo.toml -p indexer_client",
                None,
                None,
            )?;
        }
        Commands::GenerateAlgod => {
            // Generate the client
            run(
                "uv run python -m rust_oas_generator.cli ../specs/algod.oas3.json --output ../../crates/algod_client/ --package-name algod_client --description \"API client for algod interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format the generated code
            run(
                "cargo fmt --manifest-path Cargo.toml -p algod_client",
                None,
                None,
            )?;
        }
        Commands::GenerateIndexer => {
            // Generate the client
            run(
                "uv run python -m rust_oas_generator.cli ../specs/indexer.oas3.json --output ../../crates/indexer_client/ --package-name indexer_client --description \"API client for indexer interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format the generated code
            run(
                "cargo fmt --manifest-path Cargo.toml -p indexer_client",
                None,
                None,
            )?;
        }
        Commands::GenerateAll => {
            // Generate algod client
            run(
                "uv run python -m rust_oas_generator.cli ../specs/algod.oas3.json --output ../../crates/algod_client/ --package-name algod_client --description \"API client for algod interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Generate indexer client
            run(
                "uv run python -m rust_oas_generator.cli ../specs/indexer.oas3.json --output ../../crates/indexer_client/ --package-name indexer_client --description \"API client for indexer interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format both generated codes
            run(
                "cargo fmt --manifest-path Cargo.toml -p algod_client",
                None,
                None,
            )?;
            run(
                "cargo fmt --manifest-path Cargo.toml -p indexer_client",
                None,
                None,
            )?;
        }
        Commands::GenerateTsAlgod => {
            // Clean package directory but preserve manual tests and node_modules
            clean_ts_package("packages/typescript/algod_client")?;
            // Generate the TypeScript client (algod)
            run(
                "uv run python -m ts_oas_generator.cli ../specs/algod.oas3.json --output ../../packages/typescript/algod_client/ --package-name algod_client --description \"TypeScript client for algod interaction.\" --verbose",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format generated code
            run(
                "npx --yes prettier --write .",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
            // Install dependencies
            run(
                "bun install",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
            // Build the generated package
            run(
                "bun run build",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
        }
        Commands::GenerateTsIndexer => {
            // Clean package directory but preserve manual tests and node_modules
            clean_ts_package("packages/typescript/indexer_client")?;
            // Generate the TypeScript client (indexer)
            run(
                "uv run python -m ts_oas_generator.cli ../specs/indexer.oas3.json --output ../../packages/typescript/indexer_client/ --package-name indexer_client --description \"TypeScript client for indexer interaction.\" --verbose",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format generated code
            run(
                "npx --yes prettier --write .",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
            // Install dependencies
            run(
                "bun install",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
            // Build the generated package
            run(
                "bun run build",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
        }
        Commands::GenerateTsAll => {
            // Clean package directories while preserving manual tests and node_modules
            clean_ts_package("packages/typescript/algod_client")?;
            clean_ts_package("packages/typescript/indexer_client")?;
            // Generate both TypeScript clients
            run(
                "uv run python -m ts_oas_generator.cli ../specs/algod.oas3.json --output ../../packages/typescript/algod_client/ --package-name algod_client --description \"TypeScript client for algod interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            run(
                "uv run python -m ts_oas_generator.cli ../specs/indexer.oas3.json --output ../../packages/typescript/indexer_client/ --package-name indexer_client --description \"TypeScript client for indexer interaction.\"",
                Some(Path::new("api/oas_generator")),
                None,
            )?;
            // Format both generated packages
            run(
                "npx --yes prettier --write .",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
            run(
                "npx --yes prettier --write .",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
            // Install dependencies for both packages
            run(
                "bun install",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
            run(
                "bun install",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
            // Build both packages
            run(
                "bun run build",
                Some(Path::new("packages/typescript/algod_client")),
                None,
            )?;
            run(
                "bun run build",
                Some(Path::new("packages/typescript/indexer_client")),
                None,
            )?;
        }
        Commands::ConvertOpenapi => {
            run(
                "bun scripts/convert-openapi.ts",
                Some(Path::new("api")),
                None,
            )?;
        }
        Commands::ConvertAlgod => {
            run(
                "bun scripts/convert-openapi.ts --algod-only",
                Some(Path::new("api")),
                None,
            )?;
        }
        Commands::ConvertIndexer => {
            run(
                "bun scripts/convert-openapi.ts --indexer-only",
                Some(Path::new("api")),
                None,
            )?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_BACKTRACE").is_err() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "full");
        }
    }

    let args = Args::parse();
    execute_command(&args.command)?;

    Ok(())
}

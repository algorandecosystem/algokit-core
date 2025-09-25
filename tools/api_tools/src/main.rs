use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
use duct::cmd;
use once_cell::sync::OnceCell;

const DEFAULT_TS_PRESERVE: &[&str] = &[
    "__tests__",
    "tests",
    "node_modules",
    "eslint.config.mjs",
    "package.json",
    "README.md",
    "rolldown.config.ts",
    "tsconfig.json",
    "tsconfig.build.json",
    "tsconfig.test.json",
];

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

fn repo_root() -> &'static Path {
    static ROOT: OnceCell<PathBuf> = OnceCell::new();

    ROOT.get_or_init(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|dir| dir.parent())
            .map(Path::to_path_buf)
            .expect("invalid repository layout")
    })
    .as_path()
}

fn run(command_str: &str, dir: Option<&Path>, env_vars: Option<&[(&str, &str)]>) -> Result<()> {
    let mut tokens = shlex::Shlex::new(command_str);
    let program = tokens
        .next()
        .ok_or_else(|| eyre!("command string must not be empty"))?;
    let args: Vec<_> = tokens.collect();

    let working_dir = dir
        .map(|path| repo_root().join(path))
        .unwrap_or_else(|| repo_root().to_path_buf());

    let mut expr = cmd(program, args).dir(&working_dir).stderr_to_stdout();

    if let Some(vars) = env_vars {
        expr = vars
            .iter()
            .fold(expr, |cmd, (key, value)| cmd.env(key, value));
    }

    expr.run()?;
    Ok(())
}

fn clean_ts_package_with_preserve(rel_dir: &str, preserve: &[&str]) -> Result<()> {
    let pkg_dir = repo_root().join(rel_dir);
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
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn clean_ts_package(rel_dir: &str) -> Result<()> {
    clean_ts_package_with_preserve(rel_dir, DEFAULT_TS_PRESERVE)
}

#[derive(Clone, Copy)]
struct TsClientConfig {
    spec: &'static str,
    output_rel: &'static str,
    package_name: &'static str,
    description: &'static str,
}

const ALGOD_TS_CLIENT: TsClientConfig = TsClientConfig {
    spec: "algod",
    output_rel: "packages/typescript/algod_client",
    package_name: "algod_client",
    description: "TypeScript client for algod interaction.",
};

const INDEXER_TS_CLIENT: TsClientConfig = TsClientConfig {
    spec: "indexer",
    output_rel: "packages/typescript/indexer_client",
    package_name: "indexer_client",
    description: "TypeScript client for indexer interaction.",
};

fn generate_ts_client(config: &TsClientConfig, verbose: bool) -> Result<()> {
    clean_ts_package(config.output_rel)?;

    let mut command = format!(
        "uv run python -m ts_oas_generator.cli ../specs/{}.oas3.json --output ../../{}/ --package-name {} --description \"{}\"",
        config.spec, config.output_rel, config.package_name, config.description
    );
    if verbose {
        command.push_str(" --verbose");
    }
    run(&command, Some(Path::new("api/oas_generator")), None)?;

    run(
        "npx --yes prettier --write src",
        Some(Path::new(config.output_rel)),
        None,
    )?;
    run("npm run build", Some(Path::new(config.output_rel)), None)?;
    run("npm run test", Some(Path::new(config.output_rel)), None)?;

    Ok(())
}

fn execute_command(command: &Commands) -> Result<()> {
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
            generate_ts_client(&ALGOD_TS_CLIENT, true)?;
        }
        Commands::GenerateTsIndexer => {
            generate_ts_client(&INDEXER_TS_CLIENT, true)?;
        }
        Commands::GenerateTsAll => {
            generate_ts_client(&ALGOD_TS_CLIENT, false)?;
            generate_ts_client(&INDEXER_TS_CLIENT, false)?;
        }
        Commands::ConvertOpenapi => {
            run("npm run convert-openapi", Some(Path::new("api")), None)?;
        }
        Commands::ConvertAlgod => {
            run(
                "npm run convert-openapi -- --algod-only",
                Some(Path::new("api")),
                None,
            )?;
        }
        Commands::ConvertIndexer => {
            run(
                "npm run convert-openapi -- --indexer-only",
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

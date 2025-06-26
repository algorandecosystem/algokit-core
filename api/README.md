# Algorand API Tools

This package contains tools for working with the Algorand API specifications and generating Rust HTTP client libraries using a custom Jinja2-based generator.

## Prerequisites

- [Bun](https://bun.sh/) - JavaScript runtime and package manager  
- [Python 3.12+](https://www.python.org/) - Required for the custom OAS generator
- [uv](https://docs.astral.sh/uv/) - Python package manager
- [Rust](https://rustup.rs/) - Required for compiling generated clients

## Setup

```bash
# Install JavaScript dependencies
bun install

# Install Python dependencies for the OAS generator
cd oas_generator
uv install
```

## Available Scripts

> NOTE: These scripts must be run inside the `./api` directory.

### Convert OpenAPI 2.0 to OpenAPI 3.0

Converts the Algod OpenAPI 2.0 spec to OpenAPI 3.0:

```bash
bun run convert-openapi
```

The converted spec will be available at `specs/algod.oas3.json`.

### Generate Rust API Clients

Generates Rust API clients using the custom Jinja2-based generator:

```bash
bun run generate:algod_client
```

The generated Rust client will be available at `../crates/algod_client/`.

### Development Scripts

```bash
# Test the OAS generator
bun run test:oas_generator

# Format the OAS generator code
bun run format:oas_generator

# Lint and type-check the OAS generator
bun run lint:oas_generator

# Format generated Rust code
bun run format:algod_client
```

## Custom Rust OAS Generator

The project uses a custom Jinja2-based generator located in `oas_generator/` that creates optimized Rust API clients from OpenAPI 3.x specifications.

### Features

- **Complete Rust Client Generation**: APIs, models, and configuration
- **Msgpack Support**: Automatic detection and handling of binary encoding
- **Signed Transactions**: Algorand-specific vendor extension support (`x-algokit-signed-txn`)
- **Type Safety**: Comprehensive OpenAPI to Rust type mapping
- **Template-based**: Customizable Jinja2 templates for code generation

### Generated Structure

The generator creates a complete Rust crate with the following structure:

```
crates/algod_client/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── apis/
    │   ├── mod.rs
    │   ├── client.rs
    │   └── {endpoint}.rs
    └── models/
        ├── mod.rs
        └── {model}.rs
```

## OpenAPI Specs for Algorand APIs

### Algod

The `algod.oas2.json` is taken directly from [go-algorand](https://github.com/algorand/go-algorand/blob/master/daemon/algod/api/algod.oas2.json). The script under [scripts/convert-openapi.ts](scripts/convert-openapi.ts) is used to convert the spec to OpenAPI 3.0 via [swagger converter](https://converter.swagger.io/) endpoint.

The current approach is to manually edit and tweak the algod.oas2.json fixing known issues from the go-algorand spec, then use the custom Rust OAS generator to generate clients from the v3 spec. OpenAPI v3 is preferred for client generation as it offers enhanced schema features, better component reusability, and improved type definitions compared to v2.

## Generator Configuration

The custom Rust generator is configured with:

- **Package name**: `algod_client`
- **Msgpack detection**: Automatic handling of binary-encoded fields
- **Algorand extensions**: Support for signed transaction vendor extensions
- **Type safety**: Complete OpenAPI to Rust type mapping
- **Error handling**: Comprehensive error types and response handling

For detailed information about the generator architecture and customization options, see [`oas_generator/ARCHITECTURE.md`](oas_generator/ARCHITECTURE.md).

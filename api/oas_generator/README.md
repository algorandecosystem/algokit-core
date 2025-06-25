# Rust OpenAPI Client Generator

A sophisticated Jinja2-based generator that produces high-quality Rust API clients from OpenAPI 3.x specifications. This generator is designed for maintainability, LLM-friendliness, and follows modern Rust best practices.

## 🚀 Features

- **Rust-Native Output**: Generates idiomatic Rust code that follows community conventions
- **Type Safety**: Comprehensive type annotations and error handling
- **Async/Await Support**: Modern async Rust patterns with reqwest
- **Serde Integration**: Automatic serialization/deserialization with serde
- **Modular Architecture**: Clean separation between models, APIs, and configuration
- **Individual Endpoint Files**: Each API endpoint generates its own file for better organization
- **Template-Driven**: Fully customizable through Jinja2 templates
- **LLM-Optimized**: Clear, readable code structure optimized for AI analysis
- **Reference-Based**: Uses existing Rust clients as structural reference

## 📦 Installation

### Prerequisites

- Python 3.8 or higher
- [uv](https://docs.astral.sh/uv/) (recommended) or pip

### Using uv (Recommended)

```bash
# Install directly from the project directory
cd rust_oas_generator
uv sync

# Run the generator
uv run rust_oas_generator spec.json

# Or install in development mode with dev dependencies
uv sync --extra dev
```

### Using pip

```bash
pip install .
```

- Jinja2 template engine

### Setup

```bash
# Clone or download the generator
git clone <repository-url>
cd rust_client_generator

# Install dependencies
pip install jinja2

# Run the generator
python generate_rust_client.py
```

## 🔧 Usage

### Basic Generation

```python
from rust_client_generator.parser import OASParser
from rust_client_generator.generator import RustCodeGenerator

# Parse OpenAPI specification
parser = OASParser()
spec = parser.parse_file("path/to/openapi.json")

# Generate Rust client
generator = RustCodeGenerator()
files = generator.generate_client(spec, "output_directory", "my_api_client")

# Write files to disk
from rust_client_generator.utils.file_utils import write_files_to_disk
write_files_to_disk(files)
```

### Command Line Usage

```bash
# Generate client from OpenAPI spec
python generate_rust_client.py

# The script will:
# 1. Parse the OpenAPI specification
# 2. Generate Rust client code
# 3. Write files to generated_rust_client/
```

## 📁 Generated Structure

The generator produces a complete Rust crate with the following structure:

```
my_api_client/
├── Cargo.toml              # Rust project configuration
├── README.md               # Generated client documentation
└── src/
    ├── lib.rs              # Library root
    ├── apis/               # API operations
    │   ├── mod.rs          # API module exports and re-exports
    │   ├── configuration.rs # Client configuration
    │   ├── health_check.rs # Individual endpoint file
    │   ├── get_status.rs   # Individual endpoint file
    │   └── *.rs            # One file per API endpoint
    └── models/             # Data models
        ├── mod.rs          # Model exports
        └── *.rs            # Individual model files
```

## 🏗️ Architecture

### Design Principles

1. **Reference-Based Generation**: Uses existing Rust clients as structural templates
2. **Type Safety First**: Comprehensive error types and response handling
3. **Async by Default**: Modern async/await patterns throughout
4. **Serde Integration**: Automatic JSON serialization/deserialization
5. **Modular Design**: Clear separation of concerns

### Key Components

#### Parser Module (`parser/`)

- **OASParser**: Comprehensive OpenAPI 3.x specification parser
- **Type Resolution**: Converts OpenAPI types to Rust types
- **Reference Handling**: Resolves $ref references and circular dependencies
- **Validation**: Ensures specification completeness

#### Generator Module (`generator/`)

- **RustTemplateEngine**: Jinja2-based template rendering engine
- **RustCodeGenerator**: Orchestrates the complete generation process
- **Custom Filters**: Rust-specific naming and type conversion filters
- **Template Management**: Loads and renders all necessary templates

#### Templates (`templates/`)

- **Base Templates**: Core library files (lib.rs, Cargo.toml, README.md)
- **API Templates**: Operation and error type generation
- **Model Templates**: Struct and enum generation
- **Configuration**: Client setup and authentication

## 🎯 Generated Code Features

### API Operations

Each API operation generates:

```rust
/// Operation documentation from OpenAPI spec
pub async fn operation_name(
    configuration: &configuration::Configuration,
    param1: &str,
    param2: Option<i32>,
) -> Result<ResponseType, Error<OperationError>> {
    // Implementation with proper error handling
}
```

### Error Handling

Comprehensive error types for each operation:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperationError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status500(models::ErrorResponse),
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}
```

### Data Models

Type-safe structs with serde integration:

```rust
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelName {
    #[serde(rename = "field_name")]
    pub field_name: String,
    
    #[serde(rename = "optional_field", skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<i32>,
}
```

### Configuration

Flexible client configuration:

```rust
#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub api_key: Option<ApiKey>,
    // ... other auth options
}
```

## 🔧 Customization

### Template Modification

The generator uses Jinja2 templates that can be easily customized:

1. **API Templates** (`templates/apis/`): Modify operation generation
2. **Model Templates** (`templates/models/`): Customize struct generation
3. **Base Templates** (`templates/base/`): Change project structure

### Custom Filters

Add custom Jinja2 filters in `RustTemplateEngine`:

```python
def _register_filters(self):
    self.env.filters['custom_filter'] = self._custom_filter

def _custom_filter(self, value):
    # Custom transformation logic
    return transformed_value
```

### Type Mapping

Customize OpenAPI to Rust type conversion in `rust_type_from_openapi()`:

```python
type_mapping = {
    'string': {
        'date': 'chrono::NaiveDate',  # Custom date handling
        'date-time': 'chrono::DateTime<chrono::Utc>',
    }
}
```

## 📊 Comparison with Reference

The generator produces code that closely matches the structure of existing Rust OpenAPI clients:

| Feature | Reference Client | Generated Client | Status |
|---------|------------------|------------------|---------|
| Project Structure | ✅ | ✅ | Matches |
| Error Types | ✅ | ✅ | Matches |
| Async Operations | ✅ | ✅ | Matches |
| Serde Integration | ✅ | ✅ | Matches |
| Configuration | ✅ | ✅ | Matches |
| Type Safety | ✅ | ✅ | Matches |

## 🧪 Testing

### Generated Client Testing

```rust
#[tokio::test]
async fn test_api_operation() {
    let config = configuration::Configuration::new();
    let result = default_api::some_operation(&config, "param").await;
    assert!(result.is_ok());
}
```

### Generator Testing

```python
def test_generation():
    parser = OASParser()
    spec = parser.parse_file("test_spec.json")
    
    generator = RustCodeGenerator()
    files = generator.generate_client(spec, "test_output", "test_client")
    
    assert len(files) > 0
    assert "src/lib.rs" in [Path(f).name for f in files.keys()]
```

## 🚀 Future Enhancements

### Planned Features

1. **Enhanced Type System**: Support for more complex OpenAPI schemas
2. **Authentication Middleware**: Built-in auth handling patterns
3. **CLI Tool**: Command-line interface for easy generation
4. **Multiple Output Formats**: Support for different Rust project structures
5. **Validation**: Runtime request/response validation
6. **Documentation**: Enhanced rustdoc generation

### UniFFI Compatibility

The generated code is designed with UniFFI compatibility in mind:

- Clean interface definitions
- Minimal external dependencies
- Clear error handling patterns
- Serializable data structures

### WASM Bindgen Support

Future versions will support WebAssembly compilation:

- Browser-compatible HTTP clients
- JavaScript interop patterns
- Minimal runtime dependencies

## 📝 Contributing

### Development Setup

1. Clone the repository
2. Install Python dependencies: `pip install jinja2`
3. Run tests: `python -m pytest tests/`
4. Generate example client: `python generate_rust_client.py`

### Adding Templates

1. Create new template in appropriate directory
2. Update template engine to load new template
3. Add template context variables
4. Test generation with sample OpenAPI spec

### Reporting Issues

Please report issues with:

- OpenAPI specification that caused the issue
- Generated code that's incorrect
- Expected vs actual behavior
- Steps to reproduce

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- OpenAPI Initiative for the specification standard
- Rust community for excellent HTTP and serialization libraries
- Jinja2 project for the powerful templating engine
- Reference Rust client implementations for structural guidance

---

**Generated by**: Rust OpenAPI Client Generator  
**Version**: 1.0.0  
**Rust Support**: 2021 Edition  
**OpenAPI Support**: 3.x specifications

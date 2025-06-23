# Architecture Documentation

## Rust OpenAPI Client Generator

This document provides detailed architectural information about the Rust OpenAPI Client Generator, including design decisions, implementation details, and extensibility patterns.

## 🏗️ System Architecture

### High-Level Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   OpenAPI       │    │   Generator     │    │   Rust Client   │
│ Specification   │───▶│    Engine       │───▶│     Code        │
│   (JSON/YAML)   │    │  (Jinja2 +      │    │  (Cargo Crate)  │
└─────────────────┘    │   Templates)    │    └─────────────────┘
                       └─────────────────┘
```

### Core Components

#### 1. Parser Module (`parser/`)

**Purpose**: Parse and validate OpenAPI 3.x specifications

**Key Classes**:
- `OASParser`: Main parsing orchestrator
- `ParsedSpec`: Structured representation of the specification
- `Operation`: Individual API operation representation
- `Schema`: Data model representation
- `Parameter`: Operation parameter representation
- `Response`: Operation response representation

**Design Decisions**:
- **Immutable Data Structures**: Use dataclasses for parsed representations
- **Type Safety**: Strong typing throughout the parsing pipeline
- **Reference Resolution**: Handle $ref references and circular dependencies
- **Rust Type Mapping**: Convert OpenAPI types to appropriate Rust types

#### 2. Generator Module (`generator/`)

**Purpose**: Generate Rust code using Jinja2 templates

**Key Classes**:
- `RustTemplateEngine`: Template rendering and filter management
- `RustCodeGenerator`: Code generation orchestration

**Design Decisions**:
- **Template-Driven**: All code generation through Jinja2 templates
- **Custom Filters**: Rust-specific naming and type conversion
- **Modular Generation**: Separate generation phases for different components
- **Context Management**: Rich context objects for template rendering

#### 3. Template System (`templates/`)

**Purpose**: Define the structure and content of generated Rust code

**Template Categories**:
- **Base Templates**: Core library structure (lib.rs, Cargo.toml, etc.)
- **API Templates**: Operation and error type generation
- **Model Templates**: Struct and enum generation

**Design Decisions**:
- **Reference-Based**: Templates mirror the structure of existing Rust clients
- **Extensible**: Easy to modify and extend templates
- **Consistent**: Uniform code style and patterns across all generated code

## 🎯 Design Principles

### 1. Reference-Based Generation

**Principle**: Use existing, well-structured Rust clients as templates for generation.

**Implementation**:
- Analyzed the provided Rust client structure
- Extracted common patterns and conventions
- Replicated the same file organization and code patterns
- Maintained compatibility with existing Rust ecosystem tools

**Benefits**:
- Familiar structure for Rust developers
- Proven patterns and conventions
- Easier integration with existing projects
- Reduced learning curve

### 2. Type Safety First

**Principle**: Generate type-safe Rust code with comprehensive error handling.

**Implementation**:
- Strong typing for all API operations
- Dedicated error enums for each operation
- Proper Result<T, E> return types
- Serde integration for serialization safety

**Benefits**:
- Compile-time error detection
- Clear error handling patterns
- IDE support and autocompletion
- Runtime safety guarantees

### 3. Async by Default

**Principle**: Modern async/await patterns throughout the generated code.

**Implementation**:
- All API operations are async functions
- Uses reqwest for HTTP client (async by default)
- Proper async error handling
- Compatible with tokio runtime

**Benefits**:
- Non-blocking I/O operations
- Better resource utilization
- Modern Rust patterns
- Scalable applications

### 4. Template-Driven Architecture

**Principle**: Separate code generation logic from output format through templates.

**Implementation**:
- Jinja2 templates for all generated code
- Custom filters for Rust-specific transformations
- Rich context objects for template rendering
- Easy template customization and extension

**Benefits**:
- Easy customization without code changes
- Clear separation of concerns
- Maintainable generation logic
- Extensible for new features

## 🔧 Implementation Details

### Type System Mapping

The generator maps OpenAPI types to Rust types using the following strategy:

```python
type_mapping = {
    'string': {
        None: 'String',
        'date': 'String',        # Could be chrono::NaiveDate
        'date-time': 'String',   # Could be chrono::DateTime
        'byte': 'String',        # Base64 encoded
        'binary': 'Vec<u8>',
    },
    'integer': {
        None: 'i32',
        'int32': 'i32',
        'int64': 'i64',
    },
    'number': {
        None: 'f64',
        'float': 'f32',
        'double': 'f64',
    },
    'boolean': {
        None: 'bool',
    },
    'array': {
        None: 'Vec<T>',  # Where T is the items type
    },
    'object': {
        None: 'serde_json::Value',  # Generic object
    }
}
```

### Error Handling Strategy

Each API operation generates a dedicated error enum:

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

**Design Rationale**:
- **Type Safety**: Each operation has its own error type
- **Comprehensive**: Covers all possible HTTP status codes
- **Extensible**: UnknownValue variant for unexpected responses
- **Serializable**: Can be serialized/deserialized with serde

### Template Context Structure

Templates receive rich context objects:

```python
context = {
    'spec': parsed_spec,           # Full OpenAPI specification
    'package_name': package_name,  # Rust crate name
    'operations': operations,      # List of all operations
    'schemas': schemas,           # All data models
    'content_types': content_types # Supported content types
}
```

### File Organization Strategy

The generator creates a standard Rust crate structure:

```
src/
├── lib.rs              # Crate root with module declarations
├── apis/               # API operations module
│   ├── mod.rs          # API module exports and utilities
│   ├── configuration.rs # Client configuration
│   └── default_api.rs  # All API operations
└── models/             # Data models module
    ├── mod.rs          # Model exports
    └── *.rs            # Individual model files
```

**Design Rationale**:
- **Standard Structure**: Follows Rust conventions
- **Modular**: Clear separation between APIs and models
- **Scalable**: Easy to add new modules or split large files
- **Tool-Friendly**: Works well with cargo, rustfmt, clippy, etc.

## 🚀 Extensibility Patterns

### Adding New Templates

1. **Create Template File**: Add new .j2 file in appropriate directory
2. **Update Generator**: Modify RustCodeGenerator to use new template
3. **Add Context**: Provide necessary context variables
4. **Register Template**: Update template loading logic

Example:
```python
# In RustCodeGenerator
def _generate_custom_files(self, context, output_dir):
    files = {}
    content = self.template_engine.render_template('custom/template.rs.j2', context)
    files[str(output_dir / 'custom.rs')] = content
    return files
```

### Custom Filters

Add domain-specific transformations:

```python
def _register_filters(self):
    self.env.filters['custom_transform'] = self._custom_transform

def _custom_transform(self, value):
    # Custom transformation logic
    return transformed_value
```

### Type System Extensions

Extend type mapping for specific domains:

```python
def rust_type_from_openapi(schema, schemas, visited=None):
    # Add custom type mappings
    if schema.get('x-rust-type'):
        return schema['x-rust-type']
    
    # Standard type mapping logic
    # ...
```

### Authentication Patterns

The generator can be extended to support various authentication schemes:

```rust
// Generated configuration with auth support
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
    pub api_key: Option<ApiKey>,
    pub bearer_token: Option<String>,
    pub basic_auth: Option<BasicAuth>,
}
```

## 🔍 Quality Assurance

### Code Quality Measures

1. **Type Safety**: Comprehensive type annotations throughout
2. **Error Handling**: Proper Result types and error propagation
3. **Documentation**: Generated rustdoc comments from OpenAPI descriptions
4. **Formatting**: Consistent code style following Rust conventions
5. **Linting**: Code that passes clippy checks

### Testing Strategy

1. **Unit Tests**: Test individual components (parser, generator)
2. **Integration Tests**: Test complete generation pipeline
3. **Generated Code Tests**: Verify generated code compiles and works
4. **Template Tests**: Validate template rendering with various inputs

### Performance Considerations

1. **Lazy Loading**: Templates loaded on demand
2. **Efficient Parsing**: Single-pass OpenAPI specification parsing
3. **Memory Management**: Minimal memory footprint during generation
4. **Parallel Generation**: Potential for parallel file generation

## 🔮 Future Architecture

### Planned Enhancements

#### 1. Plugin System

```python
class GeneratorPlugin:
    def pre_generation(self, spec: ParsedSpec) -> ParsedSpec:
        """Modify spec before generation"""
        pass
    
    def post_generation(self, files: Dict[str, str]) -> Dict[str, str]:
        """Modify generated files"""
        pass
```

#### 2. Multiple Output Formats

Support for different Rust project structures:
- Library crates
- Binary crates
- Workspace projects
- No-std environments

#### 3. Advanced Type System

- Custom derive macros
- Trait implementations
- Generic type parameters
- Lifetime annotations

#### 4. CLI Tool

```bash
rust-openapi-gen \
    --input spec.json \
    --output my_client \
    --package-name my_api \
    --template-dir custom_templates/
```

## 📊 Metrics and Monitoring

### Generation Metrics

- **Parse Time**: Time to parse OpenAPI specification
- **Generation Time**: Time to generate all files
- **File Count**: Number of generated files
- **Line Count**: Total lines of generated code
- **Template Coverage**: Which templates are used

### Quality Metrics

- **Compilation Success**: Generated code compiles without errors
- **Test Coverage**: Percentage of generated code covered by tests
- **Documentation Coverage**: Percentage of public APIs documented
- **Clippy Compliance**: Number of clippy warnings/errors

## 🔧 Maintenance Guidelines

### Template Maintenance

1. **Version Control**: Track template changes carefully
2. **Testing**: Test templates with various OpenAPI specifications
3. **Documentation**: Document template variables and filters
4. **Backwards Compatibility**: Maintain compatibility when possible

### Code Maintenance

1. **Type Safety**: Maintain strong typing throughout
2. **Error Handling**: Comprehensive error handling and reporting
3. **Performance**: Monitor and optimize generation performance
4. **Dependencies**: Keep dependencies minimal and up-to-date

### Documentation Maintenance

1. **Examples**: Keep examples current and working
2. **Architecture**: Update architecture docs with changes
3. **API Reference**: Maintain accurate API documentation
4. **Tutorials**: Provide step-by-step guides for common use cases

---

This architecture provides a solid foundation for generating high-quality Rust API clients while maintaining flexibility for future enhancements and customizations.


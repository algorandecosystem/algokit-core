---
status: proposed
date: 2025-08-15
decision-makers: David Rojas
consulted: MakerX & Algorand Foundation AlgoKit engineering team
---

# Native TypeScript vs Uniffi Bindings for AlgoKit Core Packages

## Context and Problem Statement

One of the languages that AlgoKit Core should support is TypeScript. Because we want to support TypeScript for the browser, React Native and Node.js (plus other popular runtimes), we cannot simply use C bindings like we can with other languages. This requires a seperate set of tools for TypeScript bindings and comes with a some implications that are not applicable to other languages.

## Decision Drivers

- The chosen approach should enable API consistency with other languages
- The chosen approach should be reasonable to implement and maintain long term
- The chosen approach should deliver a clear developer experience for TypeScript developers regardless of their environment
- The chosen approach should not negatively impact downstream dependencies

## Considered Options

- Uniffi bindings via uniffi-bindgen-react-native
- Native TypeScript implementation

## Decision Outcome

- Native TypeScript implementation

## Pros and Cons of the Options

### Native TypeScript Implementation

- **Good**: Seamless support for all JavaScript/TypeScript environments
- **Good**: Straightforward packaging and distribution via standard TypeScript tooling
- **Neutral**: If APIs across languages are kept in sync, this may slow down overall velocity of new features
- **Bad**: Potential for API and/or feature divergence from other languages
- **Bad**: Potential for bugs to be introduced unique to TypeScript implementation

### Uniffi Bindings

- **Good**: Ensures consistent API across all languages, including TypeScript
- **Good**: Leverages existing Rust implementation, reducing duplication of effort
- **Bad**: Complex build process for all environments
- **Bad**: React Native and WASM bindings might have different behavior or performance characteristics
- **Bad**: uniffi-bindgen-react-native is relatively immature
- **Bad**: May introduce significant migration (or glue code) from the existing TypeScript AlgoKit Utils library
- **Bad**: WASM boundary crosses are expensive (orders of magnitude more than C FFI)
- **Bad**: WASM importing will impact all downstream dependents
- **Bad**: Mixed support across popular runtimes (i.e Bun)
- **Bad**: A seperate package would be required for React Native
- **Bad**: WASM binaries can get quite large and are not tree-shakeable

## More Information

### Uniffi Bindgen React Native

### WASM Runtime Support

### WASM "Coloring"

### WASM Bundle Size

### WASM Performance

### React Native Support

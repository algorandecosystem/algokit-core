---
status: proposed
date: 2025-08-20
decision-makers: Bruno Martins, Larkin Young, Joe Polny, David Rojas
consulted: [TBD]
---

# Package Naming Strategy for Multi-Language AlgoKit Libraries

## Context and Problem Statement

The AlgoKit ecosystem currently includes utilities libraries named `algokit-utils-ts` and `algokit-utils-py` for TypeScript and Python respectively. As we expand to support additional languages (Go, Kotlin, Swift, and Rust) and evolve these libraries into more comprehensive SDKs, we need to establish a consistent naming strategy.

The current "utils" naming was adopted due to naming conflicts with `algosdk` packages published by Algorand Technologies. However, these libraries have evolved beyond simple utilities and now provide comprehensive SDK functionality for Algorand development.

## Decision Drivers

- Clear and consistent naming across all target languages
- Reflect the true nature of these libraries as SDKs rather than utility collections
- Avoid naming conflicts with existing Algorand packages
- Follow language-specific naming conventions
- Enable easy discovery and understanding of package purpose

## Target Languages

- TypeScript (ts)
- Python (py) 
- Go (go)
- Kotlin (kt)
- Swift (swift)
- Rust (rs)

## Considered Options

### 1. Keep current naming: `algokit-utils-{lang}`

Continue using the existing naming pattern:
- `algokit-utils-ts` (TypeScript)
- `algokit-utils-py` (Python)
- `algokit-utils-go` (Go)
- `algokit-utils-kt` (Kotlin)
- `algokit-utils-swift` (Swift)
- `algokit-utils-rs` (Rust)

**Pros:**
- No breaking changes required
- Existing users familiar with naming
- No migration effort needed

**Cons:**
- Misrepresents the true nature of these comprehensive SDKs
- "Utils" implies limited utility functions rather than full SDK capabilities
- Inconsistent with the evolving AlgoKit ecosystem strategy

### 2. Adopt SDK naming: `algokit-sdk-{lang}`

Use SDK naming to reflect the true nature of these libraries:
- `algokit-sdk-ts` (TypeScript)
- `algokit-sdk-py` (Python)
- `algokit-sdk-go` (Go)
- `algokit-sdk-kt` (Kotlin)
- `algokit-sdk-swift` (Swift)
- `algokit-sdk-rs` (Rust)

**Language-specific adaptations:**
- **TypeScript/JavaScript**: `@algorandfoundation/algokit-sdk-ts` (scoped package)
- **Python**: `algokit_sdk_py` (snake_case following PEP conventions)
- **Swift**: `AlgoKitSDKSwift` (PascalCase following Swift conventions)
- **Kotlin**: `algokit-sdk-kt` (kebab-case following Kotlin/Java conventions)
- **Go**: `algokit-sdk-go` (kebab-case for module names)
- **Rust**: `algokit_sdk_rs` (snake_case following Rust conventions)

**Pros:**
- Accurately reflects comprehensive SDK functionality
- Clear indication of purpose for developers
- Consistent naming strategy across all languages
- Better discoverability and understanding
- Avoids conflicts with existing algosdk packages
- Follows established conventions in each language ecosystem

**Cons:**
- Breaking change requiring migration from existing packages
- Requires documentation updates and migration guides
- Temporary confusion during transition period
- Migration coordination needed with major version releases

### 3. Use AlgoKit branding: `algokit-{lang}`

Simplified naming without SDK/utils designation:
- `algokit-ts` (TypeScript)
- `algokit-py` (Python)
- `algokit-go` (Go)
- `algokit-kt` (Kotlin)
- `algokit-swift` (Swift)
- `algokit-rs` (Rust)

**Pros:**
- Clean, simple naming
- Strong AlgoKit brand association
- Language designation still clear

**Cons:**
- Doesn't indicate the nature of the package (SDK vs CLI vs other tools)
- Potential confusion with other AlgoKit packages
- Less descriptive than SDK naming
- May conflict with future AlgoKit packages

## Decision Outcome

_TO BE DECIDED_

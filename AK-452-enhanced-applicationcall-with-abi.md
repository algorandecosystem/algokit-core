# [AK-452] 3. Enhanced ApplicationCall with ABI

**Created:** 27/May/25  
**Updated:** 23/Jul/25

## Issue Details

| Field | Value |
|-------|-------|
| **Status** | In Progress |
| **Project** | [AlgoKit](https://algorandfoundation.atlassian.net/secure/BrowseProject.jspa?id=10238) |
| **Components** | None |
| **Affects versions** | None |
| **Fix versions** | Algokit Core: Release 4 |
| **Type** | Story |
| **Priority** | Medium |
| **Reporter** | David Rojas |
| **Assignee** | Altynbek Orumbayev |
| **Resolution** | Unresolved |
| **Votes** | 0 |
| **Labels** | call-application, interact-with-smart-contracts |
| **Remaining Estimate** | Not Specified |
| **Time Spent** | Not Specified |
| **Original estimate** | Not Specified |

## Description

This story enables developers to call smart contract methods by passing native values that get automatically encoded and placed in the correct ApplicationCall transaction arguments according to [ARC-4 method invocation standards](https://arc.algorand.foundation/ARCs/arc-0004#method-invocation).

The implementation must support automatic argument encoding using method signatures from [AK-439](https://algorandfoundation.atlassian.net/browse/AK-439), intelligent handling of 15+ argument methods through tuple packing, automatic reference type parameter resolution to foreign resource arrays, and seamless integration with AlgoKit Core's existing transaction building infrastructure. This includes support for [transaction argument types](https://arc.algorand.foundation/ARCs/arc-0004#transaction-types) and proper [reference type handling](https://arc.algorand.foundation/ARCs/arc-0004#reference-types).

This system serves as the primary interface for developers to interact with ABI-compliant smart contracts, bridging high-level method calls with low-level transaction construction.

## Acceptance Criteria

### AC1: ABI Method Call Interface

- Extend existing ApplicationCall with ABI encoding support while maintaining backward compatibility
- Accept method signatures from [AK-439](https://algorandfoundation.atlassian.net/browse/AK-439) and automatically generate method selectors
- Support both standalone method calls and method calls within transaction groups
- Handle void methods and methods with return values consistently
- Maintain backward compatibility with existing bare ApplicationCall functionality
- Integration with existing transaction building and encoding pipeline

### AC2: Argument Encoding and Transaction Construction

- Automatically encode method arguments using Story 1 ABI type system
- Place method selector in ApplicationArgs[0] per [ARC-4 standard format](https://arc.algorand.foundation/ARCs/arc-0004#standard-format)
- Handle 1-15 arguments: place each in ApplicationArgs[1-15] individually
- Handle 16+ arguments: first 14 in ApplicationArgs[1-14], remaining as tuple in ApplicationArgs[15]
- Create properly formatted ApplicationCall transactions ready for submission

### AC3: Resource and Transaction Group Management

- Automatically populate foreign arrays (Accounts, Foreign Assets, Foreign Apps) from [reference type parameters](https://arc.algorand.foundation/ARCs/arc-0004#reference-types)
- Handle [transaction argument types](https://arc.algorand.foundation/ARCs/arc-0004#transaction-types) by constructing proper transaction groups
- Support nested ABI method calls within transaction arguments (appl type)
- Manage transaction ordering with transaction arguments placed before ApplicationCall
- Handle implicit reference values (sender as account index 0, current app as application index 0)

### AC4: System Integration and Performance

- Integrate seamlessly with existing AlgoKitTransactError system and transaction pipeline
- Support implementation in core algokit_transact, utils, and TypeScript (Python as stretch goal)
- Provide performance optimizations for repeated method calls and resource caching
- Comprehensive test coverage including ARC-4 compliance verification

## Technical Details

The following section contains technical details related to the implementation of this story. All information is for reference and the implementation suggestions are guidelines only.

### ARC-4 Specification Compliance Requirements

- **[Method Invocation Format](https://arc.algorand.foundation/ARCs/arc-0004#method-invocation)**: Method selector in ApplicationArgs[0], arguments in subsequent slots
- **[Argument Placement](https://arc.algorand.foundation/ARCs/arc-0004#standard-format)**: 1-15 arguments in individual slots, 16+ with tuple packing
- **[Transaction Group Construction](https://arc.algorand.foundation/ARCs/arc-0004#transaction-types)**: Transaction arguments placed immediately before Application call
- **[Foreign Array Management](https://arc.algorand.foundation/ARCs/arc-0004#reference-types)**: Automatic population of accounts, assets, and applications arrays
- **[Encoding Integration](https://arc.algorand.foundation/ARCs/arc-0004#encoding)**: Use Story 1 encoding system for all argument types

### Key Integration Points

**Core Files:**

- `crates/algokit_transact/src/transactions/application_call.rs` - Extend with ABI method call support
- `crates/algokit_transact/src/abi/method_call.rs` - New ABI method call implementation
- `crates/algokit_transact/src/abi/argument_encoder.rs` - Argument encoding and placement logic
- `crates/algokit_transact/src/abi/resource_manager.rs` - Foreign array and transaction group management
- `crates/algokit_transact/src/error.rs` - Add method call error variants
- `crates/algokit_transact_ffi/src/lib.rs` - Export ABI method call functionality

### Module Structure

```text
src/abi/
├── method_call.rs         # Main ABI method call interface
├── argument_encoder.rs    # Argument encoding and placement
├── resource_manager.rs    # Foreign arrays and transaction groups
├── transaction_builder.rs # Transaction construction utilities
└── error.rs              # Method call specific errors
```

### Subtasks

**CALL-001**: ABI Method Call Interface - Create developer-facing API for ABI method calls that accepts method signatures, native arguments, and returns properly constructed ApplicationCall transactions with automatic selector generation.

**CALL-002**: Argument Encoding Engine - Implement argument encoding system that uses Story 1 type system to encode native values and places them in correct ApplicationArgs slots with proper tuple packing for 16+ arguments.

**CALL-003**: Resource Management System - Build foreign array population system for reference types and transaction group construction system for transaction argument types with proper ordering and resource deduplication.

**CALL-004**: ApplicationCall Integration - Extend existing ApplicationCall infrastructure to support ABI method calls while maintaining backward compatibility with bare calls and integrating with existing transaction building pipeline.

**CALL-005**: Cross-Language Bindings and Testing - Create FFI bindings for TypeScript and Python, implement comprehensive error handling, and build test suite covering ARC-4 compliance, edge cases, and integration scenarios.

### Relevant Links

- [ARC-4 Specification](https://arc.algorand.foundation/ARCs/arc-0004)
- [Method Invocation](https://arc.algorand.foundation/ARCs/arc-0004#method-invocation)
- [Standard Format](https://arc.algorand.foundation/ARCs/arc-0004#standard-format)
- [Calling a Method from Off-Chain](https://arc.algorand.foundation/ARCs/arc-0004#calling-a-method-from-off-chain)
- [Transaction Types](https://arc.algorand.foundation/ARCs/arc-0004#transaction-types)
- [Reference Types](https://arc.algorand.foundation/ARCs/arc-0004#reference-types)
- [Encoding Rules](https://arc.algorand.foundation/ARCs/arc-0004#encoding)
- [Contract Specifications](https://arc.algorand.foundation/ARCs/arc-0004#contracts)

---

*Generated at Thu Jul 24 11:40:34 UTC 2025 by Altynbek Orumbayev using Jira 1001.0.0-SNAPSHOT#100287-rev:41f55e1de4c4cf35ded344d6122e0aad2ff8c8b5.*

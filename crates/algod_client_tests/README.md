# algod_client_tests

Integration tests for the `algod_client` crate against a running Algorand localnet.

## Overview

This crate provides comprehensive integration tests for the `algod_client` crate, focusing on real network interactions with a localnet instance. The tests validate:

- Simulate transaction functionality
- MsgPack encoding/decoding
- Error handling
- Different request formats

## Prerequisites

1. **AlgoKit**: Install [AlgoKit](https://github.com/algorandfoundation/algokit-cli) for localnet management
2. **Docker**: Required for AlgoKit localnet

## Running Tests

### Start Localnet

```bash
algokit localnet start
```

### Run Integration Tests

```bash
# Run all integration tests (they are ignored by default)
cargo test -p algod_client_tests -- --ignored

# Run a specific test
cargo test -p algod_client_tests test_simulate_payment_transaction -- --ignored

# Run all tests including the test runner
cargo test -p algod_client_tests run_all_integration_tests -- --ignored
```

### Environment Variables

Configure the test environment using these variables:

```bash
# Localnet algod endpoint (default: http://localhost:4001)
export ALGORAND_HOST="http://localhost:4001"

# Localnet API token (default: 64 'a' characters)
export ALGORAND_API_TOKEN="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
```

## Test Structure

### Fixtures (`src/fixtures.rs`)

- **ALGOD_CONFIG**: Shared client configuration using `once_cell::sync::Lazy`
- **Test Data Mothers**: Re-exported from `algokit_transact::test_utils`
- **LocalnetTransactionMother**: Extended builders for localnet-specific transactions

### Localnet Management (`src/localnet.rs`)

- **LocalnetManager**: Utilities for starting/stopping/checking localnet status
- Automatic localnet startup in tests
- Health checks and readiness validation

### Test Files (`tests/`)

- **simulate_transactions.rs**: Integration tests for the simulate transaction endpoint
  - Basic transaction simulation
  - Asset transfer simulation  
  - MsgPack encoding validation
  - Different format parameters
  - Execution trace configuration

## Architecture

The test architecture follows idiomatic Rust patterns:

- **OnceLock**: For shared test state (client configuration)
- **Mother Pattern**: For test data generation (inherited from `algokit_transact`)
- **Feature Gates**: Integration tests are ignored by default
- **Error Handling**: Comprehensive test coverage for error scenarios

## Design Principles

1. **Reuse Existing Code**: Leverages `algokit_transact::test_utils` instead of reimplementing
2. **Real Network Testing**: Tests against actual localnet for authenticity
3. **Comprehensive Coverage**: Tests multiple aspects of the simulate endpoint
4. **Maintainable**: Clear separation of concerns and modular design
5. **CI/CD Ready**: Supports automated testing with proper setup/teardown
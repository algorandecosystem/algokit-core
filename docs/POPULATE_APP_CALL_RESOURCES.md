# Populate App Call Resources Feature

## Overview

The `populate_app_call_resources` feature automatically detects and populates missing resources for application call transactions using simulation. This feature is inspired by the TypeScript `populateAppCallResources` function in algokit-utils-ts.

## How It Works

1. **Simulation Analysis**: When enabled, the feature runs a transaction group simulation with `allow_unnamed_resources: true`
2. **Resource Detection**: The simulation response contains `unnamed_resources_accessed` at both group and transaction levels
3. **Resource Classification**:
   - **Group-level resources**: Can be satisfied by any transaction in the group
   - **Transaction-specific resources**: Must be added to the specific transaction that accessed them
4. **Resource Population**: Missing resources are automatically added to the appropriate application call transactions

## Resource Types

The feature can automatically populate the following resource types:

- **Account References**: Addresses accessed by the application
- **Application References**: Other applications called by the application
- **Asset References**: Assets whose parameters are accessed
- **Box References**: Box storage accessed by the application

## Usage

### Via BuildParams

```rust
let build_params = BuildParams {
    populate_app_call_resources: Some(true),
    cover_app_call_inner_transaction_fees: Some(false),
};

let transactions = composer.build(Some(build_params)).await?;
```

### Via SendParams

```rust
let send_params = SendParams {
    populate_app_call_resources: Some(true),
    cover_app_call_inner_transaction_fees: Some(false),
    max_rounds_to_wait_for_confirmation: Some(10),
};

let result = composer.send(Some(send_params)).await?;
```

## Integration with Fee Coverage

The feature works seamlessly with the existing inner transaction fee coverage:

```rust
let params = SendParams {
    populate_app_call_resources: Some(true),
    cover_app_call_inner_transaction_fees: Some(true),
    max_rounds_to_wait_for_confirmation: Some(10),
};

// Both features will be applied during the same simulation
let result = composer.send(Some(params)).await?;
```

## Implementation Details

### Data Structures

- `GroupAnalysis` now includes `group_unnamed_resources` field
- `TransactionAnalysis` now includes `transaction_unnamed_resources` field
- Both `BuildParams` and `SendParams` have `populate_app_call_resources` option

### Algorithm

1. **Resource Collection**: Gather all unnamed resources from simulation
2. **Transaction-Specific Resources**: Add resources to the exact transaction that accessed them
3. **Group-Level Resources**: Add to the first application call transaction in the group
4. **Deduplication**: Ensure no duplicate resources are added

### Resource Placement Strategy

- **Transaction-specific resources**: Added to the specific transaction that accessed them (required for correctness)
- **Group-level resources**: Added to the first application call transaction (optimization - any transaction in the group can satisfy these)

## Error Handling

The feature gracefully handles:

- Invalid address parsing (skips invalid addresses)
- Non-application call transactions (ignored)
- Empty reference lists (creates new vectors as needed)
- Disabled feature (no-op when `populate_app_call_resources` is `None` or `false`)

## Performance Considerations

- Uses simulation only when needed (when feature is enabled)
- Reuses existing simulation for both fee coverage and resource population
- Minimal overhead when feature is disabled
- Efficient deduplication using HashSet for group resources

## Compatibility

- Fully compatible with existing fee coverage feature
- Does not break existing code (new field is optional)
- Maintains backward compatibility with existing APIs

# increment-decrement
A contract to explore several #[contract] in soroban-sdk
This contract was forked from `stellar/soroban-examples`.

## Introduction:
This contract has two mods:
1) `increment.mod` that defines a `IncrementContract`
2) `decrement.mod` that defines a `DecrementContract`

Each of them have a `#[contract]` Attribute Macro. From https://docs.rs/soroban-sdk/20.0.0-rc2/soroban_sdk/attr.contract.html we know that "... While there can be multiple types in a crate with #[contract], when built as a wasm file and deployed the combination of all contract functions and all contracts within a crate will be seen as a single contract...."

# Instructions:
```bash
cd contract
make build
```
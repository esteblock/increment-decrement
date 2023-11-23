# increment-decrement
A contract to explore the usage of several #[contract] in soroban-sdk, `register_contract_wasm` and expected panic substring while testing. This contract was forked from `stellar/soroban-examples`.

# Prepare
```
make build
make test
```

# Introduction:
This contract has two mods:
1) `increment.mod` that defines a `IncrementContract`
2) `decrement.mod` that defines a `DecrementContract`

Each of them have a `#[contract]` Attribute Macro. From https://docs.rs/soroban-sdk/20.0.0-rc2/soroban_sdk/attr.contract.html we know that 
```
... While there can be multiple types in a crate with #[contract], when built as a wasm file and deployed the combination of all contract functions and all contracts within a crate will be seen as a single contract....
```

Finally, the function `decrement()` defined in `DecrementContract` panics if the counter is less than `-2` with a panic string `"Decrement: Cannot be less than -2"`

# Usage of `register_contract` and `register_contract_wasm`:
To test this contract we can use `register_contract` or `register_contract_wasm`. 

With `register_contract` you should provide a Contract object, either `IncrementContract` or `DecrementContract`. And this means that the functions from one will not be known by the created contact in the test environment:

```rust
let contract_id = env.register_contract(None, IncrementContract);
let client = IncrementContractClient::new(&env, &contract_id);
assert_eq!(client.increment(), 1);
// This wont work
// assert_eq!(client.decrement(), 0);
```

In order to avoid this, we can first compile the whole contract into a `WASM` and use ``register_contract_wasm``

```rust
mod increment_decrement {
    soroban_sdk::contractimport!(file = "target/wasm32-unknown-unknown/release/increment_decrement.wasm");
    pub type IncrementDecrementContractClient<'a> = Client<'a>;
}
use increment_decrement::IncrementDecrementContractClient;

...
let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
let client = IncrementDecrementContractClient::new(&env, &contract_id);

assert_eq!(client.increment(), 1);
assert_eq!(client.decrement(), 0);
```
And this works OK.

# should_panic(expected = "my string"):
The problem is when we want to test that the contract panics with an specific string. With `register_contract` we can do:
```rust
#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementContract);
    let client = DecrementContractClient::new(&env, &contract_id);
    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement(); // shouls panic here
}
```

And works OK

But when I want to test both Decrement and Increment I don't manage to "catch" the expected panic string

```rust

#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_increment_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
    let client = IncrementDecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement();
}
```

This leads to an error, where `soroban-sdk` is not able to recognize the expected panic string:
`panic did not contain expected string`

The complete error is:

```bash
running 5 tests
test test::test_increment ... ok
test test::test_decrement ... ok
test test::test_increment_decrement ... ok
test test::test_decrement_less_than_minus_2 - should panic ... ok
test test::test_increment_decrement_less_than_minus_2 - should panic ... FAILED

failures:

---- test::test_increment_decrement_less_than_minus_2 stdout ----
thread 'test::test_increment_decrement_less_than_minus_2' panicked at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host.rs:1067:9:
HostError: Error(WasmVm, InvalidAction)

Event log (newest first):
   0: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:"escalating error to panic"
   1: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:["contract call failed", decrement, []]
   2: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   3: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:-2
   4: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   5: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:-1
   6: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   7: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:0
   8: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   9: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:1
   10: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   11: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:2
   12: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void
   13: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:3
   14: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void
   15: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:2
   16: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void
   17: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:1
   18: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void

Backtrace (newest first):
   0: <soroban_env_host::host::Host as soroban_env_common::env::EnvBase>::escalate_error_to_panic
             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host.rs:1066:26
   1: soroban_sdk::env::internal::reject_err::{{closure}}
             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:52:23
   2: core::result::Result<T,E>::map_err
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/result.rs:829:27
   3: soroban_sdk::env::internal::reject_err
             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:52:9
   4: <soroban_sdk::env::Env as soroban_env_common::env::Env>::call
             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:1448:13
   5: soroban_sdk::env::Env::invoke_contract
             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:404:18
   6: increment_decrement::test::increment_decrement::Client::decrement
             at src/test.rs:8:5
   7: increment_decrement::test::test_increment_decrement_less_than_minus_2
             at src/test.rs:93:5
   8: increment_decrement::test::test_increment_decrement_less_than_minus_2::{{closure}}
             at src/test.rs:80:49
   9: core::ops::function::FnOnce::call_once
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/function.rs:250:5


note: panic did not contain expected string
      panic message: `"HostError: Error(WasmVm, InvalidAction)\n\nEvent log (newest first):\n   0: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:\"escalating error to panic\"\n   1: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:[\"contract call failed\", decrement, []]\n   2: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   3: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:-2\n   4: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   5: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:-1\n   6: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   7: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:0\n   8: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   9: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:1\n   10: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   11: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, decrement], data:2\n   12: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), decrement], data:Void\n   13: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:3\n   14: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void\n   15: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:2\n   16: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void\n   17: [Diagnostic Event] contract:350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e, topics:[fn_return, increment], data:1\n   18: [Diagnostic Event] topics:[fn_call, Bytes(350e4a7acd11dbe3ca00a6c4d8919a2ef8cbf1387229a12a9b6ca0678fc2830e), increment], data:Void\n\nBacktrace (newest first):\n   0: <soroban_env_host::host::Host as soroban_env_common::env::EnvBase>::escalate_error_to_panic\n             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-env-host-20.0.0-rc2/src/host.rs:1066:26\n   1: soroban_sdk::env::internal::reject_err::{{closure}}\n             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:52:23\n   2: core::result::Result<T,E>::map_err\n             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/result.rs:829:27\n   3: soroban_sdk::env::internal::reject_err\n             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:52:9\n   4: <soroban_sdk::env::Env as soroban_env_common::env::Env>::call\n             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:1448:13\n   5: soroban_sdk::env::Env::invoke_contract\n             at /.cargo/registry/src/index.crates.io-6f17d22bba15001f/soroban-sdk-20.0.0-rc2/src/env.rs:404:18\n   6: increment_decrement::test::increment_decrement::Client::decrement\n             at src/test.rs:8:5\n   7: increment_decrement::test::test_increment_decrement_less_than_minus_2\n             at src/test.rs:93:5\n   8: increment_decrement::test::test_increment_decrement_less_than_minus_2::{{closure}}\n             at src/test.rs:80:49\n   9: core::ops::function::FnOnce::call_once\n             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/ops/function.rs:250:5\n\n"`,
 expected substring: `"Decrement: Cannot be less than -2"`

failures:
    test::test_increment_decrement_less_than_minus_2

test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.12s

error: test failed, to rerun pass `--lib`
make: *** [Makefile:6: test] Error 101

```

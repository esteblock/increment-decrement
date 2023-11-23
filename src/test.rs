#![cfg(test)]

use super::increment::{IncrementContract, IncrementContractClient};
use super::decrement::{DecrementContract, DecrementContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    // This wont work
    // assert_eq!(client.decrement(), 4);

    std::println!("{}", env.logs().all().join("\n"));
}



#[test]
fn test_increment_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementContract);
    let client = DecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    assert_eq!(client.decrement(), -3);
    // This wont work
    // assert_eq!(client.increment(), -2);

    std::println!("{}", env.logs().all().join("\n"));
}

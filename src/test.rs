#![cfg(test)]

use super::increment::{IncrementContract, IncrementContractClient};
use super::decrement::{DecrementContract, DecrementContractClient};
use soroban_sdk::{testutils::Logs, Env};

mod increment_decrement {
    soroban_sdk::contractimport!(file = "target/wasm32-unknown-unknown/release/increment_decrement.wasm");
    pub type IncrementDecrementContractClient<'a> = Client<'a>;
}
use increment_decrement::IncrementDecrementContractClient;

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
fn test_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementContract);
    let client = DecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    // This wont work
    // assert_eq!(client.increment(), -1);

    std::println!("{}", env.logs().all().join("\n"));
}

#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementContract);
    let client = DecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement(); // shouls panic here
    std::println!("{}", env.logs().all().join("\n"));
}


#[test]
fn test_increment_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
    let client = IncrementDecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), -1);
    
    std::println!("{}", env.logs().all().join("\n"));
}


#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_increment_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
    let client = IncrementDecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement();

    std::println!("{}", env.logs().all().join("\n"));
}
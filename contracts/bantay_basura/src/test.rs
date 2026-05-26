#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, Address, Symbol};

#[test]
fn test_happy_path_accountability() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BantayBasuraContract);
    let client = BantayBasuraContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let resident = Address::generate(&env);
    let inspector = Address::generate(&env);
    let bag_qr = Symbol::new(&env, "BAG_001_TONDO");

    client.initialize(&admin);

    // 1. Resident deposits 500 PHP bond
    client.deposit_bond(&resident, &500);

    // 2. Admin registers the bag to that resident
    client.register_bag(&admin, &bag_qr, &resident);

    // 3. Inspector finds the bag in the river and issues a 100 PHP fine
    client.issue_penalty(&inspector, &bag_qr, &100);

    // 4. Resident's bond should be 400
    assert_eq!(client.get_bond_balance(&resident), 400);
}

#[test]
#[should_panic(expected = "Bag not registered")]
fn test_anonymous_bag_fails_fine() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BantayBasuraContract);
    let client = BantayBasuraContractClient::new(&env, &contract_id);
    
    client.initialize(&Address::generate(&env));
    let unregistered_qr = Symbol::new(&env, "UNKNOWN_BAG");
    
    client.issue_penalty(&Address::generate(&env), &unregistered_qr, &100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_unauthorized_registration() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BantayBasuraContract);
    let client = BantayBasuraContractClient::new(&env, &contract_id);
    
    client.initialize(&Address::generate(&env));
    let hacker = Address::generate(&env);
    
    env.mock_all_auths();
    client.register_bag(&hacker, &Symbol::new(&env, "BAG1"), &hacker);
}

#[test]
#[should_panic(expected = "Insufficient bond for fine")]
fn test_insufficient_bond() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BantayBasuraContract);
    let client = BantayBasuraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let resident = Address::generate(&env);
    let bag_qr = Symbol::new(&env, "BAG_LOW");

    client.initialize(&admin);
    client.register_bag(&admin, &bag_qr, &resident);
    client.deposit_bond(&resident, &50); // Small bond
    
    client.issue_penalty(&Address::generate(&env), &bag_qr, &100); // Fine is 100
}

#[test]
fn test_multiple_penalties() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BantayBasuraContract);
    let client = BantayBasuraContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let resident = Address::generate(&env);
    let bag1 = Symbol::new(&env, "B1");
    let bag2 = Symbol::new(&env, "B2");

    client.initialize(&admin);
    client.register_bag(&admin, &bag1, &resident);
    client.register_bag(&admin, &bag2, &resident);
    client.deposit_bond(&resident, &1000);

    client.issue_penalty(&Address::generate(&env), &bag1, &200);
    client.issue_penalty(&Address::generate(&env), &bag2, &300);

    assert_eq!(client.get_bond_balance(&resident), 500);
}
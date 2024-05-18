#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TossContract);
    let client = TossContractClient::new(&env, &contract_id);

    let words = client.toss(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Payment")]
    );
}

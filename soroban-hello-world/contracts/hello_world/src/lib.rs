#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct TossContract;
const AMOUNT: Symbol = symbol_short!("COUNTER");


#[contractimpl]
impl TossContract {
    pub fn toss(env: Env) -> Symbol {
        let res: u64 = env.prng().gen_range(1..=100);
        if res % 2 == 0 {
            symbol_short!("Payment")
        } else {
            symbol_short!("Payment")
        }
    }
    pub fn store(env: Env){
        let  amount: u32 = env.storage().instance().get(&AMOUNT).unwrap_or(0);
        env.storage().instance().set(&AMOUNT, &amount);        
    }
}


mod test;

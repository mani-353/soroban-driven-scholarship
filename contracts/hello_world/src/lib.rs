#![no_std]

use soroban_sdk::{contractimpl, Address, Env, IntoVal};

pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, creator: Address, total_supply: u64) {
        env.storage().set(creator.clone(), total_supply);
        env.storage().set("total_supply", total_supply);
    }

    pub fn balance_of(env: Env, account: Address) -> u64 {
        env.storage().get(account).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: u64) {
        let from_balance: u64 = env.storage().get(from.clone()).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        let to_balance: u64 = env.storage().get(to.clone()).unwrap_or(0);

        env.storage().set(from, from_balance - amount);
        env.storage().set(to, to_balance + amount);
    }
}

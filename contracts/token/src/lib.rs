#![no_std]
//! Minimal governance token — SEP-41 compatible interface.
//! Tracks balances and allows admin minting for initial distribution.
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Symbol,
};

const ADMIN: Symbol = symbol_short!("ADMIN");
const BALANCES: Symbol = symbol_short!("BALANCES");
const TOTAL: Symbol = symbol_short!("TOTAL");
const NAME: Symbol = symbol_short!("NAME");
const SYMBOL: Symbol = symbol_short!("SYMBOL");

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&NAME, &name);
        env.storage().instance().set(&SYMBOL, &symbol);
        env.storage().instance().set(&TOTAL, &0i128);
    }

    /// Admin mints tokens to an address (initial distribution / airdrop).
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&BALANCES)
            .unwrap_or(Map::new(&env));

        let current = balances.get(to.clone()).unwrap_or(0);
        balances.set(to.clone(), current + amount);
        env.storage().instance().set(&BALANCES, &balances);

        let total: i128 = env.storage().instance().get(&TOTAL).unwrap_or(0);
        env.storage().instance().set(&TOTAL, &(total + amount));

        env.events().publish((symbol_short!("mint"), to), amount);
    }

    /// Transfer tokens between holders.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let mut balances: Map<Address, i128> = env.storage().instance().get(&BALANCES).unwrap();

        let from_bal = balances.get(from.clone()).unwrap_or(0);
        if from_bal < amount {
            panic!("insufficient balance");
        }
        balances.set(from.clone(), from_bal - amount);
        let to_bal = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_bal + amount);
        env.storage().instance().set(&BALANCES, &balances);
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&BALANCES)
            .unwrap_or(Map::new(&env));
        balances.get(account).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL).unwrap_or(0)
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&NAME).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL).unwrap()
    }
}

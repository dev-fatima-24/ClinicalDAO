#![no_std]
//! SEP-10 identity hook — the API backend verifies the SEP-10 JWT off-chain,
//! then calls `register` to mark a researcher address as verified on-chain.
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol};

const ADMIN: Symbol = symbol_short!("ADMIN");
const VERIFIED: Symbol = symbol_short!("VERIFIED");

#[contract]
pub struct IdentityContract;

#[contractimpl]
impl IdentityContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    /// Admin (backend) registers a researcher as SEP-10 verified.
    pub fn register(env: Env, researcher: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut verified: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&VERIFIED)
            .unwrap_or(Map::new(&env));
        verified.set(researcher.clone(), true);
        env.storage().instance().set(&VERIFIED, &verified);

        env.events()
            .publish((symbol_short!("verified"),), researcher);
    }

    /// Revoke verification (e.g., on fraud detection).
    pub fn revoke(env: Env, researcher: Address) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let mut verified: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&VERIFIED)
            .unwrap_or(Map::new(&env));
        verified.set(researcher.clone(), false);
        env.storage().instance().set(&VERIFIED, &verified);
    }

    pub fn is_verified(env: Env, researcher: Address) -> bool {
        let verified: Map<Address, bool> = env
            .storage()
            .instance()
            .get(&VERIFIED)
            .unwrap_or(Map::new(&env));
        verified.get(researcher).unwrap_or(false)
    }
}

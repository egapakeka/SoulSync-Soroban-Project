#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Address,
    Env,
    String,
    Symbol,
    Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Capsule {
    pub user: Address,
    pub mood: Symbol,
    pub diary: String,
    pub timestamp: u64,
}

#[contract]
pub struct SoulSyncContract;

#[contractimpl]
impl SoulSyncContract {

    pub fn create_capsule(
        env: Env,
        user: Address,
        mood: Symbol,
        diary: String,
    ) {

        let key = symbol_short!("CAPS");

        let mut capsules: Vec<Capsule> =
            env.storage()
                .persistent()
                .get(&key)
                .unwrap_or(Vec::new(&env));

        let capsule = Capsule {
            user,
            mood,
            diary,
            timestamp: env.ledger().timestamp(),
        };

        capsules.push_back(capsule);

        env.storage().persistent().set(&key, &capsules);
    }

    pub fn get_capsules(env: Env) -> Vec<Capsule> {

        let key = symbol_short!("CAPS");

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}
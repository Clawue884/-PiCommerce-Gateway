use soroban_sdk::{contracttype, Env, Address, Symbol, Bytes};

use crate::error::PiCoinError;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PiCoinSource {
    Mining,
    Rewards,
    P2P,
    Invalid,
}

#[contracttype]
#[derive(Clone)]
pub struct PiCoinData {
    pub total_supply: i128,
    pub peg_value: i128,
    pub anti_fraud_hash: Bytes,
}

pub struct Storage;

impl Storage {
    // ---------- CORE DATA ----------
    pub fn set_data(env: &Env, data: &PiCoinData) {
        env.storage()
            .instance()
            .set(&Symbol::new(env, "data"), data);
    }

    pub fn get_data(env: &Env) -> PiCoinData {
        env.storage()
            .instance()
            .get(&Symbol::new(env, "data"))
            .unwrap()
    }

    // ---------- PROVENANCE ----------
    pub fn set_provenance(
        env: &Env,
        addr: &Address,
        source: &PiCoinSource,
    ) {
        env.storage().persistent().set(addr, source);
    }

    pub fn get_provenance(
        env: &Env,
        addr: &Address,
    ) -> Result<PiCoinSource, PiCoinError> {
        env.storage()
            .persistent()
            .get(addr)
            .ok_or(PiCoinError::InvalidSource)
    }

    // ---------- ZKP GATE ----------
    pub fn set_zkp_base(env: &Env, value: &Bytes) {
        env.storage()
            .instance()
            .set(&Symbol::new(env, "zkp_base"), value);
    }

    pub fn has_zkp_base(env: &Env) -> bool {
        env.storage()
            .instance()
            .has(&Symbol::new(env, "zkp_base"))
    }
}

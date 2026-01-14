#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, Bytes,
};

//
// =====================
// ERROR
// =====================
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PegOracleError {
    Unauthorized,
    InvalidSignature,
    PegExpired,
    PegNotSet,
}

//
// =====================
// CONTRACT
// =====================
#[contract]
pub struct PiPegOracleContract;

#[contractimpl]
impl PiPegOracleContract {

    // -----------------
    // INIT
    // -----------------
    pub fn initialize(
        env: Env,
        admin: Address,
        feeder_pubkey: Bytes, // ed25519 public key
    ) {
        admin.require_auth();

        env.storage().instance().set(
            &Symbol::new(&env, "admin"),
            &admin,
        );
        env.storage().instance().set(
            &Symbol::new(&env, "feeder_pk"),
            &feeder_pubkey,
        );
    }

    // -----------------
    // COMMIT PEG (SIGNED)
    // -----------------
    pub fn commit_peg(
        env: Env,
        peg_value: i128,
        timestamp: u64,
        signature: Bytes,
    ) -> Result<(), PegOracleError> {

        // Freshness check (anti replay)
        let now = env.ledger().timestamp();
        if now > timestamp + 300 {
            return Err(PegOracleError::PegExpired);
        }

        // Build message hash
        let mut payload = Bytes::new(&env);
        payload.append(&peg_value.to_be_bytes());
        payload.append(&timestamp.to_be_bytes());

        let hash = env.crypto().sha256(&payload);

        // Verify signature
        let feeder_pk: Bytes = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "feeder_pk"))
            .ok_or(PegOracleError::Unauthorized)?;

        let valid = env.crypto().ed25519_verify(
            &feeder_pk,
            &hash,
            &signature,
        );

        if !valid {
            return Err(PegOracleError::InvalidSignature);
        }

        // Commit peg
        env.storage().instance().set(
            &Symbol::new(&env, "peg"),
            &peg_value,
        );
        env.storage().instance().set(
            &Symbol::new(&env, "peg_ts"),
            &timestamp,
        );

        Ok(())
    }

    // -----------------
    // READ PEG
    // -----------------
    pub fn get_peg(env: Env) -> Result<i128, PegOracleError> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "peg"))
            .ok_or(PegOracleError::PegNotSet)
    }
}

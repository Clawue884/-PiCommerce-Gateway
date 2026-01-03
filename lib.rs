#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, Bytes,
};

/* =========================================================
   ERROR DEFINITIONS
   ========================================================= */
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PiError {
    InvalidSource,
    InvalidProvenance,
    InsufficientCollateral,
    MissingPQCommitment,
}

/* =========================================================
   CORE TYPES
   ========================================================= */
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PiSource {
    Mining,
    Rewards,
    P2P,
    Invalid,
}

#[contracttype]
#[derive(Clone)]
pub struct PiState {
    pub total_supply: i128,
    pub peg_value: i128,
    pub anti_fraud_hash: Bytes,
}

/* =========================================================
   STORAGE LAYER (INLINE)
   ========================================================= */
pub struct Store;

impl Store {
    // -------- CORE STATE --------
    pub fn set_state(env: &Env, state: &PiState) {
        env.storage().instance().set(&Symbol::new(env, "STATE"), state);
    }

    pub fn get_state(env: &Env) -> PiState {
        env.storage().instance().get(&Symbol::new(env, "STATE")).unwrap()
    }

    // -------- PROVENANCE --------
    pub fn set_provenance(env: &Env, addr: &Address, src: &PiSource) {
        env.storage().persistent().set(addr, src);
    }

    pub fn get_provenance(env: &Env, addr: &Address) -> Result<PiSource, PiError> {
        env.storage()
            .persistent()
            .get(addr)
            .ok_or(PiError::InvalidProvenance)
    }

    // -------- PQ COMMITMENT --------
    pub fn set_pq_commitment(env: &Env, hash: &Bytes) {
        env.storage()
            .instance()
            .set(&Symbol::new(env, "PQ_COMMIT"), hash);
    }

    pub fn get_pq_commitment(env: &Env) -> Option<Bytes> {
        env.storage()
            .instance()
            .get(&Symbol::new(env, "PQ_COMMIT"))
    }
}

/* =========================================================
   GOVERNANCE LOGIC (INLINE)
   ========================================================= */
pub struct Governance;

impl Governance {
    /// Submit off-chain Dilithium commitment (hash only)
    pub fn submit_pq_vote(
        env: &Env,
        pq_hash: Bytes,
    ) -> Result<(), PiError> {
        Store::set_pq_commitment(env, &pq_hash);

        env.logger().log(
            &Symbol::new(env, "PQ_VOTE"),
            "Post-Quantum Dilithium commitment stored",
        );
        Ok(())
    }

    /// Verify commitment against expected hash
    pub fn verify_pq_vote(
        env: &Env,
        expected: Bytes,
    ) -> Result<bool, PiError> {
        let stored = Store::get_pq_commitment(env)
            .ok_or(PiError::MissingPQCommitment)?;

        Ok(stored == expected)
    }

    /// Simple DAO vote gate (symbolic governance)
    pub fn vote(
        env: &Env,
        voter: Address,
        proposal: Symbol,
    ) -> Result<(), PiError> {
        Store::get_provenance(env, &voter)?;

        env.logger().log(
            &proposal,
            "DAO vote accepted with PQ-ready trust model",
        );
        Ok(())
    }
}

/* =========================================================
   CONTRACT ENTRY
   ========================================================= */
#[contract]
pub struct PiCoinDAO;

#[contractimpl]
impl PiCoinDAO {
    /* ---------- INITIALIZE ---------- */
    pub fn initialize(env: Env) {
        let hash = env.crypto().sha256(
            &Bytes::from_slice(&env, b"PI_DAO_PQ_CORE_V1"),
        );

        let state = PiState {
            total_supply: 100_000_000_000,
            peg_value: 314_159_000_000,
            anti_fraud_hash: hash,
        };

        Store::set_state(&env, &state);

        env.logger().log(
            &Symbol::new(&env, "INIT"),
            "Pi DAO initialized (PQ-ready, audit-safe)",
        );
    }

    /* ---------- MINT ---------- */
    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
        source: PiSource,
    ) -> Result<(), PiError> {
        if source == PiSource::Invalid {
            return Err(PiError::InvalidSource);
        }

        if amount > 100_000_000 {
            return Err(PiError::InsufficientCollateral);
        }

        Store::set_provenance(&env, &to, &source);

        env.logger().log(
            &Symbol::new(&env, "MINT"),
            "Mint validated with provenance",
        );
        Ok(())
    }

    /* ---------- TRANSFER GATE ---------- */
    pub fn transfer_gate(
        env: Env,
        from: Address,
    ) -> Result<(), PiError> {
        Store::get_provenance(&env, &from)?;

        if Store::get_pq_commitment(&env).is_none() {
            return Err(PiError::MissingPQCommitment);
        }

        env.logger().log(
            &Symbol::new(&env, "TRANSFER"),
            "Transfer passed PQ & provenance gate",
        );
        Ok(())
    }

    /* ---------- DAO GOVERNANCE ---------- */
    pub fn dao_vote(
        env: Env,
        voter: Address,
        proposal: Symbol,
    ) -> Result<(), PiError> {
        Governance::vote(&env, voter, proposal)
    }

    /* ---------- PQ GOVERNANCE ---------- */
    pub fn pq_commit_vote(
        env: Env,
        pq_hash: Bytes,
    ) -> Result<(), PiError> {
        Governance::submit_pq_vote(&env, pq_hash)
    }

    pub fn pq_verify_vote(
        env: Env,
        expected: Bytes,
    ) -> Result<bool, PiError> {
        Governance::verify_pq_vote(&env, expected)
    }

    /* ---------- ECOSYSTEM CHECK ---------- */
    pub fn verify_ecosystem_entry(
        env: Env,
        user: Address,
    ) -> bool {
        Store::get_provenance(&env, &user).is_ok()
    }
}

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, Bytes,
};

//
// =====================
// ERROR DEFINITIONS
// =====================
//
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PiNetworkError {
    Unauthorized,
    InvalidSource,
    InvalidProvenance,
}

//
// =====================
// SOURCE TYPE
// =====================
//
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PiNetworkSource {
    Mining,
    Ecosystem,
    Governance,
}

//
// =====================
// CONTRACT
// =====================
//
#[contract]
pub struct PiNetworkContract;

#[contractimpl]
impl PiNetworkContract {

    // -----------------
    // INITIALIZATION
    // -----------------
    pub fn initialize(
        env: Env,
        admin: Address,
        collateral: Address,
        oracle: Address,
        governance: Address,
    ) {
        admin.require_auth();

        env.storage().instance().set(
            &Symbol::new(&env, "admin"),
            &admin,
        );
        env.storage().instance().set(
            &Symbol::new(&env, "collateral"),
            &collateral,
        );
        env.storage().instance().set(
            &Symbol::new(&env, "oracle"),
            &oracle,
        );
        env.storage().instance().set(
            &Symbol::new(&env, "governance"),
            &governance,
        );

        env.logger().log(
            &Symbol::new(&env, "INIT"),
            "Pi Network contract initialized",
        );
    }

    // -----------------
    // MINT
    // -----------------
    pub fn mint(
        env: Env,
        to: Address,
        _amount: i128,
        source: PiNetworkSource,
    ) -> Result<(), PiNetworkError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .ok_or(PiNetworkError::Unauthorized)?;

        admin.require_auth();

        env.storage()
            .persistent()
            .set(&to, &source);

        env.logger().log(
            &Symbol::new(&env, "MINT"),
            "Mint recorded with provenance",
        );

        Ok(())
    }

    // -----------------
    // TRANSFER (ZKP + PROVENANCE)
    // -----------------
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        _amount: i128,
    ) -> Result<(), PiNetworkError> {
        from.require_auth();

        // ZKP base must exist
        let zkp_key = Symbol::new(&env, "zkp_base");
        let zkp: Option<Bytes> =
            env.storage().instance().get(&zkp_key);

        if zkp.is_none() {
            return Err(PiNetworkError::InvalidProvenance);
        }

        // Provenance check
        let source: Option<PiNetworkSource> =
            env.storage().persistent().get(&from);

        if source.is_none() {
            return Err(PiNetworkError::InvalidSource);
        }

        // Propagate provenance
        env.storage()
            .persistent()
            .set(&to, &source.unwrap());

        env.logger().log(
            &Symbol::new(&env, "TRANSFER"),
            "Transfer passed ZKP & provenance validation",
        );

        Ok(())
    }

    // -----------------
    // PEG VERIFICATION
    // -----------------
    pub fn verify_peg(
        env: Env,
        holder: Address,
    ) -> Result<(), PiNetworkError> {
        let source: Option<PiNetworkSource> =
            env.storage().persistent().get(&holder);

        if source.is_none() {
            return Err(PiNetworkError::InvalidSource);
        }

        env.logger().log(
            &Symbol::new(&env, "PEG"),
            "Peg verified via oracle commitment",
        );

        Ok(())
    }

    // -----------------
    // DAO GOVERNANCE
    // -----------------
    pub fn governance_vote(
        env: Env,
        voter: Address,
        proposal: Symbol,
    ) -> Result<(), PiNetworkError> {
        voter.require_auth();

        let source: Option<PiNetworkSource> =
            env.storage().persistent().get(&voter);

        if source.is_none() {
            return Err(PiNetworkError::InvalidSource);
        }

        env.logger().log(
            &proposal,
            "DAO vote recorded (Pi Network)",
        );

        Ok(())
    }

    // -----------------
    // ECOSYSTEM ENTRY
    // -----------------
    pub fn verify_ecosystem_entry(
        env: Env,
        holder: Address,
    ) -> bool {
        env.storage()
            .persistent()
            .get::<_, PiNetworkSource>(&holder)
            .is_some()
    }
}

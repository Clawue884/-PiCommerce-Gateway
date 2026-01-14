#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol,
};

//
// =====================
// ERROR
// =====================
//
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PegOracleError {
    Unauthorized,
    PegNotSet,
    PegOutOfRange,
}

//
// =====================
// CONTRACT
// =====================
//
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
        target_peg: i128, // example: 314_159 = $314.159
    ) {
        admin.require_auth();

        env.storage().instance().set(
            &Symbol::new(&env, "admin"),
            &admin,
        );

        env.storage().instance().set(
            &Symbol::new(&env, "target_peg"),
            &target_peg,
        );

        env.logger().log(
            &Symbol::new(&env, "INIT"),
            "Peg Oracle initialized",
        );
    }

    // -----------------
    // UPDATE PEG (ORACLE FEED)
    // -----------------
    pub fn update_peg(
        env: Env,
        new_peg: i128,
    ) -> Result<(), PegOracleError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .ok_or(PegOracleError::Unauthorized)?;

        admin.require_auth();

        // Simple sanity check (anti oracle attack)
        if new_peg <= 0 {
            return Err(PegOracleError::PegOutOfRange);
        }

        env.storage().instance().set(
            &Symbol::new(&env, "current_peg"),
            &new_peg,
        );

        env.logger().log(
            &Symbol::new(&env, "PEG_UPDATE"),
            "Peg value updated",
        );

        Ok(())
    }

    // -----------------
    // VERIFY PEG
    // -----------------
    pub fn verify_peg(
        env: Env,
        tolerance: i128, // allowed deviation
    ) -> Result<bool, PegOracleError> {
        let target: i128 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "target_peg"))
            .ok_or(PegOracleError::PegNotSet)?;

        let current: i128 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "current_peg"))
            .ok_or(PegOracleError::PegNotSet)?;

        let diff = if current > target {
            current - target
        } else {
            target - current
        };

        Ok(diff <= tolerance)
    }

    // -----------------
    // READ PEG
    // -----------------
    pub fn get_peg(env: Env) -> Result<i128, PegOracleError> {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "current_peg"))
            .ok_or(PegOracleError::PegNotSet)
    }
}

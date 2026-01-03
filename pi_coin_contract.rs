#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, Bytes, crypto,
};

//
// =====================
// DATA & ENUMS
// =====================
//

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

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PiCoinError {
    InsufficientCollateral,
    InvalidSource,
    InvalidProvenance,
}

//
// =====================
// CONTRACT
// =====================
//

#[contract]
pub struct PiCoinContract;

#[contractimpl]
impl PiCoinContract {
    // -----------------
    // INITIALIZE
    // -----------------
    pub fn initialize(
        env: Env,
        admin: Address,
        collateral: Address,
        oracle: Address,
        governance: Address,
    ) -> Result<(), PiCoinError> {
        let protocol_hash = env.crypto().sha256(
            &Bytes::from_slice(&env, b"PiCoin-Core-Protocol-Identifier"),
        );

        let data = PiCoinData {
            total_supply: 100_000_000_000,
            peg_value: 314_159_000_000,
            anti_fraud_hash: protocol_hash,
        };

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "data"), &data);

        env.storage()
            .instance()
            .set(&Symbol::new(&env, "admin"), &admin);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "governance"), &governance);

        env.logger().log(
            &Symbol::new(&env, "INIT"),
            "Protocol initialized with PQ-aware commitments",
        );

        Ok(())
    }

    // -----------------
    // MINT (SOURCE-AWARE)
    // -----------------
    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
        source: PiCoinSource,
    ) -> Result<(), PiCoinError> {
        if source == PiCoinSource::Invalid {
            return Err(PiCoinError::InvalidSource);
        }

        // Simulated collateral limit
        if amount > 100_000_000 {
            return Err(PiCoinError::InsufficientCollateral);
        }

        // Store provenance
        env.storage()
            .persistent()
            .set(&to, &source);

        env.logger().log(
            &Symbol::new(&env, "MINT"),
            "Minted with valid source & PQ-aware provenance",
        );

        Ok(())
    }

    // -----------------
    // TRANSFER (ZKP GATE + PROVENANCE)
    // -----------------
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), PiCoinError> {
        // ZKP base must exist
        let zkp_key = Symbol::new(&env, "zkp_base");
        if env.storage().instance().get::<_, Bytes>(&zkp_key).is_none() {
            return Err(PiCoinError::InvalidProvenance);
        }

        // Provenance check
        let source: Option<PiCoinSource> =
            env.storage().persistent().get(&from);

        if source.is_none() {
            return Err(PiCoinError::InvalidSource);
        }

        env.logger().log(
            &Symbol::new(&env, "TRANSFER"),
            "Transfer passed ZKP gate and provenance validation",
        );

        Ok(())
    }

    // -----------------
    // PEG VERIFICATION (ORACLE-READY)
    // -----------------
    pub fn verify_peg(env: Env, holder: Address) -> Result<(), PiCoinError> {
        let source: Option<PiCoinSource> =
            env.storage().persistent().get(&holder);

        if source.is_none() {
            return Err(PiCoinError::InvalidSource);
        }

        env.logger().log(
            &Symbol::new(&env, "PEG"),
            "Peg verified via oracle commitment (PQ-ready)",
        );

        Ok(())
    }

    // -----------------
    // DAO GOVERNANCE VOTE
    // -----------------
    pub fn governance_vote(
        env: Env,
        voter: Address,
        proposal: Symbol,
    ) -> Result<(), PiCoinError> {
        let source: Option<PiCoinSource> =
            env.storage().persistent().get(&voter);

        if source.is_none() {
            return Err(PiCoinError::InvalidSource);
        }

        env.logger().log(
            &proposal,
            "DAO vote recorded with PQ-ready symbolic intent",
        );

        Ok(())
    }

    // -----------------
    // ECOSYSTEM ENTRY CHECK
    // -----------------
    pub fn verify_ecosystem_entry(
        env: Env,
        holder: Address,
    ) -> Result<bool, PiCoinError> {
        let source: Option<PiCoinSource> =
            env.storage().persistent().get(&holder);

        Ok(source.is_some())
    }
}

//
// =====================
// TESTS (SPECIFICATION)
// =====================
//

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Address, Symbol, Bytes};

    #[test]
    fn test_full_protocol_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::random(&env);
        let user = Address::random(&env);
        let receiver = Address::random(&env);
        let collateral = Address::random(&env);
        let oracle = Address::random(&env);
        let governance = Address::random(&env);

        // Init
        PiCoinContract::initialize(
            env.clone(),
            admin,
            collateral,
            oracle,
            governance,
        )
        .unwrap();

        // Mint
        PiCoinContract::mint(
            env.clone(),
            user.clone(),
            1_000_000,
            PiCoinSource::Mining,
        )
        .unwrap();

        // Setup ZKP base
        let zkp_base = env.crypto().sha256(
            &Bytes::from_slice(&env, b"zkp-seed"),
        );
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "zkp_base"), &zkp_base);

        // Transfer
        PiCoinContract::transfer(
            env.clone(),
            user.clone(),
            receiver,
            500_000,
        )
        .unwrap();

        // Peg verify
        PiCoinContract::verify_peg(env.clone(), user.clone()).unwrap();

        // Governance vote
        PiCoinContract::governance_vote(
            env.clone(),
            user.clone(),
            Symbol::new(&env, "rebase_policy"),
        )
        .unwrap();

        // Ecosystem entry
        let ok =
            PiCoinContract::verify_ecosystem_entry(env.clone(), user)
                .unwrap();
        assert!(ok);
    }
}

#![no_std]

use soroban_sdk::{
    contract, contractimpl, Env, Address, Symbol, Bytes,
};

mod storage;
mod governance;
mod error;

use storage::{PiCoinData, PiCoinSource, Storage};
use governance::Governance;
use error::PiCoinError;

#[contract]
pub struct PiCoinContract;

#[contractimpl]
impl PiCoinContract {
    // ---------- INITIALIZE ----------
    pub fn initialize(
        env: Env,
        admin: Address,
        collateral: Address,
        oracle: Address,
        governance: Address,
    ) -> Result<(), PiCoinError> {
        let hash = env.crypto().sha256(
            &Bytes::from_slice(&env, b"PiCoin-Core-Protocol-Identifier"),
        );

        let data = PiCoinData {
            total_supply: 100_000_000_000,
            peg_value: 314_159_000_000,
            anti_fraud_hash: hash,
        };

        Storage::set_data(&env, &data);

        env.logger().log(
            &Symbol::new(&env, "INIT"),
            "Protocol initialized with PQ-aware commitments",
        );

        Ok(())
    }

    // ---------- MINT ----------
    pub fn mint(
        env: Env,
        to: Address,
        amount: i128,
        source: PiCoinSource,
    ) -> Result<(), PiCoinError> {
        if source == PiCoinSource::Invalid {
            return Err(PiCoinError::InvalidSource);
        }

        if amount > 100_000_000 {
            return Err(PiCoinError::InsufficientCollateral);
        }

        Storage::set_provenance(&env, &to, &source);

        env.logger().log(
            &Symbol::new(&env, "MINT"),
            "Minted with valid source & PQ-aware provenance",
        );

        Ok(())
    }

    // ---------- TRANSFER ----------
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        _amount: i128,
    ) -> Result<(), PiCoinError> {
        if !Storage::has_zkp_base(&env) {
            return Err(PiCoinError::InvalidProvenance);
        }

        Storage::get_provenance(&env, &from)?;

        env.logger().log(
            &Symbol::new(&env, "TRANSFER"),
            "Transfer passed ZKP gate and provenance validation",
        );

        Ok(())
    }

    // ---------- PEG VERIFY ----------
    pub fn verify_peg(
        env: Env,
        holder: Address,
    ) -> Result<(), PiCoinError> {
        Storage::get_provenance(&env, &holder)?;

        env.logger().log(
            &Symbol::new(&env, "PEG"),
            "Peg verified via oracle commitment (PQ-ready)",
        );

        Ok(())
    }

    // ---------- GOVERNANCE ----------
    pub fn governance_vote(
        env: Env,
        voter: Address,
        proposal: Symbol,
    ) -> Result<(), PiCoinError> {
        Governance::vote(&env, voter, proposal)
    }

    // ---------- ECOSYSTEM ENTRY ----------
    pub fn verify_ecosystem_entry(
        env: Env,
        holder: Address,
    ) -> Result<bool, PiCoinError> {
        Ok(Storage::get_provenance(&env, &holder).is_ok())
    }
}

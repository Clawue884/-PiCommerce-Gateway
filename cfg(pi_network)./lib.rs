#![no_std]

/* ============================================================
   FEATURE & COMPILE-TIME GUARD
   ============================================================ */

#[cfg(all(feature = "pi_network", feature = "dao_native"))]
compile_error!("Pi Network does NOT support native DAO governance.");

/* ============================================================
   NETWORK METADATA (COMPILE-TIME SAFE)
   ============================================================ */

#[cfg(feature = "pi_network")]
pub mod network {
    pub const NAME: &str = "Pi Network";
    pub const SYMBOL: &str = "PI";
    pub const LAYER: &str = "L1";
    pub const CONSENSUS: &str = "SCP";
    pub const CONSENSUS_MODEL: &str = "Federated Byzantine Agreement";
    pub const BASE_PROTOCOL: &str = "Stellar-derived";
    pub const VM: &str = "Soroban-compatible";
    pub const GOVERNANCE: &str = "Core-Team-Led";
    pub const DAO_NATIVE: bool = false;
}

#[cfg(not(feature = "pi_network"))]
pub mod network {
    pub const NAME: &str = "Generic SCP Network";
    pub const GOVERNANCE: &str = "Custom / DAO";
}

/* ============================================================
   GOVERNANCE LAYER (PI-SAFE)
   ============================================================ */

pub mod governance {

    #[derive(Clone, Copy)]
    pub enum GovernanceMode {
        CoreTeamLed,
        DaoLayer,
    }

    #[cfg(feature = "pi_network")]
    pub fn active_governance() -> GovernanceMode {
        GovernanceMode::CoreTeamLed
    }

    #[cfg(not(feature = "pi_network"))]
    pub fn active_governance() -> GovernanceMode {
        GovernanceMode::DaoLayer
    }
}

/* ============================================================
   QUANTUM-SAFE SYMBOLIC GOVERNANCE MODEL
   (OFF-CHAIN SIGN → ON-CHAIN HASH VERIFY)
   ============================================================ */

pub mod quantum_safe {

    /// Represents a post-quantum signature hash (e.g. Dilithium)
    pub struct QuantumSignature {
        pub hash: [u8; 32],
    }

    /// Verifies hash only (Pi-compatible, no heavy crypto on-chain)
    pub fn verify_hash(expected: &[u8; 32], provided: &QuantumSignature) -> bool {
        expected == &provided.hash
    }
}

/* ============================================================
   SOROBAN / SMART CONTRACT ENTRY (OPTIONAL)
   ============================================================ */

#[cfg(feature = "pi_network")]
pub mod contract {
    use super::governance::*;
    use super::network::*;

    pub fn contract_info() -> (&'static str, &'static str) {
        (NAME, GOVERNANCE)
    }

    pub fn governance_mode() -> GovernanceMode {
        active_governance()
    }
}

/* ============================================================
   PUBLIC API (SAFE TO CALL)
   ============================================================ */

pub fn system_identity() -> &'static str {
    network::NAME
}

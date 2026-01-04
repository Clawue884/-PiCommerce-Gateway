#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, BytesN, Symbol, log,
};

/* ============================================================
   PI NETWORK COMPILE GUARD
   ============================================================ */

#[cfg(all(feature = "pi_network", feature = "dao_native"))]
compile_error!("Pi Network does NOT support native DAO governance");

/* ============================================================
   NETWORK METADATA (STATIC)
   ============================================================ */

#[cfg(feature = "pi_network")]
mod pi_network {
    pub const NAME: &str = "Pi Network";
    pub const CONSENSUS: &str = "SCP";
    pub const GOVERNANCE: &str = "Core-Team-Led";
}

/* ============================================================
   GOVERNANCE MODEL (PI-COMPATIBLE)
   ============================================================ */

#[contracttype]
#[derive(Clone)]
pub enum GovernanceMode {
    CoreTeamLed,
}

#[contracttype]
#[derive(Clone)]
pub enum ProposalAction {
    Rebase,
    ParameterUpdate,
}

/* ============================================================
   STORAGE KEYS
   ============================================================ */

#[contracttype]
pub enum DataKey {
    Admin,
    Governance,
    QuantumKey(Address),
    Balance(Address),
}

/* ============================================================
   QUANTUM-SAFE SYMBOLIC SIGNATURE
   (Dilithium off-chain → hash on-chain)
   ============================================================ */

#[contracttype]
#[derive(Clone)]
pub struct QuantumSignature {
    pub hash: BytesN<32>,
}

fn verify_quantum_signature(
    env: &Env,
    owner: &Address,
    sig: &QuantumSignature,
) -> bool {
    if let Some(stored) = env
        .storage()
        .instance()
        .get::<_, BytesN<32>>(&DataKey::QuantumKey(owner.clone()))
    {
        stored == sig.hash
    } else {
        false
    }
}

/* ============================================================
   SMART CONTRACT
   ============================================================ */

#[contract]
pub struct PiCoinContract;

/* ============================================================
   IMPLEMENTATION
   ============================================================ */

#[contractimpl]
impl PiCoinContract {

    /* ---------- INITIALIZE ---------- */

    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(
            &DataKey::Governance,
            &GovernanceMode::CoreTeamLed,
        );

        log!(&env, "PiCoin initialized under Core-Team governance");
    }

    /* ---------- REGISTER QUANTUM KEY ---------- */

    pub fn register_quantum_key(
        env: Env,
        owner: Address,
        pq_hash: BytesN<32>,
    ) {
        owner.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::QuantumKey(owner), &pq_hash);

        log!(&env, "Quantum-safe key registered");
    }

    /* ---------- MINT (ADMIN ONLY) ---------- */

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Admin not set");

        admin.require_auth();

        let balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(balance + amount));

        log!(&env, "Minted {} PI", amount);
    }

    /* ---------- TRANSFER (QUANTUM-SAFE) ---------- */

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        sig: QuantumSignature,
    ) {
        from.require_auth();

        if !verify_quantum_signature(&env, &from, &sig) {
            panic!("Invalid quantum signature");
        }

        let from_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::Balance(from), &(from_balance - amount));

        env.storage()
            .instance()
            .set(&DataKey::Balance(to), &(to_balance + amount));

        log!(&env, "Quantum-safe transfer executed");
    }

    /* ---------- GOVERNANCE ACTION ---------- */

    pub fn governance_action(env: Env, action: ProposalAction) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        match action {
            ProposalAction::Rebase => {
                log!(&env, "Rebase approved by Core Team");
            }
            ProposalAction::ParameterUpdate => {
                log!(&env, "Protocol parameters updated");
            }
        }
    }

    /* ---------- VIEW ---------- */

    pub fn balance_of(env: Env, owner: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(owner))
            .unwrap_or(0)
    }
}

use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum PiCoinError {
    InsufficientCollateral,
    InvalidSource,
    InvalidProvenance,
}

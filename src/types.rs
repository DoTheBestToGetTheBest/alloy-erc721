use alloy::primitives::{Address, U256};

#[derive(Debug, Clone)]
pub struct Token {
    pub address: Address,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub address: Address,
    pub token: Address,
    pub balance: U256,
}

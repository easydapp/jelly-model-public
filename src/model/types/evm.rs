use serde::{Deserialize, Serialize};

/// EVM compatible chain
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EvmChain {
    /// Ethereum
    #[serde(rename = "ethereum")]
    Ethereum, // chain id -> 1
    /// Ethereum test
    #[serde(rename = "ethereum-test-sepolia")]
    EthereumTestnetSepolia, // chain id -> 11155111

    /// Polygon
    #[serde(rename = "polygon")]
    Polygon, // chain id -> 137
    /// Polygon test
    #[serde(rename = "polygon-test-amoy")]
    PolygonTestnetAmoy, // chain id -> 80002

    /// BSC
    #[serde(rename = "bsc")]
    BinanceSmartChain, // chain id -> 56
    /// BSC test
    #[serde(rename = "bsc-test")]
    BinanceSmartChainTestnet, // chain id -> 97
}
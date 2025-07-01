use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenHeader {
    String(String),
    Headers(HashMap<String, String>),
}

#[derive(Debug, Clone)]
pub struct AlgoClientConfig {
    pub server: String,
    pub port: Option<u16>,
    pub token: Option<TokenHeader>,
}

impl AlgoClientConfig {
    pub fn new(server: String) -> Self {
        Self {
            server,
            port: None,
            token: None,
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_token(mut self, token: TokenHeader) -> Self {
        self.token = Some(token);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AlgoConfig {
    pub algod_config: AlgoClientConfig,
}

impl AlgoConfig {
    pub fn new(algod_config: AlgoClientConfig) -> Self {
        Self { algod_config }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkDetails {
    pub is_testnet: bool,
    pub is_mainnet: bool,
    pub is_localnet: bool,
    pub genesis_id: String,
    pub genesis_hash: String,
}

impl NetworkDetails {
    pub fn new(genesis_id: String, genesis_hash: String) -> Self {
        let is_localnet = genesis_id_is_localnet(&genesis_id);
        let is_testnet = genesis_id == "testnet-v1.0";
        let is_mainnet = genesis_id == "mainnet-v1.0";

        Self {
            is_testnet,
            is_mainnet,
            is_localnet,
            genesis_id,
            genesis_hash,
        }
    }
}

pub fn genesis_id_is_localnet(genesis_id: &str) -> bool {
    genesis_id == "devnet-v1" || genesis_id == "sandnet-v1" || genesis_id == "dockernet-v1"
}

pub trait AlgoClients {
    type AlgodClient;
    fn algod(&self) -> &Self::AlgodClient;
}

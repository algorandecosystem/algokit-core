use crate::network_client::{
    AlgoClientConfig, AlgoClients, AlgoConfig, AlgorandService, NetworkDetails, TokenHeader,
    genesis_id_is_localnet,
};
use algod_client::AlgodClient;
use algokit_http_client::DefaultHttpClient;
use base64::{Engine, engine::general_purpose};
use std::{env, sync::Arc};
use tokio::sync::OnceCell;

pub struct AlgoClientsImpl {
    pub algod: AlgodClient,
}

impl AlgoClients for AlgoClientsImpl {
    type AlgodClient = AlgodClient;

    fn algod(&self) -> &Self::AlgodClient {
        &self.algod
    }
}

pub struct ClientManager {
    clients: AlgoClientsImpl,
    cached_network_details: OnceCell<NetworkDetails>,
}

impl ClientManager {
    pub fn new(config: AlgoConfig) -> Self {
        let clients = AlgoClientsImpl {
            algod: Self::get_algod_client(&config.algod_config),
        };

        Self {
            clients,
            cached_network_details: OnceCell::new(),
        }
    }

    pub fn algod(&self) -> &AlgodClient {
        self.clients.algod()
    }

    pub async fn network(
        &self,
    ) -> Result<&NetworkDetails, Box<dyn std::error::Error + Send + Sync>> {
        self.cached_network_details
            .get_or_try_init(|| async {
                let params = self.algod().transaction_params().await?;
                Ok(NetworkDetails::new(
                    params.genesis_id.clone(),
                    general_purpose::STANDARD.encode(&params.genesis_hash),
                ))
            })
            .await
    }

    pub fn genesis_id_is_localnet(genesis_id: &str) -> bool {
        genesis_id_is_localnet(genesis_id)
    }

    pub async fn is_localnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_localnet)
    }

    pub async fn is_testnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_testnet)
    }

    pub async fn is_mainnet(&self) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.network().await?.is_mainnet)
    }

    pub fn get_config_from_environment_or_localnet() -> AlgoConfig {
        match env::var("ALGOD_SERVER") {
            Ok(_) => {
                let algod_config = Self::get_algod_config_from_environment();

                AlgoConfig { algod_config }
            }
            Err(_) => AlgoConfig {
                algod_config: Self::get_default_localnet_config(AlgorandService::Algod),
            },
        }
    }

    pub fn get_algod_config_from_environment() -> AlgoClientConfig {
        let server =
            env::var("ALGOD_SERVER").expect("ALGOD_SERVER environment variable must be defined");
        let mut config = AlgoClientConfig::new(server);

        let port = env::var("ALGOD_PORT").ok().and_then(|p| p.parse().ok());
        if let Some(port) = port {
            config = config.with_port(port);
        }

        let token = env::var("ALGOD_TOKEN").ok();
        if let Some(token) = token {
            config = config.with_token(TokenHeader::String(token));
        }

        config
    }

    pub fn get_algonode_config(network: &str, service: AlgorandService) -> AlgoClientConfig {
        let subdomain = service.algonode_subdomain();

        AlgoClientConfig::new(format!("https://{}-{}.algonode.cloud/", network, subdomain))
            .with_port(443)
    }

    pub fn get_default_localnet_config(service: AlgorandService) -> AlgoClientConfig {
        let port = service.default_localnet_port();

        AlgoClientConfig::new("http://localhost".to_string())
            .with_port(port)
            .with_token(TokenHeader::String(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            ))
    }

    pub fn get_algod_client(config: &AlgoClientConfig) -> AlgodClient {
        let base_url = if let Some(port) = config.port {
            format!("{}:{}", config.server, port)
        } else {
            config.server.clone()
        };

        let http_client = match &config.token {
            Some(TokenHeader::String(token)) => Arc::new(
                DefaultHttpClient::with_header(&base_url, "X-Algo-API-Token", token)
                    .expect("Failed to create HTTP client with token header"),
            ),
            Some(TokenHeader::Headers(headers)) => {
                let (header_name, header_value) = headers
                    .iter()
                    .next()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .unwrap_or(("X-Algo-API-Token", ""));
                Arc::new(
                    DefaultHttpClient::with_header(&base_url, header_name, header_value)
                        .expect("Failed to create HTTP client with custom header"),
                )
            }
            None => Arc::new(DefaultHttpClient::new(&base_url)),
        };

        AlgodClient::new(http_client)
    }

    pub fn get_algod_client_from_environment() -> AlgodClient {
        Self::get_algod_client(&Self::get_algod_config_from_environment())
    }
}

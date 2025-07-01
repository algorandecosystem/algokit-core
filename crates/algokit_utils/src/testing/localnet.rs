use log::{debug, error, info, warn};
use std::process::Command;
use std::time::Duration;
use tokio::sync::OnceCell;
use tokio::time::sleep;

pub struct LocalnetManager;

static LOCALNET_INIT: OnceCell<Result<(), String>> = OnceCell::const_new();

#[derive(Debug)]
pub struct HealthCheckResult {
    pub is_running: bool,
    pub url: String,
    pub status_code: Option<u16>,
    pub response_body: Option<String>,
    pub error_message: Option<String>,
}

impl LocalnetManager {
    pub fn get_algod_url() -> String {
        std::env::var("ALGORAND_HOST").unwrap_or_else(|_| "http://localhost:4001".to_string())
    }

    pub async fn is_running() -> HealthCheckResult {
        let url = format!("{}/health", Self::get_algod_url());
        debug!(target: "localnet", "Checking health endpoint: {}", url);

        match reqwest::get(&url).await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                debug!(
                    target: "localnet",
                    "Health check response: status={} ({})",
                    status_code,
                    response.status().canonical_reason().unwrap_or("Unknown")
                );

                match response.text().await {
                    Ok(body) => {
                        debug!(target: "localnet", "Health check response body: {}", body);
                        HealthCheckResult {
                            is_running: status_code == 200,
                            url,
                            status_code: Some(status_code),
                            response_body: Some(body),
                            error_message: None,
                        }
                    }
                    Err(e) => {
                        warn!(target: "localnet", "Failed to read health check response body: {}", e);
                        HealthCheckResult {
                            is_running: status_code == 200,
                            url,
                            status_code: Some(status_code),
                            response_body: None,
                            error_message: Some(format!("Failed to read response body: {}", e)),
                        }
                    }
                }
            }
            Err(e) => {
                debug!(target: "localnet", "Health check connection failed: {}", e);
                HealthCheckResult {
                    is_running: false,
                    url,
                    status_code: None,
                    response_body: None,
                    error_message: Some(format!("Connection failed: {}", e)),
                }
            }
        }
    }

    pub async fn ensure_running() -> Result<(), String> {
        let result = LOCALNET_INIT
            .get_or_init(|| async { Self::initialize_localnet().await })
            .await;

        result.clone()
    }

    async fn initialize_localnet() -> Result<(), String> {
        info!(target: "localnet", "Ensuring localnet is running (thread-safe initialization)");

        debug!(target: "localnet", "Checking prerequisites");
        Self::check_prerequisites()?;

        debug!(target: "localnet", "Performing initial health check");
        let initial_check = Self::is_running().await;
        if initial_check.is_running {
            info!(target: "localnet", "LocalNet is already running and healthy");
            return Ok(());
        }

        info!(target: "localnet", "LocalNet not running, attempting to start");

        match Self::start_localnet().await {
            Ok(()) => {
                Self::wait_for_ready().await?;
                info!(target: "localnet", "LocalNet started successfully");
                Ok(())
            }
            Err(e) => {
                warn!(target: "localnet", "Localnet start command failed: {}", e);
                info!(target: "localnet", "Checking if localnet is actually running despite start failure");

                tokio::time::sleep(Duration::from_secs(3)).await;
                let retry_check = Self::is_running().await;

                if retry_check.is_running {
                    warn!(target: "localnet", "Localnet appears to be running despite start command failure - proceeding");
                    Ok(())
                } else {
                    error!(target: "localnet", "Localnet start failed and health check confirms it's not running");
                    error!(target: "localnet", "Health check details: {:?}", retry_check);
                    Err(format!(
                        "Failed to start localnet: {} - Health check details: {:?}",
                        e, retry_check
                    ))
                }
            }
        }
    }

    fn check_prerequisites() -> Result<(), String> {
        match Command::new("docker").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                debug!(target: "localnet", "Docker available: {}", version);
            }
            Err(e) => {
                error!(target: "localnet", "Docker not available: {}", e);
                return Err(format!("Docker not available: {}", e));
            }
        }

        match Command::new("algokit").arg("--version").output() {
            Ok(output) => {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                debug!(target: "localnet", "AlgoKit available: {}", version);
            }
            Err(e) => {
                error!(target: "localnet", "AlgoKit not available: {}", e);
                return Err(format!("AlgoKit not available: {}", e));
            }
        }

        let algorand_host = std::env::var("ALGORAND_HOST");
        let docker_host = std::env::var("DOCKER_HOST");
        debug!(target: "localnet", "Environment: ALGORAND_HOST={:?}, DOCKER_HOST={:?}",
               algorand_host, docker_host);

        Ok(())
    }

    async fn start_localnet() -> Result<(), String> {
        debug!(target: "localnet", "Executing: algokit localnet start");

        let output = Command::new("algokit")
            .args(["localnet", "start"])
            .output()
            .map_err(|e| {
                error!(target: "localnet", "Failed to execute algokit command: {}", e);
                format!("Failed to execute algokit command: {}", e)
            })?;

        debug!(target: "localnet", "Command completed with exit status: {}", output.status);

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !stdout.is_empty() {
            debug!(target: "localnet", "Command stdout: {}", stdout);
        }
        if !stderr.is_empty() {
            debug!(target: "localnet", "Command stderr: {}", stderr);
        }

        if !output.status.success() {
            error!(target: "localnet", "Failed to start localnet: exit_code={}, stderr={}",
                   output.status, stderr);
            return Err(format!("Failed to start localnet: {}", stderr));
        }

        Ok(())
    }

    async fn wait_for_ready() -> Result<(), String> {
        debug!(target: "localnet", "Waiting for localnet to become ready");

        const MAX_ATTEMPTS: u32 = 30;
        const RETRY_DELAY: Duration = Duration::from_secs(2);

        for attempt in 1..=MAX_ATTEMPTS {
            debug!(target: "localnet", "Health check attempt {}/{}", attempt, MAX_ATTEMPTS);

            let health_check = Self::is_running().await;
            if health_check.is_running {
                info!(target: "localnet", "LocalNet is ready after {} attempts", attempt);
                return Ok(());
            }

            if let Some(ref error_msg) = health_check.error_message {
                debug!(target: "localnet", "Health check failed: {}", error_msg);
            }

            if attempt < MAX_ATTEMPTS {
                debug!(target: "localnet", "Waiting {}s before next attempt", RETRY_DELAY.as_secs());
                sleep(RETRY_DELAY).await;
            }
        }

        error!(target: "localnet", "Timed out waiting for localnet after {} attempts", MAX_ATTEMPTS);
        Err(format!(
            "Timed out waiting for localnet to start after {} attempts",
            MAX_ATTEMPTS
        ))
    }
}

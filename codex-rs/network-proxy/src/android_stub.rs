//! Android/Termux stub implementation.
//!
//! The network proxy runtime (Rama-based) is not supported on Android/Termux,
//! but the rest of the workspace depends on these types unconditionally.
//! Provide a small, no-op implementation so Codex can run with networking
//! effectively "unmanaged" on Android.

use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    Limited,
    #[default]
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct NetworkProxyConfig {
    #[serde(default)]
    pub network: NetworkProxySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct NetworkProxySettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub proxy_url: String,
    #[serde(default)]
    pub admin_url: String,
    #[serde(default)]
    pub enable_socks5: bool,
    #[serde(default)]
    pub socks_url: String,
    #[serde(default)]
    pub enable_socks5_udp: bool,
    #[serde(default)]
    pub allow_upstream_proxy: bool,
    #[serde(default)]
    pub dangerously_allow_non_loopback_proxy: bool,
    #[serde(default)]
    pub dangerously_allow_non_loopback_admin: bool,
    #[serde(default)]
    pub mode: NetworkMode,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub denied_domains: Vec<String>,
    #[serde(default)]
    pub allow_unix_sockets: Vec<String>,
    #[serde(default)]
    pub allow_local_binding: bool,
}

impl Default for NetworkProxySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_url: "http://127.0.0.1:3128".to_string(),
            admin_url: "http://127.0.0.1:8080".to_string(),
            enable_socks5: true,
            socks_url: "http://127.0.0.1:8081".to_string(),
            enable_socks5_udp: true,
            allow_upstream_proxy: true,
            dangerously_allow_non_loopback_proxy: false,
            dangerously_allow_non_loopback_admin: false,
            mode: NetworkMode::default(),
            allowed_domains: Vec::new(),
            denied_domains: Vec::new(),
            allow_unix_sockets: Vec::new(),
            allow_local_binding: true,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NetworkProxyConstraints {
    pub enabled: Option<bool>,
    pub mode: Option<NetworkMode>,
    pub allow_upstream_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_admin: Option<bool>,
    pub allowed_domains: Option<Vec<String>>,
    pub denied_domains: Option<Vec<String>>,
    pub allow_unix_sockets: Option<Vec<String>>,
    pub allow_local_binding: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PartialNetworkProxyConfig {
    #[serde(default)]
    pub network: PartialNetworkConfig,
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PartialNetworkConfig {
    pub enabled: Option<bool>,
    pub mode: Option<NetworkMode>,
    pub allow_upstream_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_proxy: Option<bool>,
    pub dangerously_allow_non_loopback_admin: Option<bool>,
    #[serde(default)]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(default)]
    pub denied_domains: Option<Vec<String>>,
    #[serde(default)]
    pub allow_unix_sockets: Option<Vec<String>>,
    #[serde(default)]
    pub allow_local_binding: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct NetworkProxyConstraintError {
    message: String,
}

impl NetworkProxyConstraintError {
    pub fn into_anyhow(self) -> anyhow::Error {
        anyhow::anyhow!(self.message)
    }
}

impl std::fmt::Display for NetworkProxyConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl std::error::Error for NetworkProxyConstraintError {}

pub fn validate_policy_against_constraints(
    _config: &NetworkProxyConfig,
    _constraints: &NetworkProxyConstraints,
) -> std::result::Result<(), NetworkProxyConstraintError> {
    // Android/Termux: no managed proxy enforcement, but keep config loading working.
    Ok(())
}

#[derive(Debug, Clone, Default)]
pub struct ConfigState {
    pub config: NetworkProxyConfig,
    pub constraints: NetworkProxyConstraints,
}

pub fn build_config_state(
    config: NetworkProxyConfig,
    constraints: NetworkProxyConstraints,
) -> Result<ConfigState> {
    Ok(ConfigState {
        config,
        constraints,
    })
}

#[async_trait]
pub trait ConfigReloader: Send + Sync {
    fn source_label(&self) -> String;
    async fn maybe_reload(&self) -> Result<Option<ConfigState>>;
    async fn reload_now(&self) -> Result<ConfigState>;
}

#[derive(Clone)]
pub struct NetworkProxyState {
    state: ConfigState,
    _reloader: Arc<dyn ConfigReloader>,
}

impl std::fmt::Debug for NetworkProxyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Avoid printing trait object details; this is a stub on Android anyway.
        f.debug_struct("NetworkProxyState").finish_non_exhaustive()
    }
}

impl NetworkProxyState {
    pub fn with_reloader(state: ConfigState, reloader: Arc<dyn ConfigReloader>) -> Self {
        Self {
            state,
            _reloader: reloader,
        }
    }

    pub fn current_cfg(&self) -> Result<NetworkProxyConfig> {
        Ok(self.state.config.clone())
    }

    pub fn enabled(&self) -> Result<bool> {
        Ok(false)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Args;

impl Args {
    pub fn parse() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct NetworkProxy {
    _state: Option<Arc<NetworkProxyState>>,
    http_addr: SocketAddr,
    socks_addr: SocketAddr,
    admin_addr: SocketAddr,
}

impl Default for NetworkProxy {
    fn default() -> Self {
        Self {
            _state: None,
            http_addr: SocketAddr::from(([127, 0, 0, 1], 3128)),
            socks_addr: SocketAddr::from(([127, 0, 0, 1], 8081)),
            admin_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct NetworkProxyBuilder {
    state: Option<Arc<NetworkProxyState>>,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkProxyHandle;

impl NetworkProxy {
    pub fn builder() -> NetworkProxyBuilder {
        NetworkProxyBuilder { state: None }
    }

    pub fn http_addr(&self) -> SocketAddr {
        self.http_addr
    }

    pub fn socks_addr(&self) -> SocketAddr {
        self.socks_addr
    }

    pub fn admin_addr(&self) -> SocketAddr {
        self.admin_addr
    }

    pub fn apply_to_env(&self, _env: &mut HashMap<String, String>) {
        // Android/Termux: managed proxy is disabled; do not inject proxy env vars.
    }
}

impl NetworkProxyBuilder {
    pub fn state(mut self, state: Arc<NetworkProxyState>) -> Self {
        self.state = Some(state);
        self
    }

    pub async fn build(self) -> Result<NetworkProxy> {
        Ok(NetworkProxy {
            _state: self.state,
            http_addr: SocketAddr::from(([127, 0, 0, 1], 3128)),
            socks_addr: SocketAddr::from(([127, 0, 0, 1], 8081)),
            admin_addr: SocketAddr::from(([127, 0, 0, 1], 8080)),
        })
    }
}

impl NetworkProxy {
    pub async fn run(&self) -> Result<NetworkProxyHandle> {
        Ok(NetworkProxyHandle)
    }
}

impl NetworkProxyHandle {
    pub async fn wait(self) -> Result<()> {
        Ok(())
    }
}

pub fn host_and_port_from_network_addr(network_addr: &str, default_port: u16) -> String {
    // Very small parser: we only need to support the typical values in config (`http://host:port`)
    // and return a "host:port" string.
    let trimmed = network_addr.trim();
    if trimmed.is_empty() {
        return format!("127.0.0.1:{default_port}");
    }

    let without_scheme = trimmed
        .split("://")
        .nth(1)
        .unwrap_or(trimmed)
        .split('/')
        .next()
        .unwrap_or(trimmed);

    if without_scheme.contains(':') {
        without_scheme.to_string()
    } else {
        format!("{without_scheme}:{default_port}")
    }
}

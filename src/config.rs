use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ConnectorConfig {
    pub(crate) base_url: String,
    pub(crate) dsp_url: String,
    pub(crate) bpn: String,
    pub(crate) api_key: Option<String>,
}

impl ConnectorConfig {
    pub(crate) fn from_file(file_path: &str) -> Result<Self> {
        let bytes = std::fs::read(file_path).context("failed to read connector config")?;
        let config = serde_yaml::from_slice(&bytes).context("failed to deserialize config")?;
        Ok(config)
    }
}

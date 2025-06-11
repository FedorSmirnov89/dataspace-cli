mod config;
mod consumer;
mod provider;
mod urls;

use anyhow::{Context, Result, anyhow};
pub use consumer::{AssetAccess, get_asset_access};
pub use provider::provide_data;

pub(crate) use consumer::Catalogue;

use crate::{config::ConnectorConfig, urls::catalogue_request_url};

const ASSET_REQUEST_URL: &str =
    "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/v3/assets/request";

pub async fn query_assets() {
    let client = reqwest::Client::new();

    // First request to get the key
    let key = client
        .post(ASSET_REQUEST_URL)
        .header("x-api-key", "YM2dSU2b")
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("failed to do the request");

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

pub async fn read_catalogue(consumer_file: &str, provider_file: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let consumer_config = ConnectorConfig::from_file(consumer_file)?;
    let provider_config = ConnectorConfig::from_file(provider_file)?;

    let body = format!(
        r#"
    {{
"@context": {{
  "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
  }},
"protocol": "dataspace-protocol-http",
"counterPartyAddress": "{provider_dsp_url}",
"counterPartyId": "{provider_bpn}",
"querySpec": {{
  "offset": 0,
  "limit": 50
  }}
}}
  "#,
        provider_dsp_url = &provider_config.dsp_url,
        provider_bpn = &provider_config.bpn
    );

    let consumer_api_key = consumer_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let key = client
        .post(catalogue_request_url(&consumer_config))
        .header("x-api-key", consumer_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("requesting the catalogue")?;

    let response_text = key
        .text()
        .await
        .context("reading catalogue response as json")?;

    let catalogue: Catalogue =
        serde_json::from_str(&response_text).context("failed to deser catalogue")?;

    for entry in catalogue.dataset {
        println!("catalogue entry: {entry:?}");
    }

    Ok(())
}

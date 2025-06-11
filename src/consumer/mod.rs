use anyhow::{Context, Result, anyhow, bail};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    config::ConnectorConfig,
    urls::{catalogue_request_url, edr_read_url, edrs_url, request_edr_url},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetAccess {
    #[serde(rename = "endpoint")]
    pub asset_url: String,
    #[serde(rename = "authorization")]
    pub auth_token: String,
}

// Negotiates the asset access with the provider connector
// Returns the url and the authorization token
pub async fn get_asset_access(
    asset_id: &str,
    consumer_config: &str,
    provider_config: &str,
) -> Result<AssetAccess> {
    let client = Client::new();
    let consumer_config = ConnectorConfig::from_file(consumer_config)?;
    let provider_config = ConnectorConfig::from_file(provider_config)?;

    let offer_id =
        read_policy_id_from_catalogue(&client, &consumer_config, &provider_config, asset_id)
            .await?;
    negotiate_edr(
        &client,
        &offer_id,
        asset_id,
        &provider_config,
        &consumer_config,
    )
    .await?;

    let transfer_id = request_edr(&client, asset_id, &consumer_config).await?;
    read_edr_details(&client, &consumer_config, &transfer_id).await
}

async fn read_edr_details(
    client: &Client,
    consumer_config: &ConnectorConfig,
    transfer_id: &str,
) -> Result<AssetAccess> {
    let edr_read_url = edr_read_url(consumer_config, transfer_id);

    let consumer_api_key = consumer_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let key = client
        .get(edr_read_url)
        .header("x-api-key", consumer_api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("things failed");

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    let result: AssetAccess =
        serde_json::from_str(&response_text).context("failed to deser asset access")?;

    Ok(result)
}

#[derive(Deserialize, Debug)]
struct Edr {
    #[serde(rename = "assetId")]
    asset_id: String,

    #[serde(rename = "transferProcessId")]
    transfer_id: String,
}

// Retrieves the transfer ID for the asset we are trying to get
async fn request_edr(
    client: &Client,
    asset_id: &str,
    consumer_config: &ConnectorConfig,
) -> Result<String> {
    let body = format!(
        r#"
    {{
  "@context": {{
    "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    }},
  "@type": "QuerySpec",
  "filterExpression": [
    {{
      "operandLeft": "assetId",
      "operator": "=",
      "operandRight": "{asset_id}"
    }}
  ]
}}
  "#
    );

    let consumer_api_key = consumer_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let key = client
        .post(request_edr_url(consumer_config))
        .header("x-api-key", consumer_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("requesting edrs")?;

    let response_text = key.text().await?;

    let response: Vec<Edr> = serde_json::from_str(&response_text).context("deser edr response")?;

    let transfer_id = response
        .into_iter()
        .find_map(|edr| {
            if edr.asset_id == asset_id {
                Some(edr.transfer_id)
            } else {
                None
            }
        })
        .ok_or(anyhow!("no transfer id found for asset id {asset_id}"))?;

    Ok(transfer_id)
}

async fn negotiate_edr(
    client: &Client,
    offer_id: &str,
    asset_id: &str,
    provider_config: &ConnectorConfig,
    consumer_config: &ConnectorConfig,
) -> Result<()> {
    let body = format!(
        r#"
{{
  "@context": [
    "https://w3id.org/tractusx/policy/v1.0.0",
    "http://www.w3.org/ns/odrl.jsonld",
    {{
      "@vocab": "https://w3id.org/edc/v0.0.1/ns/",
      "tx": "https://w3id.org/tractusx/v0.0.1/ns/"
    }}
  ],
  "@type": "ContractRequest",
  "counterPartyAddress": "{provider_dsp}",
  "protocol": "dataspace-protocol-http",
  "policy": {{
    "@id": "{offer_id}",
    "@type": "Offer",
    "assigner": "{provider_bpn}",
    "permission": [
      {{
        "action": "use",
        "constraint": {{
          "leftOperand": "tx:BusinessPartnerNumber",
          "operator": "eq",
          "rightOperand": "{consumer_bpn}"
        }}
      }}
    ],
    "prohibition": [],
    "obligation": [],
    "target": "{asset_id}"
  }},
  "callbackAddresses": []
}}
    "#,
        provider_dsp = provider_config.dsp_url,
        provider_bpn = provider_config.bpn,
        consumer_bpn = consumer_config.bpn
    );

    let edrs_url = edrs_url(consumer_config);

    let consumer_api_key = consumer_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    let response = client
        .post(edrs_url)
        .header("x-api-key", consumer_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("requesting edrs")?;

    match response.status().as_u16() {
        200 => Ok(()),
        other_code => bail!("edr negotiation failed with code {other_code}"),
    }
}

#[derive(Deserialize, Debug)]
struct Catalogue {
    #[serde(rename = "dcat:dataset")]
    dataset: Vec<CatalogueEntry>,
}

#[derive(Deserialize, Debug)]
struct CatalogueEntry {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "odrl:hasPolicy")]
    policy: Policy,
}

#[derive(Deserialize, Debug)]
struct Policy {
    #[serde(rename = "@id")]
    id: String,
}

// reads the catalogue and returns the policy id of the asset with the provided ID
async fn read_policy_id_from_catalogue(
    client: &Client,
    consumer_config: &ConnectorConfig,
    provider_config: &ConnectorConfig,
    asset_id: &str,
) -> Result<String> {
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
        .post(catalogue_request_url(consumer_config))
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

    let offer_id = catalogue
        .dataset
        .into_iter()
        .find_map(|entry| {
            if entry.id == asset_id {
                Some(entry.policy.id)
            } else {
                None
            }
        })
        .ok_or(anyhow!(
            "no policy for the asset id {asset_id} found in the catalogue"
        ))?;
    Ok(offer_id)
}

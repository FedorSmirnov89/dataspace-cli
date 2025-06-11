use anyhow::{Context, Result, anyhow, bail};
use reqwest::Client;

use crate::{
    config::ConnectorConfig,
    urls::{access_policy_url, asset_create_url, create_contract_url, usage_policy_url},
};

pub async fn provide_data(
    public_url: &str,
    asset_id: &str,
    provider_config: &str,
    consumer_config: &str,
) -> Result<()> {
    let provider_config = ConnectorConfig::from_file(provider_config)?;
    let consumer_config = ConnectorConfig::from_file(consumer_config)?;
    let http_client = Client::new();

    create_asset(&http_client, asset_id, public_url, &provider_config).await?;
    create_usage_policy(&http_client, asset_id, &provider_config).await?;
    create_access_policy(&http_client, asset_id, &provider_config, &consumer_config).await?;
    create_contract(&http_client, asset_id, &provider_config).await?;

    println!("asset {asset_id} created and access contract set up");
    Ok(())
}

fn usage_policy_id(asset_id: &str) -> String {
    format!("{asset_id}-usage")
}

fn access_policy_id(asset_id: &str) -> String {
    format!("{asset_id}-access")
}

fn contract_id(asset_id: &str) -> String {
    format!("{asset_id}-contract")
}

async fn create_contract(
    client: &Client,
    asset_id: &str,
    provider_config: &ConnectorConfig,
) -> Result<()> {
    let body = format!(
        r#"
{{
  "@context": {{}},
  "@id": "{contract_id}",
  "@type": "ContractDefinition",
  "accessPolicyId": "{usage_policy_id}",
  "contractPolicyId": "{access_policy_id}",
  "assetsSelector": {{
    "@type": "CriterionDto",
    "operandLeft": "https://w3id.org/edc/v0.0.1/ns/id",
    "operator": "=",
    "operandRight": "{asset_id}"
    }}
}}
    "#,
        contract_id = contract_id(asset_id),
        usage_policy_id = usage_policy_id(asset_id),
        access_policy_id = access_policy_id(asset_id)
    );

    let provider_api_key = provider_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let response = client
        .post(create_contract_url(provider_config))
        .header("x-api-key", provider_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("creating the contracty")?;

    match response.status().as_u16() {
        200 => Ok(()),
        other_code => bail!("contract creation request failed with code {other_code}"),
    }
}

async fn create_access_policy(
    client: &Client,
    asset_id: &str,
    provider_config: &ConnectorConfig,
    consumer_config: &ConnectorConfig,
) -> Result<()> {
    let body = format!(
        r#"
{{
  "@context": [
    "https://w3id.org/tractusx/edc/v0.0.1",
    "http://www.w3.org/ns/odrl.jsonld",
    {{
      "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    }}
  ],
  "@type": "PolicyDefinition",
  "@id": "{access_policy_id}",
  "policy": {{
    "@type": "Set",
    "permission": [
      {{
        "action": "use",
        "constraint": {{
          "leftOperand": "BusinessPartnerNumber",
          "operator": "eq",
          "rightOperand": "{consumer_bpn}"
        }}
    }}
    ]
    }}
}}
    "#,
        consumer_bpn = &consumer_config.bpn,
        access_policy_id = access_policy_id(asset_id)
    );

    let provider_api_key = provider_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    let access_policy_url = access_policy_url(provider_config);

    // First request to get the key
    let response = client
        .post(access_policy_url)
        .header("x-api-key", provider_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("updating access policy")?;

    match response.status().as_u16() {
        200 => Ok(()),
        other_code => bail!("access policy creation request failed with code {other_code}"),
    }
}

async fn create_usage_policy(
    client: &Client,
    asset_id: &str,
    provider_config: &ConnectorConfig,
) -> Result<()> {
    let body = format!(
        r#"
{{
  "@context": [
    "https://w3id.org/tractusx/edc/v0.0.1",
    "http://www.w3.org/ns/odrl.jsonld",
    {{
      "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    }}
  ],
  "@type": "PolicyDefinition",
  "@id": "{usage_policy}",
  "policy": {{
    "@type": "Set"
    }}
}}
    "#,
        usage_policy = usage_policy_id(asset_id)
    );

    let provider_api_key = provider_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let response = client
        .post(usage_policy_url(provider_config))
        .header("x-api-key", provider_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("creating the usage policy")?;

    let status = response.status();

    match status.as_u16() {
        200 => Ok(()),
        other_code => bail!("usage policy creation request failed with code {other_code}"),
    }
}

async fn create_asset(
    client: &Client,
    asset_id: &str,
    asset_url: &str,
    provider_config: &ConnectorConfig,
) -> Result<()> {
    let body = format!(
        r#"
      {{
            "@context": {{}},
            "@type": "Asset",
            "@id": "{asset_id}",
            "properties": {{
                "description": "informative description"
            }},
            "dataAddress": {{
                "@type": "DataAddress",
                "type": "HttpData",
                "baseUrl": "{asset_url}"
            }}
    }}
    "#
    );

    let asset_create_url = asset_create_url(provider_config);

    let provider_api_key = provider_config
        .api_key
        .clone()
        .ok_or(anyhow!("provider config does not specify API key"))?;

    // First request to get the key
    let response = client
        .post(asset_create_url)
        .header("x-api-key", provider_api_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .context("request to create asset failed")?;

    match response.status().as_u16() {
        200 => Ok(()),
        other_code => bail!("asset creation request failed with code {other_code}"),
    }
}

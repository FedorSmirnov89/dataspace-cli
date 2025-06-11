use direct_connect::get_asset_access;
use serde_json::Value;

const ASSET_ID: &str = "second id";

const PROVIDER_BPN: &str = "BPNL000000003TF1";
const CONSUMER_BPN: &str = "BPNL000000003TP1";

const CONSUMER_API_KEY: &str = "cHR83afM";
const PROVIDER_API_KEY: &str = "YM2dSU2b";

const PROVIDER_DSP_URL: &str =
    "https://thingkathon-connector-fedor-dsp.c-27d7c36.kyma.ondemand.com/api/v1/dsp";
const CONSUMER_DSP_URL: &str =
    "https://thingkathon-connector-paul-dsp.c-27d7c36.kyma.ondemand.com/api/v1/dsp";

const ASSET_URL: &str =
    "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/v3/assets";

const ASSET_REQUEST_URL: &str =
    "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/v3/assets/request";

const UPDATE_ACCESS_POLICY_URL: &str = "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/v3/policydefinitions";
const UPDATE_CONTRACT_POLICY_URL: &str = "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/v3/contractdefinitions";
const EDRS_URL: &str = "https://thingkathon-connector-fedor.c-27d7c36.kyma.ondemand.com/management/{CONSUMER_CONNECTOR_URL}/management/v3/edrs";
const EDRS_URL_CONSUMER: &str =
    "https://thingkathon-connector-paul.c-27d7c36.kyma.ondemand.com/management/v3/edrs";
const EDRS_REQUEST_URL_CONSUMER: &str =
    "https://thingkathon-connector-paul.c-27d7c36.kyma.ondemand.com/management/v3/edrs/request";

const CATALOGUE_URL: &str =
    "https://thingkathon-connector-paul.c-27d7c36.kyma.ondemand.com/management/v3/catalog/request";
const EDR_PULL_OUT_URL: &str = "https://thingkathon-connector-paul.c-27d7c36.kyma.ondemand.com/management/v3/edrs/a79c4f50-8ea3-4bc1-9f7e-5c14a10415af/dataaddress";

const URL: &str =
    "https://thingkathon-connector-fedor-dataplane.c-27d7c36.kyma.ondemand.com/api/public";

const AUTH_HEADER: &str = "eyJraWQiOiJwdWJsaWMta2V5LWFsaWFzIiwiYWxnIjoiUlMyNTYifQ.eyJpc3MiOiJCUE5MMDAwMDAwMDAzVEYxIiwiYXVkIjoiQlBOTDAwMDAwMDAwM1RQMSIsInN1YiI6IkJQTkwwMDAwMDAwMDNURjEiLCJleHAiOjE3NDk1NjU4NDMsImlhdCI6MTc0OTU2NTU0MywianRpIjoiOTdjODgyODItYjY0YS00MmVlLWI3YTYtYzIyYzUwNTU0ZGYwIn0.k2oDxZl-Rd3XVR0IB0NCgxwyrYq-7YdFsjZQO-2q5csEhZXsnxhj7-y99P9qOfuf3a7MeCrFj96IYvDt-2-ecRX10KMQ5y9ERU8ZVXSc9nfgUKhnOxzHKR6ZU2DGMb7cDwdA5pUXZ0juOkyQjdI4qluzYbllxP0AbFiPnhylWs14SCIZ3vQhbnbtxuXfQOMuvR8rfhSpxAgvOFayT4ReEFmmq0cN3bmMnh5pFS9tOqZ9zavZjG9LatQia7ZKuLrSWN2_w2qz9E2KNrVdZHN3v2a0Z_FlOjjrdtKBxsP1D35rfGACR0pfa0GBi2lN8DJjnYTqSqacEWWNLho5PL-ttw";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_asset_access("second id", "config_consumer.yaml", "config_provider.yaml").await?;
    Ok(())
}

const API_PUBLIC_URL: &str = "https://jsonplaceholder.typicode.com/todos/3";

async fn final_query() {
    let client = reqwest::Client::new();

    // First request to get the key
    let key = client
        .get(URL)
        .header("Authorization", AUTH_HEADER)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("things failed");

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn pull_out_edr() {
    let client = reqwest::Client::new();

    // First request to get the key
    let key = client
        .get(EDR_PULL_OUT_URL)
        .header("x-api-key", CONSUMER_API_KEY)
        .header("Content-Type", "application/json")
        .send()
        .await
        .expect("things failed");

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn make_edr_request() {
    let client = reqwest::Client::new();

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
      "operandRight": "{ASSET_ID}"
    }}
  ]
}}
  "#
    );

    // First request to get the key
    let key = client
        .post(EDRS_REQUEST_URL_CONSUMER)
        .header("x-api-key", CONSUMER_API_KEY)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("things failed");

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn request_catalogue() {
    let client = reqwest::Client::new();

    let body = format!(
        r#"
    {{
"@context": {{
  "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
  }},
"protocol": "dataspace-protocol-http",
"counterPartyAddress": "{PROVIDER_DSP_URL}",
"counterPartyId": "{PROVIDER_BPN}",
"querySpec": {{
  "offset": 0,
  "limit": 50
  }}
}}
  "#
    );

    // First request to get the key
    let key = client
        .post(CATALOGUE_URL)
        .header("x-api-key", CONSUMER_API_KEY)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("things failed");

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn receive_edrs() {
    let client = reqwest::Client::new();

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
  "counterPartyAddress": "{PROVIDER_DSP_URL}",
  "protocol": "dataspace-protocol-http",
  "policy": {{
    "@id": "bXkgc2Vjb25kIGNvbnRyYWN0:c2Vjb25kIGlk:MGZjYWU0NDEtZjYxYS00NmUxLWJkNzMtYmExMTZmYWZiZTU1",
    "@type": "Offer",
    "assigner": "{PROVIDER_BPN}",
    "permission": [
      {{
        "action": "use",
        "constraint": {{
          "leftOperand": "tx:BusinessPartnerNumber",
          "operator": "eq",
          "rightOperand": "{CONSUMER_BPN}"
        }}
      }}
    ],
    "prohibition": [],
    "obligation": [],
    "target": "{ASSET_ID}"
  }},
  "callbackAddresses": []
}}
    "#
    );

    // First request to get the key
    let key = client
        .post(EDRS_URL_CONSUMER)
        .header("x-api-key", CONSUMER_API_KEY)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn create_contract() {
    let client = reqwest::Client::new();

    let body = r#"
{
  "@context": {},
  "@id": "my second contract",
  "@type": "ContractDefinition",
  "accessPolicyId": "usage policy",
  "contractPolicyId": "access policy",
  "assetsSelector": {
    "@type": "CriterionDto",
    "operandLeft": "https://w3id.org/edc/v0.0.1/ns/id",
    "operator": "=",
    "operandRight": "second id"
  }
}
    "#;

    // First request to get the key
    let key = client
        .post(UPDATE_CONTRACT_POLICY_URL)
        .header("x-api-key", "YM2dSU2b")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn create_access_policy() {
    let client = reqwest::Client::new();

    let body = r#"
{
  "@context": [
    "https://w3id.org/tractusx/edc/v0.0.1",
    "http://www.w3.org/ns/odrl.jsonld",
    {
      "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    }
  ],
  "@type": "PolicyDefinition",
  "@id": "access policy",
  "policy": {
    "@type": "Set",
    "permission": [
      {
        "action": "use",
        "constraint": {
          "leftOperand": "BusinessPartnerNumber",
          "operator": "eq",
          "rightOperand": "BPNL000000003TP1"
        }
      }
    ]
  }
}
    "#;

    // First request to get the key
    let key = client
        .post(UPDATE_ACCESS_POLICY_URL)
        .header("x-api-key", "YM2dSU2b")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn create_usage_policy() {
    let client = reqwest::Client::new();

    let body = r#"
{
  "@context": [
    "https://w3id.org/tractusx/edc/v0.0.1",
    "http://www.w3.org/ns/odrl.jsonld",
    {
      "@vocab": "https://w3id.org/edc/v0.0.1/ns/"
    }
  ],
  "@type": "PolicyDefinition",
  "@id": "usage policy",
  "policy": {
    "@type": "Set"
  }
}
    "#;

    // First request to get the key
    let key = client
        .post(UPDATE_ACCESS_POLICY_URL)
        .header("x-api-key", "YM2dSU2b")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

async fn query_assets() {
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

async fn create_asset() {
    let client = reqwest::Client::new();

    let body = r#"
      {
  "@context": {},
  "@type": "Asset",
  "@id": "second id",
  "properties": {
    "description": "informative description"
  },
  "dataAddress": {
    "@type": "DataAddress",
    "type": "HttpData",
    "baseUrl": "https://jsonplaceholder.typicode.com/todos/3"
  }
}
    "#;

    // First request to get the key
    let key = client
        .post(ASSET_URL)
        .header("x-api-key", "YM2dSU2b")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    println!("Received response: {:?}", key);

    let response_text = key
        .text()
        .await
        .expect("trying to read response body as text");

    // let json: Value = key.json().await?;

    println!("received text: {:?}", response_text);
}

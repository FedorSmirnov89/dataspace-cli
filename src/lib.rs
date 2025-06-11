mod config;
mod consumer;
mod provider;
mod urls;

pub use consumer::{AssetAccess, get_asset_access};
pub use provider::provide_data;

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

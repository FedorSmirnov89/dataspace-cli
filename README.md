# Dataspace CLI

A command-line interface tool for managing data assets in a dataspace environment.

## Prerequisites

- Rust toolchain (1.75 or later recommended)
- Cargo (comes with Rust)

## Installation

1. Clone the repository
2. Build the binary:
```bash
cargo build --release
```

The compiled binary will be available at `target/release/dataspace_cli`

## Usage

To see all available commands and their options:
```bash
dataspace_cli -h
```

Each command also has its own help documentation:
```bash
dataspace_cli provide-data -h
dataspace_cli get-asset-access -h
```

The CLI provides two main commands:

### Provide Data

Makes data available at a public URL as an asset:

```bash
dataspace_cli provide-data \
    --public-url <URL> \
    --asset-id <ASSET_ID> \
    --provider-config <PROVIDER_CONFIG_PATH> \
    --consumer-config <CONSUMER_CONFIG_PATH>
```

### Get Asset Access

Retrieves access details for an asset:

```bash
dataspace_cli get-asset-access \
    --asset-id <ASSET_ID> \
    --consumer-config <CONSUMER_CONFIG_PATH> \
    --provider-config <PROVIDER_CONFIG_PATH>
```

The command outputs JSON in the following format:
```json
{
    "endpoint": "<asset_url>",
    "authorization": "<auth_token>"
}
```

## Configuration Files

Both commands require configuration files in YAML format:

### Provider Config Example
```yaml
base_url: "https://provider-connector.example.com"
dsp_url: "https://provider-dsp.example.com/api/v1/dsp"
bpn: "BPNL000000000000"
api_key: "your-api-key"  # Optional
```

### Consumer Config Example
```yaml
base_url: "https://consumer-connector.example.com"
dsp_url: "https://consumer-dsp.example.com/api/v1/dsp"
bpn: "BPNL000000000001"
api_key: "your-api-key"  # Optional
```

Note: The `api_key` field is optional in both configuration files. If your connector requires authentication, provide the API key; otherwise, you can omit this field.

## Python Example

An example Python script is provided to demonstrate how to use the CLI programmatically. The script shows how to:
- Call the CLI to get asset access details
- Parse the JSON output
- Use the access details to interact with the provider's server

### Setup

1. Build the CLI binary and copy it to the repository root:
```bash
cargo build --release
cp target/release/dataspace_cli .
```

2. Install Python dependencies:
```bash
pip install -r examples/requirements.txt
```

3. Prepare your configuration files:
   - Create `consumer_private.yaml` and `provider_private.yaml` in the repository root
   - Add your configuration details as shown in the Configuration Files section above

### Running the Example

```bash
python examples/get_asset_data.py
```

The script will:
1. Call the CLI to get access details for a specified asset
2. Print the received access details (endpoint and authorization token)
3. Demonstrate how to use these details for accessing the asset

You can modify the script to:
- Change the asset ID and configuration file paths
- Implement your own logic for interacting with the provider's server
- Add additional error handling

## Error Handling

The tool provides detailed error messages when:
- Configuration files are missing or malformed
- API keys are not provided (when required by the connector)
- Network requests fail
- Asset IDs are not found
- Authorization fails 
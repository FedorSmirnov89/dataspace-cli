#!/usr/bin/env python3
import json
import subprocess
import requests

def get_asset_access(asset_id: str, consumer_config: str, provider_config: str):
    """
    Call the dataspace CLI to get asset access details and return them as a dictionary.
    """
    # Call the CLI and capture its output
    result = subprocess.run([
        "./dataspace_cli",
        "get-asset-access",
        "--asset-id", asset_id,
        "--consumer-config", consumer_config,
        "--provider-config", provider_config
    ], capture_output=True, text=True)
    
    # Check if the command was successful
    if result.returncode != 0:
        raise Exception(f"CLI command failed: {result.stderr}")
    
    # Parse the JSON output
    return json.loads(result.stdout)

def fetch_asset_data(access_details: dict) -> dict:
    """
    Use the access details to fetch the actual asset data.

    You will have to replace this by the actual logic you use to interact with the server on the provider side.
    """
    print(access_details)
    
    # response = requests.get(
    #     access_details["endpoint"],
    #     headers={"Authorization": access_details["authorization"]}
    # )
    # response.raise_for_status()
    # return response.json()

def main():
    # Example configuration
    ASSET_ID = "id10"
    CONSUMER_CONFIG = "consumer_private.yaml"
    PROVIDER_CONFIG = "provider_private.yaml"
    
    try:
        # Get access details from the CLI
        print("Getting asset access details...")
        access_details = get_asset_access(ASSET_ID, CONSUMER_CONFIG, PROVIDER_CONFIG)
        print(f"Received access details: {json.dumps(access_details, indent=2)}")
        
        # Use the access details to fetch the actual data
        print("\nFetching asset data...")
        asset_data = fetch_asset_data(access_details)
        print(f"Received asset data: {json.dumps(asset_data, indent=2)}")
        
    except Exception as e:
        print(f"Error: {e}")
        exit(1)

if __name__ == "__main__":
    main() 
use crate::config::ConnectorConfig;

pub(crate) fn asset_create_url(provider_config: &ConnectorConfig) -> String {
    format!(
        "{connector_url}/management/v3/assets",
        connector_url = provider_config.base_url
    )
}

pub(crate) fn usage_policy_url(provider_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/policydefinitions",
        base_url = provider_config.base_url
    )
}

pub(crate) fn access_policy_url(provider_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/policydefinitions",
        base_url = provider_config.base_url
    )
}

pub(crate) fn create_contract_url(provider_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/contractdefinitions",
        base_url = provider_config.base_url
    )
}

pub(crate) fn catalogue_request_url(consumer_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/catalog/request",
        base_url = consumer_config.base_url
    )
}

pub(crate) fn edrs_url(consumer_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/edrs",
        base_url = consumer_config.base_url
    )
}

pub(crate) fn request_edr_url(consumer_config: &ConnectorConfig) -> String {
    format!(
        "{base_url}/management/v3/edrs/request",
        base_url = consumer_config.base_url
    )
}

pub(crate) fn edr_read_url(consumer_config: &ConnectorConfig, transfer_id: &str) -> String {
    format!(
        "{base_url}/management/v3/edrs/{transfer_id}/dataaddress",
        base_url = consumer_config.base_url
    )
}

/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetsRelayWebhook : Fired when the Secondary Client successfully retrieves an Asset Report by calling `asset_report/relay/get`.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetsRelayWebhook {
    /// `ASSETS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `RELAY_EVENT`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    #[serde(rename = "relay_event")]
    pub relay_event: crate::models::RelayEvent,
    /// The id of the client with whom the Asset Report is being shared.
    #[serde(rename = "secondary_client_id")]
    pub secondary_client_id: String,
    /// The `asset_relay_token` that was created by calling `/asset_report/relay/create.
    #[serde(rename = "asset_relay_token")]
    pub asset_relay_token: String,
    /// The `asset_report_id` that can be provided to `/asset_report/relay/get` to retrieve the Asset Report.
    #[serde(rename = "asset_report_id")]
    pub asset_report_id: String,
}

impl AssetsRelayWebhook {
    /// Fired when the Secondary Client successfully retrieves an Asset Report by calling `asset_report/relay/get`.
    pub fn new(webhook_type: String, webhook_code: String, relay_event: crate::models::RelayEvent, secondary_client_id: String, asset_relay_token: String, asset_report_id: String) -> AssetsRelayWebhook {
        AssetsRelayWebhook {
            webhook_type,
            webhook_code,
            relay_event,
            secondary_client_id,
            asset_relay_token,
            asset_report_id,
        }
    }
}



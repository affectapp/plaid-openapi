/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportRelayRemoveRequest : AssetReportRelayRemoveRequest defines the request schema for `/asset_report/relay/remove`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The `asset_relay_token` you would like to revoke.
    #[serde(rename = "asset_relay_token")]
    pub asset_relay_token: String,
}

impl AssetReportRelayRemoveRequest {
    /// AssetReportRelayRemoveRequest defines the request schema for `/asset_report/relay/remove`
    pub fn new(asset_relay_token: String) -> AssetReportRelayRemoveRequest {
        AssetReportRelayRemoveRequest {
            client_id: None,
            secret: None,
            asset_relay_token,
        }
    }
}



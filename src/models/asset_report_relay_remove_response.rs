/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportRelayRemoveResponse : AssetReportRelayRemoveResponse defines the response schema for `/asset_report/relay/remove`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveResponse {
    /// `true` if the Asset Relay token was successfully removed.
    #[serde(rename = "removed")]
    pub removed: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl AssetReportRelayRemoveResponse {
    /// AssetReportRelayRemoveResponse defines the response schema for `/asset_report/relay/remove`
    pub fn new(removed: bool, request_id: String) -> AssetReportRelayRemoveResponse {
        AssetReportRelayRemoveResponse {
            removed,
            request_id,
        }
    }
}


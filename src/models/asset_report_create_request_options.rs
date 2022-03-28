/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportCreateRequestOptions : An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportCreateRequestOptions {
    /// Client-generated identifier, which can be used by lenders to track loan applications.
    #[serde(rename = "client_report_id", skip_serializing_if = "Option::is_none")]
    pub client_report_id: Option<String>,
    /// URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::AssetReportUser>,
}

impl AssetReportCreateRequestOptions {
    /// An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
    pub fn new() -> AssetReportCreateRequestOptions {
        AssetReportCreateRequestOptions {
            client_report_id: None,
            webhook: None,
            user: None,
        }
    }
}

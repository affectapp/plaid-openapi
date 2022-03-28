/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetReportFilterRequest : AssetReportFilterRequest defines the request schema for `/asset_report/filter`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    #[serde(rename = "asset_report_token")]
    pub asset_report_token: String,
    /// The accounts to exclude from the Asset Report, identified by `account_id`.
    #[serde(rename = "account_ids_to_exclude")]
    pub account_ids_to_exclude: Vec<String>,
}

impl AssetReportFilterRequest {
    /// AssetReportFilterRequest defines the request schema for `/asset_report/filter`
    pub fn new(asset_report_token: String, account_ids_to_exclude: Vec<String>) -> AssetReportFilterRequest {
        AssetReportFilterRequest {
            client_id: None,
            secret: None,
            asset_report_token,
            account_ids_to_exclude,
        }
    }
}



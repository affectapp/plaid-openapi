/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountsGetRequestOptions : An optional object to filter `/accounts/get` results.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountsGetRequestOptions {
    /// An array of `account_ids` to retrieve for the Account.
    #[serde(rename = "account_ids", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl AccountsGetRequestOptions {
    /// An optional object to filter `/accounts/get` results.
    pub fn new() -> AccountsGetRequestOptions {
        AccountsGetRequestOptions { account_ids: None }
    }
}

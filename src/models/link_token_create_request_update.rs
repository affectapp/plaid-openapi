/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestUpdate : Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUpdate {
    /// If `true`, enables [update mode with Account Select](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
    #[serde(
        rename = "account_selection_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_selection_enabled: Option<bool>,
}

impl LinkTokenCreateRequestUpdate {
    /// Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).
    pub fn new() -> LinkTokenCreateRequestUpdate {
        LinkTokenCreateRequestUpdate {
            account_selection_enabled: None,
        }
    }
}

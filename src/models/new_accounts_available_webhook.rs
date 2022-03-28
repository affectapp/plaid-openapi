/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NewAccountsAvailableWebhook : Fired when Plaid detects a new account for Items created or updated with [Account Select v2](https://plaid.com/docs/link/customization/#account-select). Upon receiving this webhook, you can prompt your users to share new accounts with you through [Account Select v2 update mode](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NewAccountsAvailableWebhook {
    /// `ITEM`
    #[serde(rename = "webhook_type", skip_serializing_if = "Option::is_none")]
    pub webhook_type: Option<String>,
    /// `NEW_ACCOUNTS_AVAILABLE`
    #[serde(rename = "webhook_code", skip_serializing_if = "Option::is_none")]
    pub webhook_code: Option<String>,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::PlaidError>>,
}

impl NewAccountsAvailableWebhook {
    /// Fired when Plaid detects a new account for Items created or updated with [Account Select v2](https://plaid.com/docs/link/customization/#account-select). Upon receiving this webhook, you can prompt your users to share new accounts with you through [Account Select v2 update mode](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
    pub fn new() -> NewAccountsAvailableWebhook {
        NewAccountsAvailableWebhook {
            webhook_type: None,
            webhook_code: None,
            item_id: None,
            error: None,
        }
    }
}



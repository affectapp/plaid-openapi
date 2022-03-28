/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// IdentityDefaultUpdateWebhook : Fired when a change to identity data has been detected on an Item.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IdentityDefaultUpdateWebhook {
    /// `IDENTITY`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.  Example: `{ \"XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58\": [\"PHONES\"] }`
    #[serde(rename = "account_ids_with_updated_identity")]
    pub account_ids_with_updated_identity:
        ::std::collections::HashMap<String, Vec<crate::models::IdentityUpdateTypes>>,
    #[serde(rename = "error")]
    pub error: Box<crate::models::PlaidError>,
}

impl IdentityDefaultUpdateWebhook {
    /// Fired when a change to identity data has been detected on an Item.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        item_id: String,
        account_ids_with_updated_identity: ::std::collections::HashMap<
            String,
            Vec<crate::models::IdentityUpdateTypes>,
        >,
        error: crate::models::PlaidError,
    ) -> IdentityDefaultUpdateWebhook {
        IdentityDefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            account_ids_with_updated_identity,
            error: Box::new(error),
        }
    }
}

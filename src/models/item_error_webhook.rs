/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ItemErrorWebhook : Fired when an error is encountered with an Item. The error can be resolved by having the user go through Link’s update mode.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemErrorWebhook {
    /// `ITEM`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `ERROR`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error")]
    pub error: Box<crate::models::PlaidError>,
}

impl ItemErrorWebhook {
    /// Fired when an error is encountered with an Item. The error can be resolved by having the user go through Link’s update mode.
    pub fn new(
        webhook_type: String,
        webhook_code: String,
        item_id: String,
        error: crate::models::PlaidError,
    ) -> ItemErrorWebhook {
        ItemErrorWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error: Box::new(error),
        }
    }
}

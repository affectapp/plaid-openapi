/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsDefaultUpdateWebhook : Fired when new or canceled transactions have been detected on an investment account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentsDefaultUpdateWebhook {
    /// `INVESTMENTS_TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    /// The `item_id` of the Item associated with this webhook, warning, or error
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::PlaidError>>,
    /// The number of new transactions reported since the last time this webhook was fired.
    #[serde(rename = "new_investments_transactions")]
    pub new_investments_transactions: f32,
    /// The number of canceled transactions reported since the last time this webhook was fired.
    #[serde(rename = "canceled_investments_transactions")]
    pub canceled_investments_transactions: f32,
}

impl InvestmentsDefaultUpdateWebhook {
    /// Fired when new or canceled transactions have been detected on an investment account.
    pub fn new(webhook_type: String, webhook_code: String, item_id: String, new_investments_transactions: f32, canceled_investments_transactions: f32) -> InvestmentsDefaultUpdateWebhook {
        InvestmentsDefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            item_id,
            error: None,
            new_investments_transactions,
            canceled_investments_transactions,
        }
    }
}



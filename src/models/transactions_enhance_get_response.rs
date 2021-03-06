/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsEnhanceGetResponse : TransactionsEnhanceGetResponse defines the response schema for `/transactions/enhance`.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetResponse {
    /// An array of enhanced transactions.
    #[serde(rename = "enhanced_transactions")]
    pub enhanced_transactions: Vec<crate::models::ClientProvidedEnhancedTransaction>,
}

impl TransactionsEnhanceGetResponse {
    /// TransactionsEnhanceGetResponse defines the response schema for `/transactions/enhance`.
    pub fn new(
        enhanced_transactions: Vec<crate::models::ClientProvidedEnhancedTransaction>,
    ) -> TransactionsEnhanceGetResponse {
        TransactionsEnhanceGetResponse {
            enhanced_transactions,
        }
    }
}

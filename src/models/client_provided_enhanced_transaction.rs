/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ClientProvidedEnhancedTransaction : A client-provided transaction that Plaid has enhanced.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClientProvidedEnhancedTransaction {
    /// Unique transaction identifier to tie transactions back to clients' systems.
    #[serde(rename = "id")]
    pub id: String,
    /// The raw description of the transaction.
    #[serde(rename = "description")]
    pub description: String,
    /// The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    #[serde(rename = "amount")]
    pub amount: f64,
    /// The ISO-4217 currency code of the transaction.
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    #[serde(rename = "enhancements")]
    pub enhancements: crate::models::Enhancements,
}

impl ClientProvidedEnhancedTransaction {
    /// A client-provided transaction that Plaid has enhanced.
    pub fn new(
        id: String,
        description: String,
        amount: f64,
        iso_currency_code: String,
        enhancements: crate::models::Enhancements,
    ) -> ClientProvidedEnhancedTransaction {
        ClientProvidedEnhancedTransaction {
            id,
            description,
            amount,
            iso_currency_code,
            enhancements,
        }
    }
}

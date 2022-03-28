/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionStream : A grouping of related transactions

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionStream {
    /// The ID of the account to which the stream belongs
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// A unique id for the stream
    #[serde(rename = "stream_id")]
    pub stream_id: String,
    /// The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    #[serde(rename = "category_id")]
    pub category_id: String,
    /// A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    #[serde(rename = "category")]
    pub category: Vec<String>,
    /// A description of the transaction stream.
    #[serde(rename = "description")]
    pub description: String,
    /// The merchant associated with the transaction stream.
    #[serde(rename = "merchant_name")]
    pub merchant_name: String,
    /// The posted date of the earliest transaction in the stream.
    #[serde(rename = "first_date")]
    pub first_date: String,
    /// The posted date of the latest transaction in the stream.
    #[serde(rename = "last_date")]
    pub last_date: String,
    #[serde(rename = "frequency")]
    pub frequency: crate::models::RecurringTransactionFrequency,
    /// An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    #[serde(rename = "transaction_ids")]
    pub transaction_ids: Vec<String>,
    #[serde(rename = "average_amount")]
    pub average_amount: crate::models::TransactionStreamAmount,
    #[serde(rename = "last_amount")]
    pub last_amount: crate::models::TransactionStreamAmount,
    /// Indicates whether the transaction stream is still live.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "status")]
    pub status: crate::models::TransactionStreamStatus,
}

impl TransactionStream {
    /// A grouping of related transactions
    pub fn new(
        account_id: String,
        stream_id: String,
        category_id: String,
        category: Vec<String>,
        description: String,
        merchant_name: String,
        first_date: String,
        last_date: String,
        frequency: crate::models::RecurringTransactionFrequency,
        transaction_ids: Vec<String>,
        average_amount: crate::models::TransactionStreamAmount,
        last_amount: crate::models::TransactionStreamAmount,
        is_active: bool,
        status: crate::models::TransactionStreamStatus,
    ) -> TransactionStream {
        TransactionStream {
            account_id,
            stream_id,
            category_id,
            category,
            description,
            merchant_name,
            first_date,
            last_date,
            frequency,
            transaction_ids,
            average_amount,
            last_amount,
            is_active,
            status,
        }
    }
}

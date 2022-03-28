/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// WalletTransactionsListResponse : WalletTransactionsListResponse defines the response schema for `/wallet/transactions/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WalletTransactionsListResponse {
    /// An array of transactions of an e-wallet, associated with the given `wallet_id`
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::WalletTransaction>,
    /// Cursor used for fetching transactions created before the latest transaction provided in this response
    #[serde(rename = "next_cursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl WalletTransactionsListResponse {
    /// WalletTransactionsListResponse defines the response schema for `/wallet/transactions/list`
    pub fn new(transactions: Vec<crate::models::WalletTransaction>, request_id: String) -> WalletTransactionsListResponse {
        WalletTransactionsListResponse {
            transactions,
            next_cursor: None,
            request_id,
        }
    }
}


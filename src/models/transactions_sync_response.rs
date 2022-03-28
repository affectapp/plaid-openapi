/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsSyncResponse : TransactionsSyncResponse defines the response schema for `/transactions/sync`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsSyncResponse {
    /// Transactions that have been added to the item since `cursor` ordered by ascending last modified time.
    #[serde(rename = "added")]
    pub added: Vec<crate::models::Transaction>,
    /// Transactions that have been modified on the item since `cursor` ordered by ascending last modified time.
    #[serde(rename = "modified")]
    pub modified: Vec<crate::models::Transaction>,
    /// Transactions that have been removed from the item since `cursor` ordered by ascending last modified time.
    #[serde(rename = "removed")]
    pub removed: Vec<crate::models::RemovedTransaction>,
    /// Cursor used for fetching any future updates after the latest update provided in this response.
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
    /// Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsSyncResponse {
    /// TransactionsSyncResponse defines the response schema for `/transactions/sync`
    pub fn new(added: Vec<crate::models::Transaction>, modified: Vec<crate::models::Transaction>, removed: Vec<crate::models::RemovedTransaction>, next_cursor: String, has_more: bool, request_id: String) -> TransactionsSyncResponse {
        TransactionsSyncResponse {
            added,
            modified,
            removed,
            next_cursor,
            has_more,
            request_id,
        }
    }
}


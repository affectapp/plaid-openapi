/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransactionsRulesListResponse : TransactionsRulesListResponse defines the response schema for `/transactions/rules/list`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionsRulesListResponse {
    /// A list of the Item's transaction rules
    #[serde(rename = "rules")]
    pub rules: Vec<crate::models::TransactionsCategoryRule>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransactionsRulesListResponse {
    /// TransactionsRulesListResponse defines the response schema for `/transactions/rules/list`
    pub fn new(
        rules: Vec<crate::models::TransactionsCategoryRule>,
        request_id: String,
    ) -> TransactionsRulesListResponse {
        TransactionsRulesListResponse { rules, request_id }
    }
}

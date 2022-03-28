/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InvestmentsTransactionsGetResponse : InvestmentsTransactionsGetResponse defines the response schema for `/investments/transactions/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    #[serde(rename = "item")]
    pub item: crate::models::Item,
    /// The accounts for which transaction history is being fetched.
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::AccountBase>,
    /// All securities for which there is a corresponding transaction being fetched.
    #[serde(rename = "securities")]
    pub securities: Vec<crate::models::Security>,
    /// The transactions being fetched
    #[serde(rename = "investment_transactions")]
    pub investment_transactions: Vec<crate::models::InvestmentTransaction>,
    /// The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.'
    #[serde(rename = "total_investment_transactions")]
    pub total_investment_transactions: i32,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl InvestmentsTransactionsGetResponse {
    /// InvestmentsTransactionsGetResponse defines the response schema for `/investments/transactions/get`
    pub fn new(item: crate::models::Item, accounts: Vec<crate::models::AccountBase>, securities: Vec<crate::models::Security>, investment_transactions: Vec<crate::models::InvestmentTransaction>, total_investment_transactions: i32, request_id: String) -> InvestmentsTransactionsGetResponse {
        InvestmentsTransactionsGetResponse {
            item,
            accounts,
            securities,
            investment_transactions,
            total_investment_transactions,
            request_id,
        }
    }
}



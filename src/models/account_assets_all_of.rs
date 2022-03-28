/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountAssetsAllOf {
    /// The duration of transaction history available for this Item, typically defined as the time since the date of the earliest transaction in that account. Only returned by Assets endpoints.
    #[serde(rename = "days_available")]
    pub days_available: f32,
    /// Transaction history associated with the account. Only returned by Assets endpoints. Transaction history returned by endpoints such as `/transactions/get` or `/investments/transactions/get` will be returned in the top-level `transactions` field instead.
    #[serde(rename = "transactions")]
    pub transactions: Vec<crate::models::AssetReportTransaction>,
    /// Data returned by the financial institution about the account owner or owners. Only returned by Identity or Assets endpoints. For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array. In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29)
    #[serde(rename = "owners")]
    pub owners: Vec<crate::models::Owner>,
    /// Calculated data about the historical balances on the account. Only returned by Assets endpoints and currently not supported by `brokerage` or `investment` accounts.
    #[serde(rename = "historical_balances")]
    pub historical_balances: Vec<crate::models::HistoricalBalance>,
}

impl AccountAssetsAllOf {
    pub fn new(days_available: f32, transactions: Vec<crate::models::AssetReportTransaction>, owners: Vec<crate::models::Owner>, historical_balances: Vec<crate::models::HistoricalBalance>) -> AccountAssetsAllOf {
        AccountAssetsAllOf {
            days_available,
            transactions,
            owners,
            historical_balances,
        }
    }
}



/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IncomeVerificationPrecheckRequest : IncomeVerificationPrecheckRequest defines the request schema for `/income/verification/precheck`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::IncomeVerificationPrecheckUser>>,
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<Box<crate::models::IncomeVerificationPrecheckEmployer>>,
    #[serde(rename = "transactions_access_token", skip_serializing_if = "Option::is_none")]
    pub transactions_access_token: Option<Box<String>>,
    /// An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    #[serde(rename = "transactions_access_tokens", skip_serializing_if = "Option::is_none")]
    pub transactions_access_tokens: Option<Vec<String>>,
    #[serde(rename = "us_military_info", skip_serializing_if = "Option::is_none")]
    pub us_military_info: Option<Box<crate::models::IncomeVerificationPrecheckMilitaryInfo>>,
}

impl IncomeVerificationPrecheckRequest {
    /// IncomeVerificationPrecheckRequest defines the request schema for `/income/verification/precheck`
    pub fn new() -> IncomeVerificationPrecheckRequest {
        IncomeVerificationPrecheckRequest {
            client_id: None,
            secret: None,
            user: None,
            employer: None,
            transactions_access_token: None,
            transactions_access_tokens: None,
            us_military_info: None,
        }
    }
}



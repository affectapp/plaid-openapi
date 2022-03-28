/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeGetRequestOptions : An optional object for `/credit/bank_income/get` request options.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequestOptions {
    /// How many Bank Income Reports should be fetched. Multiple reports may be available if the report has been re-created or refreshed. If more than one report is available, the most recent reports will be returned first.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl CreditBankIncomeGetRequestOptions {
    /// An optional object for `/credit/bank_income/get` request options.
    pub fn new() -> CreditBankIncomeGetRequestOptions {
        CreditBankIncomeGetRequestOptions { count: None }
    }
}

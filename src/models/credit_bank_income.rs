/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncome : The report of the Bank Income data for an end user.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankIncome {
    /// The unique identifier associated with the Bank Income Report.
    #[serde(rename = "bank_income_id", skip_serializing_if = "Option::is_none")]
    pub bank_income_id: Option<String>,
    /// The time when the Bank Income Report was generated.
    #[serde(rename = "generated_time", skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<String>,
    /// The number of days requested by the customer for the Bank Income Report.
    #[serde(rename = "days_requested", skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i32>,
    /// The list of Items in the report along with the associated metadata about the Item.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::CreditBankIncomeItem>>,
    #[serde(rename = "bank_income_summary", skip_serializing_if = "Option::is_none")]
    pub bank_income_summary: Option<Box<crate::models::CreditBankIncomeSummary>>,
    /// If data from the Bank Income report was unable to be retrieved, the warnings will contain information about the error that caused the data to be incomplete.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::CreditBankIncomeWarning>>,
}

impl CreditBankIncome {
    /// The report of the Bank Income data for an end user.
    pub fn new() -> CreditBankIncome {
        CreditBankIncome {
            bank_income_id: None,
            generated_time: None,
            days_requested: None,
            items: None,
            bank_income_summary: None,
            warnings: None,
        }
    }
}



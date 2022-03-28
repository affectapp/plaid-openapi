/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Paystub : An object representing data extracted from the end user's paystub.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Paystub {
    #[serde(rename = "deductions")]
    pub deductions: crate::models::Deductions,
    /// An identifier of the document referenced by the document metadata.
    #[serde(rename = "doc_id")]
    pub doc_id: String,
    #[serde(rename = "earnings")]
    pub earnings: crate::models::Earnings,
    #[serde(rename = "employee")]
    pub employee: crate::models::Employee,
    #[serde(rename = "employer")]
    pub employer: crate::models::PaystubEmployer,
    #[serde(rename = "employment_details", skip_serializing_if = "Option::is_none")]
    pub employment_details: Option<crate::models::EmploymentDetails>,
    #[serde(rename = "net_pay")]
    pub net_pay: crate::models::NetPay,
    #[serde(rename = "pay_period_details")]
    pub pay_period_details: crate::models::PayPeriodDetails,
    #[serde(rename = "paystub_details", skip_serializing_if = "Option::is_none")]
    pub paystub_details: Option<crate::models::PaystubDetails>,
    #[serde(rename = "income_breakdown", skip_serializing_if = "Option::is_none")]
    pub income_breakdown: Option<Vec<crate::models::IncomeBreakdown>>,
    #[serde(rename = "ytd_earnings", skip_serializing_if = "Option::is_none")]
    pub ytd_earnings: Option<crate::models::PaystubYtdDetails>,
    #[serde(rename = "verification")]
    pub verification: Option<crate::models::PaystubVerification>,
}

impl Paystub {
    /// An object representing data extracted from the end user's paystub.
    pub fn new(deductions: crate::models::Deductions, doc_id: String, earnings: crate::models::Earnings, employee: crate::models::Employee, employer: crate::models::PaystubEmployer, net_pay: crate::models::NetPay, pay_period_details: crate::models::PayPeriodDetails, verification: Option<crate::models::PaystubVerification>) -> Paystub {
        Paystub {
            deductions,
            doc_id,
            earnings,
            employee,
            employer,
            employment_details: None,
            net_pay,
            pay_period_details,
            paystub_details: None,
            income_breakdown: None,
            ytd_earnings: None,
            verification,
        }
    }
}



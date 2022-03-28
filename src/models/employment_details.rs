/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// EmploymentDetails : An object representing employment details found on a paystub.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmploymentDetails {
    #[serde(rename = "annual_salary", skip_serializing_if = "Option::is_none")]
    pub annual_salary: Option<crate::models::Pay>,
    /// Date on which the employee was hired, in the YYYY-MM-DD format.
    #[serde(rename = "hire_date", skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<String>,
}

impl EmploymentDetails {
    /// An object representing employment details found on a paystub.
    pub fn new() -> EmploymentDetails {
        EmploymentDetails {
            annual_salary: None,
            hire_date: None,
        }
    }
}

/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditCardLiability : An object representing a credit card account.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditCardLiability {
    /// The ID of the account that this liability belongs to.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The various interest rates that apply to the account. APR information is not provided by all card issuers; if APR data is not available, this array will be empty.
    #[serde(rename = "aprs")]
    pub aprs: Vec<crate::models::Apr>,
    /// true if a payment is currently overdue. Availability for this field is limited.
    #[serde(rename = "is_overdue")]
    pub is_overdue: Option<bool>,
    /// The amount of the last payment.
    #[serde(rename = "last_payment_amount")]
    pub last_payment_amount: Option<f64>,
    /// The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited.
    #[serde(rename = "last_payment_date")]
    pub last_payment_date: Option<String>,
    /// The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "last_statement_issue_date")]
    pub last_statement_issue_date: Option<String>,
    /// The total amount owed as of the last statement issued
    #[serde(rename = "last_statement_balance")]
    pub last_statement_balance: Option<f64>,
    /// The minimum payment due for the next billing cycle.
    #[serde(rename = "minimum_payment_amount")]
    pub minimum_payment_amount: Option<f64>,
    /// The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "next_payment_due_date")]
    pub next_payment_due_date: Option<String>,
}

impl CreditCardLiability {
    /// An object representing a credit card account.
    pub fn new(
        account_id: Option<String>,
        aprs: Vec<crate::models::Apr>,
        is_overdue: Option<bool>,
        last_payment_amount: Option<f64>,
        last_payment_date: Option<String>,
        last_statement_issue_date: Option<String>,
        last_statement_balance: Option<f64>,
        minimum_payment_amount: Option<f64>,
        next_payment_due_date: Option<String>,
    ) -> CreditCardLiability {
        CreditCardLiability {
            account_id,
            aprs,
            is_overdue,
            last_payment_amount,
            last_payment_date,
            last_statement_issue_date,
            last_statement_balance,
            minimum_payment_amount,
            next_payment_due_date,
        }
    }
}

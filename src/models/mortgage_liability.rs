/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MortgageLiability : Contains details about a mortgage account.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MortgageLiability {
    /// The ID of the account that this liability belongs to.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The account number of the loan.
    #[serde(rename = "account_number")]
    pub account_number: String,
    /// The current outstanding amount charged for late payment.
    #[serde(rename = "current_late_fee")]
    pub current_late_fee: Option<f64>,
    /// Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    #[serde(rename = "escrow_balance")]
    pub escrow_balance: Option<f64>,
    /// Indicates whether the borrower has private mortgage insurance in effect.
    #[serde(rename = "has_pmi")]
    pub has_pmi: Option<bool>,
    /// Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    #[serde(rename = "has_prepayment_penalty")]
    pub has_prepayment_penalty: Option<bool>,
    #[serde(rename = "interest_rate")]
    pub interest_rate: crate::models::MortgageInterestRate,
    /// The amount of the last payment.
    #[serde(rename = "last_payment_amount")]
    pub last_payment_amount: Option<f64>,
    /// The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "last_payment_date")]
    pub last_payment_date: Option<String>,
    /// Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    #[serde(rename = "loan_type_description")]
    pub loan_type_description: Option<String>,
    /// Full duration of mortgage as at origination (e.g. `10 year`).
    #[serde(rename = "loan_term")]
    pub loan_term: Option<String>,
    /// Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "maturity_date")]
    pub maturity_date: Option<String>,
    /// The amount of the next payment.
    #[serde(rename = "next_monthly_payment")]
    pub next_monthly_payment: Option<f64>,
    /// The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "next_payment_due_date")]
    pub next_payment_due_date: Option<String>,
    /// The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    #[serde(rename = "origination_date")]
    pub origination_date: Option<String>,
    /// The original principal balance of the mortgage.
    #[serde(rename = "origination_principal_amount")]
    pub origination_principal_amount: Option<f64>,
    /// Amount of loan (principal + interest) past due for payment.
    #[serde(rename = "past_due_amount")]
    pub past_due_amount: Option<f64>,
    #[serde(rename = "property_address")]
    pub property_address: crate::models::MortgagePropertyAddress,
    /// The year to date (YTD) interest paid.
    #[serde(rename = "ytd_interest_paid")]
    pub ytd_interest_paid: Option<f64>,
    /// The YTD principal paid.
    #[serde(rename = "ytd_principal_paid")]
    pub ytd_principal_paid: Option<f64>,
}

impl MortgageLiability {
    /// Contains details about a mortgage account.
    pub fn new(account_id: String, account_number: String, current_late_fee: Option<f64>, escrow_balance: Option<f64>, has_pmi: Option<bool>, has_prepayment_penalty: Option<bool>, interest_rate: crate::models::MortgageInterestRate, last_payment_amount: Option<f64>, last_payment_date: Option<String>, loan_type_description: Option<String>, loan_term: Option<String>, maturity_date: Option<String>, next_monthly_payment: Option<f64>, next_payment_due_date: Option<String>, origination_date: Option<String>, origination_principal_amount: Option<f64>, past_due_amount: Option<f64>, property_address: crate::models::MortgagePropertyAddress, ytd_interest_paid: Option<f64>, ytd_principal_paid: Option<f64>) -> MortgageLiability {
        MortgageLiability {
            account_id,
            account_number,
            current_late_fee,
            escrow_balance,
            has_pmi,
            has_prepayment_penalty,
            interest_rate,
            last_payment_amount,
            last_payment_date,
            loan_type_description,
            loan_term,
            maturity_date,
            next_monthly_payment,
            next_payment_due_date,
            origination_date,
            origination_principal_amount,
            past_due_amount,
            property_address,
            ytd_interest_paid,
            ytd_principal_paid,
        }
    }
}



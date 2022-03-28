/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DepositSwitchGetResponse : DepositSwitchGetResponse defines the response schema for `/deposit_switch/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DepositSwitchGetResponse {
    /// The ID of the deposit switch.
    #[serde(rename = "deposit_switch_id")]
    pub deposit_switch_id: String,
    /// The ID of the bank account the direct deposit was switched to.
    #[serde(rename = "target_account_id")]
    pub target_account_id: Option<String>,
    /// The ID of the Item the direct deposit was switched to.
    #[serde(rename = "target_item_id")]
    pub target_item_id: Option<String>,
    ///  The state, or status, of the deposit switch.  - `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.  - `processing` – The deposit switch request has been submitted and is being processed.  - `completed` – The user's employer has fulfilled the deposit switch request.  - `error` – There was an error processing the deposit switch request.
    #[serde(rename = "state")]
    pub state: State,
    /// The method used to make the deposit switch.  - `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.  - `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.  - `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'
    #[serde(rename = "switch_method", skip_serializing_if = "Option::is_none")]
    pub switch_method: Option<SwitchMethod>,
    /// When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed.
    #[serde(rename = "account_has_multiple_allocations")]
    pub account_has_multiple_allocations: Option<bool>,
    /// When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed.
    #[serde(rename = "is_allocated_remainder")]
    pub is_allocated_remainder: Option<bool>,
    /// The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true.
    #[serde(rename = "percent_allocated")]
    pub percent_allocated: Option<f64>,
    /// The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed.
    #[serde(rename = "amount_allocated")]
    pub amount_allocated: Option<f64>,
    /// The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    #[serde(rename = "employer_name", skip_serializing_if = "Option::is_none")]
    pub employer_name: Option<String>,
    /// The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    #[serde(rename = "employer_id", skip_serializing_if = "Option::is_none")]
    pub employer_id: Option<String>,
    /// The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    #[serde(rename = "institution_name", skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created. 
    #[serde(rename = "date_created")]
    pub date_created: String,
    /// [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed. 
    #[serde(rename = "date_completed")]
    pub date_completed: Option<String>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl DepositSwitchGetResponse {
    /// DepositSwitchGetResponse defines the response schema for `/deposit_switch/get`
    pub fn new(deposit_switch_id: String, target_account_id: Option<String>, target_item_id: Option<String>, state: State, account_has_multiple_allocations: Option<bool>, is_allocated_remainder: Option<bool>, percent_allocated: Option<f64>, amount_allocated: Option<f64>, date_created: String, date_completed: Option<String>, request_id: String) -> DepositSwitchGetResponse {
        DepositSwitchGetResponse {
            deposit_switch_id,
            target_account_id,
            target_item_id,
            state,
            switch_method: None,
            account_has_multiple_allocations,
            is_allocated_remainder,
            percent_allocated,
            amount_allocated,
            employer_name: None,
            employer_id: None,
            institution_name: None,
            institution_id: None,
            date_created,
            date_completed,
            request_id,
        }
    }
}

///  The state, or status, of the deposit switch.  - `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.  - `processing` – The deposit switch request has been submitted and is being processed.  - `completed` – The user's employer has fulfilled the deposit switch request.  - `error` – There was an error processing the deposit switch request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "error")]
    Error,
}

impl Default for State {
    fn default() -> State {
        Self::Initialized
    }
}
/// The method used to make the deposit switch.  - `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.  - `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.  - `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SwitchMethod {
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "mail")]
    Mail,
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "null")]
    Null,
}

impl Default for SwitchMethod {
    fn default() -> SwitchMethod {
        Self::Instant
    }
}


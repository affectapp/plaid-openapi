/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferFailure : The failure reason if the type of this transfer is `\"failed\"` or `\"reversed\"`. Null value otherwise.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferFailure {
    /// The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes).
    #[serde(rename = "ach_return_code", skip_serializing_if = "Option::is_none")]
    pub ach_return_code: Option<String>,
    /// A human-readable description of the reason for the failure or reversal.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl BankTransferFailure {
    /// The failure reason if the type of this transfer is `\"failed\"` or `\"reversed\"`. Null value otherwise.
    pub fn new() -> BankTransferFailure {
        BankTransferFailure {
            ach_return_code: None,
            description: None,
        }
    }
}



/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentGetFailureReason : The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferIntentGetFailureReason {
    /// A broad categorization of the error.
    #[serde(rename = "error_type", skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    /// A code representing the reason for a failed transfer intent (i.e., an API error or the authorization being declined).  For a full listing of bank transfer errors, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/).
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// A human-readable description of the code associated with a failed transfer intent.
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl TransferIntentGetFailureReason {
    /// The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.
    pub fn new() -> TransferIntentGetFailureReason {
        TransferIntentGetFailureReason {
            error_type: None,
            error_code: None,
            error_message: None,
        }
    }
}



/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepaymentReturnListResponse : Defines the response schema for `/transfer/repayments/return/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListResponse {
    #[serde(rename = "repayment_returns")]
    pub repayment_returns: Vec<crate::models::TransferRepaymentReturn>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferRepaymentReturnListResponse {
    /// Defines the response schema for `/transfer/repayments/return/list`
    pub fn new(repayment_returns: Vec<crate::models::TransferRepaymentReturn>, request_id: String) -> TransferRepaymentReturnListResponse {
        TransferRepaymentReturnListResponse {
            repayment_returns,
            request_id,
        }
    }
}



/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferRepaymentListResponse : Defines the response schema for `/transfer/repayments/list`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferRepaymentListResponse {
    #[serde(rename = "repayments")]
    pub repayments: Vec<crate::models::TransferRepayment>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferRepaymentListResponse {
    /// Defines the response schema for `/transfer/repayments/list`
    pub fn new(repayments: Vec<crate::models::TransferRepayment>, request_id: String) -> TransferRepaymentListResponse {
        TransferRepaymentListResponse {
            repayments,
            request_id,
        }
    }
}



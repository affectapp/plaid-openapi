/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferCancelResponse : Defines the response schema for `/transfer/cancel`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferCancelResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl TransferCancelResponse {
    /// Defines the response schema for `/transfer/cancel`
    pub fn new(request_id: String) -> TransferCancelResponse {
        TransferCancelResponse {
            request_id,
        }
    }
}



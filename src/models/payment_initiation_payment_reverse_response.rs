/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentReverseResponse : PaymentInitiationPaymentReverseResponse defines the response schema for `/payment_initation/payment/reverse`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseResponse {
    /// A unique ID identifying the refund
    #[serde(rename = "refund_id")]
    pub refund_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::PaymentInitiationRefundStatus,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationPaymentReverseResponse {
    /// PaymentInitiationPaymentReverseResponse defines the response schema for `/payment_initation/payment/reverse`
    pub fn new(refund_id: String, status: crate::models::PaymentInitiationRefundStatus, request_id: String) -> PaymentInitiationPaymentReverseResponse {
        PaymentInitiationPaymentReverseResponse {
            refund_id,
            status,
            request_id,
        }
    }
}



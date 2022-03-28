/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationConsentPaymentExecuteResponse : PaymentInitiationConsentPaymentExecuteResponse defines the response schema for `/payment_initiation/consent/payment/execute`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteResponse {
    /// A unique ID identifying the payment
    #[serde(rename = "payment_id")]
    pub payment_id: String,
    #[serde(rename = "status")]
    pub status: crate::models::PaymentInitiationPaymentStatus,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl PaymentInitiationConsentPaymentExecuteResponse {
    /// PaymentInitiationConsentPaymentExecuteResponse defines the response schema for `/payment_initiation/consent/payment/execute`
    pub fn new(payment_id: String, status: crate::models::PaymentInitiationPaymentStatus, request_id: String) -> PaymentInitiationConsentPaymentExecuteResponse {
        PaymentInitiationConsentPaymentExecuteResponse {
            payment_id,
            status,
            request_id,
        }
    }
}



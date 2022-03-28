/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationRefundStatus : The status of the refund.  `PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.  `INITIATED`: The refund has been successfully initiated.  `EXECUTED`: Indicates that the refund has been successfully executed.  `FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.

/// The status of the refund.  `PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.  `INITIATED`: The refund has been successfully initiated.  `EXECUTED`: Indicates that the refund has been successfully executed.  `FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentInitiationRefundStatus {
    #[serde(rename = "PROCESSING")]
    PROCESSING,
    #[serde(rename = "EXECUTED")]
    EXECUTED,
    #[serde(rename = "INITIATED")]
    INITIATED,
    #[serde(rename = "FAILED")]
    FAILED,
}

impl ToString for PaymentInitiationRefundStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PROCESSING => String::from("PROCESSING"),
            Self::EXECUTED => String::from("EXECUTED"),
            Self::INITIATED => String::from("INITIATED"),
            Self::FAILED => String::from("FAILED"),
        }
    }
}

impl Default for PaymentInitiationRefundStatus {
    fn default() -> PaymentInitiationRefundStatus {
        Self::PROCESSING
    }
}

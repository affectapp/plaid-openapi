/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationPaymentStatus : The status of the payment.  `PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.  `PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.  `PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.  `PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.  `PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.  `PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.  `PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.  `PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.  `PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.  `PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.  Deprecated: These statuses will be removed in a future release.  `PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.  `PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.  `PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

/// The status of the payment.  `PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.  `PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.  `PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.  `PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.  `PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.  `PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.  `PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.  `PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.  `PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.  `PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.  Deprecated: These statuses will be removed in a future release.  `PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.  `PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.  `PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    INPUTNEEDED,
    #[serde(rename = "PAYMENT_STATUS_PROCESSING")]
    PROCESSING,
    #[serde(rename = "PAYMENT_STATUS_INITIATED")]
    INITIATED,
    #[serde(rename = "PAYMENT_STATUS_COMPLETED")]
    COMPLETED,
    #[serde(rename = "PAYMENT_STATUS_INSUFFICIENT_FUNDS")]
    INSUFFICIENTFUNDS,
    #[serde(rename = "PAYMENT_STATUS_FAILED")]
    FAILED,
    #[serde(rename = "PAYMENT_STATUS_BLOCKED")]
    BLOCKED,
    #[serde(rename = "PAYMENT_STATUS_UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "PAYMENT_STATUS_EXECUTED")]
    EXECUTED,
    #[serde(rename = "PAYMENT_STATUS_AUTHORISING")]
    AUTHORISING,
    #[serde(rename = "PAYMENT_STATUS_CANCELLED")]
    CANCELLED,
    #[serde(rename = "PAYMENT_STATUS_ESTABLISHED")]
    ESTABLISHED,
    #[serde(rename = "PAYMENT_STATUS_REJECTED")]
    REJECTED,

}

impl ToString for PaymentInitiationPaymentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::INPUTNEEDED => String::from("PAYMENT_STATUS_INPUT_NEEDED"),
            Self::PROCESSING => String::from("PAYMENT_STATUS_PROCESSING"),
            Self::INITIATED => String::from("PAYMENT_STATUS_INITIATED"),
            Self::COMPLETED => String::from("PAYMENT_STATUS_COMPLETED"),
            Self::INSUFFICIENTFUNDS => String::from("PAYMENT_STATUS_INSUFFICIENT_FUNDS"),
            Self::FAILED => String::from("PAYMENT_STATUS_FAILED"),
            Self::BLOCKED => String::from("PAYMENT_STATUS_BLOCKED"),
            Self::UNKNOWN => String::from("PAYMENT_STATUS_UNKNOWN"),
            Self::EXECUTED => String::from("PAYMENT_STATUS_EXECUTED"),
            Self::AUTHORISING => String::from("PAYMENT_STATUS_AUTHORISING"),
            Self::CANCELLED => String::from("PAYMENT_STATUS_CANCELLED"),
            Self::ESTABLISHED => String::from("PAYMENT_STATUS_ESTABLISHED"),
            Self::REJECTED => String::from("PAYMENT_STATUS_REJECTED"),
        }
    }
}

impl Default for PaymentInitiationPaymentStatus {
    fn default() -> PaymentInitiationPaymentStatus {
        Self::INPUTNEEDED
    }
}





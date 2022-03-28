/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferEventType : The type of event that this bank transfer represents.  `pending`: A new transfer was created; it is in the pending state.  `cancelled`: The transfer was cancelled by the client.  `failed`: The transfer failed, no funds were moved.  `posted`: The transfer has been successfully submitted to the payment network.  `reversed`: A posted transfer was reversed.

/// The type of event that this bank transfer represents.  `pending`: A new transfer was created; it is in the pending state.  `cancelled`: The transfer was cancelled by the client.  `failed`: The transfer failed, no funds were moved.  `posted`: The transfer has been successfully submitted to the payment network.  `reversed`: A posted transfer was reversed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "reversed")]
    Reversed,

}

impl ToString for BankTransferEventType {
    fn to_string(&self) -> String {
        match self {
            Self::Pending => String::from("pending"),
            Self::Cancelled => String::from("cancelled"),
            Self::Failed => String::from("failed"),
            Self::Posted => String::from("posted"),
            Self::Reversed => String::from("reversed"),
        }
    }
}

impl Default for BankTransferEventType {
    fn default() -> BankTransferEventType {
        Self::Pending
    }
}





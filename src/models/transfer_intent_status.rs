/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentStatus : The status of the transfer intent.  - `PENDING` – The transfer intent is pending. - `SUCCEEDED` – The transfer intent was successfully created. - `FAILED` – The transfer intent was unable to be created.

/// The status of the transfer intent.  - `PENDING` – The transfer intent is pending. - `SUCCEEDED` – The transfer intent was successfully created. - `FAILED` – The transfer intent was unable to be created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferIntentStatus {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "SUCCEEDED")]
    SUCCEEDED,
    #[serde(rename = "FAILED")]
    FAILED,
}

impl ToString for TransferIntentStatus {
    fn to_string(&self) -> String {
        match self {
            Self::PENDING => String::from("PENDING"),
            Self::SUCCEEDED => String::from("SUCCEEDED"),
            Self::FAILED => String::from("FAILED"),
        }
    }
}

impl Default for TransferIntentStatus {
    fn default() -> TransferIntentStatus {
        Self::PENDING
    }
}

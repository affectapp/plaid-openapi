/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentCreateMode : The direction of the flow of transfer funds.  - `PAYMENT` – Transfers funds from an end user's account to your business account.  - `DISBURSEMENT` – Transfers funds from your business account to an end user's account.

/// The direction of the flow of transfer funds.  - `PAYMENT` – Transfers funds from an end user's account to your business account.  - `DISBURSEMENT` – Transfers funds from your business account to an end user's account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferIntentCreateMode {
    #[serde(rename = "PAYMENT")]
    PAYMENT,
    #[serde(rename = "DISBURSEMENT")]
    DISBURSEMENT,
}

impl ToString for TransferIntentCreateMode {
    fn to_string(&self) -> String {
        match self {
            Self::PAYMENT => String::from("PAYMENT"),
            Self::DISBURSEMENT => String::from("DISBURSEMENT"),
        }
    }
}

impl Default for TransferIntentCreateMode {
    fn default() -> TransferIntentCreateMode {
        Self::PAYMENT
    }
}

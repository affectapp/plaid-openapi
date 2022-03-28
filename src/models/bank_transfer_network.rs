/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferNetwork : The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.

/// The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankTransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "wire")]
    Wire,
}

impl ToString for BankTransferNetwork {
    fn to_string(&self) -> String {
        match self {
            Self::Ach => String::from("ach"),
            Self::SameDayAch => String::from("same-day-ach"),
            Self::Wire => String::from("wire"),
        }
    }
}

impl Default for BankTransferNetwork {
    fn default() -> BankTransferNetwork {
        Self::Ach
    }
}

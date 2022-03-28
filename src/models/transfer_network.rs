/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferNetwork : The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`.

/// The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,

}

impl ToString for TransferNetwork {
    fn to_string(&self) -> String {
        match self {
            Self::Ach => String::from("ach"),
            Self::SameDayAch => String::from("same-day-ach"),
        }
    }
}

impl Default for TransferNetwork {
    fn default() -> TransferNetwork {
        Self::Ach
    }
}





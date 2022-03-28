/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferAuthorizationDecisionRationaleCode : A code representing the rationale for approving or declining the proposed transfer. Possible values are:  `MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid will offer `approved` as a transaction decision.  `LOGIN_REQUIRED` – Unable to collect the account information due to Item staleness. Can be rectified using Link in update mode. Plaid will offer `approved` as a transaction decision.  `ERROR` – Unable to collect the account information due to an error. Plaid will offer `approved` as a transaction decision.  `NSF` – Transaction likely to result in a return due to insufficient funds. Plaid will offer `declined` as a transaction decision.  `RISK` - Transaction is high-risk. Plaid will offer `declined` as a transaction decision.

/// A code representing the rationale for approving or declining the proposed transfer. Possible values are:  `MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid will offer `approved` as a transaction decision.  `LOGIN_REQUIRED` – Unable to collect the account information due to Item staleness. Can be rectified using Link in update mode. Plaid will offer `approved` as a transaction decision.  `ERROR` – Unable to collect the account information due to an error. Plaid will offer `approved` as a transaction decision.  `NSF` – Transaction likely to result in a return due to insufficient funds. Plaid will offer `declined` as a transaction decision.  `RISK` - Transaction is high-risk. Plaid will offer `declined` as a transaction decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransferAuthorizationDecisionRationaleCode {
    #[serde(rename = "NSF")]
    NSF,
    #[serde(rename = "RISK")]
    RISK,
    #[serde(rename = "MANUALLY_VERIFIED_ITEM")]
    MANUALLYVERIFIEDITEM,
    #[serde(rename = "LOGIN_REQUIRED")]
    LOGINREQUIRED,
    #[serde(rename = "ERROR")]
    ERROR,

}

impl ToString for TransferAuthorizationDecisionRationaleCode {
    fn to_string(&self) -> String {
        match self {
            Self::NSF => String::from("NSF"),
            Self::RISK => String::from("RISK"),
            Self::MANUALLYVERIFIEDITEM => String::from("MANUALLY_VERIFIED_ITEM"),
            Self::LOGINREQUIRED => String::from("LOGIN_REQUIRED"),
            Self::ERROR => String::from("ERROR"),
        }
    }
}

impl Default for TransferAuthorizationDecisionRationaleCode {
    fn default() -> TransferAuthorizationDecisionRationaleCode {
        Self::NSF
    }
}




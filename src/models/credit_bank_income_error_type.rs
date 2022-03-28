/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeErrorType : A broad categorization of the error. Safe for programmatic use.

/// A broad categorization of the error. Safe for programmatic use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankIncomeErrorType {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    INTERNALSERVERERROR,
    #[serde(rename = "INSUFFICIENT_CREDENTIALS")]
    INSUFFICIENTCREDENTIALS,
    #[serde(rename = "ITEM_LOCKED")]
    ITEMLOCKED,
    #[serde(rename = "USER_SETUP_REQUIRED")]
    USERSETUPREQUIRED,
    #[serde(rename = "COUNTRY_NOT_SUPPORTED")]
    COUNTRYNOTSUPPORTED,
    #[serde(rename = "INSTITUTION_DOWN")]
    INSTITUTIONDOWN,
    #[serde(rename = "INSTITUTION_NO_LONGER_SUPPORTED")]
    INSTITUTIONNOLONGERSUPPORTED,
    #[serde(rename = "INSTITUTION_NOT_RESPONDING")]
    INSTITUTIONNOTRESPONDING,
    #[serde(rename = "INVALID_CREDENTIALS")]
    INVALIDCREDENTIALS,
    #[serde(rename = "INVALID_MFA")]
    INVALIDMFA,
    #[serde(rename = "INVALID_SEND_METHOD")]
    INVALIDSENDMETHOD,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ITEMLOGINREQUIRED,
    #[serde(rename = "MFA_NOT_SUPPORTED")]
    MFANOTSUPPORTED,
    #[serde(rename = "NO_ACCOUNTS")]
    NOACCOUNTS,
    #[serde(rename = "ITEM_NOT_SUPPORTED")]
    ITEMNOTSUPPORTED,
    #[serde(rename = "ACCESS_NOT_GRANTED")]
    ACCESSNOTGRANTED,
}

impl ToString for CreditBankIncomeErrorType {
    fn to_string(&self) -> String {
        match self {
            Self::INTERNALSERVERERROR => String::from("INTERNAL_SERVER_ERROR"),
            Self::INSUFFICIENTCREDENTIALS => String::from("INSUFFICIENT_CREDENTIALS"),
            Self::ITEMLOCKED => String::from("ITEM_LOCKED"),
            Self::USERSETUPREQUIRED => String::from("USER_SETUP_REQUIRED"),
            Self::COUNTRYNOTSUPPORTED => String::from("COUNTRY_NOT_SUPPORTED"),
            Self::INSTITUTIONDOWN => String::from("INSTITUTION_DOWN"),
            Self::INSTITUTIONNOLONGERSUPPORTED => String::from("INSTITUTION_NO_LONGER_SUPPORTED"),
            Self::INSTITUTIONNOTRESPONDING => String::from("INSTITUTION_NOT_RESPONDING"),
            Self::INVALIDCREDENTIALS => String::from("INVALID_CREDENTIALS"),
            Self::INVALIDMFA => String::from("INVALID_MFA"),
            Self::INVALIDSENDMETHOD => String::from("INVALID_SEND_METHOD"),
            Self::ITEMLOGINREQUIRED => String::from("ITEM_LOGIN_REQUIRED"),
            Self::MFANOTSUPPORTED => String::from("MFA_NOT_SUPPORTED"),
            Self::NOACCOUNTS => String::from("NO_ACCOUNTS"),
            Self::ITEMNOTSUPPORTED => String::from("ITEM_NOT_SUPPORTED"),
            Self::ACCESSNOTGRANTED => String::from("ACCESS_NOT_GRANTED"),
        }
    }
}

impl Default for CreditBankIncomeErrorType {
    fn default() -> CreditBankIncomeErrorType {
        Self::INTERNALSERVERERROR
    }
}

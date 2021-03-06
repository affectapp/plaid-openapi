/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeWarningCode : The warning code identifies a specific kind of warning. `IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item `TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item `ITEM_UNAPPROVED`: User did not grant permission to share income data for the Item

/// The warning code identifies a specific kind of warning. `IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item `TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item `ITEM_UNAPPROVED`: User did not grant permission to share income data for the Item
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningCode {
    #[serde(rename = "IDENTITY_UNAVAILABLE")]
    IDENTITYUNAVAILABLE,
    #[serde(rename = "TRANSACTIONS_UNAVAILABLE")]
    TRANSACTIONSUNAVAILABLE,
    #[serde(rename = "ITEM_UNAPPROVED")]
    ITEMUNAPPROVED,
}

impl ToString for CreditBankIncomeWarningCode {
    fn to_string(&self) -> String {
        match self {
            Self::IDENTITYUNAVAILABLE => String::from("IDENTITY_UNAVAILABLE"),
            Self::TRANSACTIONSUNAVAILABLE => String::from("TRANSACTIONS_UNAVAILABLE"),
            Self::ITEMUNAPPROVED => String::from("ITEM_UNAPPROVED"),
        }
    }
}

impl Default for CreditBankIncomeWarningCode {
    fn default() -> CreditBankIncomeWarningCode {
        Self::IDENTITYUNAVAILABLE
    }
}

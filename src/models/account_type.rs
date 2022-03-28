/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountType : `investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.  `credit:` Credit card  `depository:` Depository account  `loan:` Loan account  `other:` Non-specified account type  See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.

/// `investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.  `credit:` Credit card  `depository:` Depository account  `loan:` Loan account  `other:` Non-specified account type  See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "other")]
    Other,

}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            Self::Investment => String::from("investment"),
            Self::Credit => String::from("credit"),
            Self::Depository => String::from("depository"),
            Self::Loan => String::from("loan"),
            Self::Brokerage => String::from("brokerage"),
            Self::Other => String::from("other"),
        }
    }
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Investment
    }
}





/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreditBankIncomeAccount : The Item's accounts that have Bank Income data.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreditBankIncomeAccount {
    /// Plaid's unique identifier for the account.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    #[serde(rename = "mask", skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    /// The name of the bank account.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The official name of the bank account.
    #[serde(rename = "official_name", skip_serializing_if = "Option::is_none")]
    pub official_name: Option<String>,
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<crate::models::DepositoryAccountSubtype>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::CreditBankIncomeAccountType>,
    #[serde(rename = "owners", skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<crate::models::Owner>>,
}

impl CreditBankIncomeAccount {
    /// The Item's accounts that have Bank Income data.
    pub fn new() -> CreditBankIncomeAccount {
        CreditBankIncomeAccount {
            account_id: None,
            mask: None,
            name: None,
            official_name: None,
            subtype: None,
            _type: None,
            owners: None,
        }
    }
}

/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// NumbersEft : Identifying information for transferring money to or from a Canadian bank account via EFT.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NumbersEft {
    /// The Plaid account ID associated with the account numbers
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The EFT account number for the account
    #[serde(rename = "account")]
    pub account: String,
    /// The EFT institution number for the account
    #[serde(rename = "institution")]
    pub institution: String,
    /// The EFT branch number for the account
    #[serde(rename = "branch")]
    pub branch: String,
}

impl NumbersEft {
    /// Identifying information for transferring money to or from a Canadian bank account via EFT.
    pub fn new(
        account_id: String,
        account: String,
        institution: String,
        branch: String,
    ) -> NumbersEft {
        NumbersEft {
            account_id,
            account,
            institution,
            branch,
        }
    }
}

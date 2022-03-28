/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NumbersAchNullable : Identifying information for transferring money to or from a US account via ACH or wire transfer.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NumbersAchNullable {
    /// The Plaid account ID associated with the account numbers
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The ACH account number for the account.  Note that when using OAuth with Chase Bank (`ins_56`), Chase will issue \"tokenized\" routing and account numbers, which are not the user's actual account and routing numbers. These tokenized numbers should work identically to normal account and routing numbers. The digits returned in the `mask` field will continue to reflect the actual account number, rather than the tokenized account number; for this reason, when displaying account numbers to the user to help them identify their account in your UI, always use the `mask` rather than truncating the `account` number. If a user revokes their permissions to your app, the tokenized numbers will continue to work for ACH deposits, but not withdrawals.
    #[serde(rename = "account")]
    pub account: String,
    /// The ACH routing number for the account. If the institution is `ins_56`, this may be a tokenized routing number. For more information, see the description of the `account` field.
    #[serde(rename = "routing")]
    pub routing: String,
    /// The wire transfer routing number for the account, if available
    #[serde(rename = "wire_routing")]
    pub wire_routing: Option<String>,
}

impl NumbersAchNullable {
    /// Identifying information for transferring money to or from a US account via ACH or wire transfer.
    pub fn new(account_id: String, account: String, routing: String, wire_routing: Option<String>) -> NumbersAchNullable {
        NumbersAchNullable {
            account_id,
            account,
            routing,
            wire_routing,
        }
    }
}



/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountIdentity : Identity information about an account



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountIdentity {
    /// Plaid’s unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.  The `account_id` can also change if the `access_token` is deleted and the same credentials that were used to generate that `access_token` are used to generate a new `access_token` on a later date. In that case, the new `account_id` will be different from the old `account_id`.  If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.  Like all Plaid identifiers, the `account_id` is case sensitive.
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "balances")]
    pub balances: crate::models::AccountBalance,
    /// The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    #[serde(rename = "mask")]
    pub mask: Option<String>,
    /// The name of the account, either assigned by the user or by the financial institution itself
    #[serde(rename = "name")]
    pub name: String,
    /// The official name of the account as given by the financial institution
    #[serde(rename = "official_name")]
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::AccountType,
    #[serde(rename = "subtype")]
    pub subtype: Option<crate::models::AccountSubtype>,
    /// The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.  `pending_automatic_verification`: The Item is pending automatic verification  `pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.  `automatically_verified`: The Item has successfully been automatically verified   `manually_verified`: The Item has successfully been manually verified  `verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.  `verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.   
    #[serde(rename = "verification_status", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<VerificationStatus>,
    /// Data returned by the financial institution about the account owner or owners. Only returned by Identity or Assets endpoints. For business accounts, the name reported may be either the name of the individual or the name of the business, depending on the institution. Multiple owners on a single account will be represented in the same `owner` object, not in multiple owner objects within the array. In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29)
    #[serde(rename = "owners")]
    pub owners: Vec<crate::models::Owner>,
}

impl AccountIdentity {
    /// Identity information about an account
    pub fn new(account_id: String, balances: crate::models::AccountBalance, mask: Option<String>, name: String, official_name: Option<String>, _type: crate::models::AccountType, subtype: Option<crate::models::AccountSubtype>, owners: Vec<crate::models::Owner>) -> AccountIdentity {
        AccountIdentity {
            account_id,
            balances,
            mask,
            name,
            official_name,
            _type,
            subtype,
            verification_status: None,
            owners,
        }
    }
}

/// The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.  `pending_automatic_verification`: The Item is pending automatic verification  `pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.  `automatically_verified`: The Item has successfully been automatically verified   `manually_verified`: The Item has successfully been manually verified  `verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.  `verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.   
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VerificationStatus {
    #[serde(rename = "automatically_verified")]
    AutomaticallyVerified,
    #[serde(rename = "pending_automatic_verification")]
    PendingAutomaticVerification,
    #[serde(rename = "pending_manual_verification")]
    PendingManualVerification,
    #[serde(rename = "manually_verified")]
    ManuallyVerified,
    #[serde(rename = "verification_expired")]
    VerificationExpired,
    #[serde(rename = "verification_failed")]
    VerificationFailed,
}

impl Default for VerificationStatus {
    fn default() -> VerificationStatus {
        Self::AutomaticallyVerified
    }
}


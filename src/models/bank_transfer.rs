/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BankTransfer : Represents a bank transfer within the Bank Transfers API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransfer {
    /// Plaid’s unique identifier for a bank transfer.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ach_class")]
    pub ach_class: crate::models::AchClass,
    /// The account ID that should be credited/debited for this bank transfer.
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub _type: crate::models::BankTransferType,
    #[serde(rename = "user")]
    pub user: crate::models::BankTransferUser,
    /// The amount of the bank transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount")]
    pub amount: String,
    /// The currency of the transfer amount, e.g. \"USD\"
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
    /// The description of the transfer.
    #[serde(rename = "description")]
    pub description: String,
    /// The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "status")]
    pub status: crate::models::BankTransferStatus,
    #[serde(rename = "network")]
    pub network: crate::models::BankTransferNetwork,
    /// When `true`, you can still cancel this bank transfer.
    #[serde(rename = "cancellable")]
    pub cancellable: bool,
    #[serde(rename = "failure_reason")]
    pub failure_reason: Option<crate::models::BankTransferFailure>,
    /// A string containing the custom tag provided by the client in the create request. Will be null if not provided.
    #[serde(rename = "custom_tag")]
    pub custom_tag: Option<String>,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters 
    #[serde(rename = "metadata")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// Plaid’s unique identifier for the origination account that was used for this transfer.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    #[serde(rename = "direction")]
    pub direction: Option<crate::models::BankTransferDirection>,
}

impl BankTransfer {
    /// Represents a bank transfer within the Bank Transfers API.
    pub fn new(id: String, ach_class: crate::models::AchClass, account_id: String, _type: crate::models::BankTransferType, user: crate::models::BankTransferUser, amount: String, iso_currency_code: String, description: String, created: String, status: crate::models::BankTransferStatus, network: crate::models::BankTransferNetwork, cancellable: bool, failure_reason: Option<crate::models::BankTransferFailure>, custom_tag: Option<String>, metadata: Option<::std::collections::HashMap<String, String>>, origination_account_id: String, direction: Option<crate::models::BankTransferDirection>) -> BankTransfer {
        BankTransfer {
            id,
            ach_class,
            account_id,
            _type,
            user,
            amount,
            iso_currency_code,
            description,
            created,
            status,
            network,
            cancellable,
            failure_reason,
            custom_tag,
            metadata,
            origination_account_id,
            direction,
        }
    }
}



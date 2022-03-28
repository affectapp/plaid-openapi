/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// TransferIntentCreate : Represents a transfer intent within Transfer UI.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    /// Plaid's unique identifier for the transfer intent object.
    #[serde(rename = "id")]
    pub id: String,
    /// The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "status")]
    pub status: crate::models::TransferIntentStatus,
    /// The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: String,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "mode")]
    pub mode: crate::models::TransferIntentCreateMode,
    #[serde(rename = "ach_class")]
    pub ach_class: crate::models::AchClass,
    #[serde(rename = "user")]
    pub user: crate::models::TransferUserInResponse,
    /// A description for the underlying transfer. Maximum of 8 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// The currency of the transfer amount, e.g. \"USD\"
    #[serde(rename = "iso_currency_code")]
    pub iso_currency_code: String,
}

impl TransferIntentCreate {
    /// Represents a transfer intent within Transfer UI.
    pub fn new(
        id: String,
        created: String,
        status: crate::models::TransferIntentStatus,
        origination_account_id: String,
        amount: String,
        mode: crate::models::TransferIntentCreateMode,
        ach_class: crate::models::AchClass,
        user: crate::models::TransferUserInResponse,
        description: String,
        iso_currency_code: String,
    ) -> TransferIntentCreate {
        TransferIntentCreate {
            id,
            created,
            status,
            account_id: None,
            origination_account_id,
            amount,
            mode,
            ach_class,
            user,
            description,
            metadata: None,
            iso_currency_code,
        }
    }
}

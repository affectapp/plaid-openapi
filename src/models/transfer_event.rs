/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferEvent : Represents an event in the Transfers API.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferEvent {
    /// Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    #[serde(rename = "event_id")]
    pub event_id: i32,
    /// The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "event_type")]
    pub event_type: crate::models::TransferEventType,
    /// The account ID associated with the transfer.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Plaid’s unique identifier for a transfer.
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    /// The ID of the origination account that this balance belongs to.
    #[serde(rename = "origination_account_id")]
    pub origination_account_id: Option<String>,
    #[serde(rename = "transfer_type")]
    pub transfer_type: crate::models::TransferType,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "transfer_amount")]
    pub transfer_amount: String,
    #[serde(rename = "failure_reason")]
    pub failure_reason: Option<crate::models::TransferFailure>,
    /// Plaid’s unique identifier for a sweep.
    #[serde(rename = "sweep_id")]
    pub sweep_id: Option<String>,
    /// A signed amount of how much was `swept` or `reverse_swept` (decimal string with two digits of precision e.g. \"-5.50\").
    #[serde(rename = "sweep_amount")]
    pub sweep_amount: Option<String>,
}

impl TransferEvent {
    /// Represents an event in the Transfers API.
    pub fn new(event_id: i32, timestamp: String, event_type: crate::models::TransferEventType, account_id: String, transfer_id: String, origination_account_id: Option<String>, transfer_type: crate::models::TransferType, transfer_amount: String, failure_reason: Option<crate::models::TransferFailure>, sweep_id: Option<String>, sweep_amount: Option<String>) -> TransferEvent {
        TransferEvent {
            event_id,
            timestamp,
            event_type,
            account_id,
            transfer_id,
            origination_account_id,
            transfer_type,
            transfer_amount,
            failure_reason,
            sweep_id,
            sweep_amount,
        }
    }
}


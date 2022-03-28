/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// BankTransferEventSyncResponse : Defines the response schema for `/bank_transfer/event/sync`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BankTransferEventSyncResponse {
    #[serde(rename = "bank_transfer_events")]
    pub bank_transfer_events: Vec<crate::models::BankTransferEvent>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl BankTransferEventSyncResponse {
    /// Defines the response schema for `/bank_transfer/event/sync`
    pub fn new(
        bank_transfer_events: Vec<crate::models::BankTransferEvent>,
        request_id: String,
    ) -> BankTransferEventSyncResponse {
        BankTransferEventSyncResponse {
            bank_transfer_events,
            request_id,
        }
    }
}

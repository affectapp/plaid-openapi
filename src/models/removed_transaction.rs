/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RemovedTransaction : A representation of a removed transaction



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemovedTransaction {
    /// The ID of the removed transaction.
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl RemovedTransaction {
    /// A representation of a removed transaction
    pub fn new() -> RemovedTransaction {
        RemovedTransaction {
            transaction_id: None,
        }
    }
}


/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ItemRemoveResponse : ItemRemoveResponse defines the response schema for `/item/remove`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemRemoveResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl ItemRemoveResponse {
    /// ItemRemoveResponse defines the response schema for `/item/remove`
    pub fn new(request_id: String) -> ItemRemoveResponse {
        ItemRemoveResponse { request_id }
    }
}

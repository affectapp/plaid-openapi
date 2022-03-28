/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InstitutionsGetResponse : InstitutionsGetResponse defines the response schema for `/institutions/get`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstitutionsGetResponse {
    /// A list of Plaid institutions
    #[serde(rename = "institutions")]
    pub institutions: Vec<crate::models::Institution>,
    /// The total number of institutions available via this endpoint
    #[serde(rename = "total")]
    pub total: i32,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    #[serde(rename = "request_id")]
    pub request_id: String,
}

impl InstitutionsGetResponse {
    /// InstitutionsGetResponse defines the response schema for `/institutions/get`
    pub fn new(institutions: Vec<crate::models::Institution>, total: i32, request_id: String) -> InstitutionsGetResponse {
        InstitutionsGetResponse {
            institutions,
            total,
            request_id,
        }
    }
}



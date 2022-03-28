/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorIdentityGetRequest : ProcessorIdentityGetRequest defines the request schema for `/processor/identity/get`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    #[serde(rename = "processor_token")]
    pub processor_token: String,
}

impl ProcessorIdentityGetRequest {
    /// ProcessorIdentityGetRequest defines the request schema for `/processor/identity/get`
    pub fn new(processor_token: String) -> ProcessorIdentityGetRequest {
        ProcessorIdentityGetRequest {
            client_id: None,
            secret: None,
            processor_token,
        }
    }
}

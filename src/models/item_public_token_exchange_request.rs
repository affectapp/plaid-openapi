/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ItemPublicTokenExchangeRequest : ItemPublicTokenExchangeRequest defines the request schema for `/item/public_token/exchange`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Your `public_token`, obtained from the Link `onSuccess` callback or `/sandbox/item/public_token/create`.
    #[serde(rename = "public_token")]
    pub public_token: String,
}

impl ItemPublicTokenExchangeRequest {
    /// ItemPublicTokenExchangeRequest defines the request schema for `/item/public_token/exchange`
    pub fn new(public_token: String) -> ItemPublicTokenExchangeRequest {
        ItemPublicTokenExchangeRequest {
            client_id: None,
            secret: None,
            public_token,
        }
    }
}



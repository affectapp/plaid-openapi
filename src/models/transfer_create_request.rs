/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransferCreateRequest : Defines the request schema for `/transfer/create`



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// Deprecated. `authorization_id` is now for used idempotency instead.  A random key provided by the client, per unique transfer. Maximum of 50 characters.  The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created.
    #[serde(rename = "idempotency_key", skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// The Plaid `access_token` for the account that will be debited or credited.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The Plaid `account_id` for the account that will be debited or credited.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier.
    #[serde(rename = "authorization_id")]
    pub authorization_id: String,
    #[serde(rename = "type")]
    pub _type: crate::models::TransferType,
    #[serde(rename = "network")]
    pub network: crate::models::TransferNetwork,
    /// The amount of the transfer (decimal string with two digits of precision e.g. \"10.00\").
    #[serde(rename = "amount")]
    pub amount: String,
    /// The transfer description. Maximum of 10 characters.
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "ach_class")]
    pub ach_class: crate::models::AchClass,
    #[serde(rename = "user")]
    pub user: crate::models::TransferUserInRequest,
    /// The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply: - The JSON values must be Strings (no nested JSON objects allowed) - Only ASCII characters may be used - Maximum of 50 key/value pairs - Maximum key length of 40 characters - Maximum value length of 500 characters 
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    /// Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    #[serde(rename = "origination_account_id", skip_serializing_if = "Option::is_none")]
    pub origination_account_id: Option<String>,
    /// The currency of the transfer amount. The default value is \"USD\".
    #[serde(rename = "iso_currency_code", skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
}

impl TransferCreateRequest {
    /// Defines the request schema for `/transfer/create`
    pub fn new(access_token: String, account_id: String, authorization_id: String, _type: crate::models::TransferType, network: crate::models::TransferNetwork, amount: String, description: String, ach_class: crate::models::AchClass, user: crate::models::TransferUserInRequest) -> TransferCreateRequest {
        TransferCreateRequest {
            client_id: None,
            secret: None,
            idempotency_key: None,
            access_token,
            account_id,
            authorization_id,
            _type,
            network,
            amount,
            description,
            ach_class,
            user,
            metadata: None,
            origination_account_id: None,
            iso_currency_code: None,
        }
    }
}



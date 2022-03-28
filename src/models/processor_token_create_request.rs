/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProcessorTokenCreateRequest : ProcessorTokenCreateRequest defines the request schema for `/processor/token/create`

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    /// Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    #[serde(rename = "secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    /// The access token associated with the Item data is being requested for.
    #[serde(rename = "access_token")]
    pub access_token: String,
    /// The `account_id` value obtained from the `onSuccess` callback in Link
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The processor you are integrating with.
    #[serde(rename = "processor")]
    pub processor: Processor,
}

impl ProcessorTokenCreateRequest {
    /// ProcessorTokenCreateRequest defines the request schema for `/processor/token/create`
    pub fn new(
        access_token: String,
        account_id: String,
        processor: Processor,
    ) -> ProcessorTokenCreateRequest {
        ProcessorTokenCreateRequest {
            client_id: None,
            secret: None,
            access_token,
            account_id,
            processor,
        }
    }
}

/// The processor you are integrating with.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Processor {
    #[serde(rename = "dwolla")]
    Dwolla,
    #[serde(rename = "galileo")]
    Galileo,
    #[serde(rename = "modern_treasury")]
    ModernTreasury,
    #[serde(rename = "ocrolus")]
    Ocrolus,
    #[serde(rename = "prime_trust")]
    PrimeTrust,
    #[serde(rename = "vesta")]
    Vesta,
    #[serde(rename = "drivewealth")]
    Drivewealth,
    #[serde(rename = "vopay")]
    Vopay,
    #[serde(rename = "achq")]
    Achq,
    #[serde(rename = "check")]
    Check,
    #[serde(rename = "checkbook")]
    Checkbook,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "sila_money")]
    SilaMoney,
    #[serde(rename = "rize")]
    Rize,
    #[serde(rename = "svb_api")]
    SvbApi,
    #[serde(rename = "unit")]
    Unit,
    #[serde(rename = "wyre")]
    Wyre,
    #[serde(rename = "lithic")]
    Lithic,
    #[serde(rename = "alpaca")]
    Alpaca,
    #[serde(rename = "astra")]
    Astra,
    #[serde(rename = "moov")]
    Moov,
    #[serde(rename = "treasury_prime")]
    TreasuryPrime,
}

impl Default for Processor {
    fn default() -> Processor {
        Self::Dwolla
    }
}

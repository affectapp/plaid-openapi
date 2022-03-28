/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// LinkTokenCreateRequestAuth : Specifies options for initializing Link for use with the Auth product. This field is currently only required if using the Flexible Auth product (currently in closed beta).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAuth {
    /// The optional Auth flow to use. Currently only used to enable Flexible Auth.
    #[serde(rename = "flow_type")]
    pub flow_type: FlowType,
}

impl LinkTokenCreateRequestAuth {
    /// Specifies options for initializing Link for use with the Auth product. This field is currently only required if using the Flexible Auth product (currently in closed beta).
    pub fn new(flow_type: FlowType) -> LinkTokenCreateRequestAuth {
        LinkTokenCreateRequestAuth { flow_type }
    }
}

/// The optional Auth flow to use. Currently only used to enable Flexible Auth.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowType {
    #[serde(rename = "FLEXIBLE_AUTH")]
    FLEXIBLEAUTH,
}

impl Default for FlowType {
    fn default() -> FlowType {
        Self::FLEXIBLEAUTH
    }
}

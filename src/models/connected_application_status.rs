/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ConnectedApplicationStatus : Describes if the connected item is active (i.e. has not been revoked or unlinked)

/// Describes if the connected item is active (i.e. has not been revoked or unlinked)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectedApplicationStatus {
    #[serde(rename = "ACTIVE")]
    ACTIVE,
    #[serde(rename = "INACTIVE")]
    INACTIVE,

}

impl ToString for ConnectedApplicationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::ACTIVE => String::from("ACTIVE"),
            Self::INACTIVE => String::from("INACTIVE"),
        }
    }
}

impl Default for ConnectedApplicationStatus {
    fn default() -> ConnectedApplicationStatus {
        Self::ACTIVE
    }
}




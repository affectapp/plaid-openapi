/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentScheduleInterval : The frequency interval of the payment.

/// The frequency interval of the payment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PaymentScheduleInterval {
    #[serde(rename = "WEEKLY")]
    WEEKLY,
    #[serde(rename = "MONTHLY")]
    MONTHLY,

}

impl ToString for PaymentScheduleInterval {
    fn to_string(&self) -> String {
        match self {
            Self::WEEKLY => String::from("WEEKLY"),
            Self::MONTHLY => String::from("MONTHLY"),
        }
    }
}

impl Default for PaymentScheduleInterval {
    fn default() -> PaymentScheduleInterval {
        Self::WEEKLY
    }
}





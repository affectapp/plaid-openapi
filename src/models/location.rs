/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.84.5
 *
 * Generated by: https://openapi-generator.tech
 */

/// Location : A representation of where a transaction took place

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    /// The street address where the transaction occurred.
    #[serde(rename = "address")]
    pub address: Option<String>,
    /// The city where the transaction occurred.
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// The region or state where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `state`.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The postal code where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `zip`.
    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code where the transaction occurred.
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// The latitude where the transaction occurred.
    #[serde(rename = "lat")]
    pub lat: Option<f64>,
    /// The longitude where the transaction occurred.
    #[serde(rename = "lon")]
    pub lon: Option<f64>,
    /// The merchant defined store number where the transaction occurred.
    #[serde(rename = "store_number")]
    pub store_number: Option<String>,
}

impl Location {
    /// A representation of where a transaction took place
    pub fn new(
        address: Option<String>,
        city: Option<String>,
        region: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        lat: Option<f64>,
        lon: Option<f64>,
        store_number: Option<String>,
    ) -> Location {
        Location {
            address,
            city,
            region,
            postal_code,
            country,
            lat,
            lon,
            store_number,
        }
    }
}

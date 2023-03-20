/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaxInfoResponseModel {
    #[serde(rename = "taxIdNumber", skip_serializing_if = "Option::is_none")]
    pub tax_id_number: Option<String>,
    #[serde(rename = "taxIdType", skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<String>,
    #[serde(rename = "line1", skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(rename = "line2", skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl TaxInfoResponseModel {
    pub fn new() -> TaxInfoResponseModel {
        TaxInfoResponseModel {
            tax_id_number: None,
            tax_id_type: None,
            line1: None,
            line2: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
        }
    }
}
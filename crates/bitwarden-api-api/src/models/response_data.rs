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
pub struct ResponseData {
    #[serde(rename = "attestationObject", skip_serializing_if = "Option::is_none")]
    pub attestation_object: Option<String>,
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
}

impl ResponseData {
    pub fn new() -> ResponseData {
        ResponseData {
            attestation_object: None,
            client_data_json: None,
        }
    }
}
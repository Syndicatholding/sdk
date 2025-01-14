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
pub struct DeviceRequestModel {
    #[serde(rename = "type")]
    pub _type: crate::models::DeviceType,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "pushToken", skip_serializing_if = "Option::is_none")]
    pub push_token: Option<String>,
}

impl DeviceRequestModel {
    pub fn new(
        _type: crate::models::DeviceType,
        name: String,
        identifier: String,
    ) -> DeviceRequestModel {
        DeviceRequestModel {
            _type,
            name,
            identifier,
            push_token: None,
        }
    }
}

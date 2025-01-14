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
pub struct EmergencyAccessUpdateRequestModel {
    #[serde(rename = "type")]
    pub _type: crate::models::EmergencyAccessType,
    #[serde(rename = "waitTimeDays")]
    pub wait_time_days: i32,
    #[serde(rename = "keyEncrypted", skip_serializing_if = "Option::is_none")]
    pub key_encrypted: Option<String>,
}

impl EmergencyAccessUpdateRequestModel {
    pub fn new(
        _type: crate::models::EmergencyAccessType,
        wait_time_days: i32,
    ) -> EmergencyAccessUpdateRequestModel {
        EmergencyAccessUpdateRequestModel {
            _type,
            wait_time_days,
            key_encrypted: None,
        }
    }
}

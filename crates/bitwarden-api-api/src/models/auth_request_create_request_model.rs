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
pub struct AuthRequestCreateRequestModel {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: String,
    #[serde(rename = "accessCode")]
    pub access_code: String,
    #[serde(rename = "type")]
    pub _type: crate::models::AuthRequestType,
    #[serde(rename = "fingerprintPhrase")]
    pub fingerprint_phrase: String,
}

impl AuthRequestCreateRequestModel {
    pub fn new(
        email: String,
        public_key: String,
        device_identifier: String,
        access_code: String,
        _type: crate::models::AuthRequestType,
        fingerprint_phrase: String,
    ) -> AuthRequestCreateRequestModel {
        AuthRequestCreateRequestModel {
            email,
            public_key,
            device_identifier,
            access_code,
            _type,
            fingerprint_phrase,
        }
    }
}

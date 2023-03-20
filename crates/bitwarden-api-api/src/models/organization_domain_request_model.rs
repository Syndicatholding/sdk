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
pub struct OrganizationDomainRequestModel {
    #[serde(rename = "txt")]
    pub txt: String,
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

impl OrganizationDomainRequestModel {
    pub fn new(txt: String, domain_name: String) -> OrganizationDomainRequestModel {
        OrganizationDomainRequestModel { txt, domain_name }
    }
}
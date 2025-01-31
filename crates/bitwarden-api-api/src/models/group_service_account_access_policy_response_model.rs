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
pub struct GroupServiceAccountAccessPolicyResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "write", skip_serializing_if = "Option::is_none")]
    pub write: Option<bool>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(
        rename = "grantedServiceAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub granted_service_account_id: Option<String>,
}

impl GroupServiceAccountAccessPolicyResponseModel {
    pub fn new() -> GroupServiceAccountAccessPolicyResponseModel {
        GroupServiceAccountAccessPolicyResponseModel {
            object: None,
            id: None,
            read: None,
            write: None,
            creation_date: None,
            revision_date: None,
            group_id: None,
            group_name: None,
            granted_service_account_id: None,
        }
    }
}

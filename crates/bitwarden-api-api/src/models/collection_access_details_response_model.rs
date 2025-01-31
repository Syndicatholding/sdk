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
pub struct CollectionAccessDetailsResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::SelectionReadOnlyResponseModel>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::SelectionReadOnlyResponseModel>>,
    /// True if the acting user is explicitly assigned to the collection
    #[serde(rename = "assigned", skip_serializing_if = "Option::is_none")]
    pub assigned: Option<bool>,
}

impl CollectionAccessDetailsResponseModel {
    pub fn new() -> CollectionAccessDetailsResponseModel {
        CollectionAccessDetailsResponseModel {
            object: None,
            id: None,
            organization_id: None,
            name: None,
            external_id: None,
            groups: None,
            users: None,
            assigned: None,
        }
    }
}

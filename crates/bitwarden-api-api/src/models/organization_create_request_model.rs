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
pub struct OrganizationCreateRequestModel {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "businessName", skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(rename = "billingEmail")]
    pub billing_email: String,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<crate::models::PlanType>,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Box<crate::models::OrganizationKeysRequestModel>>,
    #[serde(rename = "paymentMethodType", skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<crate::models::PaymentMethodType>,
    #[serde(rename = "paymentToken", skip_serializing_if = "Option::is_none")]
    pub payment_token: Option<String>,
    #[serde(rename = "additionalSeats", skip_serializing_if = "Option::is_none")]
    pub additional_seats: Option<i32>,
    #[serde(
        rename = "additionalStorageGb",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_storage_gb: Option<i32>,
    #[serde(rename = "premiumAccessAddon", skip_serializing_if = "Option::is_none")]
    pub premium_access_addon: Option<bool>,
    #[serde(rename = "collectionName", skip_serializing_if = "Option::is_none")]
    pub collection_name: Option<String>,
    #[serde(rename = "taxIdNumber", skip_serializing_if = "Option::is_none")]
    pub tax_id_number: Option<String>,
    #[serde(
        rename = "billingAddressLine1",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_line1: Option<String>,
    #[serde(
        rename = "billingAddressLine2",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_line2: Option<String>,
    #[serde(rename = "billingAddressCity", skip_serializing_if = "Option::is_none")]
    pub billing_address_city: Option<String>,
    #[serde(
        rename = "billingAddressState",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_state: Option<String>,
    #[serde(
        rename = "billingAddressPostalCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_postal_code: Option<String>,
    #[serde(
        rename = "billingAddressCountry",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_address_country: Option<String>,
    #[serde(rename = "maxAutoscaleSeats", skip_serializing_if = "Option::is_none")]
    pub max_autoscale_seats: Option<i32>,
}

impl OrganizationCreateRequestModel {
    pub fn new(name: String, billing_email: String, key: String) -> OrganizationCreateRequestModel {
        OrganizationCreateRequestModel {
            name,
            business_name: None,
            billing_email,
            plan_type: None,
            key,
            keys: None,
            payment_method_type: None,
            payment_token: None,
            additional_seats: None,
            additional_storage_gb: None,
            premium_access_addon: None,
            collection_name: None,
            tax_id_number: None,
            billing_address_line1: None,
            billing_address_line2: None,
            billing_address_city: None,
            billing_address_state: None,
            billing_address_postal_code: None,
            billing_address_country: None,
            max_autoscale_seats: None,
        }
    }
}
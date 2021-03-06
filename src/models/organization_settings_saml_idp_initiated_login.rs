/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationSettingsSamlIdpInitiatedLogin : Has one property enabled (boolean).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlIdpInitiatedLogin {
    /// Whether SAML IdP initiated login is enabled, learn more in the [SAML documentation](https://docs.datadoghq.com/account_management/saml/#idp-initiated-login).
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSamlIdpInitiatedLogin {
    /// Has one property enabled (boolean).
    pub fn new() -> OrganizationSettingsSamlIdpInitiatedLogin {
        OrganizationSettingsSamlIdpInitiatedLogin {
            enabled: None,
        }
    }
}



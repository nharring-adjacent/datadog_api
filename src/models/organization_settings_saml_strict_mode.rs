/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationSettingsSamlStrictMode : Has one property enabled (boolean).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSettingsSamlStrictMode {
    /// Whether or not the SAML strict mode is enabled. If true, all users must log in with SAML. Learn more on the [SAML Strict documentation](https://docs.datadoghq.com/account_management/saml/#saml-strict).
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl OrganizationSettingsSamlStrictMode {
    /// Has one property enabled (boolean).
    pub fn new() -> OrganizationSettingsSamlStrictMode {
        OrganizationSettingsSamlStrictMode {
            enabled: None,
        }
    }
}



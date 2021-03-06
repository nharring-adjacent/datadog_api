/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AzureAccount : Datadog-Azure integrations configured for your organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAccount {
    /// Your Azure web application ID.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Your Azure web application secret key.
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Errors in your configuration.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// Limit the Azure instances that are pulled into Datadog by using tags. Only hosts that match one of the defined tags are imported into Datadog.
    #[serde(rename = "host_filters", skip_serializing_if = "Option::is_none")]
    pub host_filters: Option<String>,
    /// Your New Azure web application ID.
    #[serde(rename = "new_client_id", skip_serializing_if = "Option::is_none")]
    pub new_client_id: Option<String>,
    /// Your New Azure Active Directory ID.
    #[serde(rename = "new_tenant_name", skip_serializing_if = "Option::is_none")]
    pub new_tenant_name: Option<String>,
    /// Your Azure Active Directory ID.
    #[serde(rename = "tenant_name", skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

impl AzureAccount {
    /// Datadog-Azure integrations configured for your organization.
    pub fn new() -> AzureAccount {
        AzureAccount {
            client_id: None,
            client_secret: None,
            errors: None,
            host_filters: None,
            new_client_id: None,
            new_tenant_name: None,
            tenant_name: None,
        }
    }
}



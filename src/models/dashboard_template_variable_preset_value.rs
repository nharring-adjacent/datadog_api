/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardTemplateVariablePresetValue : Template variables saved views.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardTemplateVariablePresetValue {
    /// The name of the variable.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the template variable within the saved view.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DashboardTemplateVariablePresetValue {
    /// Template variables saved views.
    pub fn new() -> DashboardTemplateVariablePresetValue {
        DashboardTemplateVariablePresetValue {
            name: None,
            value: None,
        }
    }
}



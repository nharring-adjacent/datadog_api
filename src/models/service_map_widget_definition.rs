/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceMapWidgetDefinition : This widget displays a map of a service to all of the services that call it, and all of the services that it calls.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Option<Vec<crate::models::WidgetCustomLink>>,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// The ID of the service you want to map.
    #[serde(rename = "service")]
    pub service: String,
    /// The title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: Option<crate::models::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::ServiceMapWidgetDefinitionType,
}

impl ServiceMapWidgetDefinition {
    /// This widget displays a map of a service to all of the services that call it, and all of the services that it calls.
    pub fn new(filters: Vec<String>, service: String, _type: crate::models::ServiceMapWidgetDefinitionType) -> ServiceMapWidgetDefinition {
        ServiceMapWidgetDefinition {
            custom_links: None,
            filters,
            service,
            title: None,
            title_align: None,
            title_size: None,
            _type,
        }
    }
}



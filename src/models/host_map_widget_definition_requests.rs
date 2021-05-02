/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// HostMapWidgetDefinitionRequests : List of definitions.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostMapWidgetDefinitionRequests {
    #[serde(rename = "fill", skip_serializing_if = "Option::is_none")]
    pub fill: Option<Box<crate::models::HostMapRequest>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Box<crate::models::HostMapRequest>>,
}

impl HostMapWidgetDefinitionRequests {
    /// List of definitions.
    pub fn new() -> HostMapWidgetDefinitionRequests {
        HostMapWidgetDefinitionRequests {
            fill: None,
            size: None,
        }
    }
}



/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceMapWidgetDefinitionType : Type of the service map widget.

/// Type of the service map widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceMapWidgetDefinitionType {
    #[serde(rename = "servicemap")]
    SERVICEMAP,

}

impl ToString for ServiceMapWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::SERVICEMAP => String::from("servicemap"),
        }
    }
}




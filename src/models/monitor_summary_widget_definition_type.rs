/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MonitorSummaryWidgetDefinitionType : Type of the monitor summary widget.

/// Type of the monitor summary widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MonitorSummaryWidgetDefinitionType {
    #[serde(rename = "manage_status")]
    MANAGE_STATUS,

}

impl ToString for MonitorSummaryWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::MANAGE_STATUS => String::from("manage_status"),
        }
    }
}




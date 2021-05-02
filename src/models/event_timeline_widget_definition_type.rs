/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// EventTimelineWidgetDefinitionType : Type of the event timeline widget.

/// Type of the event timeline widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventTimelineWidgetDefinitionType {
    #[serde(rename = "event_timeline")]
    EVENT_TIMELINE,

}

impl ToString for EventTimelineWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::EVENT_TIMELINE => String::from("event_timeline"),
        }
    }
}





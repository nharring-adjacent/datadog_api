/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// EventStreamWidgetDefinitionType : Type of the event stream widget.

/// Type of the event stream widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventStreamWidgetDefinitionType {
    #[serde(rename = "event_stream")]
    EVENT_STREAM,

}

impl ToString for EventStreamWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::EVENT_STREAM => String::from("event_stream"),
        }
    }
}




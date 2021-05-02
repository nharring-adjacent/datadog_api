/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// EventPriority : The priority of the event. For example, `normal` or `low`.

/// The priority of the event. For example, `normal` or `low`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventPriority {
    #[serde(rename = "normal")]
    NORMAL,
    #[serde(rename = "low")]
    LOW,

}

impl ToString for EventPriority {
    fn to_string(&self) -> String {
        match self {
            Self::NORMAL => String::from("normal"),
            Self::LOW => String::from("low"),
        }
    }
}





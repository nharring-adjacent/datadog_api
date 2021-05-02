/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// EventQueryDefinition : The event query.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventQueryDefinition {
    /// The query being made on the event.
    #[serde(rename = "search")]
    pub search: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution")]
    pub tags_execution: String,
}

impl EventQueryDefinition {
    /// The event query.
    pub fn new(search: String, tags_execution: String) -> EventQueryDefinition {
        EventQueryDefinition {
            search,
            tags_execution,
        }
    }
}



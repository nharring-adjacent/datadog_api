/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetEvent : Event overlay control options.  See the dedicated [Events JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/widget_json/#events-schema) to learn how to build the `<EVENTS_SCHEMA>`.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetEvent {
    /// Query definition.
    #[serde(rename = "q")]
    pub q: String,
    /// The execution method for multi-value filters.
    #[serde(rename = "tags_execution", skip_serializing_if = "Option::is_none")]
    pub tags_execution: Option<String>,
}

impl WidgetEvent {
    /// Event overlay control options.  See the dedicated [Events JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/widget_json/#events-schema) to learn how to build the `<EVENTS_SCHEMA>`.
    pub fn new(q: String) -> WidgetEvent {
        WidgetEvent {
            q,
            tags_execution: None,
        }
    }
}



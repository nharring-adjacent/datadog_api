/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsTraceRemapper : There are two ways to improve correlation between application traces and logs.    1. Follow the documentation on [how to inject a trace ID in the application logs](https://docs.datadoghq.com/tracing/connect_logs_and_traces)   and by default log integrations take care of all the rest of the setup.    2. Use the Trace remapper processor to define a log attribute as its associated trace ID.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsTraceRemapper {
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of source attributes.
    #[serde(rename = "sources", skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub _type: crate::models::LogsTraceRemapperType,
}

impl LogsTraceRemapper {
    /// There are two ways to improve correlation between application traces and logs.    1. Follow the documentation on [how to inject a trace ID in the application logs](https://docs.datadoghq.com/tracing/connect_logs_and_traces)   and by default log integrations take care of all the rest of the setup.    2. Use the Trace remapper processor to define a log attribute as its associated trace ID.
    pub fn new(_type: crate::models::LogsTraceRemapperType) -> LogsTraceRemapper {
        LogsTraceRemapper {
            is_enabled: None,
            name: None,
            sources: None,
            _type,
        }
    }
}



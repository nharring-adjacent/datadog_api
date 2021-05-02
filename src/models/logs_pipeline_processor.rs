/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsPipelineProcessor : Nested Pipelines are pipelines within a pipeline. Use Nested Pipelines to split the processing into two steps. For example, first use a high-level filtering such as team and then a second level of filtering based on the integration, service, or any other tag or attribute.  A pipeline can contain Nested Pipelines and Processors whereas a Nested Pipeline can only contain Processors.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipelineProcessor {
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::LogsFilter>>,
    /// Whether or not the processor is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Name of the processor.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::LogsProcessor>>,
    #[serde(rename = "type")]
    pub _type: crate::models::LogsPipelineProcessorType,
}

impl LogsPipelineProcessor {
    /// Nested Pipelines are pipelines within a pipeline. Use Nested Pipelines to split the processing into two steps. For example, first use a high-level filtering such as team and then a second level of filtering based on the integration, service, or any other tag or attribute.  A pipeline can contain Nested Pipelines and Processors whereas a Nested Pipeline can only contain Processors.
    pub fn new(_type: crate::models::LogsPipelineProcessorType) -> LogsPipelineProcessor {
        LogsPipelineProcessor {
            filter: None,
            is_enabled: None,
            name: None,
            processors: None,
            _type,
        }
    }
}



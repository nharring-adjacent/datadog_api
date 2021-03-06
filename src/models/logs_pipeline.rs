/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsPipeline : Pipelines and processors operate on incoming logs, parsing and transforming them into structured attributes for easier querying.  **Note**: These endpoints are only available for admin users. Make sure to use an application key created by an admin.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsPipeline {
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Box<crate::models::LogsFilter>>,
    /// ID of the pipeline.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether or not the pipeline is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    /// Whether or not the pipeline can be edited.
    #[serde(rename = "is_read_only", skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
    /// Name of the pipeline.
    #[serde(rename = "name")]
    pub name: String,
    /// Ordered list of processors in this pipeline.
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::models::LogsProcessor>>,
    /// Type of pipeline.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LogsPipeline {
    /// Pipelines and processors operate on incoming logs, parsing and transforming them into structured attributes for easier querying.  **Note**: These endpoints are only available for admin users. Make sure to use an application key created by an admin.
    pub fn new(name: String) -> LogsPipeline {
        LogsPipeline {
            filter: None,
            id: None,
            is_enabled: None,
            is_read_only: None,
            name,
            processors: None,
            _type: None,
        }
    }
}



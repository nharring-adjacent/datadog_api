/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogQueryDefinition : The log query.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogQueryDefinition {
    #[serde(rename = "compute", skip_serializing_if = "Option::is_none")]
    pub compute: Option<Box<crate::models::LogsQueryCompute>>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<crate::models::LogQueryDefinitionGroupBy>>,
    /// A coma separated-list of index names. Use \"*\" query all indexes at once. [Multiple Indexes](https://docs.datadoghq.com/logs/indexes/#multiple-indexes)
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    /// This field is mutually exclusive with `compute`.
    #[serde(rename = "multi_compute", skip_serializing_if = "Option::is_none")]
    pub multi_compute: Option<Vec<crate::models::LogsQueryCompute>>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Box<crate::models::LogQueryDefinitionSearch>>,
}

impl LogQueryDefinition {
    /// The log query.
    pub fn new() -> LogQueryDefinition {
        LogQueryDefinition {
            compute: None,
            group_by: None,
            index: None,
            multi_compute: None,
            search: None,
        }
    }
}


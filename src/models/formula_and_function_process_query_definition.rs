/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FormulaAndFunctionProcessQueryDefinition : Process query using formulas and functions.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FormulaAndFunctionProcessQueryDefinition {
    #[serde(rename = "aggregator", skip_serializing_if = "Option::is_none")]
    pub aggregator: Option<crate::models::FormulaAndFunctionMetricAggregation>,
    #[serde(rename = "data_source")]
    pub data_source: crate::models::FormulaAndFunctionProcessQueryDataSource,
    /// Whether to normalize the CPU percentages.
    #[serde(rename = "is_normalized_cpu", skip_serializing_if = "Option::is_none")]
    pub is_normalized_cpu: Option<bool>,
    /// Number of hits to return.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Process metric name.
    #[serde(rename = "metric")]
    pub metric: String,
    /// Name of query for use in formulas.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::models::QuerySortOrder>,
    /// An array of tags to filter by.
    #[serde(rename = "tag_filters", skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<String>>,
    /// Text to use as filter.
    #[serde(rename = "text_filter", skip_serializing_if = "Option::is_none")]
    pub text_filter: Option<String>,
}

impl FormulaAndFunctionProcessQueryDefinition {
    /// Process query using formulas and functions.
    pub fn new(data_source: crate::models::FormulaAndFunctionProcessQueryDataSource, metric: String, name: String) -> FormulaAndFunctionProcessQueryDefinition {
        FormulaAndFunctionProcessQueryDefinition {
            aggregator: None,
            data_source,
            is_normalized_cpu: None,
            limit: None,
            metric,
            name,
            sort: None,
            tag_filters: None,
            text_filter: None,
        }
    }
}



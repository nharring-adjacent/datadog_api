/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// QueryValueWidgetRequest : Updated query value widget.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryValueWidgetRequest {
    #[serde(rename = "aggregator", skip_serializing_if = "Option::is_none")]
    pub aggregator: Option<crate::models::WidgetAggregator>,
    #[serde(rename = "apm_query", skip_serializing_if = "Option::is_none")]
    pub apm_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// List of conditional formats.
    #[serde(rename = "conditional_formats", skip_serializing_if = "Option::is_none")]
    pub conditional_formats: Option<Vec<crate::models::WidgetConditionalFormat>>,
    #[serde(rename = "event_query", skip_serializing_if = "Option::is_none")]
    pub event_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// List of formulas that operate on queries. **This feature is currently in beta.**
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Vec<crate::models::WidgetFormula>>,
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "network_query", skip_serializing_if = "Option::is_none")]
    pub network_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "process_query", skip_serializing_if = "Option::is_none")]
    pub process_query: Option<Box<crate::models::ProcessQueryDefinition>>,
    #[serde(rename = "profile_metrics_query", skip_serializing_if = "Option::is_none")]
    pub profile_metrics_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// TODO.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// List of queries that can be returned directly or used in formulas. **This feature is currently in beta.**
    #[serde(rename = "queries", skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<crate::models::FormulaAndFunctionQueryDefinition>>,
    #[serde(rename = "response_format", skip_serializing_if = "Option::is_none")]
    pub response_format: Option<crate::models::FormulaAndFunctionResponseFormat>,
    #[serde(rename = "rum_query", skip_serializing_if = "Option::is_none")]
    pub rum_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "security_query", skip_serializing_if = "Option::is_none")]
    pub security_query: Option<Box<crate::models::LogQueryDefinition>>,
}

impl QueryValueWidgetRequest {
    /// Updated query value widget.
    pub fn new() -> QueryValueWidgetRequest {
        QueryValueWidgetRequest {
            aggregator: None,
            apm_query: None,
            conditional_formats: None,
            event_query: None,
            formulas: None,
            log_query: None,
            network_query: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
        }
    }
}



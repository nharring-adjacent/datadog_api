/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeseriesWidgetRequest : Updated timeseries widget.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeseriesWidgetRequest {
    #[serde(rename = "apm_query", skip_serializing_if = "Option::is_none")]
    pub apm_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "display_type", skip_serializing_if = "Option::is_none")]
    pub display_type: Option<crate::models::WidgetDisplayType>,
    #[serde(rename = "event_query", skip_serializing_if = "Option::is_none")]
    pub event_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// List of formulas that operate on queries. **This feature is currently in beta.**
    #[serde(rename = "formulas", skip_serializing_if = "Option::is_none")]
    pub formulas: Option<Vec<crate::models::WidgetFormula>>,
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// Used to define expression aliases.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<crate::models::TimeseriesWidgetExpressionAlias>>,
    #[serde(rename = "network_query", skip_serializing_if = "Option::is_none")]
    pub network_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// Whether or not to display a second y-axis on the right.
    #[serde(rename = "on_right_yaxis", skip_serializing_if = "Option::is_none")]
    pub on_right_yaxis: Option<bool>,
    #[serde(rename = "process_query", skip_serializing_if = "Option::is_none")]
    pub process_query: Option<Box<crate::models::ProcessQueryDefinition>>,
    #[serde(rename = "profile_metrics_query", skip_serializing_if = "Option::is_none")]
    pub profile_metrics_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// Widget query.
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
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<crate::models::WidgetRequestStyle>>,
}

impl TimeseriesWidgetRequest {
    /// Updated timeseries widget.
    pub fn new() -> TimeseriesWidgetRequest {
        TimeseriesWidgetRequest {
            apm_query: None,
            display_type: None,
            event_query: None,
            formulas: None,
            log_query: None,
            metadata: None,
            network_query: None,
            on_right_yaxis: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            queries: None,
            response_format: None,
            rum_query: None,
            security_query: None,
            style: None,
        }
    }
}



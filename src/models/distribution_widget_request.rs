/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DistributionWidgetRequest : Updated distribution widget.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetRequest {
    #[serde(rename = "apm_query", skip_serializing_if = "Option::is_none")]
    pub apm_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "event_query", skip_serializing_if = "Option::is_none")]
    pub event_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "log_query", skip_serializing_if = "Option::is_none")]
    pub log_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "network_query", skip_serializing_if = "Option::is_none")]
    pub network_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "process_query", skip_serializing_if = "Option::is_none")]
    pub process_query: Option<Box<crate::models::ProcessQueryDefinition>>,
    #[serde(rename = "profile_metrics_query", skip_serializing_if = "Option::is_none")]
    pub profile_metrics_query: Option<Box<crate::models::LogQueryDefinition>>,
    /// Widget query.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename = "rum_query", skip_serializing_if = "Option::is_none")]
    pub rum_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "security_query", skip_serializing_if = "Option::is_none")]
    pub security_query: Option<Box<crate::models::LogQueryDefinition>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Box<crate::models::WidgetStyle>>,
}

impl DistributionWidgetRequest {
    /// Updated distribution widget.
    pub fn new() -> DistributionWidgetRequest {
        DistributionWidgetRequest {
            apm_query: None,
            event_query: None,
            log_query: None,
            network_query: None,
            process_query: None,
            profile_metrics_query: None,
            q: None,
            rum_query: None,
            security_query: None,
            style: None,
        }
    }
}



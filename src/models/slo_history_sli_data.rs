/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloHistorySliData : An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value. This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloHistorySliData {
    /// A mapping of threshold `timeframe` to the remaining error budget.
    #[serde(rename = "error_budget_remaining", skip_serializing_if = "Option::is_none")]
    pub error_budget_remaining: Option<::std::collections::HashMap<String, f64>>,
    /// A list of errors while querying the history data for the service level objective.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::SloHistoryResponseError>>,
    /// For groups in a grouped SLO, this is the group name.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// For `monitor` based SLOs, this includes the aggregated history uptime time series.
    #[serde(rename = "history", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<Vec<f64>>>,
    /// For `monitor` based SLOs, this is the last modified timestamp in epoch seconds of the monitor.
    #[serde(rename = "monitor_modified", skip_serializing_if = "Option::is_none")]
    pub monitor_modified: Option<i64>,
    /// For `monitor` based SLOs, this describes the type of monitor.
    #[serde(rename = "monitor_type", skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<String>,
    /// For groups in a grouped SLO, this is the group name. For monitors in a multi-monitor SLO, this is the monitor name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A mapping of threshold `timeframe` to number of accurate decimals, regardless of the from && to timestamp.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<::std::collections::HashMap<String, f64>>,
    /// For `monitor` based SLOs, when `true` this indicates that a replay is in progress to give an accurate uptime calculation.
    #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    /// The current SLI value of the SLO over the history window.
    #[serde(rename = "sli_value", skip_serializing_if = "Option::is_none")]
    pub sli_value: Option<f64>,
    /// The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp.
    #[serde(rename = "span_precision", skip_serializing_if = "Option::is_none")]
    pub span_precision: Option<f64>,
    /// Use `sli_value` instead.
    #[serde(rename = "uptime", skip_serializing_if = "Option::is_none")]
    pub uptime: Option<f64>,
}

impl SloHistorySliData {
    /// An object that holds an SLI value and its associated data. It can represent an SLO's overall SLI value. This can also represent the SLI value for a specific monitor in multi-monitor SLOs, or a group in grouped SLOs.
    pub fn new() -> SloHistorySliData {
        SloHistorySliData {
            error_budget_remaining: None,
            errors: None,
            group: None,
            history: None,
            monitor_modified: None,
            monitor_type: None,
            name: None,
            precision: None,
            preview: None,
            sli_value: None,
            span_precision: None,
            uptime: None,
        }
    }
}



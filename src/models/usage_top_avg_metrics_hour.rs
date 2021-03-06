/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageTopAvgMetricsHour : Number of hourly recorded custom metrics for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTopAvgMetricsHour {
    /// Average number of timeseries per hour in which the metric occurs.
    #[serde(rename = "avg_metric_hour", skip_serializing_if = "Option::is_none")]
    pub avg_metric_hour: Option<i64>,
    /// Maximum number of timeseries per hour in which the metric occurs.
    #[serde(rename = "max_metric_hour", skip_serializing_if = "Option::is_none")]
    pub max_metric_hour: Option<i64>,
    #[serde(rename = "metric_category", skip_serializing_if = "Option::is_none")]
    pub metric_category: Option<crate::models::UsageMetricCategory>,
    /// Contains the custom metric name.
    #[serde(rename = "metric_name", skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

impl UsageTopAvgMetricsHour {
    /// Number of hourly recorded custom metrics for a given organization.
    pub fn new() -> UsageTopAvgMetricsHour {
        UsageTopAvgMetricsHour {
            avg_metric_hour: None,
            max_metric_hour: None,
            metric_category: None,
            metric_name: None,
        }
    }
}



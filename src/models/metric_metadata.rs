/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricMetadata : Object with all metric related metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metric description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of the integration that sent the metric if applicable.
    #[serde(rename = "integration", skip_serializing_if = "Option::is_none")]
    pub integration: Option<String>,
    /// Per unit of the metric such as `second` in `bytes per second`.
    #[serde(rename = "per_unit", skip_serializing_if = "Option::is_none")]
    pub per_unit: Option<String>,
    /// A more human-readable and abbreviated version of the metric name.
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    /// StatsD flush interval of the metric in seconds if applicable.
    #[serde(rename = "statsd_interval", skip_serializing_if = "Option::is_none")]
    pub statsd_interval: Option<i64>,
    /// Metric type such as `gauge` or `rate`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Primary unit of the metric such as `byte` or `operation`.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl MetricMetadata {
    /// Object with all metric related metadata.
    pub fn new() -> MetricMetadata {
        MetricMetadata {
            description: None,
            integration: None,
            per_unit: None,
            short_name: None,
            statsd_interval: None,
            _type: None,
            unit: None,
        }
    }
}



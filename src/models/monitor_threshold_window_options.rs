/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MonitorThresholdWindowOptions : Alerting time window options.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorThresholdWindowOptions {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    #[serde(rename = "recovery_window", skip_serializing_if = "Option::is_none")]
    pub recovery_window: Option<String>,
    /// Describes how long a metric must be anomalous before an alert triggers.
    #[serde(rename = "trigger_window", skip_serializing_if = "Option::is_none")]
    pub trigger_window: Option<String>,
}

impl MonitorThresholdWindowOptions {
    /// Alerting time window options.
    pub fn new() -> MonitorThresholdWindowOptions {
        MonitorThresholdWindowOptions {
            recovery_window: None,
            trigger_window: None,
        }
    }
}



/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// HttpLogItem : Logs that are sent over HTTP.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpLogItem {
    /// The integration name associated with your log: the technology from which the log originated. When it matches an integration name, Datadog automatically installs the corresponding parsers and facets. See [reserved attributes](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes).
    #[serde(rename = "ddsource", skip_serializing_if = "Option::is_none")]
    pub ddsource: Option<String>,
    /// Tags associated with your logs.
    #[serde(rename = "ddtags", skip_serializing_if = "Option::is_none")]
    pub ddtags: Option<String>,
    /// The name of the originating host of the log.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The message [reserved attribute](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes) of your log. By default, Datadog ingests the value of the message attribute as the body of the log entry. That value is then highlighted and displayed in the Logstream, where it is indexed for full text search.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The name of the application or service generating the log events. It is used to switch from Logs to APM, so make sure you define the same value when you use both products. See [reserved attributes](https://docs.datadoghq.com/logs/log_collection/#reserved-attributes).
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl HttpLogItem {
    /// Logs that are sent over HTTP.
    pub fn new() -> HttpLogItem {
        HttpLogItem {
            ddsource: None,
            ddtags: None,
            hostname: None,
            message: None,
            service: None,
        }
    }
}



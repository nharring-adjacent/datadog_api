/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageAttributionValues : Fields in Usage Summary by tag(s).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionValues {
    /// The percentage of synthetic API test usage by tag(s).
    #[serde(rename = "api_percentage", skip_serializing_if = "Option::is_none")]
    pub api_percentage: Option<f64>,
    /// The synthetic API test usage by tag(s).
    #[serde(rename = "api_usage", skip_serializing_if = "Option::is_none")]
    pub api_usage: Option<f64>,
    /// The percentage of APM host usage by tag(s).
    #[serde(rename = "apm_host_percentage", skip_serializing_if = "Option::is_none")]
    pub apm_host_percentage: Option<f64>,
    /// The APM host usage by tag(s).
    #[serde(rename = "apm_host_usage", skip_serializing_if = "Option::is_none")]
    pub apm_host_usage: Option<f64>,
    /// The percentage of synthetic browser test usage by tag(s).
    #[serde(rename = "browser_percentage", skip_serializing_if = "Option::is_none")]
    pub browser_percentage: Option<f64>,
    /// The synthetic browser test usage by tag(s).
    #[serde(rename = "browser_usage", skip_serializing_if = "Option::is_none")]
    pub browser_usage: Option<f64>,
    /// The percentage of container usage by tag(s).
    #[serde(rename = "container_percentage", skip_serializing_if = "Option::is_none")]
    pub container_percentage: Option<f64>,
    /// The container usage by tag(s).
    #[serde(rename = "container_usage", skip_serializing_if = "Option::is_none")]
    pub container_usage: Option<f64>,
    /// The percentage of custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_percentage", skip_serializing_if = "Option::is_none")]
    pub custom_timeseries_percentage: Option<f64>,
    /// The custom metrics usage by tag(s).
    #[serde(rename = "custom_timeseries_usage", skip_serializing_if = "Option::is_none")]
    pub custom_timeseries_usage: Option<f64>,
    /// The percentage of infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_percentage", skip_serializing_if = "Option::is_none")]
    pub infra_host_percentage: Option<f64>,
    /// The infrastructure host usage by tag(s).
    #[serde(rename = "infra_host_usage", skip_serializing_if = "Option::is_none")]
    pub infra_host_usage: Option<f64>,
    /// The percentage of Lambda function usage by tag(s).
    #[serde(rename = "lambda_functions_percentage", skip_serializing_if = "Option::is_none")]
    pub lambda_functions_percentage: Option<f64>,
    /// The Lambda function usage by tag(s).
    #[serde(rename = "lambda_functions_usage", skip_serializing_if = "Option::is_none")]
    pub lambda_functions_usage: Option<f64>,
    /// The percentage of Lambda invocation usage by tag(s).
    #[serde(rename = "lambda_invocations_percentage", skip_serializing_if = "Option::is_none")]
    pub lambda_invocations_percentage: Option<f64>,
    /// The Lambda invocation usage by tag(s).
    #[serde(rename = "lambda_invocations_usage", skip_serializing_if = "Option::is_none")]
    pub lambda_invocations_usage: Option<f64>,
    /// The percentage of Lambda function usage by tag(s).  **Note** this field is deprecated. Use lambda_functions_percentage instead.
    #[serde(rename = "lambda_percentage", skip_serializing_if = "Option::is_none")]
    pub lambda_percentage: Option<f64>,
    /// The Lambda function usage by tag(s).  **Note** this field is deprecated. Use lambda_functions_usage instead.
    #[serde(rename = "lambda_usage", skip_serializing_if = "Option::is_none")]
    pub lambda_usage: Option<f64>,
    /// The percentage of network host usage by tag(s).
    #[serde(rename = "npm_host_percentage", skip_serializing_if = "Option::is_none")]
    pub npm_host_percentage: Option<f64>,
    /// The network host usage by tag(s).
    #[serde(rename = "npm_host_usage", skip_serializing_if = "Option::is_none")]
    pub npm_host_usage: Option<f64>,
    /// The percentage of profiled containers usage by tag(s).
    #[serde(rename = "profiled_containers_percentage", skip_serializing_if = "Option::is_none")]
    pub profiled_containers_percentage: Option<f64>,
    /// The profiled container usage by tag(s).
    #[serde(rename = "profiled_containers_usage", skip_serializing_if = "Option::is_none")]
    pub profiled_containers_usage: Option<f64>,
    /// The percentage of profiled hosts usage by tag(s).
    #[serde(rename = "profiled_hosts_percentage", skip_serializing_if = "Option::is_none")]
    pub profiled_hosts_percentage: Option<f64>,
    /// The profiled host usage by tag(s).
    #[serde(rename = "profiled_hosts_usage", skip_serializing_if = "Option::is_none")]
    pub profiled_hosts_usage: Option<f64>,
    /// The percentage of network device usage by tag(s).
    #[serde(rename = "snmp_percentage", skip_serializing_if = "Option::is_none")]
    pub snmp_percentage: Option<f64>,
    /// The network device usage by tag(s).
    #[serde(rename = "snmp_usage", skip_serializing_if = "Option::is_none")]
    pub snmp_usage: Option<f64>,
}

impl UsageAttributionValues {
    /// Fields in Usage Summary by tag(s).
    pub fn new() -> UsageAttributionValues {
        UsageAttributionValues {
            api_percentage: None,
            api_usage: None,
            apm_host_percentage: None,
            apm_host_usage: None,
            browser_percentage: None,
            browser_usage: None,
            container_percentage: None,
            container_usage: None,
            custom_timeseries_percentage: None,
            custom_timeseries_usage: None,
            infra_host_percentage: None,
            infra_host_usage: None,
            lambda_functions_percentage: None,
            lambda_functions_usage: None,
            lambda_invocations_percentage: None,
            lambda_invocations_usage: None,
            lambda_percentage: None,
            lambda_usage: None,
            npm_host_percentage: None,
            npm_host_usage: None,
            profiled_containers_percentage: None,
            profiled_containers_usage: None,
            profiled_hosts_percentage: None,
            profiled_hosts_usage: None,
            snmp_percentage: None,
            snmp_usage: None,
        }
    }
}



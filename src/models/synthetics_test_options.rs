/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsTestOptions : Object describing the extra options for a Synthetic test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsTestOptions {
    /// For SSL test, whether or not the test should allow self signed certificates.
    #[serde(rename = "accept_self_signed", skip_serializing_if = "Option::is_none")]
    pub accept_self_signed: Option<bool>,
    /// Allows loading insecure content for an HTTP request.
    #[serde(rename = "allow_insecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    /// For browser test, array with the different device IDs used to run the test.
    #[serde(rename = "device_ids", skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<crate::models::SyntheticsDeviceId>>,
    /// Whether or not to disable CORS mechanism. Currently only available for Chrome.
    #[serde(rename = "disableCors", skip_serializing_if = "Option::is_none")]
    pub disable_cors: Option<bool>,
    /// For API HTTP test, whether or not the test should follow redirects.
    #[serde(rename = "follow_redirects", skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    /// Minimum amount of time in failure required to trigger an alert.
    #[serde(rename = "min_failure_duration", skip_serializing_if = "Option::is_none")]
    pub min_failure_duration: Option<i64>,
    /// Minimum number of locations in failure required to trigger an alert.
    #[serde(rename = "min_location_failed", skip_serializing_if = "Option::is_none")]
    pub min_location_failed: Option<i64>,
    #[serde(rename = "monitor_options", skip_serializing_if = "Option::is_none")]
    pub monitor_options: Option<Box<crate::models::SyntheticsTestOptionsMonitorOptions>>,
    /// Prevents saving screenshots of the steps.
    #[serde(rename = "noScreenshot", skip_serializing_if = "Option::is_none")]
    pub no_screenshot: Option<bool>,
    #[serde(rename = "retry", skip_serializing_if = "Option::is_none")]
    pub retry: Option<Box<crate::models::SyntheticsTestOptionsRetry>>,
    #[serde(rename = "tick_every", skip_serializing_if = "Option::is_none")]
    pub tick_every: Option<crate::models::SyntheticsTickInterval>,
}

impl SyntheticsTestOptions {
    /// Object describing the extra options for a Synthetic test.
    pub fn new() -> SyntheticsTestOptions {
        SyntheticsTestOptions {
            accept_self_signed: None,
            allow_insecure: None,
            device_ids: None,
            disable_cors: None,
            follow_redirects: None,
            min_failure_duration: None,
            min_location_failed: None,
            monitor_options: None,
            no_screenshot: None,
            retry: None,
            tick_every: None,
        }
    }
}


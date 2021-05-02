/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsBrowserTestResultShortResult : Object with the result of the last browser test run.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultShortResult {
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::SyntheticsDevice>>,
    /// Length in second of the browser test run.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Amount of errors collected for a single browser test run.
    #[serde(rename = "errorCount", skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    /// Amount of browser test steps completed before failing.
    #[serde(rename = "stepCountCompleted", skip_serializing_if = "Option::is_none")]
    pub step_count_completed: Option<i64>,
    /// Total amount of browser test steps.
    #[serde(rename = "stepCountTotal", skip_serializing_if = "Option::is_none")]
    pub step_count_total: Option<i64>,
}

impl SyntheticsBrowserTestResultShortResult {
    /// Object with the result of the last browser test run.
    pub fn new() -> SyntheticsBrowserTestResultShortResult {
        SyntheticsBrowserTestResultShortResult {
            device: None,
            duration: None,
            error_count: None,
            step_count_completed: None,
            step_count_total: None,
        }
    }
}


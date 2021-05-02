/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsBrowserTestResultData : Object containing results for your Synthetic browser test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBrowserTestResultData {
    /// Type of browser device used for the browser test.
    #[serde(rename = "browserType", skip_serializing_if = "Option::is_none")]
    pub browser_type: Option<String>,
    /// Browser version used for the browser test.
    #[serde(rename = "browserVersion", skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::SyntheticsDevice>>,
    /// Global duration in second of the browser test.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Error returned for the browser test.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Whether or not the browser test was conducted.
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    /// The amount of email received during the browser test.
    #[serde(rename = "receivedEmailCount", skip_serializing_if = "Option::is_none")]
    pub received_email_count: Option<i64>,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl", skip_serializing_if = "Option::is_none")]
    pub start_url: Option<String>,
    /// Array containing the different browser test steps.
    #[serde(rename = "stepDetails", skip_serializing_if = "Option::is_none")]
    pub step_details: Option<Vec<crate::models::SyntheticsStepDetail>>,
    /// Whether or not a thumbnail is associated with the browser test.
    #[serde(rename = "thumbnailsBucketKey", skip_serializing_if = "Option::is_none")]
    pub thumbnails_bucket_key: Option<bool>,
    /// Time in second to wait before the browser test starts after reaching the start URL.
    #[serde(rename = "timeToInteractive", skip_serializing_if = "Option::is_none")]
    pub time_to_interactive: Option<f64>,
}

impl SyntheticsBrowserTestResultData {
    /// Object containing results for your Synthetic browser test.
    pub fn new() -> SyntheticsBrowserTestResultData {
        SyntheticsBrowserTestResultData {
            browser_type: None,
            browser_version: None,
            device: None,
            duration: None,
            error: None,
            passed: None,
            received_email_count: None,
            start_url: None,
            step_details: None,
            thumbnails_bucket_key: None,
            time_to_interactive: None,
        }
    }
}



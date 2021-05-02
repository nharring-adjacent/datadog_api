/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsApiTestResultShortResult : Result of the last API test run.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsApiTestResultShortResult {
    /// Describes if the test run has passed or failed.
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    #[serde(rename = "timings", skip_serializing_if = "Option::is_none")]
    pub timings: Option<Box<crate::models::SyntheticsTiming>>,
}

impl SyntheticsApiTestResultShortResult {
    /// Result of the last API test run.
    pub fn new() -> SyntheticsApiTestResultShortResult {
        SyntheticsApiTestResultShortResult {
            passed: None,
            timings: None,
        }
    }
}



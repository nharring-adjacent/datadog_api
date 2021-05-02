/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AwsLogsAsyncError : Description of errors.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsLogsAsyncError {
    /// Code properties
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Message content.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl AwsLogsAsyncError {
    /// Description of errors.
    pub fn new() -> AwsLogsAsyncError {
        AwsLogsAsyncError {
            code: None,
            message: None,
        }
    }
}


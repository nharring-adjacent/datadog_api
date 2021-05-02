/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsApiErrorResponse : Response returned by the Logs API when errors occur.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsApiErrorResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::LogsApiError>>,
}

impl LogsApiErrorResponse {
    /// Response returned by the Logs API when errors occur.
    pub fn new() -> LogsApiErrorResponse {
        LogsApiErrorResponse {
            error: None,
        }
    }
}


/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogsIndexListResponse : Object with all Index configurations for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogsIndexListResponse {
    /// Array of Log index configurations.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<crate::models::LogsIndex>>,
}

impl LogsIndexListResponse {
    /// Object with all Index configurations for a given organization.
    pub fn new() -> LogsIndexListResponse {
        LogsIndexListResponse {
            indexes: None,
        }
    }
}


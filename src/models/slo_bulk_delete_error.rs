/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloBulkDeleteError : Object describing the error.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloBulkDeleteError {
    /// The ID of the service level objective object associated with this error.
    #[serde(rename = "id")]
    pub id: String,
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "timeframe")]
    pub timeframe: crate::models::SloErrorTimeframe,
}

impl SloBulkDeleteError {
    /// Object describing the error.
    pub fn new(id: String, message: String, timeframe: crate::models::SloErrorTimeframe) -> SloBulkDeleteError {
        SloBulkDeleteError {
            id,
            message,
            timeframe,
        }
    }
}



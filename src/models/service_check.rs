/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceCheck : An object containing service check and status.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCheck {
    /// The check.
    #[serde(rename = "check")]
    pub check: String,
    /// The host name correlated with the check.
    #[serde(rename = "host_name")]
    pub host_name: String,
    /// Message containing check status.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "status")]
    pub status: crate::models::ServiceCheckStatus,
    /// Tags related to a check.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// Time of check.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl ServiceCheck {
    /// An object containing service check and status.
    pub fn new(check: String, host_name: String, status: crate::models::ServiceCheckStatus, tags: Vec<String>) -> ServiceCheck {
        ServiceCheck {
            check,
            host_name,
            message: None,
            status,
            tags,
            timestamp: None,
        }
    }
}



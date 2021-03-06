/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DeletedMonitor : Response from the delete monitor call.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedMonitor {
    /// ID of the deleted monitor.
    #[serde(rename = "deleted_monitor_id", skip_serializing_if = "Option::is_none")]
    pub deleted_monitor_id: Option<i64>,
}

impl DeletedMonitor {
    /// Response from the delete monitor call.
    pub fn new() -> DeletedMonitor {
        DeletedMonitor {
            deleted_monitor_id: None,
        }
    }
}



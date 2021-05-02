/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// CheckCanDeleteMonitorResponseData : Wrapper object with the list of monitor IDs.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckCanDeleteMonitorResponseData {
    /// An array of of Monitor IDs that can be safely deleted.
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<Vec<i64>>,
}

impl CheckCanDeleteMonitorResponseData {
    /// Wrapper object with the list of monitor IDs.
    pub fn new() -> CheckCanDeleteMonitorResponseData {
        CheckCanDeleteMonitorResponseData {
            ok: None,
        }
    }
}



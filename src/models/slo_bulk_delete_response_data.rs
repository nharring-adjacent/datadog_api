/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloBulkDeleteResponseData : An array of service level objective objects.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloBulkDeleteResponseData {
    /// An array of service level objective object IDs that indicates which objects that were completely deleted.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<String>>,
    /// An array of service level objective object IDs that indicates which objects that were modified (objects for which at least one threshold was deleted, but that were not completely deleted).
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<Vec<String>>,
}

impl SloBulkDeleteResponseData {
    /// An array of service level objective objects.
    pub fn new() -> SloBulkDeleteResponseData {
        SloBulkDeleteResponseData {
            deleted: None,
            updated: None,
        }
    }
}



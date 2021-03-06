/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsDeleteTestsResponse : Response object for deleting Synthetic tests.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsDeleteTestsResponse {
    /// Array of objects containing a deleted Synthetic test ID with the associated deletion timestamp.
    #[serde(rename = "deleted_tests", skip_serializing_if = "Option::is_none")]
    pub deleted_tests: Option<Vec<crate::models::SyntheticsDeletedTest>>,
}

impl SyntheticsDeleteTestsResponse {
    /// Response object for deleting Synthetic tests.
    pub fn new() -> SyntheticsDeleteTestsResponse {
        SyntheticsDeleteTestsResponse {
            deleted_tests: None,
        }
    }
}



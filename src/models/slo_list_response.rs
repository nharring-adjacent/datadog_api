/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloListResponse : A response with one or more service level objective.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloListResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::ServiceLevelObjective>>,
    /// An array of error messages. Each endpoint documents how/whether this field is used.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::SloListResponseMetadata>>,
}

impl SloListResponse {
    /// A response with one or more service level objective.
    pub fn new() -> SloListResponse {
        SloListResponse {
            data: None,
            errors: None,
            metadata: None,
        }
    }
}



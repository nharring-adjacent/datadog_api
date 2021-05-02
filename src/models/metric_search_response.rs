/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricSearchResponse : Object containing the list of metrics matching the search query.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricSearchResponse {
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Box<crate::models::MetricSearchResponseResults>>,
}

impl MetricSearchResponse {
    /// Object containing the list of metrics matching the search query.
    pub fn new() -> MetricSearchResponse {
        MetricSearchResponse {
            results: None,
        }
    }
}



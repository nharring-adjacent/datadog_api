/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricsQueryResponse : Response Object that includes your query and the list of metrics retrieved.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryResponse {
    /// Message indicating the errors if status is not `ok`.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Start of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "from_date", skip_serializing_if = "Option::is_none")]
    pub from_date: Option<i64>,
    /// List of tag keys on which to group.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    /// Message indicating `success` if status is `ok`.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Query string
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Type of response.
    #[serde(rename = "res_type", skip_serializing_if = "Option::is_none")]
    pub res_type: Option<String>,
    /// List of timeseries queried.
    #[serde(rename = "series", skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<crate::models::MetricsQueryMetadata>>,
    /// Status of the query.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// End of requested time window, milliseconds since Unix epoch.
    #[serde(rename = "to_date", skip_serializing_if = "Option::is_none")]
    pub to_date: Option<i64>,
}

impl MetricsQueryResponse {
    /// Response Object that includes your query and the list of metrics retrieved.
    pub fn new() -> MetricsQueryResponse {
        MetricsQueryResponse {
            error: None,
            from_date: None,
            group_by: None,
            message: None,
            query: None,
            res_type: None,
            series: None,
            status: None,
            to_date: None,
        }
    }
}



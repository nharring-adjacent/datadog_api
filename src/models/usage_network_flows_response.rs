/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageNetworkFlowsResponse : Response containing the number of netflow events indexed for each hour for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageNetworkFlowsResponse {
    /// Get hourly usage for Network Flows.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<crate::models::UsageNetworkFlowsHour>>,
}

impl UsageNetworkFlowsResponse {
    /// Response containing the number of netflow events indexed for each hour for a given organization.
    pub fn new() -> UsageNetworkFlowsResponse {
        UsageNetworkFlowsResponse {
            usage: None,
        }
    }
}


/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageBillableSummaryResponse : Response with monthly summary of data billed by Datadog.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryResponse {
    /// An array of objects regarding usage of billable summary.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<crate::models::UsageBillableSummaryHour>>,
}

impl UsageBillableSummaryResponse {
    /// Response with monthly summary of data billed by Datadog.
    pub fn new() -> UsageBillableSummaryResponse {
        UsageBillableSummaryResponse {
            usage: None,
        }
    }
}


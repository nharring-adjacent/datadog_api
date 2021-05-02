/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageLambdaResponse : Response containing the number of lambda functions and sum of the invocations of all lambda functions for each hour for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageLambdaResponse {
    /// Get hourly usage for Lambda.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<crate::models::UsageLambdaHour>>,
}

impl UsageLambdaResponse {
    /// Response containing the number of lambda functions and sum of the invocations of all lambda functions for each hour for a given organization.
    pub fn new() -> UsageLambdaResponse {
        UsageLambdaResponse {
            usage: None,
        }
    }
}



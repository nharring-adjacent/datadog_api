/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageCustomReportsResponse : Response containing available custom reports.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCustomReportsResponse {
    /// An array of available custom reports.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::UsageCustomReportsData>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::UsageCustomReportsMeta>>,
}

impl UsageCustomReportsResponse {
    /// Response containing available custom reports.
    pub fn new() -> UsageCustomReportsResponse {
        UsageCustomReportsResponse {
            data: None,
            meta: None,
        }
    }
}



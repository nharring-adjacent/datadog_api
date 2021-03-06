/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageCustomReportsMeta : The object containing document metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCustomReportsMeta {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<Box<crate::models::UsageCustomReportsPage>>,
}

impl UsageCustomReportsMeta {
    /// The object containing document metadata.
    pub fn new() -> UsageCustomReportsMeta {
        UsageCustomReportsMeta {
            page: None,
        }
    }
}



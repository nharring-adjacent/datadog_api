/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageSpecifiedCustomReportsAttributes : The response containing attributes for specified custom reports.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsAttributes {
    /// The date the specified custom report was computed.
    #[serde(rename = "computed_on", skip_serializing_if = "Option::is_none")]
    pub computed_on: Option<String>,
    /// The ending date of specified custom report.
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// A downloadable file for the specified custom reporting file.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The starting date of specified custom report.
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// A list of tags to apply to specified custom reports.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl UsageSpecifiedCustomReportsAttributes {
    /// The response containing attributes for specified custom reports.
    pub fn new() -> UsageSpecifiedCustomReportsAttributes {
        UsageSpecifiedCustomReportsAttributes {
            computed_on: None,
            end_date: None,
            location: None,
            size: None,
            start_date: None,
            tags: None,
        }
    }
}



/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageAttributionPagination : The page count for the current pagination.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAttributionPagination {
    /// Maximum amount of records to be returned.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Records to be skipped before beginning to return.
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Direction to sort by.
    #[serde(rename = "sort_direction", skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// Field to sort by.
    #[serde(rename = "sort_name", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// Total number of records.
    #[serde(rename = "total_number_of_records", skip_serializing_if = "Option::is_none")]
    pub total_number_of_records: Option<i64>,
}

impl UsageAttributionPagination {
    /// The page count for the current pagination.
    pub fn new() -> UsageAttributionPagination {
        UsageAttributionPagination {
            limit: None,
            offset: None,
            sort_direction: None,
            sort_name: None,
            total_number_of_records: None,
        }
    }
}



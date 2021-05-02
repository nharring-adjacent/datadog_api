/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloCorrectionResponseAttributes : The attribute object associated with the SLO correction



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloCorrectionResponseAttributes {
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<crate::models::SloCorrectionCategory>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<Box<crate::models::Creator>>,
    /// Description of the correction being made.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Ending time of the correction in epoch seconds
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    /// ID of the SLO that this correction will be applied to
    #[serde(rename = "slo_id", skip_serializing_if = "Option::is_none")]
    pub slo_id: Option<String>,
    /// Starting time of the correction in epoch seconds
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    /// The timezone to display in the UI for the correction times (defaults to \"UTC\")
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl SloCorrectionResponseAttributes {
    /// The attribute object associated with the SLO correction
    pub fn new() -> SloCorrectionResponseAttributes {
        SloCorrectionResponseAttributes {
            category: None,
            creator: None,
            description: None,
            end: None,
            slo_id: None,
            start: None,
            timezone: None,
        }
    }
}



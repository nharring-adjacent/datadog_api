/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloCorrectionCreateRequestAttributes : The attribute object associated with the SLO correction to be created



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloCorrectionCreateRequestAttributes {
    #[serde(rename = "category")]
    pub category: crate::models::SloCorrectionCategory,
    /// Description of the correction being made.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Ending time of the correction in epoch seconds
    #[serde(rename = "end")]
    pub end: i64,
    /// ID of the SLO that this correction will be applied to
    #[serde(rename = "slo_id")]
    pub slo_id: String,
    /// Starting time of the correction in epoch seconds
    #[serde(rename = "start")]
    pub start: i64,
    /// The timezone to display in the UI for the correction times (defaults to \"UTC\")
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl SloCorrectionCreateRequestAttributes {
    /// The attribute object associated with the SLO correction to be created
    pub fn new(category: crate::models::SloCorrectionCategory, end: i64, slo_id: String, start: i64) -> SloCorrectionCreateRequestAttributes {
        SloCorrectionCreateRequestAttributes {
            category,
            description: None,
            end,
            slo_id,
            start,
            timezone: None,
        }
    }
}



/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloThreshold : SLO thresholds (target and optionally warning) for a single time window.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloThreshold {
    /// The target value for the service level indicator within the corresponding timeframe.
    #[serde(rename = "target")]
    pub target: f64,
    /// A string representation of the target that indicates its precision. It uses trailing zeros to show significant decimal places (e.g. `98.00`).  Always included in service level objective responses. Ignored in create/update requests.
    #[serde(rename = "target_display", skip_serializing_if = "Option::is_none")]
    pub target_display: Option<String>,
    #[serde(rename = "timeframe")]
    pub timeframe: crate::models::SloTimeframe,
    /// The warning value for the service level objective.
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<f64>,
    /// A string representation of the warning target (see the description of the `target_display` field for details).  Included in service level objective responses if a warning target exists. Ignored in create/update requests.
    #[serde(rename = "warning_display", skip_serializing_if = "Option::is_none")]
    pub warning_display: Option<String>,
}

impl SloThreshold {
    /// SLO thresholds (target and optionally warning) for a single time window.
    pub fn new(target: f64, timeframe: crate::models::SloTimeframe) -> SloThreshold {
        SloThreshold {
            target,
            target_display: None,
            timeframe,
            warning: None,
            warning_display: None,
        }
    }
}


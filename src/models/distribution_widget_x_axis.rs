/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DistributionWidgetXAxis : X Axis controls for the distribution widget.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionWidgetXAxis {
    /// True includes zero.
    #[serde(rename = "include_zero", skip_serializing_if = "Option::is_none")]
    pub include_zero: Option<bool>,
    /// Specifies maximum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    /// Specifies minimum value to show on the x-axis. It takes a number, percentile (p90 === 90th percentile), or auto for default behavior.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    /// Specifies the scale type. Possible values are `linear`.
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
}

impl DistributionWidgetXAxis {
    /// X Axis controls for the distribution widget.
    pub fn new() -> DistributionWidgetXAxis {
        DistributionWidgetXAxis {
            include_zero: None,
            max: None,
            min: None,
            scale: None,
        }
    }
}


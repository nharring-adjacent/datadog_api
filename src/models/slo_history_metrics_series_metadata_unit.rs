/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloHistoryMetricsSeriesMetadataUnit : An Object of metric units.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloHistoryMetricsSeriesMetadataUnit {
    /// The family of metric unit, for example `bytes` is the family for `kibibyte`, `byte`, and `bit` units.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// The ID of the metric unit.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The unit of the metric, for instance `byte`.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plural Unit of metric, for instance `bytes`.
    #[serde(rename = "plural", skip_serializing_if = "Option::is_none")]
    pub plural: Option<String>,
    /// The scale factor of metric unit, for instance `1.0`.
    #[serde(rename = "scale_factor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<f64>,
    /// A shorter and abbreviated version of the metric unit, for instance `B`.
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

impl SloHistoryMetricsSeriesMetadataUnit {
    /// An Object of metric units.
    pub fn new() -> SloHistoryMetricsSeriesMetadataUnit {
        SloHistoryMetricsSeriesMetadataUnit {
            family: None,
            id: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
        }
    }
}



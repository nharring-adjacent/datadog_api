/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// MetricsQueryUnit : Object containing the metric unit family, scale factor, name, and short name.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsQueryUnit {
    /// Unit family, allows for conversion between units of the same family, for scaling.
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Unit name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Plural form of the unit name.
    #[serde(rename = "plural", skip_serializing_if = "Option::is_none")]
    pub plural: Option<String>,
    /// Factor for scaling between units of the same family.
    #[serde(rename = "scale_factor", skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<f64>,
    /// Abbreviation of the unit.
    #[serde(rename = "short_name", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

impl MetricsQueryUnit {
    /// Object containing the metric unit family, scale factor, name, and short name.
    pub fn new() -> MetricsQueryUnit {
        MetricsQueryUnit {
            family: None,
            name: None,
            plural: None,
            scale_factor: None,
            short_name: None,
        }
    }
}



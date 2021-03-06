/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TimeseriesWidgetLegendLayout : Layout of the legend.

/// Layout of the legend.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeseriesWidgetLegendLayout {
    #[serde(rename = "auto")]
    AUTO,
    #[serde(rename = "horizontal")]
    HORIZONTAL,
    #[serde(rename = "vertical")]
    VERTICAL,

}

impl ToString for TimeseriesWidgetLegendLayout {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::HORIZONTAL => String::from("horizontal"),
            Self::VERTICAL => String::from("vertical"),
        }
    }
}





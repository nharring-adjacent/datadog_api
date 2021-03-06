/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetMonitorSummaryDisplayFormat : What to display on the widget.

/// What to display on the widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetMonitorSummaryDisplayFormat {
    #[serde(rename = "counts")]
    COUNTS,
    #[serde(rename = "countsAndList")]
    COUNTS_AND_LIST,
    #[serde(rename = "list")]
    LIST,

}

impl ToString for WidgetMonitorSummaryDisplayFormat {
    fn to_string(&self) -> String {
        match self {
            Self::COUNTS => String::from("counts"),
            Self::COUNTS_AND_LIST => String::from("countsAndList"),
            Self::LIST => String::from("list"),
        }
    }
}





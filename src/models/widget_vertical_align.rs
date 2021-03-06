/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetVerticalAlign : Vertical alignment.

/// Vertical alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetVerticalAlign {
    #[serde(rename = "center")]
    CENTER,
    #[serde(rename = "top")]
    TOP,
    #[serde(rename = "bottom")]
    BOTTOM,

}

impl ToString for WidgetVerticalAlign {
    fn to_string(&self) -> String {
        match self {
            Self::CENTER => String::from("center"),
            Self::TOP => String::from("top"),
            Self::BOTTOM => String::from("bottom"),
        }
    }
}





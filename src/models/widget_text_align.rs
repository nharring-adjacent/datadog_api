/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetTextAlign : How to align the text on the widget.

/// How to align the text on the widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetTextAlign {
    #[serde(rename = "center")]
    CENTER,
    #[serde(rename = "left")]
    LEFT,
    #[serde(rename = "right")]
    RIGHT,

}

impl ToString for WidgetTextAlign {
    fn to_string(&self) -> String {
        match self {
            Self::CENTER => String::from("center"),
            Self::LEFT => String::from("left"),
            Self::RIGHT => String::from("right"),
        }
    }
}




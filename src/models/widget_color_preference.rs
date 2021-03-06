/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetColorPreference : Which color to use on the widget.

/// Which color to use on the widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WidgetColorPreference {
    #[serde(rename = "background")]
    BACKGROUND,
    #[serde(rename = "text")]
    TEXT,

}

impl ToString for WidgetColorPreference {
    fn to_string(&self) -> String {
        match self {
            Self::BACKGROUND => String::from("background"),
            Self::TEXT => String::from("text"),
        }
    }
}





/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetRequestStyle : Define request widget style.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetRequestStyle {
    #[serde(rename = "line_type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<crate::models::WidgetLineType>,
    #[serde(rename = "line_width", skip_serializing_if = "Option::is_none")]
    pub line_width: Option<crate::models::WidgetLineWidth>,
    /// Color palette to apply to the widget.
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,
}

impl WidgetRequestStyle {
    /// Define request widget style.
    pub fn new() -> WidgetRequestStyle {
        WidgetRequestStyle {
            line_type: None,
            line_width: None,
            palette: None,
        }
    }
}


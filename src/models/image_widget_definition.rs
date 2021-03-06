/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ImageWidgetDefinition : The image widget allows you to embed an image on your dashboard. An image can be a PNG, JPG, or animated GIF. Only available on FREE layout dashboards.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageWidgetDefinition {
    /// Whether to display a background or not.
    #[serde(rename = "has_background", skip_serializing_if = "Option::is_none")]
    pub has_background: Option<bool>,
    /// Whether to display a border or not.
    #[serde(rename = "has_border", skip_serializing_if = "Option::is_none")]
    pub has_border: Option<bool>,
    #[serde(rename = "horizontal_align", skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<crate::models::WidgetHorizontalAlign>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<crate::models::WidgetMargin>,
    #[serde(rename = "sizing", skip_serializing_if = "Option::is_none")]
    pub sizing: Option<crate::models::WidgetImageSizing>,
    #[serde(rename = "type")]
    pub _type: crate::models::ImageWidgetDefinitionType,
    /// URL of the image.
    #[serde(rename = "url")]
    pub url: String,
    /// URL of the image in dark mode.
    #[serde(rename = "url_dark_theme", skip_serializing_if = "Option::is_none")]
    pub url_dark_theme: Option<String>,
    #[serde(rename = "vertical_align", skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<crate::models::WidgetVerticalAlign>,
}

impl ImageWidgetDefinition {
    /// The image widget allows you to embed an image on your dashboard. An image can be a PNG, JPG, or animated GIF. Only available on FREE layout dashboards.
    pub fn new(_type: crate::models::ImageWidgetDefinitionType, url: String) -> ImageWidgetDefinition {
        ImageWidgetDefinition {
            has_background: None,
            has_border: None,
            horizontal_align: None,
            margin: None,
            sizing: None,
            _type,
            url,
            url_dark_theme: None,
            vertical_align: None,
        }
    }
}



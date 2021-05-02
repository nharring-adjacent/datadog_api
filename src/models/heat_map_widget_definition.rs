/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// HeatMapWidgetDefinition : The heat map visualization shows metrics aggregated across many tags, such as hosts. The more hosts that have a particular value, the darker that square is.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatMapWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Option<Vec<crate::models::WidgetCustomLink>>,
    /// List of widget events.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::WidgetEvent>>,
    /// Available legend sizes for a widget. Should be one of \"0\", \"2\", \"4\", \"8\", \"16\", or \"auto\".
    #[serde(rename = "legend_size", skip_serializing_if = "Option::is_none")]
    pub legend_size: Option<String>,
    /// List of widget types.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::models::HeatMapWidgetRequest>,
    /// Whether or not to display the legend on this widget.
    #[serde(rename = "show_legend", skip_serializing_if = "Option::is_none")]
    pub show_legend: Option<bool>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Box<crate::models::WidgetTime>>,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: Option<crate::models::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::HeatMapWidgetDefinitionType,
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: Option<Box<crate::models::WidgetAxis>>,
}

impl HeatMapWidgetDefinition {
    /// The heat map visualization shows metrics aggregated across many tags, such as hosts. The more hosts that have a particular value, the darker that square is.
    pub fn new(requests: Vec<crate::models::HeatMapWidgetRequest>, _type: crate::models::HeatMapWidgetDefinitionType) -> HeatMapWidgetDefinition {
        HeatMapWidgetDefinition {
            custom_links: None,
            events: None,
            legend_size: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            _type,
            yaxis: None,
        }
    }
}


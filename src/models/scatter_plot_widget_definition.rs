/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ScatterPlotWidgetDefinition : The scatter plot visualization allows you to graph a chosen scope over two different metrics with their respective aggregation.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScatterPlotWidgetDefinition {
    /// List of groups used for colors.
    #[serde(rename = "color_by_groups", skip_serializing_if = "Option::is_none")]
    pub color_by_groups: Option<Vec<String>>,
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Option<Vec<crate::models::WidgetCustomLink>>,
    #[serde(rename = "requests")]
    pub requests: Box<crate::models::ScatterPlotWidgetDefinitionRequests>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<Box<crate::models::WidgetTime>>,
    /// Title of your widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: Option<crate::models::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::ScatterPlotWidgetDefinitionType,
    #[serde(rename = "xaxis", skip_serializing_if = "Option::is_none")]
    pub xaxis: Option<Box<crate::models::WidgetAxis>>,
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: Option<Box<crate::models::WidgetAxis>>,
}

impl ScatterPlotWidgetDefinition {
    /// The scatter plot visualization allows you to graph a chosen scope over two different metrics with their respective aggregation.
    pub fn new(requests: crate::models::ScatterPlotWidgetDefinitionRequests, _type: crate::models::ScatterPlotWidgetDefinitionType) -> ScatterPlotWidgetDefinition {
        ScatterPlotWidgetDefinition {
            color_by_groups: None,
            custom_links: None,
            requests: Box::new(requests),
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            _type,
            xaxis: None,
            yaxis: None,
        }
    }
}



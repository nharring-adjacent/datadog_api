/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SloWidgetDefinition : Use the SLO and uptime widget to track your SLOs (Service Level Objectives) and uptime on screenboards and timeboards.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SloWidgetDefinition {
    /// Defined global time target.
    #[serde(rename = "global_time_target", skip_serializing_if = "Option::is_none")]
    pub global_time_target: Option<String>,
    /// Defined error budget.
    #[serde(rename = "show_error_budget", skip_serializing_if = "Option::is_none")]
    pub show_error_budget: Option<bool>,
    /// ID of the SLO displayed.
    #[serde(rename = "slo_id", skip_serializing_if = "Option::is_none")]
    pub slo_id: Option<String>,
    /// Times being monitored.
    #[serde(rename = "time_windows", skip_serializing_if = "Option::is_none")]
    pub time_windows: Option<Vec<crate::models::WidgetTimeWindows>>,
    /// Title of the widget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_align", skip_serializing_if = "Option::is_none")]
    pub title_align: Option<crate::models::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size", skip_serializing_if = "Option::is_none")]
    pub title_size: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::models::SloWidgetDefinitionType,
    #[serde(rename = "view_mode", skip_serializing_if = "Option::is_none")]
    pub view_mode: Option<crate::models::WidgetViewMode>,
    /// Type of view displayed by the widget.
    #[serde(rename = "view_type")]
    pub view_type: String,
}

impl SloWidgetDefinition {
    /// Use the SLO and uptime widget to track your SLOs (Service Level Objectives) and uptime on screenboards and timeboards.
    pub fn new(_type: crate::models::SloWidgetDefinitionType, view_type: String) -> SloWidgetDefinition {
        SloWidgetDefinition {
            global_time_target: None,
            show_error_budget: None,
            slo_id: None,
            time_windows: None,
            title: None,
            title_align: None,
            title_size: None,
            _type,
            view_mode: None,
            view_type,
        }
    }
}



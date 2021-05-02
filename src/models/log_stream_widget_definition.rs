/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// LogStreamWidgetDefinition : The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogStreamWidgetDefinition {
    /// Which columns to display on the widget.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// An array of index names to query in the stream. Use [] to query all indexes at once.
    #[serde(rename = "indexes", skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<String>>,
    /// ID of the log set to use.
    #[serde(rename = "logset", skip_serializing_if = "Option::is_none")]
    pub logset: Option<String>,
    #[serde(rename = "message_display", skip_serializing_if = "Option::is_none")]
    pub message_display: Option<crate::models::WidgetMessageDisplay>,
    /// Query to filter the log stream with.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Whether to show the date column or not
    #[serde(rename = "show_date_column", skip_serializing_if = "Option::is_none")]
    pub show_date_column: Option<bool>,
    /// Whether to show the message column or not
    #[serde(rename = "show_message_column", skip_serializing_if = "Option::is_none")]
    pub show_message_column: Option<bool>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Box<crate::models::WidgetFieldSort>>,
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
    pub _type: crate::models::LogStreamWidgetDefinitionType,
}

impl LogStreamWidgetDefinition {
    /// The Log Stream displays a log flow matching the defined query. Only available on FREE layout dashboards.
    pub fn new(_type: crate::models::LogStreamWidgetDefinitionType) -> LogStreamWidgetDefinition {
        LogStreamWidgetDefinition {
            columns: None,
            indexes: None,
            logset: None,
            message_display: None,
            query: None,
            show_date_column: None,
            show_message_column: None,
            sort: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            _type,
        }
    }
}


/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// WidgetDefinition : [Definition of the widget](https://docs.datadoghq.com/dashboards/widgets/).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WidgetDefinition {
    /// ID of the alert to use in the widget.
    #[serde(rename = "alert_id")]
    pub alert_id: String,
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
    pub _type: crate::models::ToplistWidgetDefinitionType,
    #[serde(rename = "viz_type")]
    pub viz_type: crate::models::WidgetVizType,
    /// Number of decimals to show. If not defined, the widget uses the raw value.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(rename = "text_align", skip_serializing_if = "Option::is_none")]
    pub text_align: Option<crate::models::WidgetTextAlign>,
    /// Unit to display with the value.
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// List of custom links.
    #[serde(rename = "custom_links", skip_serializing_if = "Option::is_none")]
    pub custom_links: Option<Vec<crate::models::WidgetCustomLink>>,
    /// List of top list widget requests.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::models::ToplistWidgetRequest>,
    /// Name of the check to use in the widget.
    #[serde(rename = "check")]
    pub check: String,
    /// List of tag prefixes to group by.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
    /// List of tag prefixes to group by in the case of a cluster check.
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    #[serde(rename = "grouping")]
    pub grouping: crate::models::WidgetGrouping,
    /// List of tags used to filter the groups reporting a cluster check.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Available legend sizes for a widget. Should be one of \"0\", \"2\", \"4\", \"8\", \"16\", or \"auto\".
    #[serde(rename = "legend_size", skip_serializing_if = "Option::is_none")]
    pub legend_size: Option<String>,
    /// List of markers.
    #[serde(rename = "markers", skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<crate::models::WidgetMarker>>,
    /// (screenboard only) Show the legend for this widget.
    #[serde(rename = "show_legend", skip_serializing_if = "Option::is_none")]
    pub show_legend: Option<bool>,
    #[serde(rename = "xaxis", skip_serializing_if = "Option::is_none")]
    pub xaxis: Option<Box<crate::models::WidgetAxis>>,
    #[serde(rename = "yaxis", skip_serializing_if = "Option::is_none")]
    pub yaxis: Option<Box<crate::models::WidgetAxis>>,
    #[serde(rename = "event_size", skip_serializing_if = "Option::is_none")]
    pub event_size: Option<crate::models::WidgetEventSize>,
    /// Query to filter the monitors with.
    #[serde(rename = "query")]
    pub query: String,
    /// The execution method for multi-value filters. Can be either and or or.
    #[serde(rename = "tags_execution", skip_serializing_if = "Option::is_none")]
    pub tags_execution: Option<String>,
    /// Color of the text.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Size of the text.
    #[serde(rename = "font_size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    /// Text to display.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "style")]
    pub style: Box<crate::models::HostMapWidgetDefinitionStyle>,
    #[serde(rename = "view")]
    pub view: Box<crate::models::GeomapWidgetDefinitionView>,
    /// Background color of the note.
    #[serde(rename = "background_color", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// URL of image to display as a banner for the group.
    #[serde(rename = "banner_img", skip_serializing_if = "Option::is_none")]
    pub banner_img: Option<String>,
    #[serde(rename = "layout_type")]
    pub layout_type: crate::models::WidgetLayoutType,
    /// Whether to show the title or not.
    #[serde(rename = "show_title", skip_serializing_if = "Option::is_none")]
    pub show_title: Option<bool>,
    /// List of widget groups.
    #[serde(rename = "widgets")]
    pub widgets: Vec<crate::models::Widget>,
    /// List of widget events.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::WidgetEvent>>,
    /// Whether to show the hosts that don’t fit in a group.
    #[serde(rename = "no_group_hosts", skip_serializing_if = "Option::is_none")]
    pub no_group_hosts: Option<bool>,
    /// Whether to show the hosts with no metrics.
    #[serde(rename = "no_metric_hosts", skip_serializing_if = "Option::is_none")]
    pub no_metric_hosts: Option<bool>,
    #[serde(rename = "node_type", skip_serializing_if = "Option::is_none")]
    pub node_type: Option<crate::models::WidgetNodeType>,
    /// Notes on the title.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// List of tags used to filter the map.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    /// URL of the image.
    #[serde(rename = "url")]
    pub url: String,
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
    /// URL of the image in dark mode.
    #[serde(rename = "url_dark_theme", skip_serializing_if = "Option::is_none")]
    pub url_dark_theme: Option<String>,
    #[serde(rename = "vertical_align", skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<crate::models::WidgetVerticalAlign>,
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
    /// Whether to show the date column or not
    #[serde(rename = "show_date_column", skip_serializing_if = "Option::is_none")]
    pub show_date_column: Option<bool>,
    /// Whether to show the message column or not
    #[serde(rename = "show_message_column", skip_serializing_if = "Option::is_none")]
    pub show_message_column: Option<bool>,
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::models::WidgetMonitorSummarySort>,
    #[serde(rename = "color_preference", skip_serializing_if = "Option::is_none")]
    pub color_preference: Option<crate::models::WidgetColorPreference>,
    /// The number of monitors to display.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "display_format", skip_serializing_if = "Option::is_none")]
    pub display_format: Option<crate::models::WidgetServiceSummaryDisplayFormat>,
    /// Whether to show counts of 0 or not.
    #[serde(rename = "hide_zero_counts", skip_serializing_if = "Option::is_none")]
    pub hide_zero_counts: Option<bool>,
    /// Whether to show the time that has elapsed since the monitor/group triggered.
    #[serde(rename = "show_last_triggered", skip_serializing_if = "Option::is_none")]
    pub show_last_triggered: Option<bool>,
    /// The start of the list. Typically 0.
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "summary_type", skip_serializing_if = "Option::is_none")]
    pub summary_type: Option<crate::models::WidgetSummaryType>,
    /// Content of the note.
    #[serde(rename = "content")]
    pub content: String,
    /// Whether to add padding or not.
    #[serde(rename = "has_padding", skip_serializing_if = "Option::is_none")]
    pub has_padding: Option<bool>,
    /// Whether to show a tick or not.
    #[serde(rename = "show_tick", skip_serializing_if = "Option::is_none")]
    pub show_tick: Option<bool>,
    #[serde(rename = "tick_edge", skip_serializing_if = "Option::is_none")]
    pub tick_edge: Option<crate::models::WidgetTickEdge>,
    /// Where to position the tick on an edge.
    #[serde(rename = "tick_pos", skip_serializing_if = "Option::is_none")]
    pub tick_pos: Option<String>,
    /// Whether to use auto-scaling or not.
    #[serde(rename = "autoscale", skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<bool>,
    /// Display a unit of your choice on the widget.
    #[serde(rename = "custom_unit", skip_serializing_if = "Option::is_none")]
    pub custom_unit: Option<String>,
    /// List of groups used for colors.
    #[serde(rename = "color_by_groups", skip_serializing_if = "Option::is_none")]
    pub color_by_groups: Option<Vec<String>>,
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
    #[serde(rename = "view_mode", skip_serializing_if = "Option::is_none")]
    pub view_mode: Option<crate::models::WidgetViewMode>,
    /// Type of view displayed by the widget.
    #[serde(rename = "view_type")]
    pub view_type: String,
    /// Your environment and primary tag (or * if enabled for your account).
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    /// APM service.
    #[serde(rename = "service")]
    pub service: String,
    /// APM environment.
    #[serde(rename = "env")]
    pub env: String,
    /// Whether to show the latency breakdown or not.
    #[serde(rename = "show_breakdown", skip_serializing_if = "Option::is_none")]
    pub show_breakdown: Option<bool>,
    /// Whether to show the latency distribution or not.
    #[serde(rename = "show_distribution", skip_serializing_if = "Option::is_none")]
    pub show_distribution: Option<bool>,
    /// Whether to show the error metrics or not.
    #[serde(rename = "show_errors", skip_serializing_if = "Option::is_none")]
    pub show_errors: Option<bool>,
    /// Whether to show the hits metrics or not.
    #[serde(rename = "show_hits", skip_serializing_if = "Option::is_none")]
    pub show_hits: Option<bool>,
    /// Whether to show the latency metrics or not.
    #[serde(rename = "show_latency", skip_serializing_if = "Option::is_none")]
    pub show_latency: Option<bool>,
    /// Whether to show the resource list or not.
    #[serde(rename = "show_resource_list", skip_serializing_if = "Option::is_none")]
    pub show_resource_list: Option<bool>,
    #[serde(rename = "size_format", skip_serializing_if = "Option::is_none")]
    pub size_format: Option<crate::models::WidgetSizeFormat>,
    /// APM span name.
    #[serde(rename = "span_name")]
    pub span_name: String,
    #[serde(rename = "has_search_bar", skip_serializing_if = "Option::is_none")]
    pub has_search_bar: Option<crate::models::TableWidgetHasSearchBar>,
    /// Columns displayed in the legend.
    #[serde(rename = "legend_columns", skip_serializing_if = "Option::is_none")]
    pub legend_columns: Option<Vec<crate::models::TimeseriesWidgetLegendColumn>>,
    #[serde(rename = "legend_layout", skip_serializing_if = "Option::is_none")]
    pub legend_layout: Option<crate::models::TimeseriesWidgetLegendLayout>,
    #[serde(rename = "right_yaxis", skip_serializing_if = "Option::is_none")]
    pub right_yaxis: Option<Box<crate::models::WidgetAxis>>,
}

impl WidgetDefinition {
    /// [Definition of the widget](https://docs.datadoghq.com/dashboards/widgets/).
    pub fn new(alert_id: String, _type: crate::models::ToplistWidgetDefinitionType, viz_type: crate::models::WidgetVizType, requests: Vec<crate::models::ToplistWidgetRequest>, check: String, grouping: crate::models::WidgetGrouping, query: String, text: String, style: crate::models::HostMapWidgetDefinitionStyle, view: crate::models::GeomapWidgetDefinitionView, layout_type: crate::models::WidgetLayoutType, widgets: Vec<crate::models::Widget>, url: String, content: String, view_type: String, filters: Vec<String>, service: String, env: String, span_name: String) -> WidgetDefinition {
        WidgetDefinition {
            alert_id,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            _type,
            viz_type,
            precision: None,
            text_align: None,
            unit: None,
            custom_links: None,
            requests,
            check,
            group: None,
            group_by: None,
            grouping,
            tags: None,
            legend_size: None,
            markers: None,
            show_legend: None,
            xaxis: None,
            yaxis: None,
            event_size: None,
            query,
            tags_execution: None,
            color: None,
            font_size: None,
            text,
            style: Box::new(style),
            view: Box::new(view),
            background_color: None,
            banner_img: None,
            layout_type,
            show_title: None,
            widgets,
            events: None,
            no_group_hosts: None,
            no_metric_hosts: None,
            node_type: None,
            notes: None,
            scope: None,
            url,
            has_background: None,
            has_border: None,
            horizontal_align: None,
            margin: None,
            sizing: None,
            url_dark_theme: None,
            vertical_align: None,
            columns: None,
            indexes: None,
            logset: None,
            message_display: None,
            show_date_column: None,
            show_message_column: None,
            sort: None,
            color_preference: None,
            count: None,
            display_format: None,
            hide_zero_counts: None,
            show_last_triggered: None,
            start: None,
            summary_type: None,
            content,
            has_padding: None,
            show_tick: None,
            tick_edge: None,
            tick_pos: None,
            autoscale: None,
            custom_unit: None,
            color_by_groups: None,
            global_time_target: None,
            show_error_budget: None,
            slo_id: None,
            time_windows: None,
            view_mode: None,
            view_type,
            filters,
            service,
            env,
            show_breakdown: None,
            show_distribution: None,
            show_errors: None,
            show_hits: None,
            show_latency: None,
            show_resource_list: None,
            size_format: None,
            span_name,
            has_search_bar: None,
            legend_columns: None,
            legend_layout: None,
            right_yaxis: None,
        }
    }
}


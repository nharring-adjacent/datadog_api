# WidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_id** | **String** | ID of the alert to use in the widget. | 
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | Title of your widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::ToplistWidgetDefinitionType**](ToplistWidgetDefinitionType.md) |  | 
**viz_type** | [**crate::models::WidgetVizType**](WidgetVizType.md) |  | 
**precision** | Option<**i64**> | Number of decimals to show. If not defined, the widget uses the raw value. | [optional]
**text_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**unit** | Option<**String**> | Unit to display with the value. | [optional]
**custom_links** | Option<[**Vec<crate::models::WidgetCustomLink>**](WidgetCustomLink.md)> | List of custom links. | [optional]
**requests** | [**Vec<crate::models::ToplistWidgetRequest>**](ToplistWidgetRequest.md) | List of top list widget requests. | 
**check** | **String** | Name of the check to use in the widget. | 
**group** | Option<**Vec<String>**> | List of tag prefixes to group by. | [optional]
**group_by** | Option<**Vec<String>**> | List of tag prefixes to group by in the case of a cluster check. | [optional]
**grouping** | [**crate::models::WidgetGrouping**](WidgetGrouping.md) |  | 
**tags** | Option<**Vec<String>**> | List of tags used to filter the groups reporting a cluster check. | [optional]
**legend_size** | Option<**String**> | Available legend sizes for a widget. Should be one of \"0\", \"2\", \"4\", \"8\", \"16\", or \"auto\". | [optional]
**markers** | Option<[**Vec<crate::models::WidgetMarker>**](WidgetMarker.md)> | List of markers. | [optional]
**show_legend** | Option<**bool**> | (screenboard only) Show the legend for this widget. | [optional]
**xaxis** | Option<[**crate::models::WidgetAxis**](WidgetAxis.md)> |  | [optional]
**yaxis** | Option<[**crate::models::WidgetAxis**](WidgetAxis.md)> |  | [optional]
**event_size** | Option<[**crate::models::WidgetEventSize**](WidgetEventSize.md)> |  | [optional]
**query** | **String** | Query to filter the monitors with. | 
**tags_execution** | Option<**String**> | The execution method for multi-value filters. Can be either and or or. | [optional]
**color** | Option<**String**> | Color of the text. | [optional]
**font_size** | Option<**String**> | Size of the text. | [optional]
**text** | **String** | Text to display. | 
**style** | [**crate::models::HostMapWidgetDefinitionStyle**](HostMapWidgetDefinition_style.md) |  | 
**view** | [**crate::models::GeomapWidgetDefinitionView**](GeomapWidgetDefinition_view.md) |  | 
**background_color** | Option<**String**> | Background color of the note. | [optional]
**banner_img** | Option<**String**> | URL of image to display as a banner for the group. | [optional]
**layout_type** | [**crate::models::WidgetLayoutType**](WidgetLayoutType.md) |  | 
**show_title** | Option<**bool**> | Whether to show the title or not. | [optional][default to true]
**widgets** | [**Vec<crate::models::Widget>**](Widget.md) | List of widget groups. | 
**events** | Option<[**Vec<crate::models::WidgetEvent>**](WidgetEvent.md)> | List of widget events. | [optional]
**no_group_hosts** | Option<**bool**> | Whether to show the hosts that don’t fit in a group. | [optional]
**no_metric_hosts** | Option<**bool**> | Whether to show the hosts with no metrics. | [optional]
**node_type** | Option<[**crate::models::WidgetNodeType**](WidgetNodeType.md)> |  | [optional]
**notes** | Option<**String**> | Notes on the title. | [optional]
**scope** | Option<**Vec<String>**> | List of tags used to filter the map. | [optional]
**url** | **String** | URL of the image. | 
**has_background** | Option<**bool**> | Whether to display a background or not. | [optional][default to true]
**has_border** | Option<**bool**> | Whether to display a border or not. | [optional][default to true]
**horizontal_align** | Option<[**crate::models::WidgetHorizontalAlign**](WidgetHorizontalAlign.md)> |  | [optional]
**margin** | Option<[**crate::models::WidgetMargin**](WidgetMargin.md)> |  | [optional]
**sizing** | Option<[**crate::models::WidgetImageSizing**](WidgetImageSizing.md)> |  | [optional]
**url_dark_theme** | Option<**String**> | URL of the image in dark mode. | [optional]
**vertical_align** | Option<[**crate::models::WidgetVerticalAlign**](WidgetVerticalAlign.md)> |  | [optional]
**columns** | Option<**Vec<String>**> | Which columns to display on the widget. | [optional]
**indexes** | Option<**Vec<String>**> | An array of index names to query in the stream. Use [] to query all indexes at once. | [optional]
**logset** | Option<**String**> | ID of the log set to use. | [optional]
**message_display** | Option<[**crate::models::WidgetMessageDisplay**](WidgetMessageDisplay.md)> |  | [optional]
**show_date_column** | Option<**bool**> | Whether to show the date column or not | [optional]
**show_message_column** | Option<**bool**> | Whether to show the message column or not | [optional]
**sort** | Option<[**crate::models::WidgetMonitorSummarySort**](WidgetMonitorSummarySort.md)> |  | [optional]
**color_preference** | Option<[**crate::models::WidgetColorPreference**](WidgetColorPreference.md)> |  | [optional]
**count** | Option<**i64**> | The number of monitors to display. | [optional]
**display_format** | Option<[**crate::models::WidgetServiceSummaryDisplayFormat**](WidgetServiceSummaryDisplayFormat.md)> |  | [optional]
**hide_zero_counts** | Option<**bool**> | Whether to show counts of 0 or not. | [optional]
**show_last_triggered** | Option<**bool**> | Whether to show the time that has elapsed since the monitor/group triggered. | [optional]
**start** | Option<**i64**> | The start of the list. Typically 0. | [optional]
**summary_type** | Option<[**crate::models::WidgetSummaryType**](WidgetSummaryType.md)> |  | [optional]
**content** | **String** | Content of the note. | 
**has_padding** | Option<**bool**> | Whether to add padding or not. | [optional][default to true]
**show_tick** | Option<**bool**> | Whether to show a tick or not. | [optional]
**tick_edge** | Option<[**crate::models::WidgetTickEdge**](WidgetTickEdge.md)> |  | [optional]
**tick_pos** | Option<**String**> | Where to position the tick on an edge. | [optional]
**autoscale** | Option<**bool**> | Whether to use auto-scaling or not. | [optional]
**custom_unit** | Option<**String**> | Display a unit of your choice on the widget. | [optional]
**color_by_groups** | Option<**Vec<String>**> | List of groups used for colors. | [optional]
**global_time_target** | Option<**String**> | Defined global time target. | [optional]
**show_error_budget** | Option<**bool**> | Defined error budget. | [optional]
**slo_id** | Option<**String**> | ID of the SLO displayed. | [optional]
**time_windows** | Option<[**Vec<crate::models::WidgetTimeWindows>**](WidgetTimeWindows.md)> | Times being monitored. | [optional]
**view_mode** | Option<[**crate::models::WidgetViewMode**](WidgetViewMode.md)> |  | [optional]
**view_type** | **String** | Type of view displayed by the widget. | [default to detail]
**filters** | **Vec<String>** | Your environment and primary tag (or * if enabled for your account). | 
**service** | **String** | APM service. | 
**env** | **String** | APM environment. | 
**show_breakdown** | Option<**bool**> | Whether to show the latency breakdown or not. | [optional]
**show_distribution** | Option<**bool**> | Whether to show the latency distribution or not. | [optional]
**show_errors** | Option<**bool**> | Whether to show the error metrics or not. | [optional]
**show_hits** | Option<**bool**> | Whether to show the hits metrics or not. | [optional]
**show_latency** | Option<**bool**> | Whether to show the latency metrics or not. | [optional]
**show_resource_list** | Option<**bool**> | Whether to show the resource list or not. | [optional]
**size_format** | Option<[**crate::models::WidgetSizeFormat**](WidgetSizeFormat.md)> |  | [optional]
**span_name** | **String** | APM span name. | 
**has_search_bar** | Option<[**crate::models::TableWidgetHasSearchBar**](TableWidgetHasSearchBar.md)> |  | [optional]
**legend_columns** | Option<[**Vec<crate::models::TimeseriesWidgetLegendColumn>**](TimeseriesWidgetLegendColumn.md)> | Columns displayed in the legend. | [optional]
**legend_layout** | Option<[**crate::models::TimeseriesWidgetLegendLayout**](TimeseriesWidgetLegendLayout.md)> |  | [optional]
**right_yaxis** | Option<[**crate::models::WidgetAxis**](WidgetAxis.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



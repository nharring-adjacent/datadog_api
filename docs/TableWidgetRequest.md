# TableWidgetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregator** | Option<[**crate::models::WidgetAggregator**](WidgetAggregator.md)> |  | [optional]
**alias** | Option<**String**> | The column name (defaults to the metric name). | [optional]
**apm_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**apm_stats_query** | Option<[**crate::models::ApmStatsQueryDefinition**](ApmStatsQueryDefinition.md)> |  | [optional]
**cell_display_mode** | Option<[**Vec<crate::models::TableWidgetCellDisplayMode>**](TableWidgetCellDisplayMode.md)> | A list of display modes for each table cell. | [optional]
**conditional_formats** | Option<[**Vec<crate::models::WidgetConditionalFormat>**](WidgetConditionalFormat.md)> | List of conditional formats. | [optional]
**event_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**limit** | Option<**i64**> | For metric queries, the number of lines to show in the table. Only one request should have this property. | [optional]
**log_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**network_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**order** | Option<[**crate::models::WidgetSort**](WidgetSort.md)> |  | [optional]
**process_query** | Option<[**crate::models::ProcessQueryDefinition**](ProcessQueryDefinition.md)> |  | [optional]
**profile_metrics_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**q** | Option<**String**> | Query definition. | [optional]
**rum_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**security_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



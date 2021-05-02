# DistributionWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**legend_size** | Option<**String**> | (Deprecated) The widget legend was replaced by a tooltip and sidebar. | [optional]
**markers** | Option<[**Vec<crate::models::WidgetMarker>**](WidgetMarker.md)> | List of markers. | [optional]
**requests** | [**Vec<crate::models::DistributionWidgetRequest>**](DistributionWidgetRequest.md) | Array of one request object to display in the widget.  See the dedicated [Request JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/request_json)  to learn how to build the `REQUEST_SCHEMA`. | 
**show_legend** | Option<**bool**> | (Deprecated) The widget legend was replaced by a tooltip and sidebar. | [optional]
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | Title of the widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::DistributionWidgetDefinitionType**](DistributionWidgetDefinitionType.md) |  | 
**xaxis** | Option<[**crate::models::DistributionWidgetXAxis**](DistributionWidgetXAxis.md)> |  | [optional]
**yaxis** | Option<[**crate::models::DistributionWidgetYAxis**](DistributionWidgetYAxis.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



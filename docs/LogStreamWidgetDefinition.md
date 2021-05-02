# LogStreamWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**columns** | Option<**Vec<String>**> | Which columns to display on the widget. | [optional]
**indexes** | Option<**Vec<String>**> | An array of index names to query in the stream. Use [] to query all indexes at once. | [optional]
**logset** | Option<**String**> | ID of the log set to use. | [optional]
**message_display** | Option<[**crate::models::WidgetMessageDisplay**](WidgetMessageDisplay.md)> |  | [optional]
**query** | Option<**String**> | Query to filter the log stream with. | [optional]
**show_date_column** | Option<**bool**> | Whether to show the date column or not | [optional]
**show_message_column** | Option<**bool**> | Whether to show the message column or not | [optional]
**sort** | Option<[**crate::models::WidgetFieldSort**](WidgetFieldSort.md)> |  | [optional]
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | Title of the widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::LogStreamWidgetDefinitionType**](LogStreamWidgetDefinitionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



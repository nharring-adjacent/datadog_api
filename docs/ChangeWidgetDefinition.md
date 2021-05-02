# ChangeWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_links** | Option<[**Vec<crate::models::WidgetCustomLink>**](WidgetCustomLink.md)> | List of custom links. | [optional]
**requests** | [**Vec<crate::models::ChangeWidgetRequest>**](ChangeWidgetRequest.md) | Array of one request object to display in the widget.  See the dedicated [Request JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/request_json)  to learn how to build the `REQUEST_SCHEMA`. | 
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | Title of the widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::ChangeWidgetDefinitionType**](ChangeWidgetDefinitionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



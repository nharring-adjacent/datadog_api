# GeomapWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_links** | Option<[**Vec<crate::models::WidgetCustomLink>**](WidgetCustomLink.md)> | A list of custom links. | [optional]
**requests** | [**Vec<crate::models::GeomapWidgetRequest>**](GeomapWidgetRequest.md) | Array of one request object to display in the widget. The request must contain a `group-by` tag whose value is a country ISO code.  See the [Request JSON schema documentation](https://docs.datadoghq.com/dashboards/graphing_json/request_json) for information about building the `REQUEST_SCHEMA`. | 
**style** | [**crate::models::GeomapWidgetDefinitionStyle**](GeomapWidgetDefinition_style.md) |  | 
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | The title of your widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | The size of the title. | [optional]
**_type** | [**crate::models::GeomapWidgetDefinitionType**](GeomapWidgetDefinitionType.md) |  | 
**view** | [**crate::models::GeomapWidgetDefinitionView**](GeomapWidgetDefinition_view.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



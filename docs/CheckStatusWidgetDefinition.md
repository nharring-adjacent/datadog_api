# CheckStatusWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**check** | **String** | Name of the check to use in the widget. | 
**group** | Option<**String**> | Group reporting a single check. | [optional]
**group_by** | Option<**Vec<String>**> | List of tag prefixes to group by in the case of a cluster check. | [optional]
**grouping** | [**crate::models::WidgetGrouping**](WidgetGrouping.md) |  | 
**tags** | Option<**Vec<String>**> | List of tags used to filter the groups reporting a cluster check. | [optional]
**time** | Option<[**crate::models::WidgetTime**](WidgetTime.md)> |  | [optional]
**title** | Option<**String**> | Title of the widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::CheckStatusWidgetDefinitionType**](CheckStatusWidgetDefinitionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



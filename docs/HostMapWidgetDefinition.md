# HostMapWidgetDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_links** | Option<[**Vec<crate::models::WidgetCustomLink>**](WidgetCustomLink.md)> | List of custom links. | [optional]
**group** | Option<**Vec<String>**> | List of tag prefixes to group by. | [optional]
**no_group_hosts** | Option<**bool**> | Whether to show the hosts that don’t fit in a group. | [optional]
**no_metric_hosts** | Option<**bool**> | Whether to show the hosts with no metrics. | [optional]
**node_type** | Option<[**crate::models::WidgetNodeType**](WidgetNodeType.md)> |  | [optional]
**notes** | Option<**String**> | Notes on the title. | [optional]
**requests** | [**crate::models::HostMapWidgetDefinitionRequests**](HostMapWidgetDefinition_requests.md) |  | 
**scope** | Option<**Vec<String>**> | List of tags used to filter the map. | [optional]
**style** | Option<[**crate::models::HostMapWidgetDefinitionStyle**](HostMapWidgetDefinition_style.md)> |  | [optional]
**title** | Option<**String**> | Title of the widget. | [optional]
**title_align** | Option<[**crate::models::WidgetTextAlign**](WidgetTextAlign.md)> |  | [optional]
**title_size** | Option<**String**> | Size of the title. | [optional]
**_type** | [**crate::models::HostMapWidgetDefinitionType**](HostMapWidgetDefinitionType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Dashboard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**author_handle** | Option<**String**> | Identifier of the dashboard author. | [optional][readonly]
**created_at** | Option<**String**> | Creation date of the dashboard. | [optional][readonly]
**description** | Option<**String**> | Description of the dashboard. | [optional]
**id** | Option<**String**> | ID of the dashboard. | [optional][readonly]
**is_read_only** | Option<**bool**> | Whether this dashboard is read-only. If True, only the author and admins can make changes to it. | [optional][default to false]
**layout_type** | [**crate::models::DashboardLayoutType**](DashboardLayoutType.md) |  | 
**modified_at** | Option<**String**> | Modification date of the dashboard. | [optional][readonly]
**notify_list** | Option<**Vec<String>**> | List of handles of users to notify when changes are made to this dashboard. | [optional]
**reflow_type** | Option<[**crate::models::DashboardReflowType**](DashboardReflowType.md)> |  | [optional]
**template_variable_presets** | Option<[**Vec<crate::models::DashboardTemplateVariablePreset>**](DashboardTemplateVariablePreset.md)> | Array of template variables saved views. | [optional]
**template_variables** | Option<[**Vec<crate::models::DashboardTemplateVariable>**](DashboardTemplateVariable.md)> | List of template variables for this dashboard. | [optional]
**title** | **String** | Title of the dashboard. | 
**url** | Option<**String**> | The URL of the dashboard. | [optional][readonly]
**widgets** | [**Vec<crate::models::Widget>**](Widget.md) | List of widgets to display on the dashboard. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



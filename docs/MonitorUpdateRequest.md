# MonitorUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | Option<**String**> | Timestamp of the monitor creation. | [optional][readonly]
**creator** | Option<[**crate::models::Creator**](Creator.md)> |  | [optional]
**deleted** | Option<**String**> | Whether or not the monitor is deleted. (Always `null`) | [optional][readonly]
**id** | Option<**i64**> | ID of this monitor. | [optional][readonly]
**message** | Option<**String**> | A message to include with notifications for this monitor. | [optional]
**modified** | Option<**String**> | Last timestamp when the monitor was edited. | [optional][readonly]
**multi** | Option<**bool**> | Whether or not the monitor is broken down on different groups. | [optional][readonly]
**name** | Option<**String**> | The monitor name. | [optional]
**options** | Option<[**crate::models::MonitorOptions**](MonitorOptions.md)> |  | [optional]
**overall_state** | Option<[**crate::models::MonitorOverallStates**](MonitorOverallStates.md)> |  | [optional]
**priority** | Option<**i64**> | Integer from 1 (high) to 5 (low) indicating alert severity. | [optional]
**query** | Option<**String**> | The monitor query. | [optional]
**restricted_roles** | Option<**Vec<String>**> | A list of role identifiers that can be pulled from the Roles API. Cannot be used with `locked` option. | [optional]
**state** | Option<[**crate::models::MonitorState**](MonitorState.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | Tags associated to your monitor. | [optional]
**_type** | Option<[**crate::models::MonitorType**](MonitorType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



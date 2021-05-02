# MonitorStateGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_nodata_ts** | Option<**i64**> | Latest timestamp the monitor was in NO_DATA state. | [optional]
**last_notified_ts** | Option<**i64**> | Latest timestamp of the notification sent for this monitor group. | [optional]
**last_resolved_ts** | Option<**i64**> | Latest timestamp the monitor group was resolved. | [optional]
**last_triggered_ts** | Option<**i64**> | Latest timestamp the monitor group triggered. | [optional]
**name** | Option<**String**> | The name of the monitor. | [optional]
**status** | Option<[**crate::models::MonitorOverallStates**](MonitorOverallStates.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



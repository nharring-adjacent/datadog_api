# Downtime

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | Option<**bool**> | If a scheduled downtime currently exists. | [optional][readonly]
**canceled** | Option<**i64**> | If a scheduled downtime is canceled. | [optional][readonly]
**creator_id** | Option<**i32**> | User ID of the downtime creator. | [optional][readonly]
**disabled** | Option<**bool**> | If a downtime has been disabled. | [optional]
**downtime_type** | Option<**i32**> | `0` for a downtime applied on `*` or all, `1` when the downtime is only scoped to hosts, or `2` when the downtime is scoped to anything but hosts. | [optional][readonly]
**end** | Option<**i64**> | POSIX timestamp to end the downtime. If not provided, the downtime is in effect indefinitely until you cancel it. | [optional]
**id** | Option<**i64**> | The downtime ID. | [optional][readonly]
**message** | Option<**String**> | A message to include with notifications for this downtime. Email notifications can be sent to specific users by using the same `@username` notation as events. | [optional]
**monitor_id** | Option<**i64**> | A single monitor to which the downtime applies. If not provided, the downtime applies to all monitors. | [optional]
**monitor_tags** | Option<**Vec<String>**> | A comma-separated list of monitor tags. For example, tags that are applied directly to monitors, not tags that are used in monitor queries (which are filtered by the scope parameter), to which the downtime applies. The resulting downtime applies to monitors that match ALL provided monitor tags. For example, `service:postgres` **AND** `team:frontend`. | [optional]
**parent_id** | Option<**i64**> | ID of the parent Downtime. | [optional]
**recurrence** | Option<[**crate::models::DowntimeRecurrence**](DowntimeRecurrence.md)> |  | [optional]
**scope** | Option<**Vec<String>**> | The scope(s) to which the downtime applies. For example, `host:app2`. Provide multiple scopes as a comma-separated list like `env:dev,env:prod`. The resulting downtime applies to sources that matches ALL provided scopes (`env:dev` **AND** `env:prod`). | [optional]
**start** | Option<**i64**> | POSIX timestamp to start the downtime. If not provided, the downtime starts the moment it is created. | [optional]
**timezone** | Option<**String**> | The timezone in which to display the downtime's start and end times in Datadog applications. | [optional]
**updater_id** | Option<**i32**> | ID of the last user that updated the downtime. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



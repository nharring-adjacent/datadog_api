# MonitorOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation** | Option<[**crate::models::MonitorOptionsAggregation**](MonitorOptions_aggregation.md)> |  | [optional]
**device_ids** | Option<[**Vec<crate::models::MonitorDeviceId>**](MonitorDeviceID.md)> | IDs of the device the Synthetics monitor is running on. | [optional][readonly]
**enable_logs_sample** | Option<**bool**> | Whether or not to send a log sample when the log monitor triggers. | [optional]
**escalation_message** | Option<**String**> | A message to include with a re-notification. Supports the `@username` notification we allow elsewhere. Not applicable if `renotify_interval` is `None`. | [optional][default to none]
**evaluation_delay** | Option<**i64**> | Time (in seconds) to delay evaluation, as a non-negative integer. For example, if the value is set to `300` (5min), the timeframe is set to `last_5m` and the time is 7:00, the monitor evaluates data from 6:50 to 6:55. This is useful for AWS CloudWatch and other backfilled metrics to ensure the monitor always has data during evaluation. | [optional]
**groupby_simple_monitor** | Option<**bool**> | Whether the log alert monitor triggers a single alert or multiple alerts when any group breaches a threshold. | [optional]
**include_tags** | Option<**bool**> | A Boolean indicating whether notifications from this monitor automatically inserts its triggering tags into the title.  **Examples** - If `True`, `[Triggered on {host:h1}] Monitor Title` - If `False`, `[Triggered] Monitor Title` | [optional][default to true]
**locked** | Option<**bool**> | Whether or not the monitor is locked (only editable by creator and admins). | [optional]
**min_failure_duration** | Option<**i64**> | How long the test should be in failure before alerting (integer, number of seconds, max 7200). | [optional][default to 0]
**min_location_failed** | Option<**i64**> | The minimum number of locations in failure at the same time during at least one moment in the `min_failure_duration` period (`min_location_failed` and `min_failure_duration` are part of the advanced alerting rules - integer, >= 1). | [optional][default to 1]
**new_host_delay** | Option<**i64**> | Time (in seconds) to allow a host to boot and applications to fully start before starting the evaluation of monitor results. Should be a non negative integer. | [optional][default to 300]
**no_data_timeframe** | Option<**i64**> | The number of minutes before a monitor notifies after data stops reporting. Datadog recommends at least 2x the monitor timeframe for metric alerts or 2 minutes for service checks. If omitted, 2x the evaluation timeframe is used for metric alerts, and 24 hours is used for service checks. | [optional]
**notify_audit** | Option<**bool**> | A Boolean indicating whether tagged users is notified on changes to this monitor. | [optional][default to false]
**notify_no_data** | Option<**bool**> | A Boolean indicating whether this monitor notifies when data stops reporting. | [optional][default to false]
**renotify_interval** | Option<**i64**> | The number of minutes after the last notification before a monitor re-notifies on the current status. It only re-notifies if it’s not resolved. | [optional]
**require_full_window** | Option<**bool**> | A Boolean indicating whether this monitor needs a full window of data before it’s evaluated. We highly recommend you set this to `false` for sparse metrics, otherwise some evaluations are skipped. Default is false. | [optional]
**silenced** | Option<**::std::collections::HashMap<String, i64>**> | Information about the downtime applied to the monitor. | [optional]
**synthetics_check_id** | Option<**String**> | ID of the corresponding Synthetic check. | [optional]
**threshold_windows** | Option<[**crate::models::MonitorThresholdWindowOptions**](MonitorThresholdWindowOptions.md)> |  | [optional]
**thresholds** | Option<[**crate::models::MonitorThresholds**](MonitorThresholds.md)> |  | [optional]
**timeout_h** | Option<**i64**> | The number of hours of the monitor not reporting data before it automatically resolves from a triggered state. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



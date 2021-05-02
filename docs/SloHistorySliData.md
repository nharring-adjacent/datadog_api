# SloHistorySliData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_budget_remaining** | Option<**::std::collections::HashMap<String, f64>**> | A mapping of threshold `timeframe` to the remaining error budget. | [optional]
**errors** | Option<[**Vec<crate::models::SloHistoryResponseError>**](SLOHistoryResponseError.md)> | A list of errors while querying the history data for the service level objective. | [optional]
**group** | Option<**String**> | For groups in a grouped SLO, this is the group name. | [optional]
**history** | Option<[**Vec<Vec<f64>>**](array.md)> | For `monitor` based SLOs, this includes the aggregated history uptime time series. | [optional]
**monitor_modified** | Option<**i64**> | For `monitor` based SLOs, this is the last modified timestamp in epoch seconds of the monitor. | [optional]
**monitor_type** | Option<**String**> | For `monitor` based SLOs, this describes the type of monitor. | [optional]
**name** | Option<**String**> | For groups in a grouped SLO, this is the group name. For monitors in a multi-monitor SLO, this is the monitor name. | [optional]
**precision** | Option<**::std::collections::HashMap<String, f64>**> | A mapping of threshold `timeframe` to number of accurate decimals, regardless of the from && to timestamp. | [optional]
**preview** | Option<**bool**> | For `monitor` based SLOs, when `true` this indicates that a replay is in progress to give an accurate uptime calculation. | [optional]
**sli_value** | Option<**f64**> | The current SLI value of the SLO over the history window. | [optional]
**span_precision** | Option<**f64**> | The amount of decimal places the SLI value is accurate to for the given from `&&` to timestamp. | [optional]
**uptime** | Option<**f64**> | Use `sli_value` instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



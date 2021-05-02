# SloHistoryResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_ts** | Option<**i64**> | The `from` timestamp in epoch seconds. | [optional]
**group_by** | Option<**Vec<String>**> | For `metric` based SLOs where the query includes a group-by clause, this represents the list of grouping parameters.  This is not included in responses for `monitor` based SLOs. | [optional]
**groups** | Option<[**Vec<crate::models::SloHistorySliData>**](SLOHistorySLIData.md)> | For grouped SLOs, this represents SLI data for specific groups.  This is not included in the responses for `metric` based SLOs. | [optional]
**monitors** | Option<[**Vec<crate::models::SloHistorySliData>**](SLOHistorySLIData.md)> | For multi-monitor SLOs, this represents SLI data for specific monitors.  This is not included in the responses for `metric` based SLOs. | [optional]
**overall** | Option<[**crate::models::SloHistorySliData**](SLOHistorySLIData.md)> |  | [optional]
**series** | Option<[**crate::models::SloHistoryMetrics**](SLOHistoryMetrics.md)> |  | [optional]
**thresholds** | Option<[**::std::collections::HashMap<String, crate::models::SloThreshold>**](SLOThreshold.md)> | mapping of string timeframe to the SLO threshold. | [optional]
**to_ts** | Option<**i64**> | The `to` timestamp in epoch seconds. | [optional]
**_type** | Option<[**crate::models::SloType**](SLOType.md)> |  | [optional]
**type_id** | Option<[**crate::models::SloTypeNumeric**](SLOTypeNumeric.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



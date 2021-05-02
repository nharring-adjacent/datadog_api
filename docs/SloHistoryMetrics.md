# SloHistoryMetrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**denominator** | [**crate::models::SloHistoryMetricsSeries**](SLOHistoryMetricsSeries.md) |  | 
**interval** | **i64** | The aggregated query interval for the series data. It's implicit based on the query time window. | 
**message** | Option<**String**> | Optional message if there are specific query issues/warnings. | [optional]
**numerator** | [**crate::models::SloHistoryMetricsSeries**](SLOHistoryMetricsSeries.md) |  | 
**query** | **String** | The combined numerator and denominator query CSV. | 
**res_type** | **String** | The series result type. This mimics `batch_query` response type. | 
**resp_version** | **i64** | The series response version type. This mimics `batch_query` response type. | 
**times** | **Vec<f64>** | An array of query timestamps in EPOCH milliseconds | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



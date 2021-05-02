# GraphSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**graph_def** | Option<**String**> | A JSON document defining the graph. `graph_def` can be used instead of `metric_query`. The JSON document uses the [grammar defined here](https://docs.datadoghq.com/graphing/graphing_json/#grammar) and should be formatted to a single line then URL encoded. | [optional]
**metric_query** | Option<**String**> | The metric query. One of `metric_query` or `graph_def` is required. | [optional]
**snapshot_url** | Option<**String**> | URL of your [graph snapshot](https://docs.datadoghq.com/metrics/explorer/#snapshot). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



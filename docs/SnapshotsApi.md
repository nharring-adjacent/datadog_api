# \SnapshotsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_graph_snapshot**](SnapshotsApi.md#get_graph_snapshot) | **get** /api/v1/graph/snapshot | Take graph snapshots



## get_graph_snapshot

> crate::models::GraphSnapshot get_graph_snapshot(start, end, metric_query, event_query, graph_def, title)
Take graph snapshots

Take graph snapshots. **Note**: When a snapshot is created, there is some delay before it is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **i64** | The POSIX timestamp of the start of the query. | [required] |
**end** | **i64** | The POSIX timestamp of the end of the query. | [required] |
**metric_query** | Option<**String**> | The metric query. |  |
**event_query** | Option<**String**> | A query that adds event bands to the graph. |  |
**graph_def** | Option<**String**> | A JSON document defining the graph. `graph_def` can be used instead of `metric_query`. The JSON document uses the [grammar defined here](https://docs.datadoghq.com/graphing/graphing_json/#grammar) and should be formatted to a single line then URL encoded. |  |
**title** | Option<**String**> | A title for the graph. If no title is specified, the graph does not have a title. |  |

### Return type

[**crate::models::GraphSnapshot**](GraphSnapshot.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


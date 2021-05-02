# \MetricsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metric_metadata**](MetricsApi.md#get_metric_metadata) | **get** /api/v1/metrics/{metric_name} | Get metric metadata
[**list_active_metrics**](MetricsApi.md#list_active_metrics) | **get** /api/v1/metrics | Get active metrics list
[**list_metrics**](MetricsApi.md#list_metrics) | **get** /api/v1/search | Search metrics
[**query_metrics**](MetricsApi.md#query_metrics) | **get** /api/v1/query | Query timeseries points
[**submit_metrics**](MetricsApi.md#submit_metrics) | **post** /api/v1/series | Submit metrics
[**update_metric_metadata**](MetricsApi.md#update_metric_metadata) | **put** /api/v1/metrics/{metric_name} | Edit metric metadata



## get_metric_metadata

> crate::models::MetricMetadata get_metric_metadata(metric_name)
Get metric metadata

Get metadata about a specific metric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_name** | **String** | Name of the metric for which to get metadata. | [required] |

### Return type

[**crate::models::MetricMetadata**](MetricMetadata.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_active_metrics

> crate::models::MetricsListResponse list_active_metrics(from, host, tag_filter)
Get active metrics list

Get the list of actively reporting metrics from a given time until now.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **i64** | Seconds since the Unix epoch. | [required] |
**host** | Option<**String**> | Hostname for filtering the list of metrics returned. If set, metrics retrieved are those with the corresponding hostname tag. |  |
**tag_filter** | Option<**String**> | Filter metrics that have been submitted with the given tags. Supports boolean and wildcard expressions. Cannot be combined with other filters. |  |

### Return type

[**crate::models::MetricsListResponse**](MetricsListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metrics

> crate::models::MetricSearchResponse list_metrics(q)
Search metrics

Search for metrics from the last 24 hours in Datadog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** | Query string to search metrics upon. Must be prefixed with `metrics:`. | [required] |

### Return type

[**crate::models::MetricSearchResponse**](MetricSearchResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_metrics

> crate::models::MetricsQueryResponse query_metrics(from, to, query)
Query timeseries points

Query timeseries points.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **i64** | Start of the queried time period, seconds since the Unix epoch. | [required] |
**to** | **i64** | End of the queried time period, seconds since the Unix epoch. | [required] |
**query** | **String** | Query string. | [required] |

### Return type

[**crate::models::MetricsQueryResponse**](MetricsQueryResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_metrics

> crate::models::IntakePayloadAccepted submit_metrics(body)
Submit metrics

The metrics end-point allows you to post time-series data that can be graphed on Datadog’s dashboards. The maximum payload size is 3.2 megabytes (3200000). Compressed payloads must have a decompressed size of up to 62 megabytes (62914560).  If you’re submitting metrics directly to the Datadog API without using DogStatsD, expect  - 64 bits for the timestamp - 32 bits for the value - 20 bytes for the metric names - 50 bytes for the timeseries - The full payload is approximately ~ 100 bytes. However, with the DogStatsD API, compression is applied, which reduces the payload size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MetricsPayload**](MetricsPayload.md) |  | [required] |

### Return type

[**crate::models::IntakePayloadAccepted**](IntakePayloadAccepted.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metric_metadata

> crate::models::MetricMetadata update_metric_metadata(metric_name, body)
Edit metric metadata

Edit metadata of a specific metric. Find out more about [supported types](https://docs.datadoghq.com/developers/metrics).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_name** | **String** | Name of the metric for which to edit metadata. | [required] |
**body** | [**MetricMetadata**](MetricMetadata.md) | New metadata. | [required] |

### Return type

[**crate::models::MetricMetadata**](MetricMetadata.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


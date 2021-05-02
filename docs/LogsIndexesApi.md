# \LogsIndexesApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_logs_index**](LogsIndexesApi.md#create_logs_index) | **post** /api/v1/logs/config/indexes | Create an index
[**get_logs_index**](LogsIndexesApi.md#get_logs_index) | **get** /api/v1/logs/config/indexes/{name} | Get an index
[**get_logs_index_order**](LogsIndexesApi.md#get_logs_index_order) | **get** /api/v1/logs/config/index-order | Get indexes order
[**list_log_indexes**](LogsIndexesApi.md#list_log_indexes) | **get** /api/v1/logs/config/indexes | Get all indexes
[**update_logs_index**](LogsIndexesApi.md#update_logs_index) | **put** /api/v1/logs/config/indexes/{name} | Update an index
[**update_logs_index_order**](LogsIndexesApi.md#update_logs_index_order) | **put** /api/v1/logs/config/index-order | Update indexes order



## create_logs_index

> crate::models::LogsIndex create_logs_index(body)
Create an index

Creates a new index. Returns the Index object passed in the request body when the request is successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LogsIndex**](LogsIndex.md) | Object containing the new index. | [required] |

### Return type

[**crate::models::LogsIndex**](LogsIndex.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs_index

> crate::models::LogsIndex get_logs_index(name)
Get an index

Get one log index from your organization. This endpoint takes no JSON arguments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the log index. | [required] |

### Return type

[**crate::models::LogsIndex**](LogsIndex.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs_index_order

> crate::models::LogsIndexesOrder get_logs_index_order()
Get indexes order

Get the current order of your log indexes. This endpoint takes no JSON arguments.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogsIndexesOrder**](LogsIndexesOrder.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_indexes

> crate::models::LogsIndexListResponse list_log_indexes()
Get all indexes

The Index object describes the configuration of a log index. This endpoint returns an array of the `LogIndex` objects of your organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LogsIndexListResponse**](LogsIndexListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_logs_index

> crate::models::LogsIndex update_logs_index(name, body)
Update an index

Update an index as identified by its name. Returns the Index object passed in the request body when the request is successful.  Using the `PUT` method updates your index’s configuration by **replacing** your current configuration with the new one sent to your Datadog organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the log index. | [required] |
**body** | [**LogsIndexUpdateRequest**](LogsIndexUpdateRequest.md) | Object containing the new `LogsIndexUpdateRequest`. | [required] |

### Return type

[**crate::models::LogsIndex**](LogsIndex.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_logs_index_order

> crate::models::LogsIndexesOrder update_logs_index_order(body)
Update indexes order

This endpoint updates the index order of your organization. It returns the index order object passed in the request body when the request is successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LogsIndexesOrder**](LogsIndexesOrder.md) | Object containing the new ordered list of index names | [required] |

### Return type

[**crate::models::LogsIndexesOrder**](LogsIndexesOrder.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


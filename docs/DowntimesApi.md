# \DowntimesApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_downtime**](DowntimesApi.md#cancel_downtime) | **delete** /api/v1/downtime/{downtime_id} | Cancel a downtime
[**cancel_downtimes_by_scope**](DowntimesApi.md#cancel_downtimes_by_scope) | **post** /api/v1/downtime/cancel/by_scope | Cancel downtimes by scope
[**create_downtime**](DowntimesApi.md#create_downtime) | **post** /api/v1/downtime | Schedule a downtime
[**get_downtime**](DowntimesApi.md#get_downtime) | **get** /api/v1/downtime/{downtime_id} | Get a downtime
[**list_downtimes**](DowntimesApi.md#list_downtimes) | **get** /api/v1/downtime | Get all downtimes
[**list_monitor_downtimes**](DowntimesApi.md#list_monitor_downtimes) | **get** /api/v1/monitor/{monitor_id}/downtimes | Get all downtimes for a monitor
[**update_downtime**](DowntimesApi.md#update_downtime) | **put** /api/v1/downtime/{downtime_id} | Update a downtime



## cancel_downtime

> cancel_downtime(downtime_id)
Cancel a downtime

Cancel a downtime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**downtime_id** | **i64** | ID of the downtime to cancel. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_downtimes_by_scope

> crate::models::CanceledDowntimesIds cancel_downtimes_by_scope(body)
Cancel downtimes by scope

Delete all downtimes that match the scope of `X`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CancelDowntimesByScopeRequest**](CancelDowntimesByScopeRequest.md) | Scope to cancel downtimes for. | [required] |

### Return type

[**crate::models::CanceledDowntimesIds**](CanceledDowntimesIds.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_downtime

> crate::models::Downtime create_downtime(body)
Schedule a downtime

Schedule a downtime.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Downtime**](Downtime.md) | Schedule a downtime request body. | [required] |

### Return type

[**crate::models::Downtime**](Downtime.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_downtime

> crate::models::Downtime get_downtime(downtime_id)
Get a downtime

Get downtime detail by `downtime_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**downtime_id** | **i64** | ID of the downtime to fetch. | [required] |

### Return type

[**crate::models::Downtime**](Downtime.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_downtimes

> Vec<crate::models::Downtime> list_downtimes(current_only)
Get all downtimes

Get all scheduled downtimes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**current_only** | Option<**bool**> | Only return downtimes that are active when the request is made. |  |

### Return type

[**Vec<crate::models::Downtime>**](Downtime.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_monitor_downtimes

> Vec<crate::models::Downtime> list_monitor_downtimes(monitor_id)
Get all downtimes for a monitor

Get all downtimes for the specified monitor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**monitor_id** | **i64** | The id of the monitor | [required] |

### Return type

[**Vec<crate::models::Downtime>**](Downtime.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_downtime

> crate::models::Downtime update_downtime(downtime_id, body)
Update a downtime

Update a single downtime by `downtime_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**downtime_id** | **i64** | ID of the downtime to update. | [required] |
**body** | [**Downtime**](Downtime.md) | Update a downtime request body. | [required] |

### Return type

[**crate::models::Downtime**](Downtime.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


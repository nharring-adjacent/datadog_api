# \ServiceLevelObjectivesApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_can_delete_slo**](ServiceLevelObjectivesApi.md#check_can_delete_slo) | **get** /api/v1/slo/can_delete | Check if SLOs can be safely deleted
[**create_slo**](ServiceLevelObjectivesApi.md#create_slo) | **post** /api/v1/slo | Create an SLO object
[**delete_slo**](ServiceLevelObjectivesApi.md#delete_slo) | **delete** /api/v1/slo/{slo_id} | Delete an SLO
[**delete_slo_timeframe_in_bulk**](ServiceLevelObjectivesApi.md#delete_slo_timeframe_in_bulk) | **post** /api/v1/slo/bulk_delete | Bulk Delete SLO Timeframes
[**get_slo**](ServiceLevelObjectivesApi.md#get_slo) | **get** /api/v1/slo/{slo_id} | Get an SLO's details
[**get_slo_history**](ServiceLevelObjectivesApi.md#get_slo_history) | **get** /api/v1/slo/{slo_id}/history | Get an SLO's history
[**list_slos**](ServiceLevelObjectivesApi.md#list_slos) | **get** /api/v1/slo | Get all SLOs
[**update_slo**](ServiceLevelObjectivesApi.md#update_slo) | **put** /api/v1/slo/{slo_id} | Update an SLO



## check_can_delete_slo

> crate::models::CheckCanDeleteSloResponse check_can_delete_slo(ids)
Check if SLOs can be safely deleted

Check if an SLO can be safely deleted. For example, assure an SLO can be deleted without disrupting a dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | A comma separated list of the IDs of the service level objectives objects. | [required] |

### Return type

[**crate::models::CheckCanDeleteSloResponse**](CheckCanDeleteSLOResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_slo

> crate::models::SloListResponse create_slo(body)
Create an SLO object

Create a service level objective object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ServiceLevelObjectiveRequest**](ServiceLevelObjectiveRequest.md) | Service level objective request object. | [required] |

### Return type

[**crate::models::SloListResponse**](SLOListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_slo

> crate::models::SloDeleteResponse delete_slo(slo_id, force)
Delete an SLO

Permanently delete the specified service level objective object.  If an SLO is used in a dashboard, the `DELETE /v1/slo/` endpoint returns a 409 conflict error because the SLO is referenced in a dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_id** | **String** | The ID of the service level objective. | [required] |
**force** | Option<**String**> | Delete the monitor even if it's referenced by other resources (e.g. SLO, composite monitor). |  |

### Return type

[**crate::models::SloDeleteResponse**](SLODeleteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_slo_timeframe_in_bulk

> crate::models::SloBulkDeleteResponse delete_slo_timeframe_in_bulk(body)
Bulk Delete SLO Timeframes

Delete (or partially delete) multiple service level objective objects.  This endpoint facilitates deletion of one or more thresholds for one or more service level objective objects. If all thresholds are deleted, the service level objective object is deleted as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**::std::collections::HashMap<String, Vec<crate::models::SloTimeframe>>**](array.md) | Delete multiple service level objective objects request body. | [required] |

### Return type

[**crate::models::SloBulkDeleteResponse**](SLOBulkDeleteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slo

> crate::models::SloResponse get_slo(slo_id)
Get an SLO's details

Get a service level objective object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_id** | **String** | The ID of the service level objective object. | [required] |

### Return type

[**crate::models::SloResponse**](SLOResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slo_history

> crate::models::SloHistoryResponse get_slo_history(slo_id, from_ts, to_ts, target)
Get an SLO's history

Get a specific SLO’s history, regardless of its SLO type.  The detailed history data is structured according to the source data type. For example, metric data is included for event SLOs that use the metric source, and monitor SLO types include the monitor transition history.  **Note:** There are different response formats for event based and time based SLOs. Examples of both are shown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_id** | **String** | The ID of the service level objective object. | [required] |
**from_ts** | **i64** | The `from` timestamp for the query window in epoch seconds. | [required] |
**to_ts** | **i64** | The `to` timestamp for the query window in epoch seconds. | [required] |
**target** | Option<**f64**> | The SLO target. If `target` is passed in, the response will include the error budget that remains. |  |

### Return type

[**crate::models::SloHistoryResponse**](SLOHistoryResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_slos

> crate::models::SloListResponse list_slos(ids, query, tags_query, metrics_query)
Get all SLOs

Get a list of service level objective objects for your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | A comma separated list of the IDs of the service level objectives objects. |  |
**query** | Option<**String**> | The query string to filter results based on SLO names. |  |
**tags_query** | Option<**String**> | The query string to filter results based on a single SLO tag. |  |
**metrics_query** | Option<**String**> | The query string to filter results based on SLO numerator and denominator. |  |

### Return type

[**crate::models::SloListResponse**](SLOListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_slo

> crate::models::SloListResponse update_slo(slo_id, body)
Update an SLO

Update the specified service level objective object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_id** | **String** | The ID of the service level objective object. | [required] |
**body** | [**ServiceLevelObjective**](ServiceLevelObjective.md) | The edited service level objective request object. | [required] |

### Return type

[**crate::models::SloListResponse**](SLOListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DashboardListsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dashboard_list**](DashboardListsApi.md#create_dashboard_list) | **post** /api/v1/dashboard/lists/manual | Create a dashboard list
[**delete_dashboard_list**](DashboardListsApi.md#delete_dashboard_list) | **delete** /api/v1/dashboard/lists/manual/{list_id} | Delete a dashboard list
[**get_dashboard_list**](DashboardListsApi.md#get_dashboard_list) | **get** /api/v1/dashboard/lists/manual/{list_id} | Get a dashboard list
[**list_dashboard_lists**](DashboardListsApi.md#list_dashboard_lists) | **get** /api/v1/dashboard/lists/manual | Get all dashboard lists
[**update_dashboard_list**](DashboardListsApi.md#update_dashboard_list) | **put** /api/v1/dashboard/lists/manual/{list_id} | Update a dashboard list



## create_dashboard_list

> crate::models::DashboardList create_dashboard_list(body)
Create a dashboard list

Create an empty dashboard list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DashboardList**](DashboardList.md) | Create a dashboard list request body. | [required] |

### Return type

[**crate::models::DashboardList**](DashboardList.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_list

> crate::models::DashboardListDeleteResponse delete_dashboard_list(list_id)
Delete a dashboard list

Delete a dashboard list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | ID of the dashboard list to delete. | [required] |

### Return type

[**crate::models::DashboardListDeleteResponse**](DashboardListDeleteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_list

> crate::models::DashboardList get_dashboard_list(list_id)
Get a dashboard list

Fetch an existing dashboard list's definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | ID of the dashboard list to fetch. | [required] |

### Return type

[**crate::models::DashboardList**](DashboardList.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dashboard_lists

> crate::models::DashboardListListResponse list_dashboard_lists()
Get all dashboard lists

Fetch all of your existing dashboard list definitions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DashboardListListResponse**](DashboardListListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_list

> crate::models::DashboardList update_dashboard_list(list_id, body)
Update a dashboard list

Update the name of a dashboard list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_id** | **i64** | ID of the dashboard list to update. | [required] |
**body** | [**DashboardList**](DashboardList.md) | Update a dashboard list request body. | [required] |

### Return type

[**crate::models::DashboardList**](DashboardList.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DashboardsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dashboard**](DashboardsApi.md#create_dashboard) | **post** /api/v1/dashboard | Create a new dashboard
[**delete_dashboard**](DashboardsApi.md#delete_dashboard) | **delete** /api/v1/dashboard/{dashboard_id} | Delete a dashboard
[**get_dashboard**](DashboardsApi.md#get_dashboard) | **get** /api/v1/dashboard/{dashboard_id} | Get a dashboard
[**list_dashboards**](DashboardsApi.md#list_dashboards) | **get** /api/v1/dashboard | Get all dashboards
[**update_dashboard**](DashboardsApi.md#update_dashboard) | **put** /api/v1/dashboard/{dashboard_id} | Update a dashboard



## create_dashboard

> crate::models::Dashboard create_dashboard(body)
Create a new dashboard

Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the `as_count()` or `as_rate()` modifiers appended. Refer to the following [documentation](https://docs.datadoghq.com/developers/metrics/type_modifiers/?tab=count#in-application-modifiers) for more information on these modifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Dashboard**](Dashboard.md) | Create a dashboard request body. | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard

> crate::models::DashboardDeleteResponse delete_dashboard(dashboard_id)
Delete a dashboard

Delete a dashboard using the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |

### Return type

[**crate::models::DashboardDeleteResponse**](DashboardDeleteResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard

> crate::models::Dashboard get_dashboard(dashboard_id)
Get a dashboard

Get a dashboard using the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dashboards

> crate::models::DashboardSummary list_dashboards(filter_shared)
Get all dashboards

Get all dashboards.  **Note**: This query will only return custom created or cloned dashboards. This query will not return preset dashboards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_shared** | Option<**bool**> | When `true`, this query only returns shared custom created or cloned dashboards. |  |

### Return type

[**crate::models::DashboardSummary**](DashboardSummary.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard

> crate::models::Dashboard update_dashboard(dashboard_id, body)
Update a dashboard

Update a dashboard using the specified ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** | The ID of the dashboard. | [required] |
**body** | [**Dashboard**](Dashboard.md) | Update Dashboard request body. | [required] |

### Return type

[**crate::models::Dashboard**](Dashboard.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


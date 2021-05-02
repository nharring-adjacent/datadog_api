# \PagerDutyIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pager_duty_integration_service**](PagerDutyIntegrationApi.md#create_pager_duty_integration_service) | **post** /api/v1/integration/pagerduty/configuration/services | Create a new service object
[**delete_pager_duty_integration_service**](PagerDutyIntegrationApi.md#delete_pager_duty_integration_service) | **delete** /api/v1/integration/pagerduty/configuration/services/{service_name} | Delete a single service object
[**get_pager_duty_integration_service**](PagerDutyIntegrationApi.md#get_pager_duty_integration_service) | **get** /api/v1/integration/pagerduty/configuration/services/{service_name} | Get a single service object
[**update_pager_duty_integration_service**](PagerDutyIntegrationApi.md#update_pager_duty_integration_service) | **put** /api/v1/integration/pagerduty/configuration/services/{service_name} | Update a single service object



## create_pager_duty_integration_service

> crate::models::PagerDutyServiceName create_pager_duty_integration_service(body)
Create a new service object

Create a new service object in the PagerDuty integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PagerDutyService**](PagerDutyService.md) | Create a new service object request body. | [required] |

### Return type

[**crate::models::PagerDutyServiceName**](PagerDutyServiceName.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pager_duty_integration_service

> delete_pager_duty_integration_service(service_name)
Delete a single service object

Delete a single service object in the Datadog-PagerDuty integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** | The service name | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pager_duty_integration_service

> crate::models::PagerDutyServiceName get_pager_duty_integration_service(service_name)
Get a single service object

Get service name in the Datadog-PagerDuty integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** | The service name. | [required] |

### Return type

[**crate::models::PagerDutyServiceName**](PagerDutyServiceName.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pager_duty_integration_service

> update_pager_duty_integration_service(service_name, body)
Update a single service object

Update a single service object in the Datadog-PagerDuty integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** | The service name | [required] |
**body** | [**PagerDutyServiceKey**](PagerDutyServiceKey.md) | Update an existing service object request body. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


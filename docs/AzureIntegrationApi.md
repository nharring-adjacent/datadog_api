# \AzureIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_azure_integration**](AzureIntegrationApi.md#create_azure_integration) | **post** /api/v1/integration/azure | Create an Azure integration
[**delete_azure_integration**](AzureIntegrationApi.md#delete_azure_integration) | **delete** /api/v1/integration/azure | Delete an Azure integration
[**list_azure_integration**](AzureIntegrationApi.md#list_azure_integration) | **get** /api/v1/integration/azure | List all Azure integrations
[**update_azure_host_filters**](AzureIntegrationApi.md#update_azure_host_filters) | **post** /api/v1/integration/azure/host_filters | Update Azure integration host filters
[**update_azure_integration**](AzureIntegrationApi.md#update_azure_integration) | **put** /api/v1/integration/azure | Update an Azure integration



## create_azure_integration

> serde_json::Value create_azure_integration(body)
Create an Azure integration

Create a Datadog-Azure integration.  Using the `POST` method updates your integration configuration by adding your new configuration to the existing one in your Datadog organization.  Using the `PUT` method updates your integration configuration by replacing your current configuration with the new one sent to your Datadog organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AzureAccount**](AzureAccount.md) | Create a Datadog-Azure integration for your Datadog account request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_azure_integration

> serde_json::Value delete_azure_integration(body)
Delete an Azure integration

Delete a given Datadog-Azure integration from your Datadog account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AzureAccount**](AzureAccount.md) | Delete a given Datadog-Azure integration request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_azure_integration

> Vec<crate::models::AzureAccount> list_azure_integration()
List all Azure integrations

List all Datadog-Azure integrations configured in your Datadog account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AzureAccount>**](AzureAccount.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_azure_host_filters

> serde_json::Value update_azure_host_filters(body)
Update Azure integration host filters

Update the defined list of host filters for a given Datadog-Azure integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AzureAccount**](AzureAccount.md) | Update a Datadog-Azure integration's host filters request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_azure_integration

> serde_json::Value update_azure_integration(body)
Update an Azure integration

Update a Datadog-Azure integration. Requires an existing `tenant_name` and `client_id`. Any other fields supplied will overwrite existing values. To overwrite `tenant_name` or `client_id`, use `new_tenant_name` and `new_client_id`. To leave a field unchanged, do not supply that field in the payload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AzureAccount**](AzureAccount.md) | Update a Datadog-Azure integration request body. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


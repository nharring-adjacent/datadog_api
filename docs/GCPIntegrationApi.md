# \GCPIntegrationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_gcp_integration**](GCPIntegrationApi.md#create_gcp_integration) | **post** /api/v1/integration/gcp | Create a GCP integration
[**delete_gcp_integration**](GCPIntegrationApi.md#delete_gcp_integration) | **delete** /api/v1/integration/gcp | Delete a GCP integration
[**list_gcp_integration**](GCPIntegrationApi.md#list_gcp_integration) | **get** /api/v1/integration/gcp | List all GCP integrations
[**update_gcp_integration**](GCPIntegrationApi.md#update_gcp_integration) | **put** /api/v1/integration/gcp | Update a GCP integration



## create_gcp_integration

> serde_json::Value create_gcp_integration(body)
Create a GCP integration

Create a Datadog-GCP integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GcpAccount**](GcpAccount.md) | Create a Datadog-GCP integration. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_gcp_integration

> serde_json::Value delete_gcp_integration(body)
Delete a GCP integration

Delete a given Datadog-GCP integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GcpAccount**](GcpAccount.md) | Delete a given Datadog-GCP integration. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_gcp_integration

> Vec<crate::models::GcpAccount> list_gcp_integration()
List all GCP integrations

List all Datadog-GCP integrations configured in your Datadog account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::GcpAccount>**](GCPAccount.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_gcp_integration

> serde_json::Value update_gcp_integration(body)
Update a GCP integration

Update a Datadog-GCP integrations host_filters and/or auto-mute. Requires a `project_id` and `client_email`, however these fields cannot be updated. If you need to update these fields, delete and use the create (`POST`) endpoint. The unspecified fields will keep their original values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GcpAccount**](GcpAccount.md) | Update a Datadog-GCP integration. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


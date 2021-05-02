# \ServiceChecksApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submit_service_check**](ServiceChecksApi.md#submit_service_check) | **post** /api/v1/check_run | Submit a Service Check



## submit_service_check

> crate::models::IntakePayloadAccepted submit_service_check(body)
Submit a Service Check

Submit a list of Service Checks.  **Note**: A valid API key is required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::ServiceCheck>**](ServiceCheck.md) | Service Check request body. | [required] |

### Return type

[**crate::models::IntakePayloadAccepted**](IntakePayloadAccepted.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


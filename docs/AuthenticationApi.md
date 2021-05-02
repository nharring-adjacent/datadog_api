# \AuthenticationApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**validate**](AuthenticationApi.md#validate) | **get** /api/v1/validate | Validate API key



## validate

> crate::models::AuthenticationValidationResponse validate()
Validate API key

Check if the API key (not the APP key) is valid. If invalid, a 403 is returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthenticationValidationResponse**](AuthenticationValidationResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ServiceLevelObjectiveCorrectionsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_slo_correction**](ServiceLevelObjectiveCorrectionsApi.md#create_slo_correction) | **post** /api/v1/slo/correction | Create an SLO correction
[**delete_slo_correction**](ServiceLevelObjectiveCorrectionsApi.md#delete_slo_correction) | **delete** /api/v1/slo/correction/{slo_correction_id} | Delete an SLO correction
[**get_slo_correction**](ServiceLevelObjectiveCorrectionsApi.md#get_slo_correction) | **get** /api/v1/slo/correction/{slo_correction_id} | Get an SLO correction for an SLO
[**list_slo_correction**](ServiceLevelObjectiveCorrectionsApi.md#list_slo_correction) | **get** /api/v1/slo/correction | Get all SLO corrections
[**update_slo_correction**](ServiceLevelObjectiveCorrectionsApi.md#update_slo_correction) | **patch** /api/v1/slo/correction/{slo_correction_id} | Update an SLO correction



## create_slo_correction

> crate::models::SloCorrectionResponse create_slo_correction(body)
Create an SLO correction

Create an SLO Correction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SloCorrectionCreateRequest**](SloCorrectionCreateRequest.md) | Create an SLO Correction | [required] |

### Return type

[**crate::models::SloCorrectionResponse**](SLOCorrectionResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_slo_correction

> delete_slo_correction(slo_correction_id)
Delete an SLO correction

Permanently delete the specified SLO correction object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_correction_id** | **String** | The ID of the SLO correction object | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slo_correction

> crate::models::SloCorrectionResponse get_slo_correction(slo_correction_id)
Get an SLO correction for an SLO

Get an SLO correction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_correction_id** | **String** | The ID of the SLO correction object | [required] |

### Return type

[**crate::models::SloCorrectionResponse**](SLOCorrectionResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_slo_correction

> crate::models::SloCorrectionListResponse list_slo_correction()
Get all SLO corrections

Get all Service Level Objective corrections

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SloCorrectionListResponse**](SLOCorrectionListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_slo_correction

> crate::models::SloCorrectionResponse update_slo_correction(slo_correction_id, body)
Update an SLO correction

Update the specified SLO correction object object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_correction_id** | **String** | The ID of the SLO correction object | [required] |
**body** | [**SloCorrectionUpdateRequest**](SloCorrectionUpdateRequest.md) | The edited SLO correction object. | [required] |

### Return type

[**crate::models::SloCorrectionResponse**](SLOCorrectionResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


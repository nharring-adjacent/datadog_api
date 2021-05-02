# \SyntheticsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_global_variable**](SyntheticsApi.md#create_global_variable) | **post** /api/v1/synthetics/variables | Create a global variable
[**create_private_location**](SyntheticsApi.md#create_private_location) | **post** /api/v1/synthetics/private-locations | Create a private location
[**create_synthetics_api_test**](SyntheticsApi.md#create_synthetics_api_test) | **post** /api/v1/synthetics/tests/api | Create an API test
[**create_synthetics_browser_test**](SyntheticsApi.md#create_synthetics_browser_test) | **post** /api/v1/synthetics/tests/browser | Create a browser test
[**delete_global_variable**](SyntheticsApi.md#delete_global_variable) | **delete** /api/v1/synthetics/variables/{variable_id} | Delete a global variable
[**delete_private_location**](SyntheticsApi.md#delete_private_location) | **delete** /api/v1/synthetics/private-locations/{location_id} | Delete a private location
[**delete_tests**](SyntheticsApi.md#delete_tests) | **post** /api/v1/synthetics/tests/delete | Delete tests
[**edit_global_variable**](SyntheticsApi.md#edit_global_variable) | **put** /api/v1/synthetics/variables/{variable_id} | Edit a global variable
[**get_api_test**](SyntheticsApi.md#get_api_test) | **get** /api/v1/synthetics/tests/api/{public_id} | Get an API test
[**get_api_test_latest_results**](SyntheticsApi.md#get_api_test_latest_results) | **get** /api/v1/synthetics/tests/{public_id}/results | Get an API test's latest results summaries
[**get_api_test_result**](SyntheticsApi.md#get_api_test_result) | **get** /api/v1/synthetics/tests/{public_id}/results/{result_id} | Get an API test result
[**get_browser_test**](SyntheticsApi.md#get_browser_test) | **get** /api/v1/synthetics/tests/browser/{public_id} | Get a browser test
[**get_browser_test_latest_results**](SyntheticsApi.md#get_browser_test_latest_results) | **get** /api/v1/synthetics/tests/browser/{public_id}/results | Get a browser test's latest results summaries
[**get_browser_test_result**](SyntheticsApi.md#get_browser_test_result) | **get** /api/v1/synthetics/tests/browser/{public_id}/results/{result_id} | Get a browser test result
[**get_global_variable**](SyntheticsApi.md#get_global_variable) | **get** /api/v1/synthetics/variables/{variable_id} | Get a global variable
[**get_private_location**](SyntheticsApi.md#get_private_location) | **get** /api/v1/synthetics/private-locations/{location_id} | Get a private location
[**get_test**](SyntheticsApi.md#get_test) | **get** /api/v1/synthetics/tests/{public_id} | Get a test configuration
[**list_locations**](SyntheticsApi.md#list_locations) | **get** /api/v1/synthetics/locations | Get all locations (public and private)
[**list_tests**](SyntheticsApi.md#list_tests) | **get** /api/v1/synthetics/tests | Get the list of all tests
[**trigger_ci_tests**](SyntheticsApi.md#trigger_ci_tests) | **post** /api/v1/synthetics/tests/trigger/ci | Trigger tests from CI/CD pipelines
[**update_api_test**](SyntheticsApi.md#update_api_test) | **put** /api/v1/synthetics/tests/api/{public_id} | Edit an API test
[**update_browser_test**](SyntheticsApi.md#update_browser_test) | **put** /api/v1/synthetics/tests/browser/{public_id} | Edit a browser test
[**update_private_location**](SyntheticsApi.md#update_private_location) | **put** /api/v1/synthetics/private-locations/{location_id} | Edit a private location
[**update_test_pause_status**](SyntheticsApi.md#update_test_pause_status) | **put** /api/v1/synthetics/tests/{public_id}/status | Pause or start a test



## create_global_variable

> crate::models::SyntheticsGlobalVariable create_global_variable(body)
Create a global variable

Create a Synthetics global variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsGlobalVariable**](SyntheticsGlobalVariable.md) | Details of the global variable to create. | [required] |

### Return type

[**crate::models::SyntheticsGlobalVariable**](SyntheticsGlobalVariable.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_private_location

> crate::models::SyntheticsPrivateLocationCreationResponse create_private_location(body)
Create a private location

Create a new Synthetics private location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsPrivateLocation**](SyntheticsPrivateLocation.md) | Details of the private location to create. | [required] |

### Return type

[**crate::models::SyntheticsPrivateLocationCreationResponse**](SyntheticsPrivateLocationCreationResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_synthetics_api_test

> crate::models::SyntheticsApiTest create_synthetics_api_test(body)
Create an API test

Create a Synthetic API test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsApiTest**](SyntheticsApiTest.md) | Details of the test to create. | [required] |

### Return type

[**crate::models::SyntheticsApiTest**](SyntheticsAPITest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_synthetics_browser_test

> crate::models::SyntheticsBrowserTest create_synthetics_browser_test(body)
Create a browser test

Create a Synthetic browser test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsBrowserTest**](SyntheticsBrowserTest.md) | Details of the test to create. | [required] |

### Return type

[**crate::models::SyntheticsBrowserTest**](SyntheticsBrowserTest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_global_variable

> delete_global_variable(variable_id)
Delete a global variable

Delete a Synthetics global variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **String** | The ID of the global variable. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_location

> delete_private_location(location_id)
Delete a private location

Delete a Synthetics private location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the private location. | [required] |

### Return type

 (empty response body)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tests

> crate::models::SyntheticsDeleteTestsResponse delete_tests(body)
Delete tests

Delete multiple Synthetic tests by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsDeleteTestsPayload**](SyntheticsDeleteTestsPayload.md) | Public ID list of the Synthetic tests to be deleted. | [required] |

### Return type

[**crate::models::SyntheticsDeleteTestsResponse**](SyntheticsDeleteTestsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_global_variable

> crate::models::SyntheticsGlobalVariable edit_global_variable(variable_id, body)
Edit a global variable

Edit a Synthetics global variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **String** | The ID of the global variable. | [required] |
**body** | [**SyntheticsGlobalVariable**](SyntheticsGlobalVariable.md) | Details of the global variable to update. | [required] |

### Return type

[**crate::models::SyntheticsGlobalVariable**](SyntheticsGlobalVariable.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_test

> crate::models::SyntheticsApiTest get_api_test(public_id)
Get an API test

Get the detailed configuration associated with a Synthetic API test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test to get details from. | [required] |

### Return type

[**crate::models::SyntheticsApiTest**](SyntheticsAPITest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_test_latest_results

> crate::models::SyntheticsGetApiTestLatestResultsResponse get_api_test_latest_results(public_id, from_ts, to_ts, probe_dc)
Get an API test's latest results summaries

Get the last 50 test results summaries for a given Synthetics API test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test for which to search results for. | [required] |
**from_ts** | Option<**i64**> | Timestamp from which to start querying results. |  |
**to_ts** | Option<**i64**> | Timestamp up to which to query results. |  |
**probe_dc** | Option<[**Vec<String>**](String.md)> | Locations for which to query results. |  |

### Return type

[**crate::models::SyntheticsGetApiTestLatestResultsResponse**](SyntheticsGetAPITestLatestResultsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_test_result

> crate::models::SyntheticsApiTestResultFull get_api_test_result(public_id, result_id)
Get an API test result

Get a specific full result from a given (API) Synthetic test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the API test to which the target result belongs. | [required] |
**result_id** | **String** | The ID of the result to get. | [required] |

### Return type

[**crate::models::SyntheticsApiTestResultFull**](SyntheticsAPITestResultFull.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browser_test

> crate::models::SyntheticsBrowserTest get_browser_test(public_id)
Get a browser test

Get the detailed configuration (including steps) associated with a Synthetic browser test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test to get details from. | [required] |

### Return type

[**crate::models::SyntheticsBrowserTest**](SyntheticsBrowserTest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browser_test_latest_results

> crate::models::SyntheticsGetBrowserTestLatestResultsResponse get_browser_test_latest_results(public_id, from_ts, to_ts, probe_dc)
Get a browser test's latest results summaries

Get the last 50 test results summaries for a given Synthetics Browser test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the browser test for which to search results for. | [required] |
**from_ts** | Option<**i64**> | Timestamp from which to start querying results. |  |
**to_ts** | Option<**i64**> | Timestamp up to which to query results. |  |
**probe_dc** | Option<[**Vec<String>**](String.md)> | Locations for which to query results. |  |

### Return type

[**crate::models::SyntheticsGetBrowserTestLatestResultsResponse**](SyntheticsGetBrowserTestLatestResultsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browser_test_result

> crate::models::SyntheticsBrowserTestResultFull get_browser_test_result(public_id, result_id)
Get a browser test result

Get a specific full result from a given (browser) Synthetic test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the browser test to which the target result belongs. | [required] |
**result_id** | **String** | The ID of the result to get. | [required] |

### Return type

[**crate::models::SyntheticsBrowserTestResultFull**](SyntheticsBrowserTestResultFull.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_variable

> crate::models::SyntheticsGlobalVariable get_global_variable(variable_id)
Get a global variable

Get the detailed configuration of a global variable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**variable_id** | **String** | The ID of the global variable. | [required] |

### Return type

[**crate::models::SyntheticsGlobalVariable**](SyntheticsGlobalVariable.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_location

> crate::models::SyntheticsPrivateLocation get_private_location(location_id)
Get a private location

Get a Synthetics private location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the private location. | [required] |

### Return type

[**crate::models::SyntheticsPrivateLocation**](SyntheticsPrivateLocation.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_test

> crate::models::SyntheticsTestDetails get_test(public_id)
Get a test configuration

Get the detailed configuration associated with a Synthetics test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test to get details from. | [required] |

### Return type

[**crate::models::SyntheticsTestDetails**](SyntheticsTestDetails.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_locations

> crate::models::SyntheticsLocations list_locations()
Get all locations (public and private)

Get the list of public and private locations available for Synthetic tests. No arguments required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SyntheticsLocations**](SyntheticsLocations.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tests

> crate::models::SyntheticsListTestsResponse list_tests()
Get the list of all tests

Get the list of all Synthetic tests.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SyntheticsListTestsResponse**](SyntheticsListTestsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_ci_tests

> crate::models::SyntheticsTriggerCiTestsResponse trigger_ci_tests(body)
Trigger tests from CI/CD pipelines

Trigger a set of Synthetics tests for continuous integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SyntheticsCiTestBody**](SyntheticsCiTestBody.md) | Details of the test to trigger. | [required] |

### Return type

[**crate::models::SyntheticsTriggerCiTestsResponse**](SyntheticsTriggerCITestsResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_test

> crate::models::SyntheticsApiTest update_api_test(public_id, body)
Edit an API test

Edit the configuration of a Synthetic API test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test to get details from. | [required] |
**body** | [**SyntheticsApiTest**](SyntheticsApiTest.md) | New test details to be saved. | [required] |

### Return type

[**crate::models::SyntheticsApiTest**](SyntheticsAPITest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_browser_test

> crate::models::SyntheticsBrowserTest update_browser_test(public_id, body)
Edit a browser test

Edit the configuration of a Synthetic browser test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the test to get details from. | [required] |
**body** | [**SyntheticsBrowserTest**](SyntheticsBrowserTest.md) | New test details to be saved. | [required] |

### Return type

[**crate::models::SyntheticsBrowserTest**](SyntheticsBrowserTest.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_location

> crate::models::SyntheticsPrivateLocation update_private_location(location_id, body)
Edit a private location

Edit a Synthetics private location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | The ID of the private location. | [required] |
**body** | [**SyntheticsPrivateLocation**](SyntheticsPrivateLocation.md) | Details of the private location to be updated. | [required] |

### Return type

[**crate::models::SyntheticsPrivateLocation**](SyntheticsPrivateLocation.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_test_pause_status

> bool update_test_pause_status(public_id, body)
Pause or start a test

Pause or start a Synthetics test by changing the status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The public ID of the Synthetic test to update. | [required] |
**body** | [**SyntheticsUpdateTestPauseStatusPayload**](SyntheticsUpdateTestPauseStatusPayload.md) | Status to set the given Synthetic test to. | [required] |

### Return type

**bool**

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


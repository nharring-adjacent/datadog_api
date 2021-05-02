# \UsersApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **post** /api/v1/user | Create a user
[**disable_user**](UsersApi.md#disable_user) | **delete** /api/v1/user/{user_handle} | Disable a user
[**get_user**](UsersApi.md#get_user) | **get** /api/v1/user/{user_handle} | Get user details
[**list_users**](UsersApi.md#list_users) | **get** /api/v1/user | List all users
[**update_user**](UsersApi.md#update_user) | **put** /api/v1/user/{user_handle} | Update a user



## create_user

> crate::models::UserResponse create_user(body)
Create a user

Create a user for your organization.  **Note**: Users can only be created with the admin access role if application keys belong to administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**User**](User.md) | User object that needs to be created. | [required] |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user

> crate::models::UserDisableResponse disable_user(user_handle)
Disable a user

Delete a user from an organization.  **Note**: This endpoint can only be used with application keys belonging to administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_handle** | [**String**](.md) | The handle of the user. | [required] |

### Return type

[**crate::models::UserDisableResponse**](UserDisableResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::UserResponse get_user(user_handle)
Get user details

Get a user's details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_handle** | [**String**](.md) | The ID of the user. | [required] |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> crate::models::UserListResponse list_users()
List all users

List all users for your organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserListResponse**](UserListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::UserResponse update_user(user_handle, body)
Update a user

Update a user information.  **Note**: It can only be used with application keys belonging to administrators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_handle** | [**String**](.md) | The ID of the user. | [required] |
**body** | [**User**](User.md) | Description of the update. | [required] |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


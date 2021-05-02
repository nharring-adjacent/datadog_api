# \OrganizationsApi

All URIs are relative to *https://api.datadoghq.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_child_org**](OrganizationsApi.md#create_child_org) | **post** /api/v1/org | Create a child organization
[**get_org**](OrganizationsApi.md#get_org) | **get** /api/v1/org/{public_id} | Get organization information
[**list_orgs**](OrganizationsApi.md#list_orgs) | **get** /api/v1/org | List your managed organizations
[**update_org**](OrganizationsApi.md#update_org) | **put** /api/v1/org/{public_id} | Update your organization
[**upload_id_p_for_org**](OrganizationsApi.md#upload_id_p_for_org) | **post** /api/v1/org/{public_id}/idp_metadata | Upload IdP metadata



## create_child_org

> crate::models::OrganizationCreateResponse create_child_org(body)
Create a child organization

Create a child organization.  This endpoint requires the [multi-organization account](https://docs.datadoghq.com/account_management/multi_organization/) feature and must be enabled by [contacting support](https://docs.datadoghq.com/help/).  Once a new child organization is created, you can interact with it by using the `org.public_id`, `pi_key.key`, and `application_key.hash` provided in the response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OrganizationCreateBody**](OrganizationCreateBody.md) | Organization object that needs to be created | [required] |

### Return type

[**crate::models::OrganizationCreateResponse**](OrganizationCreateResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org

> crate::models::OrganizationResponse get_org(public_id)
Get organization information

Get organization information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The `public_id` of the organization you are operating within. | [required] |

### Return type

[**crate::models::OrganizationResponse**](OrganizationResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_orgs

> crate::models::OrganizationListResponse list_orgs()
List your managed organizations

List your managed organizations.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OrganizationListResponse**](OrganizationListResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org

> crate::models::OrganizationResponse update_org(public_id, body)
Update your organization

Update your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The `public_id` of the organization you are operating within. | [required] |
**body** | [**Organization**](Organization.md) |  | [required] |

### Return type

[**crate::models::OrganizationResponse**](OrganizationResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_id_p_for_org

> crate::models::IdpResponse upload_id_p_for_org(public_id, idp_file)
Upload IdP metadata

There are a couple of options for updating the Identity Provider (IdP) metadata from your SAML IdP.  * **Multipart Form-Data**: Post the IdP metadata file using a form post.  * **XML Body:** Post the IdP metadata file as the body of the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_id** | **String** | The `public_id` of the organization you are operating with | [required] |
**idp_file** | **std::path::PathBuf** | The path to the XML metadata file you wish to upload. | [required] |

### Return type

[**crate::models::IdpResponse**](IdpResponse.md)

### Authorization

[apiKeyAuth](../README.md#apiKeyAuth), [appKeyAuth](../README.md#appKeyAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


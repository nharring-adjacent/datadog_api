# OrganizationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**private_widget_share** | Option<**bool**> | Whether or not the organization users can share widgets outside of Datadog. | [optional]
**saml** | Option<[**crate::models::OrganizationSettingsSaml**](Organization_settings_saml.md)> |  | [optional]
**saml_autocreate_access_role** | Option<[**crate::models::AccessRole**](AccessRole.md)> |  | [optional]
**saml_autocreate_users_domains** | Option<[**crate::models::OrganizationSettingsSamlAutocreateUsersDomains**](Organization_settings_saml_autocreate_users_domains.md)> |  | [optional]
**saml_can_be_enabled** | Option<**bool**> | Whether or not SAML can be enabled for this organization. | [optional]
**saml_idp_endpoint** | Option<**String**> | Identity provider endpoint for SAML authentication. | [optional]
**saml_idp_initiated_login** | Option<[**crate::models::OrganizationSettingsSamlIdpInitiatedLogin**](Organization_settings_saml_idp_initiated_login.md)> |  | [optional]
**saml_idp_metadata_uploaded** | Option<**bool**> | Whether or not a SAML identity provider metadata file was provided to the Datadog organization. | [optional]
**saml_login_url** | Option<**String**> | URL for SAML logging. | [optional]
**saml_strict_mode** | Option<[**crate::models::OrganizationSettingsSamlStrictMode**](Organization_settings_saml_strict_mode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



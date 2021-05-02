# GcpAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_provider_x509_cert_url** | Option<**String**> | Should be `https://www.googleapis.com/oauth2/v1/certs`. | [optional]
**auth_uri** | Option<**String**> | Should be `https://accounts.google.com/o/oauth2/auth`. | [optional]
**automute** | Option<**bool**> | Silence monitors for expected GCE instance shutdowns. | [optional]
**client_email** | Option<**String**> | Your email found in your JSON service account key. | [optional]
**client_id** | Option<**String**> | Your ID found in your JSON service account key. | [optional]
**client_x509_cert_url** | Option<**String**> | Should be `https://www.googleapis.com/robot/v1/metadata/x509/<CLIENT_EMAIL>` where `<CLIENT_EMAIL>` is the email found in your JSON service account key. | [optional]
**errors** | Option<**Vec<String>**> | An array of errors. | [optional]
**host_filters** | Option<**String**> | Limit the GCE instances that are pulled into Datadog by using tags. Only hosts that match one of the defined tags are imported into Datadog. | [optional]
**private_key** | Option<**String**> | Your private key name found in your JSON service account key. | [optional]
**private_key_id** | Option<**String**> | Your private key ID found in your JSON service account key. | [optional]
**project_id** | Option<**String**> | Your Google Cloud project ID found in your JSON service account key. | [optional]
**token_uri** | Option<**String**> | Should be `https://accounts.google.com/o/oauth2/token`. | [optional]
**_type** | Option<**String**> | The value for service_account found in your JSON service account key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



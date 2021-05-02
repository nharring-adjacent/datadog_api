# SyntheticsSslCertificate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cipher** | Option<**String**> | Cipher used for the connection. | [optional]
**exponent** | Option<**f64**> | Exponent associated to the certificate. | [optional]
**ext_key_usage** | Option<**Vec<String>**> | Array of extensions and details used for the certificate. | [optional]
**fingerprint** | Option<**String**> | MD5 digest of the DER-encoded Certificate information. | [optional]
**fingerprint256** | Option<**String**> | SHA-1 digest of the DER-encoded Certificate information. | [optional]
**issuer** | Option<[**crate::models::SyntheticsSslCertificateIssuer**](SyntheticsSSLCertificate_issuer.md)> |  | [optional]
**modulus** | Option<**String**> | Modulus associated to the SSL certificate private key. | [optional]
**protocol** | Option<**String**> | TLS protocol used for the test. | [optional]
**serial_number** | Option<**String**> | Serial Number assigned by Symantec to the SSL certificate. | [optional]
**subject** | Option<[**crate::models::SyntheticsSslCertificateSubject**](SyntheticsSSLCertificate_subject.md)> |  | [optional]
**valid_from** | Option<**String**> | Date from which the SSL certificate is valid. | [optional]
**valid_to** | Option<**String**> | Date until which the SSL certificate is valid. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



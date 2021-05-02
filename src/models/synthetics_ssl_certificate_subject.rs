/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsSslCertificateSubject : Object describing the SSL certificate used for the test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsSslCertificateSubject {
    /// Country Name associated with the certificate.
    #[serde(rename = "C", skip_serializing_if = "Option::is_none")]
    pub C: Option<String>,
    /// Common Name that associated with the certificate.
    #[serde(rename = "CN", skip_serializing_if = "Option::is_none")]
    pub CN: Option<String>,
    /// Locality associated with the certificate.
    #[serde(rename = "L", skip_serializing_if = "Option::is_none")]
    pub L: Option<String>,
    /// Organization associated with the certificate.
    #[serde(rename = "O", skip_serializing_if = "Option::is_none")]
    pub O: Option<String>,
    /// Organizational Unit associated with the certificate.
    #[serde(rename = "OU", skip_serializing_if = "Option::is_none")]
    pub OU: Option<String>,
    /// State Or Province Name associated with the certificate.
    #[serde(rename = "ST", skip_serializing_if = "Option::is_none")]
    pub ST: Option<String>,
    /// Subject Alternative Name associated with the certificate.
    #[serde(rename = "altName", skip_serializing_if = "Option::is_none")]
    pub alt_name: Option<String>,
}

impl SyntheticsSslCertificateSubject {
    /// Object describing the SSL certificate used for the test.
    pub fn new() -> SyntheticsSslCertificateSubject {
        SyntheticsSslCertificateSubject {
            C: None,
            CN: None,
            L: None,
            O: None,
            OU: None,
            ST: None,
            alt_name: None,
        }
    }
}


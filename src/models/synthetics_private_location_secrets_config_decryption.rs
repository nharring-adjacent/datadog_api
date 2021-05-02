/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsPrivateLocationSecretsConfigDecryption : Private key for the private location.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationSecretsConfigDecryption {
    /// Private key for the private location.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl SyntheticsPrivateLocationSecretsConfigDecryption {
    /// Private key for the private location.
    pub fn new() -> SyntheticsPrivateLocationSecretsConfigDecryption {
        SyntheticsPrivateLocationSecretsConfigDecryption {
            key: None,
        }
    }
}



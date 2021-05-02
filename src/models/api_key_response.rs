/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiKeyResponse : An API key with its associated metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<Box<crate::models::ApiKey>>,
}

impl ApiKeyResponse {
    /// An API key with its associated metadata.
    pub fn new() -> ApiKeyResponse {
        ApiKeyResponse {
            api_key: None,
        }
    }
}



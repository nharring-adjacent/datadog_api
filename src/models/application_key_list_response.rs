/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationKeyListResponse : An application key response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKeyListResponse {
    /// Array of application keys.
    #[serde(rename = "application_keys", skip_serializing_if = "Option::is_none")]
    pub application_keys: Option<Vec<crate::models::ApplicationKey>>,
}

impl ApplicationKeyListResponse {
    /// An application key response.
    pub fn new() -> ApplicationKeyListResponse {
        ApplicationKeyListResponse {
            application_keys: None,
        }
    }
}



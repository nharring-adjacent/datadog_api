/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationKey : An application key with its associated metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationKey {
    /// Hash of an application key.
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Name of an application key.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Owner of an application key.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl ApplicationKey {
    /// An application key with its associated metadata.
    pub fn new() -> ApplicationKey {
        ApplicationKey {
            hash: None,
            name: None,
            owner: None,
        }
    }
}



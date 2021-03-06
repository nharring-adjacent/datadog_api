/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsPrivateLocation : Object containing information about the private location to create.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocation {
    /// Description of the private location.
    #[serde(rename = "description")]
    pub description: String,
    /// Unique identifier of the private location.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the private location.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Box<crate::models::SyntheticsPrivateLocationSecrets>>,
    /// Array of tags attached to the private location.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

impl SyntheticsPrivateLocation {
    /// Object containing information about the private location to create.
    pub fn new(description: String, name: String, tags: Vec<String>) -> SyntheticsPrivateLocation {
        SyntheticsPrivateLocation {
            description,
            id: None,
            name,
            secrets: None,
            tags,
        }
    }
}



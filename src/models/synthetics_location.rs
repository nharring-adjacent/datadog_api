/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsLocation : Synthetic location that can be used when creating or editing a test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsLocation {
    /// Unique identifier of the location.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the location.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SyntheticsLocation {
    /// Synthetic location that can be used when creating or editing a test.
    pub fn new() -> SyntheticsLocation {
        SyntheticsLocation {
            id: None,
            name: None,
        }
    }
}



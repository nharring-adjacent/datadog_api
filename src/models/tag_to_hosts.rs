/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TagToHosts : In this object, the key is the tag, the value is a list of host names that are reporting that tag.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagToHosts {
    /// A list of tags to apply to the host.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, Vec<String>>>,
}

impl TagToHosts {
    /// In this object, the key is the tag, the value is a list of host names that are reporting that tag.
    pub fn new() -> TagToHosts {
        TagToHosts {
            tags: None,
        }
    }
}



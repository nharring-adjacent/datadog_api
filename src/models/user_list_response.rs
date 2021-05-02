/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UserListResponse : Array of Datadog users for a given organization.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListResponse {
    /// Array of users.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::User>>,
}

impl UserListResponse {
    /// Array of Datadog users for a given organization.
    pub fn new() -> UserListResponse {
        UserListResponse {
            users: None,
        }
    }
}


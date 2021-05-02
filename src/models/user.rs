/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// User : Create, edit, and disable users.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "access_role", skip_serializing_if = "Option::is_none")]
    pub access_role: Option<crate::models::AccessRole>,
    /// The new disabled status of the user.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The new email of the user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user handle, must be a valid email.
    #[serde(rename = "handle", skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    /// Gravatar icon associated to the user.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// The name of the user.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not the user logged in Datadog at least once.
    #[serde(rename = "verified", skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
}

impl User {
    /// Create, edit, and disable users.
    pub fn new() -> User {
        User {
            access_role: None,
            disabled: None,
            email: None,
            handle: None,
            icon: None,
            name: None,
            verified: None,
        }
    }
}


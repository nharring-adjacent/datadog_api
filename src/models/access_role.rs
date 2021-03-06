/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AccessRole : The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).

/// The access role of the user. Options are **st** (standard user), **adm** (admin user), or **ro** (read-only user).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessRole {
    #[serde(rename = "st")]
    STANDARD,
    #[serde(rename = "adm")]
    ADMIN,
    #[serde(rename = "ro")]
    READ_ONLY,
    #[serde(rename = "ERROR")]
    ERROR,

}

impl ToString for AccessRole {
    fn to_string(&self) -> String {
        match self {
            Self::STANDARD => String::from("st"),
            Self::ADMIN => String::from("adm"),
            Self::READ_ONLY => String::from("ro"),
            Self::ERROR => String::from("ERROR"),
        }
    }
}





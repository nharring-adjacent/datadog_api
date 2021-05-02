/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardLayoutType : Layout type of the dashboard.

/// Layout type of the dashboard.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DashboardLayoutType {
    #[serde(rename = "ordered")]
    ORDERED,
    #[serde(rename = "free")]
    FREE,

}

impl ToString for DashboardLayoutType {
    fn to_string(&self) -> String {
        match self {
            Self::ORDERED => String::from("ordered"),
            Self::FREE => String::from("free"),
        }
    }
}





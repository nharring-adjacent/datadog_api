/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardReflowType : Reflow type for a **new dashboard layout** dashboard. Set this only when layout type is 'ordered'. If set to 'fixed', the dashboard expects all widgets to have a layout, and if it's set to 'auto', widgets should not have layouts.

/// Reflow type for a **new dashboard layout** dashboard. Set this only when layout type is 'ordered'. If set to 'fixed', the dashboard expects all widgets to have a layout, and if it's set to 'auto', widgets should not have layouts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DashboardReflowType {
    #[serde(rename = "auto")]
    AUTO,
    #[serde(rename = "fixed")]
    FIXED,

}

impl ToString for DashboardReflowType {
    fn to_string(&self) -> String {
        match self {
            Self::AUTO => String::from("auto"),
            Self::FIXED => String::from("fixed"),
        }
    }
}





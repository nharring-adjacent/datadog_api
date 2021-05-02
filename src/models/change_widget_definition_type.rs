/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ChangeWidgetDefinitionType : Type of the change widget.

/// Type of the change widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChangeWidgetDefinitionType {
    #[serde(rename = "change")]
    CHANGE,

}

impl ToString for ChangeWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::CHANGE => String::from("change"),
        }
    }
}




/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// FreeTextWidgetDefinitionType : Type of the free text widget.

/// Type of the free text widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FreeTextWidgetDefinitionType {
    #[serde(rename = "free_text")]
    FREE_TEXT,

}

impl ToString for FreeTextWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::FREE_TEXT => String::from("free_text"),
        }
    }
}





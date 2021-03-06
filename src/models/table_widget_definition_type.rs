/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// TableWidgetDefinitionType : Type of the table widget.

/// Type of the table widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableWidgetDefinitionType {
    #[serde(rename = "query_table")]
    QUERY_TABLE,

}

impl ToString for TableWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::QUERY_TABLE => String::from("query_table"),
        }
    }
}





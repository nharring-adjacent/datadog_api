/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ApmStatsQueryColumnType : Column properties.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApmStatsQueryColumnType {
    /// A user-assigned alias for the column.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "cell_display_mode", skip_serializing_if = "Option::is_none")]
    pub cell_display_mode: Option<crate::models::TableWidgetCellDisplayMode>,
    /// Column name.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::models::WidgetSort>,
}

impl ApmStatsQueryColumnType {
    /// Column properties.
    pub fn new(name: String) -> ApmStatsQueryColumnType {
        ApmStatsQueryColumnType {
            alias: None,
            cell_display_mode: None,
            name,
            order: None,
        }
    }
}



/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// GeomapWidgetDefinitionView : The view of the world that the map should render.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeomapWidgetDefinitionView {
    /// The 2-letter ISO code of a country to focus the map on. Or `WORLD`.
    #[serde(rename = "focus")]
    pub focus: String,
}

impl GeomapWidgetDefinitionView {
    /// The view of the world that the map should render.
    pub fn new(focus: String) -> GeomapWidgetDefinitionView {
        GeomapWidgetDefinitionView {
            focus,
        }
    }
}


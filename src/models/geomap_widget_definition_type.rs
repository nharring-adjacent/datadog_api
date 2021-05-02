/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// GeomapWidgetDefinitionType : Type of the geomap widget.

/// Type of the geomap widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GeomapWidgetDefinitionType {
    #[serde(rename = "geomap")]
    GEOMAP,

}

impl ToString for GeomapWidgetDefinitionType {
    fn to_string(&self) -> String {
        match self {
            Self::GEOMAP => String::from("geomap"),
        }
    }
}





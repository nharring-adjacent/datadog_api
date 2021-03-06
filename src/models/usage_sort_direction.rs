/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageSortDirection : The direction to sort by.

/// The direction to sort by.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageSortDirection {
    #[serde(rename = "desc")]
    DESC,
    #[serde(rename = "asc")]
    ASC,

}

impl ToString for UsageSortDirection {
    fn to_string(&self) -> String {
        match self {
            Self::DESC => String::from("desc"),
            Self::ASC => String::from("asc"),
        }
    }
}





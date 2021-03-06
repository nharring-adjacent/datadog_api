/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsApiTestType : Type of the Synthetic test, `api`.

/// Type of the Synthetic test, `api`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SyntheticsApiTestType {
    #[serde(rename = "api")]
    API,

}

impl ToString for SyntheticsApiTestType {
    fn to_string(&self) -> String {
        match self {
            Self::API => String::from("api"),
        }
    }
}





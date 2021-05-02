/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// SyntheticsApiTest : Object containing details about a Synthetic API test.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsApiTest {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::SyntheticsApiTestConfig>>,
    /// Array of locations used to run the test.
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,
    /// Notification message associated with the test.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The associated monitor ID.
    #[serde(rename = "monitor_id", skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<i64>,
    /// Name of the test.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::SyntheticsTestOptions>>,
    /// The public ID for the test.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::SyntheticsTestPauseStatus>,
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<crate::models::SyntheticsTestDetailsSubType>,
    /// Array of tags attached to the test.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::SyntheticsApiTestType>,
}

impl SyntheticsApiTest {
    /// Object containing details about a Synthetic API test.
    pub fn new() -> SyntheticsApiTest {
        SyntheticsApiTest {
            config: None,
            locations: None,
            message: None,
            monitor_id: None,
            name: None,
            options: None,
            public_id: None,
            status: None,
            subtype: None,
            tags: None,
            _type: None,
        }
    }
}



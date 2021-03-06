/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceLevelObjectiveRequest : A service level objective object includes a service level indicator, thresholds for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveRequest {
    /// A user-defined description of the service level objective.  Always included in service level objective responses (but may be `null`). Optional in create/update requests.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of (up to 20) monitor groups that narrow the scope of a monitor service level objective.  Included in service level objective responses if it is not empty. Optional in create/update requests for monitor service level objectives, but may only be used when then length of the `monitor_ids` field is one.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// A list of monitor ids that defines the scope of a monitor service level objective. **Required if type is `monitor`**.
    #[serde(rename = "monitor_ids", skip_serializing_if = "Option::is_none")]
    pub monitor_ids: Option<Vec<i64>>,
    /// The name of the service level objective object.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::models::ServiceLevelObjectiveQuery>>,
    /// A list of tags associated with this service level objective. Always included in service level objective responses (but may be empty). Optional in create/update requests.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The thresholds (timeframes and associated targets) for this service level objective object.
    #[serde(rename = "thresholds")]
    pub thresholds: Vec<crate::models::SloThreshold>,
    #[serde(rename = "type")]
    pub _type: crate::models::SloType,
}

impl ServiceLevelObjectiveRequest {
    /// A service level objective object includes a service level indicator, thresholds for one or more timeframes, and metadata (`name`, `description`, `tags`, etc.).
    pub fn new(name: String, thresholds: Vec<crate::models::SloThreshold>, _type: crate::models::SloType) -> ServiceLevelObjectiveRequest {
        ServiceLevelObjectiveRequest {
            description: None,
            groups: None,
            monitor_ids: None,
            name,
            query: None,
            tags: None,
            thresholds,
            _type,
        }
    }
}



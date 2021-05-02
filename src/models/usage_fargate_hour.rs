/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageFargateHour : Number of Fargate tasks run and hourly usage.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageFargateHour {
    /// The hour for the usage.
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// The number of Fargate tasks run.
    #[serde(rename = "tasks_count", skip_serializing_if = "Option::is_none")]
    pub tasks_count: Option<i64>,
}

impl UsageFargateHour {
    /// Number of Fargate tasks run and hourly usage.
    pub fn new() -> UsageFargateHour {
        UsageFargateHour {
            hour: None,
            tasks_count: None,
        }
    }
}



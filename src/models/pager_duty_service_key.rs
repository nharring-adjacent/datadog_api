/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// PagerDutyServiceKey : PagerDuty service object key.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagerDutyServiceKey {
    /// Your service key in PagerDuty.
    #[serde(rename = "service_key")]
    pub service_key: String,
}

impl PagerDutyServiceKey {
    /// PagerDuty service object key.
    pub fn new(service_key: String) -> PagerDutyServiceKey {
        PagerDutyServiceKey {
            service_key,
        }
    }
}



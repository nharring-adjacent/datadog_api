/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// ServiceLevelObjectiveQuery : A metric SLI query. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator to be used because this will sum up all request counts instead of averaging them, or taking the max or min of all of those requests.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLevelObjectiveQuery {
    /// A Datadog metric query for total (valid) events.
    #[serde(rename = "denominator")]
    pub denominator: String,
    /// A Datadog metric query for good events.
    #[serde(rename = "numerator")]
    pub numerator: String,
}

impl ServiceLevelObjectiveQuery {
    /// A metric SLI query. **Required if type is `metric`**. Note that Datadog only allows the sum by aggregator to be used because this will sum up all request counts instead of averaging them, or taking the max or min of all of those requests.
    pub fn new(denominator: String, numerator: String) -> ServiceLevelObjectiveQuery {
        ServiceLevelObjectiveQuery {
            denominator,
            numerator,
        }
    }
}


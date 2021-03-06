/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// AwsLogsServicesRequest : A list of current AWS services for which Datadog offers automatic log collection.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsLogsServicesRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Array of services IDs set to enable automatic log collection. Discover the list of available services with the get list of AWS log ready services API endpoint.
    #[serde(rename = "services")]
    pub services: Vec<String>,
}

impl AwsLogsServicesRequest {
    /// A list of current AWS services for which Datadog offers automatic log collection.
    pub fn new(account_id: String, services: Vec<String>) -> AwsLogsServicesRequest {
        AwsLogsServicesRequest {
            account_id,
            services,
        }
    }
}



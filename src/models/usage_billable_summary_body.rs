/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// UsageBillableSummaryBody : Response with properties for each aggregated usage type.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageBillableSummaryBody {
    /// The total account usage.
    #[serde(rename = "account_billable_usage", skip_serializing_if = "Option::is_none")]
    pub account_billable_usage: Option<i64>,
    /// Elapsed usage hours for some billable product.
    #[serde(rename = "elapsed_usage_hours", skip_serializing_if = "Option::is_none")]
    pub elapsed_usage_hours: Option<i64>,
    /// The first billable hour for the org.
    #[serde(rename = "first_billable_usage_hour", skip_serializing_if = "Option::is_none")]
    pub first_billable_usage_hour: Option<String>,
    /// The last billable hour for the org.
    #[serde(rename = "last_billable_usage_hour", skip_serializing_if = "Option::is_none")]
    pub last_billable_usage_hour: Option<String>,
    /// The number of units used within the billable timeframe.
    #[serde(rename = "org_billable_usage", skip_serializing_if = "Option::is_none")]
    pub org_billable_usage: Option<i64>,
    /// The percentage of account usage the org represents.
    #[serde(rename = "percentage_in_account", skip_serializing_if = "Option::is_none")]
    pub percentage_in_account: Option<f64>,
    /// Units pertaining to the usage.
    #[serde(rename = "usage_unit", skip_serializing_if = "Option::is_none")]
    pub usage_unit: Option<String>,
}

impl UsageBillableSummaryBody {
    /// Response with properties for each aggregated usage type.
    pub fn new() -> UsageBillableSummaryBody {
        UsageBillableSummaryBody {
            account_billable_usage: None,
            elapsed_usage_hours: None,
            first_billable_usage_hour: None,
            last_billable_usage_hour: None,
            org_billable_usage: None,
            percentage_in_account: None,
            usage_unit: None,
        }
    }
}



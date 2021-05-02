/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IpPrefixesWebhooks : Available prefix information for the Webhook endpoints.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpPrefixesWebhooks {
    /// List of IPv4 prefixes.
    #[serde(rename = "prefixes_ipv4", skip_serializing_if = "Option::is_none")]
    pub prefixes_ipv4: Option<Vec<String>>,
    /// List of IPv6 prefixes.
    #[serde(rename = "prefixes_ipv6", skip_serializing_if = "Option::is_none")]
    pub prefixes_ipv6: Option<Vec<String>>,
}

impl IpPrefixesWebhooks {
    /// Available prefix information for the Webhook endpoints.
    pub fn new() -> IpPrefixesWebhooks {
        IpPrefixesWebhooks {
            prefixes_ipv4: None,
            prefixes_ipv6: None,
        }
    }
}


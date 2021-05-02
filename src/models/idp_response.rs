/*
 * Datadog API V1 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// IdpResponse : The IdP response object.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdpResponse {
    /// Identity provider response.
    #[serde(rename = "message")]
    pub message: String,
}

impl IdpResponse {
    /// The IdP response object.
    pub fn new(message: String) -> IdpResponse {
        IdpResponse {
            message,
        }
    }
}



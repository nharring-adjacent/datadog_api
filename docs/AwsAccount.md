# AwsAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | Option<**String**> | Your AWS access key ID. Only required if your AWS account is a GovCloud or China account. | [optional]
**account_id** | Option<**String**> | Your AWS Account ID without dashes. | [optional]
**account_specific_namespace_rules** | Option<**::std::collections::HashMap<String, bool>**> | An object, (in the form `{\"namespace1\":true/false, \"namespace2\":true/false}`), that enables or disables metric collection for specific AWS namespaces for this AWS account only. | [optional]
**excluded_regions** | Option<**Vec<String>**> | An array of AWS regions to exclude from metrics collection. | [optional]
**filter_tags** | Option<**Vec<String>**> | The array of EC2 tags (in the form `key:value`) defines a filter that Datadog uses when collecting metrics from EC2. Wildcards, such as `?` (for single characters) and `*` (for multiple characters) can also be used. Only hosts that match one of the defined tags will be imported into Datadog. The rest will be ignored. Host matching a given tag can also be excluded by adding `!` before the tag. For example, `env:production,instance-type:c1.*,!region:us-east-1` | [optional]
**host_tags** | Option<**Vec<String>**> | Array of tags (in the form `key:value`) to add to all hosts and metrics reporting through this integration. | [optional]
**role_name** | Option<**String**> | Your Datadog role delegation name. | [optional]
**secret_access_key** | Option<**String**> | Your AWS secret access key. Only required if your AWS account is a GovCloud or China account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



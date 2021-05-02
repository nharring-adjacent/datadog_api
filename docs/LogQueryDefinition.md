# LogQueryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**compute** | Option<[**crate::models::LogsQueryCompute**](LogsQueryCompute.md)> |  | [optional]
**group_by** | Option<[**Vec<crate::models::LogQueryDefinitionGroupBy>**](LogQueryDefinitionGroupBy.md)> | List of tag prefixes to group by in the case of a cluster check. | [optional]
**index** | Option<**String**> | A coma separated-list of index names. Use \"*\" query all indexes at once. [Multiple Indexes](https://docs.datadoghq.com/logs/indexes/#multiple-indexes) | [optional]
**multi_compute** | Option<[**Vec<crate::models::LogsQueryCompute>**](LogsQueryCompute.md)> | This field is mutually exclusive with `compute`. | [optional]
**search** | Option<[**crate::models::LogQueryDefinitionSearch**](LogQueryDefinition_search.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# FormulaAndFunctionQueryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregator** | Option<[**crate::models::FormulaAndFunctionMetricAggregation**](FormulaAndFunctionMetricAggregation.md)> |  | [optional]
**data_source** | [**crate::models::FormulaAndFunctionProcessQueryDataSource**](FormulaAndFunctionProcessQueryDataSource.md) |  | 
**name** | **String** | Name of query for use in formulas. | 
**query** | **String** | Metrics query definition. | 
**compute** | [**crate::models::FormulaAndFunctionEventQueryDefinitionCompute**](FormulaAndFunctionEventQueryDefinition_compute.md) |  | 
**group_by** | Option<[**Vec<crate::models::FormulaAndFunctionEventQueryGroupBy>**](FormulaAndFunctionEventQueryGroupBy.md)> | Group by options. | [optional]
**indexes** | Option<**Vec<String>**> | An array of index names to query in the stream. Omit or use `[]` to query all indexes at once. | [optional]
**search** | Option<[**crate::models::FormulaAndFunctionEventQueryDefinitionSearch**](FormulaAndFunctionEventQueryDefinition_search.md)> |  | [optional]
**is_normalized_cpu** | Option<**bool**> | Whether to normalize the CPU percentages. | [optional]
**limit** | Option<**i64**> | Number of hits to return. | [optional]
**metric** | **String** | Process metric name. | 
**sort** | Option<[**crate::models::QuerySortOrder**](QuerySortOrder.md)> |  | [optional]
**tag_filters** | Option<**Vec<String>**> | An array of tags to filter by. | [optional]
**text_filter** | Option<**String**> | Text to use as filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



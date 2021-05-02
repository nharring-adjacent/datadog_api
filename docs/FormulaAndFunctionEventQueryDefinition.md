# FormulaAndFunctionEventQueryDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**compute** | [**crate::models::FormulaAndFunctionEventQueryDefinitionCompute**](FormulaAndFunctionEventQueryDefinition_compute.md) |  | 
**data_source** | [**crate::models::FormulaAndFunctionEventsDataSource**](FormulaAndFunctionEventsDataSource.md) |  | 
**group_by** | Option<[**Vec<crate::models::FormulaAndFunctionEventQueryGroupBy>**](FormulaAndFunctionEventQueryGroupBy.md)> | Group by options. | [optional]
**indexes** | Option<**Vec<String>**> | An array of index names to query in the stream. Omit or use `[]` to query all indexes at once. | [optional]
**name** | **String** | Name of the query for use in formulas. | 
**search** | Option<[**crate::models::FormulaAndFunctionEventQueryDefinitionSearch**](FormulaAndFunctionEventQueryDefinition_search.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



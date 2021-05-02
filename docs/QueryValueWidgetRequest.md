# QueryValueWidgetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregator** | Option<[**crate::models::WidgetAggregator**](WidgetAggregator.md)> |  | [optional]
**apm_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**conditional_formats** | Option<[**Vec<crate::models::WidgetConditionalFormat>**](WidgetConditionalFormat.md)> | List of conditional formats. | [optional]
**event_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**formulas** | Option<[**Vec<crate::models::WidgetFormula>**](WidgetFormula.md)> | List of formulas that operate on queries. **This feature is currently in beta.** | [optional]
**log_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**network_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**process_query** | Option<[**crate::models::ProcessQueryDefinition**](ProcessQueryDefinition.md)> |  | [optional]
**profile_metrics_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**q** | Option<**String**> | TODO. | [optional]
**queries** | Option<[**Vec<crate::models::FormulaAndFunctionQueryDefinition>**](FormulaAndFunctionQueryDefinition.md)> | List of queries that can be returned directly or used in formulas. **This feature is currently in beta.** | [optional]
**response_format** | Option<[**crate::models::FormulaAndFunctionResponseFormat**](FormulaAndFunctionResponseFormat.md)> |  | [optional]
**rum_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**security_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



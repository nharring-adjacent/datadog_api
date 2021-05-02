# TimeseriesWidgetRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apm_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**display_type** | Option<[**crate::models::WidgetDisplayType**](WidgetDisplayType.md)> |  | [optional]
**event_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**formulas** | Option<[**Vec<crate::models::WidgetFormula>**](WidgetFormula.md)> | List of formulas that operate on queries. **This feature is currently in beta.** | [optional]
**log_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**metadata** | Option<[**Vec<crate::models::TimeseriesWidgetExpressionAlias>**](TimeseriesWidgetExpressionAlias.md)> | Used to define expression aliases. | [optional]
**network_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**on_right_yaxis** | Option<**bool**> | Whether or not to display a second y-axis on the right. | [optional]
**process_query** | Option<[**crate::models::ProcessQueryDefinition**](ProcessQueryDefinition.md)> |  | [optional]
**profile_metrics_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**q** | Option<**String**> | Widget query. | [optional]
**queries** | Option<[**Vec<crate::models::FormulaAndFunctionQueryDefinition>**](FormulaAndFunctionQueryDefinition.md)> | List of queries that can be returned directly or used in formulas. **This feature is currently in beta.** | [optional]
**response_format** | Option<[**crate::models::FormulaAndFunctionResponseFormat**](FormulaAndFunctionResponseFormat.md)> |  | [optional]
**rum_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**security_query** | Option<[**crate::models::LogQueryDefinition**](LogQueryDefinition.md)> |  | [optional]
**style** | Option<[**crate::models::WidgetRequestStyle**](WidgetRequestStyle.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



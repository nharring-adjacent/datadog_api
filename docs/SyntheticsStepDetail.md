# SyntheticsStepDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**browser_errors** | Option<[**Vec<crate::models::SyntheticsBrowserError>**](SyntheticsBrowserError.md)> | Array of errors collected for a browser test. | [optional]
**check_type** | Option<[**crate::models::SyntheticsCheckType**](SyntheticsCheckType.md)> |  | [optional]
**description** | Option<**String**> | Description of the test. | [optional]
**duration** | Option<**f64**> | Total duration in millisecond of the test. | [optional]
**error** | Option<**String**> | Error returned by the test. | [optional]
**playing_tab** | Option<[**crate::models::SyntheticsPlayingTab**](SyntheticsPlayingTab.md)> |  | [optional]
**screenshot_bucket_key** | Option<**bool**> | Whether or not screenshots where collected by the test. | [optional]
**skipped** | Option<**bool**> | Whether or not to skip this step. | [optional]
**snapshot_bucket_key** | Option<**bool**> | Whether or not snapshots where collected by the test. | [optional]
**step_id** | Option<**i64**> | The step ID. | [optional]
**sub_test_step_details** | Option<[**Vec<crate::models::SyntheticsStepDetail>**](SyntheticsStepDetail.md)> | If this steps include a sub-test. [Subtests documentation](https://docs.datadoghq.com/synthetics/browser_tests/advanced_options/#subtests). | [optional]
**time_to_interactive** | Option<**f64**> | Time before starting the step. | [optional]
**_type** | Option<[**crate::models::SyntheticsStepType**](SyntheticsStepType.md)> |  | [optional]
**url** | Option<**String**> | URL to perform the step against. | [optional]
**value** | Option<[**serde_json::Value**](.md)> | Value for the step. | [optional]
**vitals_metrics** | Option<[**Vec<crate::models::SyntheticsCoreWebVitals>**](SyntheticsCoreWebVitals.md)> | Array of Core Web Vitals metrics for the step. | [optional]
**warnings** | Option<[**Vec<crate::models::SyntheticsStepDetailWarning>**](SyntheticsStepDetailWarning.md)> | Warning collected that didn't failed the step. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



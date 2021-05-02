# SyntheticsBrowserTestResultData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**browser_type** | Option<**String**> | Type of browser device used for the browser test. | [optional]
**browser_version** | Option<**String**> | Browser version used for the browser test. | [optional]
**device** | Option<[**crate::models::SyntheticsDevice**](SyntheticsDevice.md)> |  | [optional]
**duration** | Option<**f64**> | Global duration in second of the browser test. | [optional]
**error** | Option<**String**> | Error returned for the browser test. | [optional]
**passed** | Option<**bool**> | Whether or not the browser test was conducted. | [optional]
**received_email_count** | Option<**i64**> | The amount of email received during the browser test. | [optional]
**start_url** | Option<**String**> | Starting URL for the browser test. | [optional]
**step_details** | Option<[**Vec<crate::models::SyntheticsStepDetail>**](SyntheticsStepDetail.md)> | Array containing the different browser test steps. | [optional]
**thumbnails_bucket_key** | Option<**bool**> | Whether or not a thumbnail is associated with the browser test. | [optional]
**time_to_interactive** | Option<**f64**> | Time in second to wait before the browser test starts after reaching the start URL. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



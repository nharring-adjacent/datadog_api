# SloThreshold

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target** | **f64** | The target value for the service level indicator within the corresponding timeframe. | 
**target_display** | Option<**String**> | A string representation of the target that indicates its precision. It uses trailing zeros to show significant decimal places (e.g. `98.00`).  Always included in service level objective responses. Ignored in create/update requests. | [optional]
**timeframe** | [**crate::models::SloTimeframe**](SLOTimeframe.md) |  | 
**warning** | Option<**f64**> | The warning value for the service level objective. | [optional]
**warning_display** | Option<**String**> | A string representation of the warning target (see the description of the `target_display` field for details).  Included in service level objective responses if a warning target exists. Ignored in create/update requests. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



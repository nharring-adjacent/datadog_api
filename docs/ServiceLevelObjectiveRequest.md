# ServiceLevelObjectiveRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A user-defined description of the service level objective.  Always included in service level objective responses (but may be `null`). Optional in create/update requests. | [optional]
**groups** | Option<**Vec<String>**> | A list of (up to 20) monitor groups that narrow the scope of a monitor service level objective.  Included in service level objective responses if it is not empty. Optional in create/update requests for monitor service level objectives, but may only be used when then length of the `monitor_ids` field is one. | [optional]
**monitor_ids** | Option<**Vec<i64>**> | A list of monitor ids that defines the scope of a monitor service level objective. **Required if type is `monitor`**. | [optional]
**name** | **String** | The name of the service level objective object. | 
**query** | Option<[**crate::models::ServiceLevelObjectiveQuery**](ServiceLevelObjectiveQuery.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | A list of tags associated with this service level objective. Always included in service level objective responses (but may be empty). Optional in create/update requests. | [optional]
**thresholds** | [**Vec<crate::models::SloThreshold>**](SLOThreshold.md) | The thresholds (timeframes and associated targets) for this service level objective object. | 
**_type** | [**crate::models::SloType**](SLOType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



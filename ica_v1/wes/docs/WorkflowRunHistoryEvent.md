# WorkflowRunHistoryEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the event, such as the name of the step/task for state-level events and run name for run-level events | [optional]
**event_id** | Option<**i64**> | Identifier for the history event, if any | [optional]
**previous_event_id** | Option<**i64**> | Identifier for any previous history event (if available) | [optional]
**event_type** | Option<**String**> | Type of history event. The associated details entry will be populated based on the type of event. | [optional]
**timestamp** | Option<**String**> | Timestamp for the history event | [optional]
**event_details** | Option<[**serde_json::Value**](.md)> | Details for history event | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



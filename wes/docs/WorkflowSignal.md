# WorkflowSignal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique resource ID | [optional]
**urn** | Option<**String**> | URN of the resource | [optional]
**href** | Option<**String**> | HREF to the resource | [optional]
**send_heartbeat_href** | Option<**String**> | HREF to send a heartbeat to a workflow signal | [optional]
**send_success_response_href** | Option<**String**> | HREF to succeed a workflow signal | [optional]
**send_failure_response_href** | Option<**String**> | HREF to fail a workflow signal | [optional]
**name** | Option<**String**> | Unique name of the signal | [optional]
**status** | Option<**String**> | Current status of the signal | [optional]
**_type** | Option<**String**> | User-defined type associated with the signal | [optional]
**description** | Option<**String**> | Description of the signal | [optional]
**inputs** | Option<[**serde_json::Value**](.md)> | Inputs defined by the originating WaitForSignal state, in JSON. | [optional]
**workflow_run** | Option<[**crate::models::WorkflowRunCompact**](WorkflowRunCompact.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | Signal timeout in seconds. The Signal will timeout if a heartbeat, succeed or fail is not received in this time interval. | [optional]
**result** | Option<[**serde_json::Value**](.md)> | The result of a successful signalling action in JSON. | [optional]
**error** | Option<**String**> | The error of a failed signal. | [optional]
**error_cause** | Option<**String**> | The error cause of a failed signal. | [optional]
**created_by_client_id** | Option<**String**> | Client ID of the Origin Request | [optional]
**time_created** | Option<**String**> | Time (in UTC) the resource was created | [optional]
**time_modified** | Option<**String**> | Time (in UTC) the resource was modified | [optional]
**created_by** | Option<**String**> | User that created the resource | [optional]
**modified_by** | Option<**String**> | User that modified the resource | [optional]
**tenant_id** | Option<**String**> | Tenant ID the resource belongs to | [optional]
**acl** | Option<**Vec<String>**> | Access control list of the resource | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# WorkflowRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique resource ID | [optional]
**urn** | Option<**String**> | URN of the resource | [optional]
**href** | Option<**String**> | HREF to the resource | [optional]
**name** | Option<**String**> | Name of the workflow run | [optional]
**time_started** | Option<**String**> | The time (in UTC) the workflow run started | [optional]
**time_stopped** | Option<**String**> | The time (in UTC) the Workflow Run stopped | [optional]
**status** | Option<**String**> | Workflow run status | [optional]
**idempotency_key** | Option<**String**> |  | [optional]
**status_summary** | Option<**String**> | Workflow run status summary | [optional]
**error** | Option<**String**> | Error for a failed workflow run | [optional]
**error_cause** | Option<**String**> | Error cause for a failed workflow run | [optional]
**workflow_version** | Option<[**crate::models::WorkflowVersionCompact**](WorkflowVersionCompact.md)> |  | [optional]
**created_by_client_id** | Option<**String**> | Client ID of the Origin Request | [optional]
**input** | Option<[**serde_json::Value**](.md)> | Input to workflow run, as JSON | [optional]
**output** | Option<[**serde_json::Value**](.md)> | Output from workflow run, as JSON | [optional]
**definition** | Option<**String**> | Definition of the workflow version | [optional]
**engine_parameters** | Option<**String**> | Workflow Engine Parameters | [optional]
**time_created** | Option<**String**> | Time (in UTC) the resource was created | [optional]
**time_modified** | Option<**String**> | Time (in UTC) the resource was modified | [optional]
**created_by** | Option<**String**> | User that created the resource | [optional]
**modified_by** | Option<**String**> | User that modified the resource | [optional]
**tenant_id** | Option<**String**> | Tenant ID the resource belongs to | [optional]
**acl** | Option<**Vec<String>**> | Access control list of the resource | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



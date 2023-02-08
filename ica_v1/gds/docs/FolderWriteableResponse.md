# FolderWriteableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier for this Folder | [optional]
**name** | Option<**String**> | The name of this Folder | [optional]
**volume_id** | Option<**String**> | The unique identifier for this Folder's Volume | [optional]
**parent_folder_id** | Option<**String**> | The unique identifier for this folder's parent folder | [optional]
**volume_name** | Option<**String**> | The name of this Folder's Volume | [optional]
**tenant_id** | Option<**String**> | The unique identifier for this Folders's Tenant | [optional]
**sub_tenant_id** | Option<**String**> | The unique identifier for this Folder's Sub Tenant | [optional]
**urn** | Option<**String**> | The Universal Resource Name, unique to this Folder | [optional]
**path** | Option<**String**> | The (GDS) folder path to this Folder | [optional]
**acl** | Option<**Vec<String>**> | The list of directly specified Id(s) that have access to this Folder | [optional]
**inherited_acl** | Option<**Vec<String>**> | The inherited list of Id(s) that have access to this Folder | [optional]
**time_created** | Option<**String**> | The date & time this Folder was created, in GDS | [optional]
**created_by** | Option<**String**> | The creator of this Folder | [optional]
**time_modified** | Option<**String**> | The date & time this Folder was updated, in GDS | [optional]
**modified_by** | Option<**String**> | The updator of this Folder | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this folder | [optional]
**volume_metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this folder's volume | [optional]
**job_status** | Option<[**crate::models::JobStatus**](JobStatus.md)> |  | [optional]
**job_id** | Option<**String**> | The job identifier for the current folder operation. Currently only being used for the delete folder operation. | [optional]
**archive_job_storage_tier** | Option<[**crate::models::StorageTier**](StorageTier.md)> |  | [optional]
**object_store_access** | Option<[**crate::models::ObjectStoreAccess**](ObjectStoreAccess.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



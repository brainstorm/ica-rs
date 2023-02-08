# FileWriteableResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | A unique identifier for this File | [optional]
**name** | Option<**String**> | The name of this File | [optional]
**volume_id** | Option<**String**> | The unique identifier of the volume where the file resides | [optional]
**parent_folder_id** | Option<**String**> | The unique identifier of the folder where the file resides | [optional]
**volume_name** | Option<**String**> | The name of the volume where the file resides | [optional]
**volume_configuration_name** | Option<**String**> | The name of the volume configuration (BYOB only) | [optional]
**_type** | Option<**String**> | The type of the File | [optional]
**tenant_id** | Option<**String**> | The unique identifier for this File's Tenant | [optional]
**sub_tenant_id** | Option<**String**> | The unique identifier for this File's Sub Tenant | [optional]
**path** | Option<**String**> | The (GDS) path to this File | [optional]
**time_created** | Option<**String**> | The date & time this File was created, in GDS | [optional]
**created_by** | Option<**String**> | The creator of this File | [optional]
**time_modified** | Option<**String**> | The date & time this File was updated, in GDS | [optional]
**modified_by** | Option<**String**> | The updator of this File | [optional]
**inherited_acl** | Option<**Vec<String>**> | The inherited list of Id(s) that have access to this File | [optional]
**urn** | Option<**String**> | The Universal Resource Name, unique to this File | [optional]
**size_in_bytes** | Option<**i64**> | The File's Size in bytes | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this File | [optional]
**is_uploaded** | Option<**bool**> | The current upload state of the File | [optional]
**archive_status** | Option<[**crate::models::ArchiveStatuses**](ArchiveStatuses.md)> |  | [optional]
**time_archived** | Option<**String**> | The date & time this File was archived | [optional]
**storage_tier** | Option<[**crate::models::StorageTier**](StorageTier.md)> |  | [optional]
**e_tag** | Option<**String**> | The File's ETag | [optional]
**format** | Option<**String**> | The File's Format | [optional]
**format_edam** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::FileStatus**](FileStatus.md)> |  | [optional]
**life_cycle** | Option<[**crate::models::FileLifeCycleSettings**](FileLifeCycleSettings.md)> |  | [optional]
**object_store_access** | Option<[**crate::models::ObjectStoreAccess**](ObjectStoreAccess.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# CreateVolumeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**import_session_id** | Option<**String**> | Unique identifier of the import Session for this Volume. This only applies to Volumes created from custom  Volume configurations. | [optional]
**object_store_access** | Option<[**crate::models::ObjectStoreAccess**](ObjectStoreAccess.md)> |  | [optional]
**id** | Option<**String**> | A unique identifier for this Volume | [optional]
**name** | Option<**String**> | The name of this Volume | [optional]
**tenant_id** | Option<**String**> | The unique identifier for this Volume's Tenant | [optional]
**sub_tenant_id** | Option<**String**> | The unique identifier for this Volume's Sub Tenant | [optional]
**urn** | Option<**String**> | The Universal Resource Name, unique to this Volume | [optional]
**root_folder_id** | Option<**String**> | The unique identifier for the root Folder of this Volume | [optional]
**root_key_prefix** | Option<**String**> | The base bucket location for Volumes associated with custom VolumeConfigurations otherwise this field is not set. | [optional]
**volume_configuration_name** | Option<**String**> | Unique name of the Volume configuration for this Volume.  This field will only be set if a custom Volume configuration is associated. | [optional]
**inherited_acl** | Option<**Vec<String>**> | The inherited list of Id(s) that have access to this Volume | [optional]
**time_created** | Option<**String**> | The date & time this Volume was created, in GDS | [optional]
**created_by** | Option<**String**> | The creator of this Volume | [optional]
**time_modified** | Option<**String**> | The date & time this Volume was updated, in GDS | [optional]
**modified_by** | Option<**String**> | The updator of this Volume | [optional]
**job_status** | Option<[**crate::models::JobStatus**](JobStatus.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this Volume | [optional]
**life_cycle** | Option<[**crate::models::VolumeLifeCycleSettings**](VolumeLifeCycleSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



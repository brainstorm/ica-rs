# VolumeConfigurationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name for the volume configuration | [optional]
**tenant_id** | Option<**String**> | The unique identifier for this Volume Configuration's Tenant | [optional]
**sub_tenant_id** | Option<**String**> | The unique identifier for this Volume Configurations's Sub Tenant | [optional]
**urn** | Option<**String**> | The Universal Resource Name, unique to this Volume Configuration | [optional]
**online_status** | Option<[**crate::models::VolumeConfigurationOnlineStatus**](VolumeConfigurationOnlineStatus.md)> |  | [optional]
**error_code** | Option<**String**> | Error code returned from the object store | [optional]
**error_message** | Option<**String**> | Error message returned from the object store | [optional]
**time_of_last_error** | Option<**String**> | Timestamp of the last observed error. | [optional]
**time_created** | Option<**String**> | The date & time this Volume was created, in GDS | [optional]
**created_by** | Option<**String**> | The creator of this Volume | [optional]
**time_modified** | Option<**String**> | The date & time this Volume was updated, in GDS | [optional]
**modified_by** | Option<**String**> | The updator of this Volume | [optional]
**object_store_settings** | Option<[**crate::models::ObjectStoreSettings**](ObjectStoreSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



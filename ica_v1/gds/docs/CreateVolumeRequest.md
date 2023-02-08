# CreateVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name for the volume | 
**volume_configuration_name** | Option<**String**> | Unique name of the volume configuration to use | [optional]
**root_key_prefix** | Option<**String**> | The base bucket location for volumes associated with custom VolumeConfigurations. If not provided, the given volume Name is used.  If provided, it must start with the VolumeConfiguration's keyprefix and end with a /.  To create a volume representing the root of a bucket, use the value '/'. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata about this volume and its contents | [optional]
**life_cycle** | Option<[**crate::models::VolumeLifeCycleSettings**](VolumeLifeCycleSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



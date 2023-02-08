# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**urn** | Option<**String**> | The URN of the project. The format is urn:ilmn:ica:\\<type of the object\\>:\\<ID of the object\\>#\\<optional human readable hint representing the object\\>. The hint can be omitted, in that case the hashtag (#) must also be omitted. | [optional]
**name** | **String** |  | 
**active** | **bool** | Indicates whether the project is active or hidden. | 
**base_enabled** | Option<**bool**> | Indicates whether the project is base enabled. | [optional]
**short_description** | Option<**String**> |  | [optional]
**information** | Option<**String**> | Information about the project. Note that the value of this field can be arbitrary large. | [optional]
**region** | [**crate::models::Region**](Region.md) |  | 
**billing_mode** | **String** | The billing mode of the project. It determines who pays for the costs linked to the project. | 
**data_sharing_enabled** | Option<**bool**> | Indicates whether the Data and Samples created in this Project can be linked to other Projects. | [optional]
**tags** | [**crate::models::ProjectTag**](ProjectTag.md) |  | 
**storage_bundle** | Option<[**crate::models::StorageBundle**](StorageBundle.md)> |  | [optional]
**self_managed_storage_configuration** | Option<[**crate::models::StorageConfiguration**](StorageConfiguration.md)> |  | [optional]
**analysis_priority** | Option<**String**> | Indicates the priority given to a project and its analyses within a single tenant. Note that for a PUT call, when not providing a value for this attribute (null value or absent attribute), the persisted value will not change. | [optional]
**metadata_model** | Option<[**crate::models::MetadataModel**](MetadataModel.md)> |  | [optional]
**application** | Option<[**crate::models::Application**](Application.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



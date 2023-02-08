# DataDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**creator_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**owning_project_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**owning_project_name** | Option<**String**> |  | [optional]
**name** | **String** | The name of the file/folder as it was uploaded. | 
**path** | Option<**String**> | The user friendly path of the parent of this data. | [optional]
**file_size_in_bytes** | Option<**i64**> | The size of the file in bytes. Folders do not have a size. | [optional]
**status** | **String** |  | 
**tags** | [**crate::models::DataTag**](DataTag.md) |  | 
**format** | Option<[**crate::models::DataFormat**](DataFormat.md)> |  | [optional]
**data_type** | **String** |  | 
**object_e_tag** | Option<**String**> | The file's ETag, as received from the cloud provider. Not to be confused with the ETag reponse header of this API. | [optional]
**stored_for_the_first_time_at** | Option<**String**> | Specifies when the data object was stored for the first time | [optional]
**region** | Option<[**crate::models::Region**](Region.md)> |  | [optional]
**will_be_archived_at** | Option<**String**> | Specifies when the data object will be archived. | [optional]
**will_be_deleted_at** | Option<**String**> | Specifies when the data object will be deleted. | [optional]
**sequencing_run** | Option<[**crate::models::SequencingRun**](SequencingRun.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# Sample

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**time_created** | **String** |  | 
**time_modified** | **String** |  | 
**owner_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**tenant_name** | Option<**String**> |  | [optional]
**name** | **String** | The name of the sample | 
**description** | Option<**String**> | The description of the sample | [optional]
**tags** | [**crate::models::SampleTag**](SampleTag.md) |  | 
**region** | [**crate::models::Region**](Region.md) |  | 
**status** | **String** |  | 
**metadata_valid** | **bool** | Whether the metadata is valid | 
**metadata** | [**Vec<crate::models::MetadataField>**](MetadataField.md) | The metadata of the sample | 
**sequencing_runs** | Option<[**Vec<crate::models::SequencingRun>**](SequencingRun.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



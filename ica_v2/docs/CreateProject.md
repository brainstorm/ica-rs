# CreateProject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**short_description** | Option<**String**> |  | [optional]
**information** | Option<**String**> | Information about the project. Note that the value of this field can be arbitrary large. | [optional]
**project_owner_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Owner of the project. Defaults to the current user. | [optional]
**region_id** | [**uuid::Uuid**](uuid::Uuid.md) | The region of the project. All data and pipeline executions will reside in this region. | 
**billing_mode** | **String** | The billing mode of the project. It determines who pays for the costs linked to the project. | 
**data_sharing_enabled** | **bool** | Indicates whether the Data and Samples created in this Project can be linked to other Projects. | 
**tags** | Option<[**crate::models::ProjectTag**](ProjectTag.md)> |  | [optional]
**storage_bundle_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**metadata_model_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**storage_configuration_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | An optional storage configuration id to have self managed storage. | [optional]
**storage_configuration_subfolder** | Option<**String**> | Required when specifying a storageConfigurationId. The subfolder determines the object prefix of your self managed storage. | [optional]
**analysis_priority** | Option<**String**> | Indicates the priority given to a project and its analyses within a single tenant, where Medium is the default value. | [optional][default to Medium]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



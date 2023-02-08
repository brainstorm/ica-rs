# FindProjectSamples

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<crate::models::FindSampleCondition>**](FindSampleCondition.md) | Adds a condition on a string field. | 
**date_conditions** | [**Vec<crate::models::FindSampleDateCondition>**](FindSampleDateCondition.md) | Adds a condition on a date metadate field. If both the dateBefore and dateAfter parameter are null it will return any sample that has no value for the date field. | 
**number_conditions** | [**Vec<crate::models::FindSampleNumberCondition>**](FindSampleNumberCondition.md) | Adds a condition on a number metadata field. If both the lowerBoundary and upperBoundary parameter are null it will return any sample that has no value for the number field. | 
**boolean_conditions** | [**Vec<crate::models::FindSampleBooleanCondition>**](FindSampleBooleanCondition.md) | Adds a condition on a boolean field. | 
**full_text_search_string** | Option<**String**> | Adds a fuzzy matching condition for the text on all string fields of the sample i.e. on both the fixed fields (name, description) as any metadata text field. | [optional]
**include_deleted** | Option<**bool**> | Indicates whether deleted samples should be included. | [optional][default to false]
**user_tags** | Option<**Vec<String>**> | The usertags to filter on. The userTagMatchMode-parameter determines how the filtering is done. | [optional]
**user_tag_match_mode** | Option<**String**> | How the usertags are filtered. | [optional]
**run_input_tags** | Option<**Vec<String>**> | The runInputTags to filter on. The runInputTagMatchMode-parameter determines how the filtering is done. | [optional]
**run_input_tag_match_mode** | Option<**String**> | How the runInputTags are filtered. | [optional]
**connector_tags** | Option<**Vec<String>**> | The connectorTags to filter on. The connectorTagMatchMode-parameter determines how the filtering is done. | [optional]
**connector_tag_match_mode** | Option<**String**> | How the connectorTags are filtered. | [optional]
**tech_tags** | Option<**Vec<String>**> | The technicalTags to filter on. The techTagMatchMode-parameter determines how the filtering is done. | [optional]
**tech_tag_match_mode** | Option<**String**> | How the technicalTags are filtered. | [optional]
**instrument_run_ids** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



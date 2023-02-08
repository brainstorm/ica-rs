# LoadDataInBaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_jagged_rows** | Option<**bool**> | Enable to accept rows that are missing trailing optional columns. Missing values will be treated as nulls. | [optional][default to false]
**allow_quoted_newlines** | Option<**bool**> | Enable to include newlines contained in quoted data sections in the cell’s value. When disabled, newlines will signal a new row | [optional][default to false]
**data_id** | **String** | ID of the data to load into the table | 
**delimiter** | Option<**String**> | field delimiter | [optional][default to ,]
**encoding** | Option<**String**> | Encoding | [optional][default to Utf8]
**force_load** | Option<**bool**> | When false (default): the data will not be loaded if it was already previously loaded to table ; when true, the data will be loaded even if already loaded in the past | [optional][default to false]
**header_rows_to_skip** | Option<**i32**> | number of rows to skip (usually for headers) | [optional][default to 1]
**ignore_unknown_values** | Option<**bool**> | When enabled, rows with extra column values that do not match the schema will be ignored and will not be loaded into the table | [optional][default to false]
**include_references** | Option<**bool**> | Include references | [optional][default to true]
**include_data_reference** | Option<**bool**> | Include Data Reference | [optional][default to true]
**include_sample_reference** | Option<**bool**> | Include Sample Reference | [optional][default to true]
**include_pipeline_reference** | Option<**bool**> | Include Pipeline Reference | [optional][default to true]
**include_pipeline_execution_reference** | Option<**bool**> | Include Pipeline Execution Reference | [optional][default to true]
**include_tenant_reference** | Option<**bool**> | Include Tenant Reference | [optional][default to true]
**null_marker** | Option<**String**> | Specifies a string that represents a null value in a CSV/TSV file. | [optional]
**number_of_errors_allowed** | Option<**i32**> | The maximum number of bad records that Base can ignore when running the job | [optional][default to 0]
**quote** | Option<**String**> | The value that is used to quote data sections in a CSV/TSV file | [optional]
**write_preference** | Option<**String**> | specifies how to write data in the table. | [optional][default to Appendtotable]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



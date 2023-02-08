/*
 * ICA Rest API
 *
 * This API can be used to interact with Illumina Connected Analytics.<br> <p> Authentication to the  API can be done in multiple ways:<br> <ul><li>For the entire API, except for the POST /tokens endpoint: API-key + JWT</li> <li>Only for the POST /tokens endpoint: API-key + Basic Authentication</li></ul> </p> <p> <b>API-key</b><br> API keys are managed within the Illumina portal where you can manage your profile after you have logged on. The API-key has to be provided in the X-API-Key header parameter when executing API calls to ICA. In the background, a JWT will be requested at the IDP of Illumina to create a session. A good practice is to not use the API-key for every API call, but to first generate a JWT and to use that for authentication in subsequent calls.<br> </p> <p> <b>JWT</b><br> To avoid using an API-key for each call, we recommend to request a JWT via the POST /tokens endpoint  using this API-key. The JWT will expire after a pre-configured period specified by a tenant administrator through the IAM console in the Illumina portal. The JWT is the preferred way for authentication.<br>A not yet expired, still valid JWT could be refreshed using the POST /tokens:refresh endpoint.<br> </p> <p> <b>Basic Authentication</b><br> Basic authentication is only supported by the POST /tokens endpoint for generating a JWT. Use \"Basic base64encoded(emailaddress:password)\" in the \"Authorization\" header parameter for this authentication method. In case having access to multiple tenants using the same email-address, also provide the \"tenant\" request parameter to indicate what tenant you would like to request a JWT for. </p> 
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateProject {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "shortDescription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<Option<String>>,
    /// Information about the project. Note that the value of this field can be arbitrary large.
    #[serde(rename = "information", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub information: Option<Option<String>>,
    /// Owner of the project. Defaults to the current user.
    #[serde(rename = "projectOwnerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_owner_id: Option<Option<uuid::Uuid>>,
    /// The region of the project. All data and pipeline executions will reside in this region.
    #[serde(rename = "regionId")]
    pub region_id: uuid::Uuid,
    /// The billing mode of the project. It determines who pays for the costs linked to the project.
    #[serde(rename = "billingMode")]
    pub billing_mode: BillingMode,
    /// Indicates whether the Data and Samples created in this Project can be linked to other Projects.
    #[serde(rename = "dataSharingEnabled")]
    pub data_sharing_enabled: bool,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Box<crate::models::ProjectTag>>,
    #[serde(rename = "storageBundleId")]
    pub storage_bundle_id: uuid::Uuid,
    #[serde(rename = "metadataModelId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata_model_id: Option<Option<uuid::Uuid>>,
    /// An optional storage configuration id to have self managed storage.
    #[serde(rename = "storageConfigurationId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub storage_configuration_id: Option<Option<uuid::Uuid>>,
    /// Required when specifying a storageConfigurationId. The subfolder determines the object prefix of your self managed storage.
    #[serde(rename = "storageConfigurationSubfolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub storage_configuration_subfolder: Option<Option<String>>,
    /// Indicates the priority given to a project and its analyses within a single tenant, where Medium is the default value.
    #[serde(rename = "analysisPriority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub analysis_priority: Option<Option<AnalysisPriority>>,
}

impl CreateProject {
    pub fn new(name: String, region_id: uuid::Uuid, billing_mode: BillingMode, data_sharing_enabled: bool, storage_bundle_id: uuid::Uuid) -> CreateProject {
        CreateProject {
            name,
            short_description: None,
            information: None,
            project_owner_id: None,
            region_id,
            billing_mode,
            data_sharing_enabled,
            tags: None,
            storage_bundle_id,
            metadata_model_id: None,
            storage_configuration_id: None,
            storage_configuration_subfolder: None,
            analysis_priority: None,
        }
    }
}

/// The billing mode of the project. It determines who pays for the costs linked to the project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BillingMode {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "TENANT")]
    Tenant,
}

impl Default for BillingMode {
    fn default() -> BillingMode {
        Self::Project
    }
}
/// Indicates the priority given to a project and its analyses within a single tenant, where Medium is the default value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisPriority {
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
}

impl Default for AnalysisPriority {
    fn default() -> AnalysisPriority {
        Self::Low
    }
}

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
pub struct Bundle {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "timeModified")]
    pub time_modified: String,
    #[serde(rename = "ownerId")]
    pub owner_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(
        rename = "tenantName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub tenant_name: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(
        rename = "shortDescription",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_description: Option<Option<String>>,
    #[serde(rename = "region")]
    pub region: Box<crate::models::Region>,
    #[serde(
        rename = "metadataModel",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_model: Option<Option<Box<crate::models::MetadataModel>>>,
    #[serde(rename = "releaseVersion")]
    pub release_version: String,
    #[serde(
        rename = "versionComment",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_comment: Option<Option<String>>,
    #[serde(rename = "status")]
    pub status: Status,
    /// category tags as string array
    #[serde(
        rename = "categories",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub categories: Option<Option<Vec<String>>>,
    #[serde(
        rename = "links",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub links: Option<Option<Box<crate::models::Links>>>,
}

impl Bundle {
    pub fn new(
        id: uuid::Uuid,
        time_created: String,
        time_modified: String,
        owner_id: uuid::Uuid,
        tenant_id: uuid::Uuid,
        name: String,
        region: crate::models::Region,
        release_version: String,
        status: Status,
    ) -> Bundle {
        Bundle {
            id,
            time_created,
            time_modified,
            owner_id,
            tenant_id,
            tenant_name: None,
            name,
            short_description: None,
            region: Box::new(region),
            metadata_model: None,
            release_version,
            version_comment: None,
            status,
            categories: None,
            links: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "RELEASED")]
    Released,
    #[serde(rename = "DEPRECATED")]
    Deprecated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Draft
    }
}

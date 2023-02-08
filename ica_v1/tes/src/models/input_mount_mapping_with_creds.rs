/*
 * Task Execution Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InputMountMappingWithCreds {
    #[serde(rename = "storageProvider", skip_serializing_if = "Option::is_none")]
    pub storage_provider: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "urn", skip_serializing_if = "Option::is_none")]
    pub urn: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl InputMountMappingWithCreds {
    pub fn new() -> InputMountMappingWithCreds {
        InputMountMappingWithCreds {
            storage_provider: None,
            credentials: None,
            path: None,
            url: None,
            urn: None,
            mode: None,
            _type: None,
        }
    }
}
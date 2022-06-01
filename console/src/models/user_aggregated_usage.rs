/*
 * Developer Console Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserAggregatedUsage {
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::User>>,
    #[serde(rename = "iCredit", skip_serializing_if = "Option::is_none")]
    pub i_credit: Option<f64>,
    #[serde(rename = "usages", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Vec<crate::models::ProductUsage>>,
}

impl UserAggregatedUsage {
    pub fn new() -> UserAggregatedUsage {
        UserAggregatedUsage {
            user: None,
            i_credit: None,
            usages: None,
        }
    }
}



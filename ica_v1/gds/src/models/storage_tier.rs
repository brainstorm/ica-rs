/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// StorageTier : StorageTier

/// StorageTier
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Standard")]
    Standard,
    #[serde(rename = "Archive")]
    Archive,
    #[serde(rename = "DeepArchive")]
    DeepArchive,
}

impl ToString for StorageTier {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Standard => String::from("Standard"),
            Self::Archive => String::from("Archive"),
            Self::DeepArchive => String::from("DeepArchive"),
        }
    }
}

impl Default for StorageTier {
    fn default() -> StorageTier {
        Self::None
    }
}
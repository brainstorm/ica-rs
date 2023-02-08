/*
 * Genomic Data Store Service
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

/// ArchiveStatuses : The valid Archive Status values for files in GDS

/// The valid Archive Status values for files in GDS
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ArchiveStatuses {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Archived")]
    Archived,
    #[serde(rename = "Unarchiving")]
    Unarchiving,
}

impl ToString for ArchiveStatuses {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Archived => String::from("Archived"),
            Self::Unarchiving => String::from("Unarchiving"),
        }
    }
}

impl Default for ArchiveStatuses {
    fn default() -> ArchiveStatuses {
        Self::None
    }
}
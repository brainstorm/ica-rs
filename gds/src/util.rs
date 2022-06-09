use dirs::home_dir;
use std::fs::File;
use url::{ParseError, Url};

use thiserror::Error;

use crate::apis::configuration::ApiKey;
use crate::apis::configuration::Configuration;
use crate::apis::files_api::GetFileError;
use crate::apis::files_api::{list_files, ListFilesError};
use crate::apis::volumes_api::{get_volume, GetVolumeError};
use crate::models::FileListResponse;
use crate::models::VolumeResponse;

#[derive(Error, Debug)]
pub enum GDSError {
    #[error("The returned presigned URL is invalid")]
    ParseError(#[from] ParseError),
    #[error("The returned file ID is invalid")]
    FilesListResponseError(#[from] crate::apis::Error<ListFilesError>),
    #[error("GDS GetFile Error")]
    GetFileError(#[from] crate::apis::Error<GetFileError>),
    #[error("GDS GetVolume Error")]
    GetVolumeError(#[from] crate::apis::Error<GetVolumeError>),
    #[error("GDS session YAML file is invalid")]
    InvalidSessionYamlError(#[from] serde_yaml::Error),
    #[error("GDS session YAML file cannot be opened")]
    SessionYamlError(#[from] std::io::Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct IcaConfig {
    #[serde(rename(deserialize = "access-token"))]
    access_token: String,
    #[serde(rename(deserialize = "session-token"))]
    session_token: String,
}

#[derive(Debug)]
pub struct GdsUrl {
    pub volume: String,
    pub path: String,
}

/// Sets up the GDS endpoints and keys to provide access to the GDS API.
pub async fn setup_conf() -> Result<Configuration, GDSError> {
    let mut conf = Configuration::default();
    let key = read_access_token().await?;
    let apikey = ApiKey {
        prefix: Some("Bearer".to_string()),
        key,
    };
    conf.api_key = Some(apikey);
    Ok(conf)
}

/// Given a volume string, returns the volume.id
pub async fn gds_volume_to_volume_id(
    conf: &Configuration,
    volume: &str,
) -> Result<VolumeResponse, GDSError> {
    Ok(get_volume(&conf, volume, None, None, None).await?)
}

/// Separates volume and path from a GDS URL. gds://volume/<path...>
pub async fn gds_url_to_volume_and_path(url: &str) -> Result<GdsUrl, GDSError> {
    let volume = Url::parse(url)?.host_str().unwrap().to_string();
    let path = Url::parse(url)?.path().to_string();
    Ok(GdsUrl { volume, path })
}

/// Read the access token from the ICA session YAML file
pub async fn read_access_token() -> Result<String, GDSError> {
    if (std::option_env!("GDS_ACCESS_TOKEN")).is_some() {
        return Ok(std::option_env!("GDS_ACCESS_TOKEN").unwrap().to_string());
    } else {
        // In which region are we? Infer from AWS side.
        let gds_region = match std::option_env!("AWS_DEFAULT_REGION").or(Some("AWS_REGION")) {
            Some("ap-southeast-2") => "aps2".to_string(),
            Some("ap-northeast-1") => "apn1".to_string(),
            Some("us-east-1") => "use1".to_string(),
            Some("us-west-1") => "usw1".to_string(),
            Some("us-west-2") => "usw2".to_string(),
            Some("eu-west-1") => "euw1".to_string(),
            Some("eu-central-1") => "euc1".to_string(),
            Some("ap-southeast-1") => "aps1".to_string(),
            Some("ap-northeast-2") => "apn2".to_string(),
            Some("eu-west-2") => "euw2".to_string(),
            Some("sa-east-1") => "sae1".to_string(),
            Some("us-east-2") => "use2".to_string(),
            None => todo!(),
            Some(&_) => todo!(),
        };
        let f = File::open(
            home_dir().unwrap().to_str().unwrap().to_owned()
                + format!("/.ica/.session.{}.yaml", gds_region).as_str(),
        )?;
        let access_token: Result<String, serde_yaml::Error> = serde_yaml::from_reader(f);
        return Ok(access_token?);
    }
}

/// Returns a list of files in the given GDS path .
pub async fn gds_urls_to_file_ids(
    conf: &Configuration,
    volume_id: Vec<String>,
    gds_urls: Vec<String>,
) -> Result<FileListResponse, GDSError> {
    Ok(list_files(
        &conf,
        Some(volume_id),
        None,
        Some(gds_urls),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?)
}

/// Returns a (AWS S3) presigned URL from gds:// URL directly, without many of intermediate steps visible to the user.
pub async fn presigned_url(gds: &str) -> Result<Url, GDSError> {
    let config = setup_conf().await?;
    let input_gds_url = gds_url_to_volume_and_path(gds.as_ref()).await;
    let volume_ids =
        vec![gds_volume_to_volume_id(&config, &input_gds_url.as_ref().unwrap().volume).await];
    let gds_urls = vec![input_gds_url.unwrap().path];

    // TODO: Disambiguate which file_ids to get
    // TODO: This means that only a GDS path involving a file should be passed, no paths with several file_ids are supported... yet
    let first_id_in_volume = volume_ids[0].as_ref().unwrap().id.as_ref().unwrap().clone();
    let file_ids = gds_urls_to_file_ids(&config, vec![first_id_in_volume], gds_urls).await?;
    Ok(Url::parse(
        file_ids.items.unwrap()[0]
            .presigned_url
            .as_ref()
            .unwrap()
            .as_str(),
    )?)
}
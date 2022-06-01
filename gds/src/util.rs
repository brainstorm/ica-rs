use dirs::home_dir;
use std::fs::File;
use url::{Url, ParseError};

use crate::apis::Error;
use crate::apis::configuration::ApiKey;
use crate::apis::configuration::Configuration;
use crate::apis::volumes_api::{get_volume, GetVolumeError};
use crate::apis::files_api::{list_files, get_file, ListFilesError};
use crate::models::FileListResponse;
use crate::models::VolumeResponse;

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
    pub path: String
}

pub async fn setup_conf() -> Configuration {
    let mut conf = Configuration::default();
    let key = read_access_token().await;
    let apikey = ApiKey {
        prefix: Some("Bearer".to_string()),
        key,
    };
    conf.api_key = Some(apikey);
    conf
}

pub async fn gds_volume_to_volume_id(conf: &Configuration, volume: &str) -> Result<VolumeResponse, Error<GetVolumeError>> {
    get_volume(&conf, volume, None, None, None).await
}

pub async fn gds_url_to_volume_and_path(url: &str) -> Result<GdsUrl, ParseError> {
    let volume = Url::parse(url)?.host_str().unwrap().to_string();
    let path = Url::parse(url)?.path().to_string();
    Ok(GdsUrl{ volume, path })
    
}

pub async fn read_access_token() -> String {
    // TODO: env_vars
    let f =
        File::open(home_dir().unwrap().to_str().unwrap().to_owned() + "/.ica/.session.aps2.yaml")
            .expect("Cannot open file");
    let ica: IcaConfig = serde_yaml::from_reader(f).expect("Could not read values.");
    ica.access_token
}

pub async fn gds_urls_to_file_ids(conf: &Configuration, volume_id: Vec<String>, gds_urls: Vec<String>) -> Result<FileListResponse, Error<ListFilesError>> {
    list_files(&conf, Some(volume_id), None, Some(gds_urls), None, None, None, None, None, None, None, None, None, None).await
}

pub async fn get_presigned_url(conf: &Configuration, file_id: Option<&String>) -> String {
    get_file(
        &conf,
        &file_id.unwrap(),
        None,
        None,
        None,
        None,
        None,
    )
    .await.unwrap().presigned_url.unwrap()
}
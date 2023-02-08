use serde::{Deserialize, Serialize};
use url::{Url, ParseError};

use ica_gds::apis::Error;
use ica_gds::apis::configuration::ApiKey;
use ica_gds::apis::configuration::Configuration;
use ica_gds::apis::volumes_api::{get_volume, GetVolumeError};
use ica_gds::apis::files_api::{list_files, get_file, ListFilesError};
use ica_gds::models::FileListResponse;
use ica_gds::models::VolumeResponse;
use ica_gds::util::{read_access_token};

#[derive(Debug, Serialize, Deserialize)]
struct IcaConfig {
    #[serde(rename(deserialize = "access-token"))]
    access_token: String,
    #[serde(rename(deserialize = "session-token"))]
    session_token: String,
}

#[derive(Debug)]
struct GdsUrl {
    volume: String,
    path: String
}

#[tokio::main]
async fn main() {
    let conf = setup_conf().await;

    let input_str_url = "gds://development/analysis_data/SBJ00006/wgs_alignment_qc/20220510altBC/NTC_Tsqn220329_altBC_dragen_alignment_multiqc/NTC_Tsqn220329_altBC_dragen_alignment_multiqc_data/dragen_frag_len.txt";
    let input_gds_url = gds_url_to_volume_and_path(&input_str_url).await;
    let volume_ids = vec!(gds_volume_to_volume_id(&conf, &input_gds_url.as_ref().unwrap().volume).await);
    let gds_urls = vec!(input_gds_url.unwrap().path);

    // TODO: map iterate over those
    let file_ids = gds_urls_to_file_ids(&conf, vec!(volume_ids[0].as_ref().unwrap().id.as_ref().unwrap().clone()), gds_urls).await;
    for file in &file_ids.unwrap().items.unwrap() {
        get_presigned_url(&conf, file.id.as_ref()).await;
    }
}

async fn setup_conf() -> Configuration {
    let mut conf = Configuration::default();
    let key = read_access_token().await.unwrap();
    let apikey = ApiKey {
        prefix: Some("Bearer".to_string()),
        key,
    };
    conf.api_key = Some(apikey);
    conf
}

async fn gds_volume_to_volume_id(conf: &Configuration, volume: &str) -> Result<VolumeResponse, Error<GetVolumeError>> {
    get_volume(&conf, volume, None, None, None).await
}

async fn gds_url_to_volume_and_path(url: &str) -> Result<GdsUrl, ParseError> {
    let volume = Url::parse(url)?.host_str().unwrap().to_string();
    let path = Url::parse(url)?.path().to_string();
    Ok(GdsUrl{ volume, path })
    
}

async fn gds_urls_to_file_ids(conf: &Configuration, volume_id: Vec<String>, gds_urls: Vec<String>) -> Result<FileListResponse, Error<ListFilesError>> {
    list_files(&conf, Some(volume_id), None, Some(gds_urls), None, None, None, None, None, None, None, None, None, None).await
}

async fn get_presigned_url(conf: &Configuration, file_id: Option<&String>) {
    let res = get_file(
        &conf,
        &file_id.unwrap(),
        None,
        None,
        None,
        None,
        None,
    )
    .await;
    println!("{}", res.unwrap().presigned_url.unwrap());
}

use dirs::home_dir;
use std::fs::File;
use serde::{Deserialize, Serialize};

use ica_gds::apis::configuration::Configuration;
use ica_gds::apis::configuration::ApiKey;
use ica_gds::apis::volumes_api::list_volumes;

#[derive(Debug, Serialize, Deserialize)]
struct IcaConfig {
    #[serde(rename(deserialize = "access-token"))]
    access_token: String,
    #[serde(rename(deserialize = "session-token"))]
    session_token: String
}

#[tokio::main]
async fn main() {
    list_gds_volumes().await;
}

// TODO: Tidy up this messy function
async fn read_access_token() -> String {
    let f = File::open(home_dir().unwrap().to_str().unwrap().to_owned() + "/.ica/.session.aps2.yaml").expect("Cannot open file");
    let ica: IcaConfig = serde_yaml::from_reader(f).expect("Could not read values.");
    ica.access_token
}

async fn list_gds_volumes() {
    let mut conf = Configuration::default();
    let default_pagination = "eyJwYWdlU2l6ZSI6MTAsInBhZ2VTdGFydElkIjowLCJwYWdlRW5kSWQiOjEwfQ=="; // TODO: base64-encoded '{"pageSize":10,"pageStartId":0,"pageEndId":10}', build this dynamically
    let tenant_id = "void"; // TODO: Obtain this programatically
    let key = read_access_token().await;
    let apikey = ApiKey{ prefix: Some("Bearer".to_string()), key };
    conf.api_key = Some(apikey);

    // page_size: Option<i32>, page_token: Option<&str'>, include: Option<&str>, tenant_id: Option<&str>, volume_configuration_name: Option<&str>) -> Result<crate::models::VolumeListResponse, Error<ListVolumesError>> {
    let res = list_volumes(&conf, Some(10), Some(default_pagination), Some("ObjectStoreAccess"), Some(tenant_id), Some("vol_config_name")).await;
    println!("{:#?}", res);

}
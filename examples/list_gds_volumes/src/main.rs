use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Error;

use ica_gds::apis::configuration::ApiKey;
use ica_gds::apis::configuration::Configuration;
use ica_gds::apis::volumes_api::list_volumes;

#[derive(Debug, Serialize, Deserialize)]
struct IcaConfig {
    #[serde(rename(deserialize = "access-token"))]
    access_token: String,
    #[serde(rename(deserialize = "session-token"))]
    session_token: String,
}

#[tokio::main]
async fn main() {
    list_gds_volumes().await;
}

// TODO: Tidy up this messy function
// TODO: Read from env vars first and then check filesystem, in this order
async fn read_access_token() -> Result<String, Error> {
    let f =
        File::open(home_dir().unwrap().to_str().unwrap().to_owned() + "/.ica/.session.aps2.yaml")
            .expect("Cannot open file");
    let ica: IcaConfig = serde_yaml::from_reader(f).expect("Could not read values.");
    Ok(ica.access_token)
}

async fn list_gds_volumes() {
    let mut conf = Configuration::default();
    let key = read_access_token().await.unwrap();
    let apikey = ApiKey {
        prefix: Some("Bearer".to_string()),
        key,
    };
    conf.api_key = Some(apikey);

    let res = list_volumes(&conf, None, None, None, None, None).await;
    println!("{:#?}", res);
}

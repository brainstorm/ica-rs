use ica_gds::apis::configuration::Configuration;

#[tokio::main]
async fn main() {
    list_gds_volumes().await;
}

async fn list_gds_volumes() {
    let conf = Configuration::default();
    let res = conf.client.get("http://httpbin.org/").send().await;
    println!("{:#?}", res.unwrap());
}
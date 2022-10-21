#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerCreate { container_write_base :
        ContainerWriteBase(::serde_json::json!({})), barcode : "your barcode".to_owned(),
        project_id : Some("your project id".to_owned()), schema_id : "your schema id"
        .to_owned() }
    ];
    let response = client.bulk_create_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}

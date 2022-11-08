#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerCreate { project_id : Some("your project id".to_owned()), schema_id :
        "your schema id".to_owned(), barcode : "your barcode".to_owned(),
        container_write_base : ContainerWriteBase(::serde_json::json!({})) }
    ];
    let response = client.bulk_create_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}

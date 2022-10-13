#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerBulkUpdateItem { container_id : "your container id".to_owned(),
        container_write_base : ContainerWriteBase(::serde_json::json!({})), volume :
        DeprecatedContainerVolumeForInput { units : Some("your units".to_owned()), value
        : Some(1.0) }, quantity : ContainerQuantity { units : Some("your units"
        .to_owned()), value : Some(1.0) } }
    ];
    let response = client.bulk_update_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}

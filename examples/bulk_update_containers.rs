#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerBulkUpdateItem { volume : DeprecatedContainerVolumeForInput { units :
        Some("your units".to_owned()), value : Some(1.0) }, container_id :
        "your container id".to_owned(), container_write_base :
        ContainerWriteBase(::serde_json::json!({})), quantity : ContainerQuantity { units
        : Some("your units".to_owned()), value : Some(1.0) } }
    ];
    let response = client.bulk_update_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}

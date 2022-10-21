#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateContainerRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateContainerRequired {
        name: "your name",
        container_id: "your container id",
        parent_storage_id: "your parent storage id",
        quantity: ContainerQuantity {
            units: Some("your units".to_owned()),
            value: Some(1.0),
        },
        volume: DeprecatedContainerVolumeForInput {
            value: Some(1.0),
            units: Some("your units".to_owned()),
        },
        fields: Fields {},
    };
    let response = client
        .update_container(args)
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

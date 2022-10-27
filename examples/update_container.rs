#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateContainerRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateContainerRequired {
        quantity: ContainerQuantity {
            value: Some(1.0),
            units: Some("your units".to_owned()),
        },
        name: "your name",
        parent_storage_id: "your parent storage id",
        volume: DeprecatedContainerVolumeForInput {
            value: Some(1.0),
            units: Some("your units".to_owned()),
        },
        container_id: "your container id",
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

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let organization_id = "your organization id";
    let response = client.get_organization(organization_id).send().await.unwrap();
    println!("{:#?}", response);
}

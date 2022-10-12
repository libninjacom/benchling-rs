#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let name = "your name";
    let options = vec![DropdownOptionCreate { name : "your name".to_owned() }];
    let response = client
        .create_dropdown(name, options)
        .registry_id("your registry id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

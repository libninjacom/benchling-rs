#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dropdown_id = "your dropdown id";
    let options = vec![
        DropdownOptionUpdate { name : "your name".to_owned(), id : Some("your id"
        .to_owned()) }
    ];
    let response = client.update_dropdown(dropdown_id, options).send().await.unwrap();
    println!("{:#?}", response);
}

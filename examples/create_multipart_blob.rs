#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let name = "your name";
    let type_ = "your type";
    let response = client
        .create_multipart_blob(name, type_)
        .mime_type("your mime type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

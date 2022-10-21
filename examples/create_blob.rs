#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateBlobRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateBlobRequired {
        type_: "your type",
        name: "your name",
        md5: "your md 5",
        data64: "your data 64",
    };
    let response = client
        .create_blob(args)
        .mime_type("your mime type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

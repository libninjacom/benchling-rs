#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateBlobPartRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateBlobPartRequired {
        md5: "your md 5",
        data64: "your data 64",
        part_number: 1,
        blob_id: "your blob id",
    };
    let response = client.create_blob_part(args).send().await.unwrap();
    println!("{:#?}", response);
}

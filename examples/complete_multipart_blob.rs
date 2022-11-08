#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let blob_id = "your blob id";
    let response = client
        .complete_multipart_blob(blob_id)
        .parts(
            vec![
                BlobPart { part_number : Some(1), e_tag : Some("your e tag".to_owned()) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

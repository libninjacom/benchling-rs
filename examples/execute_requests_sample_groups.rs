#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let request_id = "your request id";
    let sample_groups = vec![
        SampleGroupStatusUpdate { status : "your status".to_owned(), sample_group_id :
        "your sample group id".to_owned() }
    ];
    let response = client
        .execute_requests_sample_groups(request_id, sample_groups)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::PatchRequestRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = PatchRequestRequired {
        fields: Fields {},
        project_id: "your project id",
        sample_groups: vec![
            RequestSampleGroupCreate { samples : RequestSampleGroupSamples {} }
        ],
        request_id: "your request id",
        scheduled_on: "your scheduled on",
        request_status: "your request status",
        assignees: vec![::serde_json::json!({})],
    };
    let response = client
        .patch_request(args)
        .requestor_id("your requestor id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

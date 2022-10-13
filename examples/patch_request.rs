#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::PatchRequestRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = PatchRequestRequired {
        project_id: "your project id",
        request_id: "your request id",
        sample_groups: vec![
            RequestSampleGroupCreate { samples : RequestSampleGroupSamples {} }
        ],
        scheduled_on: "your scheduled on",
        request_status: "your request status",
        fields: Fields {},
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

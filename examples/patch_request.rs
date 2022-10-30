#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::PatchRequestRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = PatchRequestRequired {
        fields: Fields {},
        sample_groups: vec![
            RequestSampleGroupCreate { samples : RequestSampleGroupSamples {} }
        ],
        assignees: vec![::serde_json::json!({})],
        request_status: "your request status",
        scheduled_on: "your scheduled on",
        request_id: "your request id",
        project_id: "your project id",
    };
    let response = client
        .patch_request(args)
        .requestor_id("your requestor id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

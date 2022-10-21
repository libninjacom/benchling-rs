#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRequestRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRequestRequired {
        project_id: "your project id",
        schema_id: "your schema id",
        assignees: vec![::serde_json::json!({})],
        fields: Fields {},
        sample_groups: vec![
            RequestSampleGroupCreate { samples : RequestSampleGroupSamples {} }
        ],
        scheduled_on: "your scheduled on",
    };
    let response = client
        .create_request(args)
        .requestor_id("your requestor id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

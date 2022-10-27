#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRequestRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRequestRequired {
        sample_groups: vec![
            RequestSampleGroupCreate { samples : RequestSampleGroupSamples {} }
        ],
        fields: Fields {},
        scheduled_on: "your scheduled on",
        project_id: "your project id",
        assignees: vec![::serde_json::json!({})],
        schema_id: "your schema id",
    };
    let response = client
        .create_request(args)
        .requestor_id("your requestor id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

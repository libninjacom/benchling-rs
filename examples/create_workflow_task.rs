#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateWorkflowTaskRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateWorkflowTaskRequired {
        workflow_task_group_id: "your workflow task group id",
        assignee_id: "your assignee id",
        fields: Fields {},
        scheduled_on: "your scheduled on",
    };
    let response = client.create_workflow_task(args).send().await.unwrap();
    println!("{:#?}", response);
}

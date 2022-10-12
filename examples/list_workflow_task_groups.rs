#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_workflow_task_groups()
        .ids("your ids")
        .schema_id("your schema id")
        .folder_id("your folder id")
        .project_id("your project id")
        .mentioned_in("your mentioned in")
        .watcher_ids("your watcher ids")
        .execution_types("your execution types")
        .responsible_team_ids("your responsible team ids")
        .status_ids_any_of("your status ids.any of")
        .status_ids_none_of("your status ids.none of")
        .status_ids_only("your status ids.only")
        .name("your name")
        .name_includes("your name includes")
        .creator_ids("your creator ids")
        .modified_at("your modified at")
        .next_token("your next token")
        .page_size(1)
        .display_ids("your display ids")
        .archive_reason("your archive reason")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

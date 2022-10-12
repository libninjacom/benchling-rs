#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_workflow_outputs()
        .ids("your ids")
        .workflow_task_group_ids("your workflow task group ids")
        .workflow_task_ids("your workflow task ids")
        .schema_id("your schema id")
        .watcher_ids("your watcher ids")
        .responsible_team_ids("your responsible team ids")
        .creation_origin_ids("your creation origin ids")
        .linked_item_ids_any_of("your linked item ids.any of")
        .linked_item_ids_all_of("your linked item ids.all of")
        .linked_item_ids_none_of("your linked item ids.none of")
        .schema_fields(SchemaFieldsQueryParam {})
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

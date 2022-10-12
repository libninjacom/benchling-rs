#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_workflow_tasks()
        .ids("your ids")
        .workflow_task_group_ids("your workflow task group ids")
        .schema_id("your schema id")
        .status_ids("your status ids")
        .assignee_ids("your assignee ids")
        .watcher_ids("your watcher ids")
        .responsible_team_ids("your responsible team ids")
        .execution_origin_ids("your execution origin ids")
        .execution_types("your execution types")
        .linked_item_ids_any_of("your linked item ids.any of")
        .linked_item_ids_all_of("your linked item ids.all of")
        .linked_item_ids_none_of("your linked item ids.none of")
        .schema_fields(SchemaFieldsQueryParam {})
        .name("your name")
        .name_includes("your name includes")
        .creator_ids("your creator ids")
        .scheduled_on(::serde_json::json!({}))
        .scheduled_on_lt("your scheduled on.lt")
        .scheduled_on_lte("your scheduled on.lte")
        .scheduled_on_gte("your scheduled on.gte")
        .scheduled_on_gt("your scheduled on.gt")
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

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client
        .list_assay_results(schema_id)
        .created_at_lt("your created at.lt")
        .created_at_gt("your created at.gt")
        .created_at_lte("your created at.lte")
        .created_at_gte("your created at.gte")
        .min_created_time(1)
        .max_created_time(1)
        .sort("your sort")
        .next_token("your next token")
        .page_size(1)
        .entity_ids("your entity ids")
        .storage_ids("your storage ids")
        .assay_run_ids("your assay run ids")
        .ids("your ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let folder_id = "your folder id";
    let name = "your name";
    let response = client
        .create_entry(folder_id, name)
        .author_ids(::serde_json::json!({}))
        .custom_fields(CustomFields {})
        .entry_template_id("your entry template id")
        .fields(Fields {})
        .initial_tables(
            vec![
                InitialTable { template_table_id : Some("your template table id"
                .to_owned()), csv_data : Some("your csv data".to_owned()) }
            ],
        )
        .schema_id("your schema id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

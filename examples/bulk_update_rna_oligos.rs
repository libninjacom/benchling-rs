#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_rna_oligos()
        .rna_oligos(
            vec![
                RnaOligoBulkUpdate(::serde_json::json!({}),
                RnaOligoUpdate(OligoUpdate(OligoBaseRequest { bases : Some("your bases"
                .to_owned()), author_ids : Some(vec!["your author ids".to_owned()]),
                custom_fields : Some(CustomFields {}), fields : Some(Fields {}),
                folder_id : Some("your folder id".to_owned()), name : Some("your name"
                .to_owned()), schema_id : Some("your schema id".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]) })))
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

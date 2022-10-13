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
                RnaOligoBulkUpdate { rna_oligo_update : RnaOligoUpdate { oligo_update :
                OligoUpdate { oligo_base_request : OligoBaseRequest { folder_id :
                Some("your folder id".to_owned()), schema_id : Some("your schema id"
                .to_owned()), aliases : Some(vec!["your aliases".to_owned()]), fields :
                Some(Fields {}), bases : Some("your bases".to_owned()), name :
                Some("your name".to_owned()), author_ids : Some(vec!["your author ids"
                .to_owned()]), custom_fields : Some(CustomFields {}) } } }, id :
                "your id".to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

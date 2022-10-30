#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_dna_oligos()
        .dna_oligos(
            vec![
                DnaOligoBulkUpdate { id : "your id".to_owned(), dna_oligo_update :
                DnaOligoUpdate { oligo_update : OligoUpdate { oligo_base_request :
                OligoBaseRequest { bases : Some("your bases".to_owned()), name :
                Some("your name".to_owned()), schema_id : Some("your schema id"
                .to_owned()), fields : Some(Fields {}), author_ids :
                Some(vec!["your author ids".to_owned()]), folder_id :
                Some("your folder id".to_owned()), aliases : Some(vec!["your aliases"
                .to_owned()]), custom_fields : Some(CustomFields {}) } } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

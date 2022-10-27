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
                DnaOligoBulkUpdate { dna_oligo_update : DnaOligoUpdate { oligo_update :
                OligoUpdate { oligo_base_request : OligoBaseRequest { schema_id :
                Some("your schema id".to_owned()), bases : Some("your bases".to_owned()),
                folder_id : Some("your folder id".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), aliases :
                Some(vec!["your aliases".to_owned()]), fields : Some(Fields {}),
                custom_fields : Some(CustomFields {}), name : Some("your name"
                .to_owned()) } } }, id : "your id".to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_aa_sequences()
        .aa_sequences(
            vec![
                AaSequenceBulkUpdate { aa_sequence_base_request : AaSequenceBaseRequest {
                author_ids : Some(vec!["your author ids".to_owned()]), aliases :
                Some(vec!["your aliases".to_owned()]), amino_acids :
                Some("your amino acids".to_owned()), custom_fields : Some(CustomFields
                {}), name : Some("your name".to_owned()), fields : Some(Fields {}),
                annotations : Some(vec![AaAnnotation { name : Some("your name"
                .to_owned()), color : Some("your color".to_owned()), type_ :
                Some("your type".to_owned()), id : Some("your id".to_owned()), end :
                Some(1), start : Some(1) }]), folder_id : Some("your folder id"
                .to_owned()), schema_id : Some("your schema id".to_owned()) }, id :
                "your id".to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

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
                AaSequenceBulkUpdate { id : "your id".to_owned(),
                aa_sequence_base_request : AaSequenceBaseRequest { folder_id :
                Some("your folder id".to_owned()), name : Some("your name".to_owned()),
                fields : Some(Fields {}), author_ids : Some(vec!["your author ids"
                .to_owned()]), aliases : Some(vec!["your aliases".to_owned()]),
                annotations : Some(vec![AaAnnotation { start : Some(1), type_ :
                Some("your type".to_owned()), end : Some(1), id : Some("your id"
                .to_owned()), name : Some("your name".to_owned()), color :
                Some("your color".to_owned()) }]), schema_id : Some("your schema id"
                .to_owned()), amino_acids : Some("your amino acids".to_owned()),
                custom_fields : Some(CustomFields {}) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

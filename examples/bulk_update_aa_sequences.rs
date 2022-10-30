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
                aa_sequence_base_request : AaSequenceBaseRequest { annotations :
                Some(vec![AaAnnotation { end : Some(1), color : Some("your color"
                .to_owned()), id : Some("your id".to_owned()), name : Some("your name"
                .to_owned()), start : Some(1), type_ : Some("your type".to_owned()) }]),
                folder_id : Some("your folder id".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]), custom_fields : Some(CustomFields
                {}), fields : Some(Fields {}), schema_id : Some("your schema id"
                .to_owned()), author_ids : Some(vec!["your author ids".to_owned()]), name
                : Some("your name".to_owned()), amino_acids : Some("your amino acids"
                .to_owned()) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

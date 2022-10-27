#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateAaSequenceRequired {
        amino_acids: "your amino acids",
        annotations: vec![
            AaAnnotation { start : Some(1), type_ : Some("your type".to_owned()), end :
            Some(1), id : Some("your id".to_owned()), name : Some("your name"
            .to_owned()), color : Some("your color".to_owned()) }
        ],
        entity_registry_id: "your entity registry id",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        aa_sequence_id: "your aa sequence id",
        custom_fields: CustomFields {},
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        fields: Fields {},
        name: "your name",
    };
    let response = client.update_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

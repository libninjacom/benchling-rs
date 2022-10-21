#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateAaSequenceRequired {
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        fields: Fields {},
        schema_id: "your schema id",
        annotations: vec![
            AaAnnotation { end : Some(1), start : Some(1), name : Some("your name"
            .to_owned()), type_ : Some("your type".to_owned()), id : Some("your id"
            .to_owned()), color : Some("your color".to_owned()) }
        ],
        amino_acids: "your amino acids",
        aliases: &["your aliases"],
        aa_sequence_id: "your aa sequence id",
        custom_fields: CustomFields {},
        name: "your name",
    };
    let response = client.update_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

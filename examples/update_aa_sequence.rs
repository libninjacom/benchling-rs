#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateAaSequenceRequired {
        fields: Fields {},
        aa_sequence_id: "your aa sequence id",
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        annotations: vec![
            AaAnnotation { color : Some("your color".to_owned()), id : Some("your id"
            .to_owned()), end : Some(1), name : Some("your name".to_owned()), start :
            Some(1), type_ : Some("your type".to_owned()) }
        ],
        amino_acids: "your amino acids",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        name: "your name",
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
    };
    let response = client.update_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

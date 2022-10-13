#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateAaSequenceRequired {
        aa_sequence_id: "your aa sequence id",
        folder_id: "your folder id",
        aliases: &["your aliases"],
        entity_registry_id: "your entity registry id",
        amino_acids: "your amino acids",
        annotations: vec![
            AaAnnotation { color : Some("your color".to_owned()), end : Some(1), name :
            Some("your name".to_owned()), id : Some("your id".to_owned()), type_ :
            Some("your type".to_owned()), start : Some(1) }
        ],
        author_ids: &["your author ids"],
        fields: Fields {},
        name: "your name",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
    };
    let response = client.update_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

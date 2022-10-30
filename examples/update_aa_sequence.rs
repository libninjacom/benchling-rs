#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateAaSequenceRequired {
        schema_id: "your schema id",
        aa_sequence_id: "your aa sequence id",
        name: "your name",
        aliases: &["your aliases"],
        amino_acids: "your amino acids",
        entity_registry_id: "your entity registry id",
        fields: Fields {},
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        annotations: vec![
            AaAnnotation { end : Some(1), color : Some("your color".to_owned()), id :
            Some("your id".to_owned()), name : Some("your name".to_owned()), start :
            Some(1), type_ : Some("your type".to_owned()) }
        ],
    };
    let response = client.update_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

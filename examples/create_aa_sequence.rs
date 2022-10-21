#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        entity_registry_id: "your entity registry id",
        amino_acids: "your amino acids",
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        name: "your name",
        schema_id: "your schema id",
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        annotations: vec![
            AaAnnotation { end : Some(1), start : Some(1), name : Some("your name"
            .to_owned()), type_ : Some("your type".to_owned()), id : Some("your id"
            .to_owned()), color : Some("your color".to_owned()) }
        ],
        fields: Fields {},
        aliases: &["your aliases"],
        custom_fields: CustomFields {},
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        registry_id: "your registry id",
        annotations: vec![
            AaAnnotation { name : Some("your name".to_owned()), color : Some("your color"
            .to_owned()), type_ : Some("your type".to_owned()), id : Some("your id"
            .to_owned()), end : Some(1), start : Some(1) }
        ],
        naming_strategy: "your naming strategy",
        folder_id: "your folder id",
        name: "your name",
        amino_acids: "your amino acids",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        fields: Fields {},
        aliases: &["your aliases"],
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

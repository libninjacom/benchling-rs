#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        aliases: &["your aliases"],
        fields: Fields {},
        folder_id: "your folder id",
        name: "your name",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        amino_acids: "your amino acids",
        schema_id: "your schema id",
        registry_id: "your registry id",
        custom_fields: CustomFields {},
        annotations: vec![
            AaAnnotation { start : Some(1), type_ : Some("your type".to_owned()), end :
            Some(1), id : Some("your id".to_owned()), name : Some("your name"
            .to_owned()), color : Some("your color".to_owned()) }
        ],
        author_ids: &["your author ids"],
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

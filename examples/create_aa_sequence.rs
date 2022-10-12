#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        amino_acids: "your amino acids",
        annotations: vec![
            AaAnnotation { color : Some("your color".to_owned()), id : Some("your id"
            .to_owned()), end : Some(1), name : Some("your name".to_owned()), start :
            Some(1), type_ : Some("your type".to_owned()) }
        ],
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        aliases: &["your aliases"],
        name: "your name",
        folder_id: "your folder id",
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        fields: Fields {},
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

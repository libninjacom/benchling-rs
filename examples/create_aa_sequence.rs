#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        aliases: &["your aliases"],
        amino_acids: "your amino acids",
        fields: Fields {},
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        annotations: vec![
            AaAnnotation { color : Some("your color".to_owned()), end : Some(1), name :
            Some("your name".to_owned()), id : Some("your id".to_owned()), type_ :
            Some("your type".to_owned()), start : Some(1) }
        ],
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        name: "your name",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

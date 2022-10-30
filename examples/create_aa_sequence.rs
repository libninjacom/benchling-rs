#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateAaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateAaSequenceRequired {
        name: "your name",
        entity_registry_id: "your entity registry id",
        registry_id: "your registry id",
        amino_acids: "your amino acids",
        naming_strategy: "your naming strategy",
        fields: Fields {},
        folder_id: "your folder id",
        schema_id: "your schema id",
        aliases: &["your aliases"],
        annotations: vec![
            AaAnnotation { end : Some(1), color : Some("your color".to_owned()), id :
            Some("your id".to_owned()), name : Some("your name".to_owned()), start :
            Some(1), type_ : Some("your type".to_owned()) }
        ],
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
    };
    let response = client.create_aa_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

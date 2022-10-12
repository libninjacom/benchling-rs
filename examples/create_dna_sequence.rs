#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaSequenceRequired {
        bases: "your bases",
        custom_fields: CustomFields {},
        folder_id: "your folder id",
        registry_id: "your registry id",
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        aliases: &["your aliases"],
        translations: vec![
            Translation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        name: "your name",
        naming_strategy: "your naming strategy",
        author_ids: &["your author ids"],
        is_circular: true,
        annotations: vec![
            DnaAnnotation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        fields: Fields {},
        primers: vec![
            Primer { start : Some(1), color : Some("your color".to_owned()), oligo_id :
            Some("your oligo id".to_owned()), strand : Some(1), name : Some("your name"
            .to_owned()), bases : Some("your bases".to_owned()), bind_position : Some(1),
            end : Some(1), overhang_length : Some(1) }
        ],
    };
    let response = client.create_dna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

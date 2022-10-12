#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaSequenceRequired {
        naming_strategy: "your naming strategy",
        name: "your name",
        annotations: vec![
            RnaAnnotation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        aliases: &["your aliases"],
        folder_id: "your folder id",
        bases: "your bases",
        registry_id: "your registry id",
        translations: vec![
            Translation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        is_circular: true,
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        fields: Fields {},
        schema_id: "your schema id",
        primers: vec![
            Primer { start : Some(1), color : Some("your color".to_owned()), oligo_id :
            Some("your oligo id".to_owned()), strand : Some(1), name : Some("your name"
            .to_owned()), bases : Some("your bases".to_owned()), bind_position : Some(1),
            end : Some(1), overhang_length : Some(1) }
        ],
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

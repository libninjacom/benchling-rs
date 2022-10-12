#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaSequenceRequired {
        fields: Fields {},
        rna_sequence_id: "your rna sequence id",
        bases: "your bases",
        annotations: vec![
            RnaAnnotation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        is_circular: true,
        primers: vec![
            Primer { start : Some(1), color : Some("your color".to_owned()), oligo_id :
            Some("your oligo id".to_owned()), strand : Some(1), name : Some("your name"
            .to_owned()), bases : Some("your bases".to_owned()), bind_position : Some(1),
            end : Some(1), overhang_length : Some(1) }
        ],
        schema_id: "your schema id",
        translations: vec![
            Translation(SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()) }, ::serde_json::json!({}))
        ],
        name: "your name",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
    };
    let response = client.update_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

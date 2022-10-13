#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaSequenceRequired {
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        primers: vec![
            Primer { bases : Some("your bases".to_owned()), color : Some("your color"
            .to_owned()), end : Some(1), name : Some("your name".to_owned()), oligo_id :
            Some("your oligo id".to_owned()), overhang_length : Some(1), bind_position :
            Some(1), start : Some(1), strand : Some(1) }
        ],
        annotations: vec![
            RnaAnnotation { end : 1, type_ : "your type".to_owned(), strand : 1,
            sequence_feature_base : SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { name : Some("your name".to_owned()),
            value : Some("your value".to_owned()) }]), color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), notes : Some("your notes"
            .to_owned()) }, start : 1 }
        ],
        registry_id: "your registry id",
        is_circular: true,
        name: "your name",
        aliases: &["your aliases"],
        bases: "your bases",
        folder_id: "your folder id",
        fields: Fields {},
        translations: vec![
            Translation { strand : 1, regions : vec![::serde_json::json!({})], start : 1,
            end : 1, sequence_feature_base : SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { name : Some("your name".to_owned()),
            value : Some("your value".to_owned()) }]), color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), notes : Some("your notes"
            .to_owned()) }, genetic_code : "your genetic code".to_owned(), amino_acids :
            "your amino acids".to_owned() }
        ],
        naming_strategy: "your naming strategy",
    };
    let response = client.create_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaSequenceRequired {
        registry_id: "your registry id",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        annotations: vec![
            RnaAnnotation { strand : 1, type_ : "your type".to_owned(),
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()) }, start : 1, end : 1 }
        ],
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        is_circular: true,
        primers: vec![
            Primer { strand : Some(1), bases : Some("your bases".to_owned()), end :
            Some(1), bind_position : Some(1), name : Some("your name".to_owned()), start
            : Some(1), oligo_id : Some("your oligo id".to_owned()), overhang_length :
            Some(1), color : Some("your color".to_owned()) }
        ],
        name: "your name",
        naming_strategy: "your naming strategy",
        bases: "your bases",
        fields: Fields {},
        translations: vec![
            Translation { genetic_code : "your genetic code".to_owned(), end : 1,
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()) }, amino_acids : "your amino acids".to_owned(), regions :
            vec![::serde_json::json!({})], start : 1, strand : 1 }
        ],
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

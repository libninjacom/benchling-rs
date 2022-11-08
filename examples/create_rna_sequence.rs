#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaSequenceRequired {
        primers: vec![
            Primer { oligo_id : Some("your oligo id".to_owned()), strand : Some(1), end :
            Some(1), bases : Some("your bases".to_owned()), start : Some(1),
            bind_position : Some(1), color : Some("your color".to_owned()),
            overhang_length : Some(1), name : Some("your name".to_owned()) }
        ],
        author_ids: &["your author ids"],
        is_circular: true,
        entity_registry_id: "your entity registry id",
        bases: "your bases",
        annotations: vec![
            RnaAnnotation { end : 1, start : 1, strand : 1, sequence_feature_base :
            SequenceFeatureBase { custom_fields : Some(vec![SequenceFeatureCustomField {
            name : Some("your name".to_owned()), value : Some("your value".to_owned())
            }]), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()), notes : Some("your notes".to_owned()) }, type_ : "your type"
            .to_owned() }
        ],
        aliases: &["your aliases"],
        registry_id: "your registry id",
        custom_fields: CustomFields {},
        name: "your name",
        translations: vec![
            Translation { start : 1, strand : 1, amino_acids : "your amino acids"
            .to_owned(), genetic_code : "your genetic code".to_owned(), regions :
            vec![::serde_json::json!({})], end : 1, sequence_feature_base :
            SequenceFeatureBase { custom_fields : Some(vec![SequenceFeatureCustomField {
            name : Some("your name".to_owned()), value : Some("your value".to_owned())
            }]), color : Some("your color".to_owned()), name : Some("your name"
            .to_owned()), notes : Some("your notes".to_owned()) } }
        ],
        schema_id: "your schema id",
        folder_id: "your folder id",
        fields: Fields {},
        naming_strategy: "your naming strategy",
    };
    let response = client.create_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

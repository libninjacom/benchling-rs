#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaSequenceRequired {
        naming_strategy: "your naming strategy",
        bases: "your bases",
        aliases: &["your aliases"],
        name: "your name",
        fields: Fields {},
        is_circular: true,
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        registry_id: "your registry id",
        translations: vec![
            Translation { sequence_feature_base : SequenceFeatureBase { color :
            Some("your color".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), name : Some("your name".to_owned()) }, end : 1, strand : 1,
            genetic_code : "your genetic code".to_owned(), regions :
            vec![::serde_json::json!({})], start : 1, amino_acids : "your amino acids"
            .to_owned() }
        ],
        annotations: vec![
            DnaAnnotation { end : 1, strand : 1, type_ : "your type".to_owned(),
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField { value :
            Some("your value".to_owned()), name : Some("your name".to_owned()) }]), notes
            : Some("your notes".to_owned()), name : Some("your name".to_owned()) }, start
            : 1 }
        ],
        custom_fields: CustomFields {},
        primers: vec![
            Primer { name : Some("your name".to_owned()), bases : Some("your bases"
            .to_owned()), strand : Some(1), oligo_id : Some("your oligo id".to_owned()),
            end : Some(1), color : Some("your color".to_owned()), bind_position :
            Some(1), start : Some(1), overhang_length : Some(1) }
        ],
        schema_id: "your schema id",
    };
    let response = client.create_dna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

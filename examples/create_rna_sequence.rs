#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaSequenceRequired {
        aliases: &["your aliases"],
        schema_id: "your schema id",
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
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        author_ids: &["your author ids"],
        name: "your name",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
        annotations: vec![
            RnaAnnotation { strand : 1, sequence_feature_base : SequenceFeatureBase {
            color : Some("your color".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()), name : Some("your name".to_owned()) }, type_ : "your type"
            .to_owned(), end : 1, start : 1 }
        ],
        bases: "your bases",
        fields: Fields {},
        is_circular: true,
        folder_id: "your folder id",
        primers: vec![
            Primer { name : Some("your name".to_owned()), bases : Some("your bases"
            .to_owned()), strand : Some(1), oligo_id : Some("your oligo id".to_owned()),
            end : Some(1), color : Some("your color".to_owned()), bind_position :
            Some(1), start : Some(1), overhang_length : Some(1) }
        ],
    };
    let response = client.create_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

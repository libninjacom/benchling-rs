#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaSequenceRequired {
        translations: vec![
            Translation { strand : 1, regions : vec![::serde_json::json!({})], start : 1,
            end : 1, sequence_feature_base : SequenceFeatureBase { custom_fields :
            Some(vec![SequenceFeatureCustomField { name : Some("your name".to_owned()),
            value : Some("your value".to_owned()) }]), color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), notes : Some("your notes"
            .to_owned()) }, genetic_code : "your genetic code".to_owned(), amino_acids :
            "your amino acids".to_owned() }
        ],
        annotations: vec![
            DnaAnnotation { start : 1, sequence_feature_base : SequenceFeatureBase {
            custom_fields : Some(vec![SequenceFeatureCustomField { name :
            Some("your name".to_owned()), value : Some("your value".to_owned()) }]),
            color : Some("your color".to_owned()), name : Some("your name".to_owned()),
            notes : Some("your notes".to_owned()) }, strand : 1, type_ : "your type"
            .to_owned(), end : 1 }
        ],
        is_circular: true,
        bases: "your bases",
        name: "your name",
        entity_registry_id: "your entity registry id",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        aliases: &["your aliases"],
        custom_fields: CustomFields {},
        dna_sequence_id: "your dna sequence id",
        fields: Fields {},
        primers: vec![
            Primer { bases : Some("your bases".to_owned()), color : Some("your color"
            .to_owned()), end : Some(1), name : Some("your name".to_owned()), oligo_id :
            Some("your oligo id".to_owned()), overhang_length : Some(1), bind_position :
            Some(1), start : Some(1), strand : Some(1) }
        ],
        folder_id: "your folder id",
    };
    let response = client.update_dna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

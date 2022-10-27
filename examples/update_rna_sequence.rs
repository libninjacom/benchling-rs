#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaSequenceRequired {
        bases: "your bases",
        is_circular: true,
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        name: "your name",
        custom_fields: CustomFields {},
        primers: vec![
            Primer { end : Some(1), overhang_length : Some(1), oligo_id :
            Some("your oligo id".to_owned()), bind_position : Some(1), color :
            Some("your color".to_owned()), bases : Some("your bases".to_owned()), name :
            Some("your name".to_owned()), start : Some(1), strand : Some(1) }
        ],
        rna_sequence_id: "your rna sequence id",
        aliases: &["your aliases"],
        fields: Fields {},
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        translations: vec![
            Translation { amino_acids : "your amino acids".to_owned(),
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { name : Some("your name".to_owned()),
            value : Some("your value".to_owned()) }]), notes : Some("your notes"
            .to_owned()) }, end : 1, genetic_code : "your genetic code".to_owned(),
            regions : vec![::serde_json::json!({})], strand : 1, start : 1 }
        ],
        annotations: vec![
            RnaAnnotation { sequence_feature_base : SequenceFeatureBase { color :
            Some("your color".to_owned()), name : Some("your name".to_owned()),
            custom_fields : Some(vec![SequenceFeatureCustomField { name :
            Some("your name".to_owned()), value : Some("your value".to_owned()) }]),
            notes : Some("your notes".to_owned()) }, end : 1, strand : 1, start : 1,
            type_ : "your type".to_owned() }
        ],
    };
    let response = client.update_rna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

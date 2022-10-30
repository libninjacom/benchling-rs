#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaSequenceRequired {
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
        entity_registry_id: "your entity registry id",
        name: "your name",
        author_ids: &["your author ids"],
        primers: vec![
            Primer { name : Some("your name".to_owned()), bases : Some("your bases"
            .to_owned()), strand : Some(1), oligo_id : Some("your oligo id".to_owned()),
            end : Some(1), color : Some("your color".to_owned()), bind_position :
            Some(1), start : Some(1), overhang_length : Some(1) }
        ],
        bases: "your bases",
        annotations: vec![
            DnaAnnotation { end : 1, strand : 1, type_ : "your type".to_owned(),
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField { value :
            Some("your value".to_owned()), name : Some("your name".to_owned()) }]), notes
            : Some("your notes".to_owned()), name : Some("your name".to_owned()) }, start
            : 1 }
        ],
        dna_sequence_id: "your dna sequence id",
        aliases: &["your aliases"],
        custom_fields: CustomFields {},
        fields: Fields {},
        folder_id: "your folder id",
        is_circular: true,
        schema_id: "your schema id",
    };
    let response = client.update_dna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

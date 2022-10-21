#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaSequenceRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaSequenceRequired {
        folder_id: "your folder id",
        primers: vec![
            Primer { strand : Some(1), bases : Some("your bases".to_owned()), end :
            Some(1), bind_position : Some(1), name : Some("your name".to_owned()), start
            : Some(1), oligo_id : Some("your oligo id".to_owned()), overhang_length :
            Some(1), color : Some("your color".to_owned()) }
        ],
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
        bases: "your bases",
        author_ids: &["your author ids"],
        fields: Fields {},
        aliases: &["your aliases"],
        dna_sequence_id: "your dna sequence id",
        name: "your name",
        annotations: vec![
            DnaAnnotation { sequence_feature_base : SequenceFeatureBase { color :
            Some("your color".to_owned()), name : Some("your name".to_owned()),
            custom_fields : Some(vec![SequenceFeatureCustomField { value :
            Some("your value".to_owned()), name : Some("your name".to_owned()) }]), notes
            : Some("your notes".to_owned()) }, type_ : "your type".to_owned(), end : 1,
            start : 1, strand : 1 }
        ],
        is_circular: true,
        schema_id: "your schema id",
        translations: vec![
            Translation { genetic_code : "your genetic code".to_owned(), end : 1,
            sequence_feature_base : SequenceFeatureBase { color : Some("your color"
            .to_owned()), name : Some("your name".to_owned()), custom_fields :
            Some(vec![SequenceFeatureCustomField { value : Some("your value".to_owned()),
            name : Some("your name".to_owned()) }]), notes : Some("your notes"
            .to_owned()) }, amino_acids : "your amino acids".to_owned(), regions :
            vec![::serde_json::json!({})], start : 1, strand : 1 }
        ],
    };
    let response = client.update_dna_sequence(args).send().await.unwrap();
    println!("{:#?}", response);
}

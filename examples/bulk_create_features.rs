#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_features()
        .features(vec![FeatureBulkCreate { feature_create : FeatureCreate {} }])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

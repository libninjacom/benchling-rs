#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let team_id = "your team id";
    let response = client.get_team(team_id).send().await.unwrap();
    println!("{:#?}", response);
}

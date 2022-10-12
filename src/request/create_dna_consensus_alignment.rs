use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateDnaConsensusAlignmentRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub algorithm: String,
    pub files: Vec<serde_json::Value>,
    pub name: String,
    pub new_sequence: serde_json::Value,
    pub sequence_id: String,
}
impl<'a> CreateDnaConsensusAlignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self
            .client
            .client
            .post("/dna-alignments:create-consensus-alignment");
        r = r.push_json(json!({ "algorithm" : self.algorithm }));
        r = r.push_json(json!({ "files" : self.files }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "newSequence" : self.new_sequence }));
        r = r.push_json(json!({ "sequenceId" : self.sequence_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateDnaConsensusAlignmentRequired<'a> {
    pub algorithm: &'a str,
    pub files: Vec<serde_json::Value>,
    pub name: &'a str,
    pub new_sequence: serde_json::Value,
    pub sequence_id: &'a str,
}
impl<'a> CreateDnaConsensusAlignmentRequired<'a> {}

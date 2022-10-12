use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveAaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub aa_sequence_ids: Vec<String>,
}
impl<'a> UnarchiveAaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AaSequencesArchivalChange> {
        let mut r = self.client.client.post("/aa-sequences:unarchive");
        r = r.push_json(json!({ "aaSequenceIds" : self.aa_sequence_ids }));
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

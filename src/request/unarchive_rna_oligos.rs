use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveRnaOligosRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub rna_oligo_ids: Vec<String>,
}
impl<'a> UnarchiveRnaOligosRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RnaOligosArchivalChange> {
        let mut r = self.client.client.post("/rna-oligos:unarchive");
        r = r.push_json(json!({ "rnaOligoIds" : self.rna_oligo_ids }));
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

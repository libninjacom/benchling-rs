use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ArchiveDnaOligosRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dna_oligo_ids: Vec<String>,
    pub reason: String,
}
impl<'a> ArchiveDnaOligosRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DnaOligosArchivalChange> {
        let mut r = self.client.client.post("/dna-oligos:archive");
        r = r.push_json(json!({ "dnaOligoIds" : self.dna_oligo_ids }));
        r = r.push_json(json!({ "reason" : self.reason }));
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

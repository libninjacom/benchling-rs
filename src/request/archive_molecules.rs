use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ArchiveMoleculesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub molecule_ids: Vec<String>,
    pub reason: String,
}
impl<'a> ArchiveMoleculesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<MoleculesArchivalChange> {
        let mut r = self.client.client.post("/molecules:archive");
        r = r.push_json(json!({ "moleculeIds" : self.molecule_ids }));
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

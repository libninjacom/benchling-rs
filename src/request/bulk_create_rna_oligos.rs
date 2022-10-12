use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkCreateRnaOligosRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub rna_oligos: Option<Vec<RnaOligoCreate>>,
}
impl<'a> BulkCreateRnaOligosRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/rna-oligos:bulk-create");
        if let Some(ref unwrapped) = self.rna_oligos {
            r = r.push_json(json!({ "rnaOligos" : unwrapped }));
        }
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
    pub fn rna_oligos(mut self, rna_oligos: Vec<RnaOligoCreate>) -> Self {
        self.rna_oligos = Some(rna_oligos);
        self
    }
}

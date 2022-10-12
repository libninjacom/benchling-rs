use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkUpdateDnaOligosRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dna_oligos: Option<Vec<DnaOligoBulkUpdate>>,
}
impl<'a> BulkUpdateDnaOligosRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/dna-oligos:bulk-update");
        if let Some(ref unwrapped) = self.dna_oligos {
            r = r.push_json(json!({ "dnaOligos" : unwrapped }));
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
    pub fn dna_oligos(mut self, dna_oligos: Vec<DnaOligoBulkUpdate>) -> Self {
        self.dna_oligos = Some(dna_oligos);
        self
    }
}

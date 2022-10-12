use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkCreateRnaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub rna_sequences: Option<Vec<RnaSequenceBulkCreate>>,
}
impl<'a> BulkCreateRnaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/rna-sequences:bulk-create");
        if let Some(ref unwrapped) = self.rna_sequences {
            r = r.push_json(json!({ "rnaSequences" : unwrapped }));
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
    pub fn rna_sequences(mut self, rna_sequences: Vec<RnaSequenceBulkCreate>) -> Self {
        self.rna_sequences = Some(rna_sequences);
        self
    }
}

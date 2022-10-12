use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkCreateDnaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dna_sequences: Option<Vec<DnaSequenceBulkCreate>>,
}
impl<'a> BulkCreateDnaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/dna-sequences:bulk-create");
        if let Some(ref unwrapped) = self.dna_sequences {
            r = r.push_json(json!({ "dnaSequences" : unwrapped }));
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
    pub fn dna_sequences(mut self, dna_sequences: Vec<DnaSequenceBulkCreate>) -> Self {
        self.dna_sequences = Some(dna_sequences);
        self
    }
}

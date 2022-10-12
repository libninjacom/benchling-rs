use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetDnaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dna_sequence_ids: String,
}
impl<'a> BulkGetDnaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DnaSequencesBulkGet> {
        let mut r = self.client.client.get("/dna-sequences:bulk-get");
        r = r.push_query("dnaSequenceIds", &self.dna_sequence_ids.to_string());
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

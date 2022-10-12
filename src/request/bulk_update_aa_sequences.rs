use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkUpdateAaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub aa_sequences: Option<Vec<AaSequenceBulkUpdate>>,
}
impl<'a> BulkUpdateAaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/aa-sequences:bulk-update");
        if let Some(ref unwrapped) = self.aa_sequences {
            r = r.push_json(json!({ "aaSequences" : unwrapped }));
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
    pub fn aa_sequences(mut self, aa_sequences: Vec<AaSequenceBulkUpdate>) -> Self {
        self.aa_sequences = Some(aa_sequences);
        self
    }
}

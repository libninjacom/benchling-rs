use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AutoAnnotateRnaSequencesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub feature_library_ids: Vec<String>,
    pub rna_sequence_ids: Vec<String>,
}
impl<'a> AutoAnnotateRnaSequencesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/rna-sequences:auto-annotate");
        r = r.push_json(json!({ "featureLibraryIds" : self.feature_library_ids }));
        r = r.push_json(json!({ "rnaSequenceIds" : self.rna_sequence_ids }));
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
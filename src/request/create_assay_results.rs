use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateAssayResultsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_results: Vec<AssayResultCreate>,
}
impl<'a> CreateAssayResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayResultsCreateResponse> {
        let mut r = self.client.client.post("/assay-results");
        r = r.push_json(json!({ "assayResults" : self.assay_results }));
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

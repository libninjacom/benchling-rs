use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetAssayResultsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_result_ids: String,
}
impl<'a> BulkGetAssayResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayResultsBulkGet> {
        let mut r = self.client.client.get("/assay-results:bulk-get");
        r = r.push_query("assayResultIds", &self.assay_result_ids.to_string());
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

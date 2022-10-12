use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetAssayRunsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_ids: String,
}
impl<'a> BulkGetAssayRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayRunsBulkGet> {
        let mut r = self.client.client.get("/assay-runs:bulk-get");
        r = r.push_query("assayRunIds", &self.assay_run_ids.to_string());
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

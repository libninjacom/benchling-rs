use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAssayRunRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: String,
}
impl<'a> GetAssayRunRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayRun> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/assay-runs/{assay_run_id}", assay_run_id = self.assay_run_id),
            );
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

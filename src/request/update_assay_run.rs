use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAssayRunRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: String,
    pub fields: Option<Fields>,
}
impl<'a> UpdateAssayRunRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayRun> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/assay-runs/{assay_run_id}", assay_run_id = self.assay_run_id),
            );
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
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
    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }
}

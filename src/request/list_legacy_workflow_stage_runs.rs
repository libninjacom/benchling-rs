use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListLegacyWorkflowStageRunsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub stage_id: String,
}
impl<'a> ListLegacyWorkflowStageRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LegacyWorkflowStageRunList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/legacy-workflow-stages/{stage_id}/workflow-stage-runs", stage_id =
                    self.stage_id
                ),
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

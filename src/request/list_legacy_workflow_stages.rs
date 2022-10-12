use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListLegacyWorkflowStagesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub legacy_workflow_id: String,
}
impl<'a> ListLegacyWorkflowStagesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LegacyWorkflowStageList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/legacy-workflows/{legacy_workflow_id}/workflow-stages",
                    legacy_workflow_id = self.legacy_workflow_id
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

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListWorkflowStagesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_id: String,
}
impl<'a> ListWorkflowStagesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowStageList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/workflows/{workflow_id}/workflow-stages", workflow_id = self
                    .workflow_id
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

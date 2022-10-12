use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkUpdateWorkflowOutputsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_outputs: Option<Vec<WorkflowOutputBulkUpdate>>,
}
impl<'a> BulkUpdateWorkflowOutputsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/workflow-outputs:bulk-update");
        if let Some(ref unwrapped) = self.workflow_outputs {
            r = r.push_json(json!({ "workflowOutputs" : unwrapped }));
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
    pub fn workflow_outputs(
        mut self,
        workflow_outputs: Vec<WorkflowOutputBulkUpdate>,
    ) -> Self {
        self.workflow_outputs = Some(workflow_outputs);
        self
    }
}

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveWorkflowTasksRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_task_ids: Vec<String>,
}
impl<'a> UnarchiveWorkflowTasksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowTasksArchivalChange> {
        let mut r = self.client.client.post("/workflow-tasks:unarchive");
        r = r.push_json(json!({ "workflowTaskIds" : self.workflow_task_ids }));
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

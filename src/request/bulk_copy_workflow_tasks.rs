use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkCopyWorkflowTasksRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_task_ids: Option<Vec<String>>,
}
impl<'a> BulkCopyWorkflowTasksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/workflow-tasks:bulk-copy");
        if let Some(ref unwrapped) = self.workflow_task_ids {
            r = r.push_json(json!({ "workflowTaskIds" : unwrapped }));
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
    pub fn workflow_task_ids(
        mut self,
        workflow_task_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .workflow_task_ids = Some(
            workflow_task_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}

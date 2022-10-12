use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateWorkflowTaskGroupRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_task_group_id: String,
    pub folder_id: String,
    pub name: String,
    pub watcher_ids: Vec<String>,
}
impl<'a> UpdateWorkflowTaskGroupRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowTaskGroup> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/workflow-task-groups/{workflow_task_group_id}",
                    workflow_task_group_id = self.workflow_task_group_id
                ),
            );
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "watcherIds" : self.watcher_ids }));
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
pub struct UpdateWorkflowTaskGroupRequired<'a> {
    pub workflow_task_group_id: &'a str,
    pub folder_id: &'a str,
    pub name: &'a str,
    pub watcher_ids: &'a [&'a str],
}
impl<'a> UpdateWorkflowTaskGroupRequired<'a> {}

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateWorkflowTaskRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assignee_id: String,
    pub fields: Fields,
    pub scheduled_on: String,
    pub workflow_task_group_id: String,
}
impl<'a> CreateWorkflowTaskRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowTask> {
        let mut r = self.client.client.post("/workflow-tasks");
        r = r.push_json(json!({ "assigneeId" : self.assignee_id }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "scheduledOn" : self.scheduled_on }));
        r = r.push_json(json!({ "workflowTaskGroupId" : self.workflow_task_group_id }));
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
pub struct CreateWorkflowTaskRequired<'a> {
    pub assignee_id: &'a str,
    pub fields: Fields,
    pub scheduled_on: &'a str,
    pub workflow_task_group_id: &'a str,
}
impl<'a> CreateWorkflowTaskRequired<'a> {}

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWorkflowTaskSchemaRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub schema_id: String,
}
impl<'a> GetWorkflowTaskSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowTaskSchema> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/workflow-task-schemas/{schema_id}", schema_id = self.schema_id
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

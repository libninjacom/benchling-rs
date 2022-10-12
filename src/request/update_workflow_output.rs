use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateWorkflowOutputRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub workflow_output_id: String,
    pub fields: Fields,
}
impl<'a> UpdateWorkflowOutputRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowOutput> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/workflow-outputs/{workflow_output_id}", workflow_output_id = self
                    .workflow_output_id
                ),
            );
        r = r.push_json(json!({ "fields" : self.fields }));
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

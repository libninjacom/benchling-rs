use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAutomationOutputProcessorRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub output_processor_id: String,
    pub file_id: String,
}
impl<'a> UpdateAutomationOutputProcessorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationOutputProcessor> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/automation-output-processors/{output_processor_id}",
                    output_processor_id = self.output_processor_id
                ),
            );
        r = r.push_json(json!({ "fileId" : self.file_id }));
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

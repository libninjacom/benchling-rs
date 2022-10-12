use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateAutomationOutputProcessorRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: String,
    pub automation_file_config_name: String,
    pub complete_with_errors: Option<bool>,
    pub file_id: String,
}
impl<'a> CreateAutomationOutputProcessorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationOutputProcessor> {
        let mut r = self.client.client.post("/automation-output-processors");
        r = r.push_json(json!({ "assayRunId" : self.assay_run_id }));
        r = r
            .push_json(
                json!({ "automationFileConfigName" : self.automation_file_config_name }),
            );
        if let Some(ref unwrapped) = self.complete_with_errors {
            r = r.push_json(json!({ "completeWithErrors" : unwrapped }));
        }
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
    pub fn complete_with_errors(mut self, complete_with_errors: bool) -> Self {
        self.complete_with_errors = Some(complete_with_errors);
        self
    }
}

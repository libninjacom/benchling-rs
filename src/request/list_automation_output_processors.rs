use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAutomationOutputProcessorsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: Option<String>,
    pub automation_file_config_name: Option<String>,
    pub archive_reason: Option<String>,
    pub modified_at: Option<String>,
    pub next_token: Option<String>,
}
impl<'a> ListAutomationOutputProcessorsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationOutputProcessorsPaginatedList> {
        let mut r = self.client.client.get("/automation-output-processors");
        if let Some(ref unwrapped) = self.assay_run_id {
            r = r.push_query("assayRunId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.automation_file_config_name {
            r = r.push_query("automationFileConfigName", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.archive_reason {
            r = r.push_query("archiveReason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
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
    pub fn assay_run_id(mut self, assay_run_id: &str) -> Self {
        self.assay_run_id = Some(assay_run_id.to_owned());
        self
    }
    pub fn automation_file_config_name(
        mut self,
        automation_file_config_name: &str,
    ) -> Self {
        self.automation_file_config_name = Some(automation_file_config_name.to_owned());
        self
    }
    pub fn archive_reason(mut self, archive_reason: &str) -> Self {
        self.archive_reason = Some(archive_reason.to_owned());
        self
    }
    pub fn modified_at(mut self, modified_at: &str) -> Self {
        self.modified_at = Some(modified_at.to_owned());
        self
    }
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
}

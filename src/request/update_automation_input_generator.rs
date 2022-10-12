use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAutomationInputGeneratorRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub input_generator_id: String,
    pub file_id: Option<String>,
}
impl<'a> UpdateAutomationInputGeneratorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationInputGenerator> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/automation-input-generators/{input_generator_id}",
                    input_generator_id = self.input_generator_id
                ),
            );
        if let Some(ref unwrapped) = self.file_id {
            r = r.push_json(json!({ "fileId" : unwrapped }));
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
    pub fn file_id(mut self, file_id: &str) -> Self {
        self.file_id = Some(file_id.to_owned());
        self
    }
}

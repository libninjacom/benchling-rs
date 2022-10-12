use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateLabAutomationTransformRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub transform_id: String,
    pub blob_id: Option<String>,
    pub errors: Option<Vec<LabAutomationBenchlingAppError>>,
}
impl<'a> UpdateLabAutomationTransformRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LabAutomationTransform> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/automation-file-transforms/{transform_id}", transform_id = self
                    .transform_id
                ),
            );
        if let Some(ref unwrapped) = self.blob_id {
            r = r.push_json(json!({ "blobId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.errors {
            r = r.push_json(json!({ "errors" : unwrapped }));
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
    pub fn blob_id(mut self, blob_id: &str) -> Self {
        self.blob_id = Some(blob_id.to_owned());
        self
    }
    pub fn errors(mut self, errors: Vec<LabAutomationBenchlingAppError>) -> Self {
        self.errors = Some(errors);
        self
    }
}

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchBenchlingAppRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub app_id: String,
    pub description: Option<String>,
    pub name: Option<String>,
}
impl<'a> PatchBenchlingAppRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BenchlingApp> {
        let mut r = self
            .client
            .client
            .patch(&format!("/apps/{app_id}", app_id = self.app_id));
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
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
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}

use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBenchlingAppRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub description: Option<String>,
    pub name: String,
}
impl<'a> CreateBenchlingAppRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BenchlingApp> {
        let mut r = self.client.client.post("/apps");
        if let Some(ref unwrapped) = self.description {
            r = r.push_json(json!({ "description" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
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
}

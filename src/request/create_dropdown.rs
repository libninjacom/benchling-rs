use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateDropdownRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub name: String,
    pub options: Vec<DropdownOptionCreate>,
    pub registry_id: Option<String>,
}
impl<'a> CreateDropdownRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dropdown> {
        let mut r = self.client.client.post("/dropdowns");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "options" : self.options }));
        if let Some(ref unwrapped) = self.registry_id {
            r = r.push_json(json!({ "registryId" : unwrapped }));
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
    pub fn registry_id(mut self, registry_id: &str) -> Self {
        self.registry_id = Some(registry_id.to_owned());
        self
    }
}

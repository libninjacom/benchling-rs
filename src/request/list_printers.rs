use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListPrintersRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub registry_id: String,
    pub name: Option<String>,
}
impl<'a> ListPrintersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PrintersList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/registries/{registry_id}/label-printers", registry_id = self
                    .registry_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}

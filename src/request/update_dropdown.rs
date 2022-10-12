use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateDropdownRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dropdown_id: String,
    pub options: Vec<DropdownOptionUpdate>,
}
impl<'a> UpdateDropdownRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Dropdown> {
        let mut r = self
            .client
            .client
            .patch(&format!("/dropdowns/{dropdown_id}", dropdown_id = self.dropdown_id));
        r = r.push_json(json!({ "options" : self.options }));
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

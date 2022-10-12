use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveDropdownOptionsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dropdown_id: String,
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl<'a> UnarchiveDropdownOptionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DropdownOptionsArchivalChange> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/dropdowns/{dropdown_id}/options:unarchive", dropdown_id = self
                    .dropdown_id
                ),
            );
        if let Some(ref unwrapped) = self.dropdown_option_ids {
            r = r.push_json(json!({ "dropdownOptionIds" : unwrapped }));
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
    pub fn dropdown_option_ids(
        mut self,
        dropdown_option_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .dropdown_option_ids = Some(
            dropdown_option_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}

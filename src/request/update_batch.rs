use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateBatchRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub batch_id: String,
    pub default_concentration: Option<DefaultConcentrationSummary>,
    pub fields: Option<Fields>,
}
impl<'a> UpdateBatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Batch> {
        let mut r = self
            .client
            .client
            .patch(&format!("/batches/{batch_id}", batch_id = self.batch_id));
        if let Some(ref unwrapped) = self.default_concentration {
            r = r.push_json(json!({ "defaultConcentration" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
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
    pub fn default_concentration(
        mut self,
        default_concentration: DefaultConcentrationSummary,
    ) -> Self {
        self.default_concentration = Some(default_concentration);
        self
    }
    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }
}

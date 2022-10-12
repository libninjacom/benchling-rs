use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetPlateRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub plate_id: String,
    pub returning: Option<String>,
}
impl<'a> GetPlateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Plate> {
        let mut r = self
            .client
            .client
            .get(&format!("/plates/{plate_id}", plate_id = self.plate_id));
        if let Some(ref unwrapped) = self.returning {
            r = r.push_query("returning", &unwrapped.to_string());
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
    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_owned());
        self
    }
}

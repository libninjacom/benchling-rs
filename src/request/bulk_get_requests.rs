use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetRequestsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub request_ids: Option<String>,
    pub display_ids: Option<String>,
}
impl<'a> BulkGetRequestsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RequestsBulkGet> {
        let mut r = self.client.client.get("/requests:bulk-get");
        if let Some(ref unwrapped) = self.request_ids {
            r = r.push_query("requestIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.display_ids {
            r = r.push_query("displayIds", &unwrapped.to_string());
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
    pub fn request_ids(mut self, request_ids: &str) -> Self {
        self.request_ids = Some(request_ids.to_owned());
        self
    }
    pub fn display_ids(mut self, display_ids: &str) -> Self {
        self.display_ids = Some(display_ids.to_owned());
        self
    }
}

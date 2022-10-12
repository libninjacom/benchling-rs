use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetRequestFulfillmentRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub request_fulfillment_id: String,
}
impl<'a> GetRequestFulfillmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RequestFulfillment> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/request-fulfillments/{request_fulfillment_id}",
                    request_fulfillment_id = self.request_fulfillment_id
                ),
            );
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

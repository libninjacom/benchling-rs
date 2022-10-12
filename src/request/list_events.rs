use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListEventsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub page_size: Option<i64>,
    pub next_token: Option<String>,
    pub created_at_gte: Option<String>,
    pub starting_after: Option<String>,
    pub event_types: Option<String>,
    pub poll: Option<bool>,
}
impl<'a> ListEventsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EventsPaginatedList> {
        let mut r = self.client.client.get("/events");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_at_gte {
            r = r.push_query("createdAt.gte", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("startingAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.event_types {
            r = r.push_query("eventTypes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.poll {
            r = r.push_query("poll", &unwrapped.to_string());
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
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn created_at_gte(mut self, created_at_gte: &str) -> Self {
        self.created_at_gte = Some(created_at_gte.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn event_types(mut self, event_types: &str) -> Self {
        self.event_types = Some(event_types.to_owned());
        self
    }
    pub fn poll(mut self, poll: bool) -> Self {
        self.poll = Some(poll);
        self
    }
}

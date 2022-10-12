use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListFeaturesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub page_size: Option<i64>,
    pub next_token: Option<String>,
    pub name: Option<String>,
    pub ids: Option<String>,
    pub names_any_of_case_sensitive: Option<String>,
    pub feature_library_id: Option<String>,
    pub feature_type: Option<String>,
    pub match_type: Option<String>,
}
impl<'a> ListFeaturesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<FeaturesPaginatedList> {
        let mut r = self.client.client.get("/features");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of_case_sensitive {
            r = r.push_query("names.anyOf.caseSensitive", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.feature_library_id {
            r = r.push_query("featureLibraryId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.feature_type {
            r = r.push_query("featureType", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.match_type {
            r = r.push_query("matchType", &unwrapped.to_string());
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
    pub fn names_any_of_case_sensitive(
        mut self,
        names_any_of_case_sensitive: &str,
    ) -> Self {
        self.names_any_of_case_sensitive = Some(names_any_of_case_sensitive.to_owned());
        self
    }
    pub fn feature_library_id(mut self, feature_library_id: &str) -> Self {
        self.feature_library_id = Some(feature_library_id.to_owned());
        self
    }
    pub fn feature_type(mut self, feature_type: &str) -> Self {
        self.feature_type = Some(feature_type.to_owned());
        self
    }
    pub fn match_type(mut self, match_type: &str) -> Self {
        self.match_type = Some(match_type.to_owned());
        self
    }
}

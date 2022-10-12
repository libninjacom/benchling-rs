use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListLocationsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub page_size: Option<i64>,
    pub next_token: Option<String>,
    pub sort: Option<String>,
    pub schema_id: Option<String>,
    pub schema_fields: Option<SchemaFieldsQueryParam>,
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub name_includes: Option<String>,
    pub ancestor_storage_id: Option<String>,
    pub archive_reason: Option<String>,
    pub ids: Option<String>,
    pub barcodes: Option<String>,
    pub names_any_of: Option<String>,
    pub names_any_of_case_sensitive: Option<String>,
    pub creator_ids: Option<String>,
}
impl<'a> ListLocationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LocationsPaginatedList> {
        let mut r = self.client.client.get("/locations");
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_query("schemaId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_fields {
            r = r.push_query("schemaFields", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name_includes {
            r = r.push_query("nameIncludes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ancestor_storage_id {
            r = r.push_query("ancestorStorageId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.archive_reason {
            r = r.push_query("archiveReason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.barcodes {
            r = r.push_query("barcodes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of {
            r = r.push_query("names.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of_case_sensitive {
            r = r.push_query("names.anyOf.caseSensitive", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.creator_ids {
            r = r.push_query("creatorIds", &unwrapped.to_string());
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
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
    pub fn schema_fields(mut self, schema_fields: SchemaFieldsQueryParam) -> Self {
        self.schema_fields = Some(schema_fields);
        self
    }
    pub fn modified_at(mut self, modified_at: &str) -> Self {
        self.modified_at = Some(modified_at.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn name_includes(mut self, name_includes: &str) -> Self {
        self.name_includes = Some(name_includes.to_owned());
        self
    }
    pub fn ancestor_storage_id(mut self, ancestor_storage_id: &str) -> Self {
        self.ancestor_storage_id = Some(ancestor_storage_id.to_owned());
        self
    }
    pub fn archive_reason(mut self, archive_reason: &str) -> Self {
        self.archive_reason = Some(archive_reason.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
    pub fn barcodes(mut self, barcodes: &str) -> Self {
        self.barcodes = Some(barcodes.to_owned());
        self
    }
    pub fn names_any_of(mut self, names_any_of: &str) -> Self {
        self.names_any_of = Some(names_any_of.to_owned());
        self
    }
    pub fn names_any_of_case_sensitive(
        mut self,
        names_any_of_case_sensitive: &str,
    ) -> Self {
        self.names_any_of_case_sensitive = Some(names_any_of_case_sensitive.to_owned());
        self
    }
    pub fn creator_ids(mut self, creator_ids: &str) -> Self {
        self.creator_ids = Some(creator_ids.to_owned());
        self
    }
}

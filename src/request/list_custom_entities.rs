use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListCustomEntitiesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub sort: Option<String>,
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub returning: Option<String>,
    pub name_includes: Option<String>,
    pub folder_id: Option<String>,
    pub mentioned_in: Option<String>,
    pub project_id: Option<String>,
    pub registry_id: Option<String>,
    pub schema_id: Option<String>,
    pub schema_fields: Option<SchemaFieldsQueryParam>,
    pub archive_reason: Option<String>,
    pub mentions: Option<String>,
    pub ids: Option<String>,
    pub names_any_of: Option<String>,
    pub names_any_of_case_sensitive: Option<String>,
    pub entity_registry_ids_any_of: Option<String>,
    pub creator_ids: Option<String>,
    pub author_ids_any_of: Option<String>,
}
impl<'a> ListCustomEntitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomEntitiesPaginatedList> {
        let mut r = self.client.client.get("/custom-entities");
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.returning {
            r = r.push_query("returning", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name_includes {
            r = r.push_query("nameIncludes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.folder_id {
            r = r.push_query("folderId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.mentioned_in {
            r = r.push_query("mentionedIn", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_query("projectId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.registry_id {
            r = r.push_query("registryId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_query("schemaId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_fields {
            r = r.push_query("schemaFields", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.archive_reason {
            r = r.push_query("archiveReason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.mentions {
            r = r.push_query("mentions", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of {
            r = r.push_query("names.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of_case_sensitive {
            r = r.push_query("names.anyOf.caseSensitive", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.entity_registry_ids_any_of {
            r = r.push_query("entityRegistryIds.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.creator_ids {
            r = r.push_query("creatorIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.author_ids_any_of {
            r = r.push_query("authorIds.anyOf", &unwrapped.to_string());
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
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
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
    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_owned());
        self
    }
    pub fn name_includes(mut self, name_includes: &str) -> Self {
        self.name_includes = Some(name_includes.to_owned());
        self
    }
    pub fn folder_id(mut self, folder_id: &str) -> Self {
        self.folder_id = Some(folder_id.to_owned());
        self
    }
    pub fn mentioned_in(mut self, mentioned_in: &str) -> Self {
        self.mentioned_in = Some(mentioned_in.to_owned());
        self
    }
    pub fn project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_owned());
        self
    }
    pub fn registry_id(mut self, registry_id: &str) -> Self {
        self.registry_id = Some(registry_id.to_owned());
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
    pub fn archive_reason(mut self, archive_reason: &str) -> Self {
        self.archive_reason = Some(archive_reason.to_owned());
        self
    }
    pub fn mentions(mut self, mentions: &str) -> Self {
        self.mentions = Some(mentions.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
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
    pub fn entity_registry_ids_any_of(
        mut self,
        entity_registry_ids_any_of: &str,
    ) -> Self {
        self.entity_registry_ids_any_of = Some(entity_registry_ids_any_of.to_owned());
        self
    }
    pub fn creator_ids(mut self, creator_ids: &str) -> Self {
        self.creator_ids = Some(creator_ids.to_owned());
        self
    }
    pub fn author_ids_any_of(mut self, author_ids_any_of: &str) -> Self {
        self.author_ids_any_of = Some(author_ids_any_of.to_owned());
        self
    }
}

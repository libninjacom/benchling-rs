use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateDnaOligoRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub oligo_id: String,
    pub aliases: Vec<String>,
    pub author_ids: Vec<String>,
    pub bases: String,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: String,
    pub name: String,
    pub schema_id: String,
}
impl<'a> UpdateDnaOligoRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DnaOligo> {
        let mut r = self
            .client
            .client
            .patch(&format!("/dna-oligos/{oligo_id}", oligo_id = self.oligo_id));
        r = r.push_json(json!({ "aliases" : self.aliases }));
        r = r.push_json(json!({ "authorIds" : self.author_ids }));
        r = r.push_json(json!({ "bases" : self.bases }));
        r = r.push_json(json!({ "customFields" : self.custom_fields }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "schemaId" : self.schema_id }));
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
pub struct UpdateDnaOligoRequired<'a> {
    pub oligo_id: &'a str,
    pub aliases: &'a [&'a str],
    pub author_ids: &'a [&'a str],
    pub bases: &'a str,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: &'a str,
    pub name: &'a str,
    pub schema_id: &'a str,
}
impl<'a> UpdateDnaOligoRequired<'a> {}

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AigGenerateInputAsyncTask {}
impl std::fmt::Display for AigGenerateInputAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AopProcessOutputAsyncTask {}
impl std::fmt::Display for AopProcessOutputAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaAnnotation {
    pub color: Option<String>,
    pub end: Option<i64>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub start: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for AaAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequence {
    ///Array of aliases
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "aminoAcids")]
    ///Amino acids of the AA sequence.
    pub amino_acids: Option<String>,
    ///Array of annotation objects on the AA sequence.
    pub annotations: Option<Vec<AaAnnotation>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the AA Sequence in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "createdAt")]
    ///DateTime the AA sequence was created.
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    ///Custom fields set on the AA sequence.
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    ///Registry ID of the AA sequence if registered.
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the AA sequence.
    pub folder_id: Option<String>,
    ///ID of the AA sequence.
    pub id: Option<String>,
    ///Number of amino acids in the AA sequence.
    pub length: Option<i64>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the AA sequence was last modified.
    pub modified_at: Option<String>,
    ///Name of the AA sequence.
    pub name: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    ///Registry the AA sequence is registered in.
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    ///URL of the protein.
    pub web_url: Option<String>,
}
impl std::fmt::Display for AaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceBaseRequest {
    ///Aliases to add to the AA sequence
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "aminoAcids")]
    /**Amino acids for the AA sequence.
*/
    pub amino_acids: Option<String>,
    /**Annotations to create on the AA sequence.
*/
    pub annotations: Option<Vec<AaAnnotation>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the AA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the AA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Fields to set on the AA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    /**ID of the folder containing the AA sequence.
*/
    pub folder_id: Option<String>,
    /**Name of the AA sequence.
*/
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    /**ID of the AA sequence's schema.
*/
    pub schema_id: Option<String>,
}
impl std::fmt::Display for AaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBaseRequestForCreate(pub AaSequenceBaseRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBulkCreate(pub AaSequenceCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBulkUpdate(pub serde_json::Value, AaSequenceBaseRequest);
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceCreate(
    pub AaSequenceBaseRequestForCreate,
    CreateEntityIntoRegistry,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceRequestRegistryFields {
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for AaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceSummary {
    #[serde(rename = "entityType")]
    pub entity_type: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for AaSequenceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceUpdate(pub AaSequenceBaseRequest, AaSequenceRequestRegistryFields);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceWithEntityType {}
impl std::fmt::Display for AaSequenceWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesArchivalChange {
    #[serde(rename = "aaSequenceIds")]
    pub aa_sequence_ids: Option<Vec<String>>,
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AaSequencesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesArchive {
    #[serde(rename = "aaSequenceIds")]
    pub aa_sequence_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for AaSequencesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesBulkCreateRequest {
    #[serde(rename = "aaSequences")]
    pub aa_sequences: Option<Vec<AaSequenceBulkCreate>>,
}
impl std::fmt::Display for AaSequencesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesBulkGet {
    #[serde(rename = "aaSequences")]
    pub aa_sequences: Option<Vec<AaSequence>>,
}
impl std::fmt::Display for AaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesBulkUpdateRequest {
    #[serde(rename = "aaSequences")]
    pub aa_sequences: Option<Vec<AaSequenceBulkUpdate>>,
}
impl std::fmt::Display for AaSequencesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesPaginatedList {
    #[serde(rename = "aaSequences")]
    pub aa_sequences: Option<Vec<AaSequence>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AaSequencesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesUnarchive {
    #[serde(rename = "aaSequenceIds")]
    pub aa_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AlignedNucleotideSequence {
    pub bases: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "pairwiseIdentity")]
    /**Fraction of bases between trimStart and trimEnd that match the template bases. Only present for Template Alignments; Will be empty for Consensus Alignments.
*/
    pub pairwise_identity: Option<f64>,
    #[serde(rename = "sequenceId")]
    pub sequence_id: Option<String>,
    #[serde(rename = "trimEnd")]
    pub trim_end: Option<i64>,
    #[serde(rename = "trimStart")]
    pub trim_start: Option<i64>,
}
impl std::fmt::Display for AlignedNucleotideSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AlignedSequence {
    pub bases: Option<String>,
    #[serde(rename = "dnaSequenceId")]
    pub dna_sequence_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "pairwiseIdentity")]
    /**Fraction of bases between trimStart and trimEnd that match the template bases. Only present for Template Alignments; Will be empty for Consensus Alignments.
*/
    pub pairwise_identity: Option<f64>,
    #[serde(rename = "trimEnd")]
    pub trim_end: Option<i64>,
    #[serde(rename = "trimStart")]
    pub trim_start: Option<i64>,
}
impl std::fmt::Display for AlignedSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ArchiveRecord {
    pub reason: Option<String>,
}
impl std::fmt::Display for ArchiveRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayFieldsCreate {}
impl std::fmt::Display for AssayFieldsCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResult {
    #[serde(rename = "archiveRecord")]
    /**ArchiveRecord Resource if the result is archived. This is null if the result is not archived.
*/
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "createdAt")]
    ///DateTime at which the the result was created
    pub created_at: Option<String>,
    ///UserSummary Resource of who created the request
    pub creator: Option<UserSummary>,
    #[serde(rename = "entryId")]
    ///ID of the entry that this result is attached to
    pub entry_id: Option<String>,
    #[serde(rename = "fieldValidation")]
    /**Object mapping field names to a UserValidation Resource object for that field. To **set** validation for a result, you *must* use this object.
*/
    pub field_validation: Option<serde_json::Value>,
    pub fields: Option<Fields>,
    ///ID of the result
    pub id: Option<String>,
    #[serde(rename = "isReviewed")]
    ///Whether or not this result is attached to an accepted entry
    pub is_reviewed: Option<bool>,
    #[serde(rename = "projectId")]
    ///ID of the project to insert the result into
    pub project_id: Option<String>,
    ///Schema that the result belongs to
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "validationComment")]
    pub validation_comment: Option<String>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}
impl std::fmt::Display for AssayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultCreate {
    #[serde(rename = "fieldValidation")]
    /**Dictionary mapping field names to UserValidation Resources.
*/
    pub field_validation: Option<serde_json::Value>,
    ///Dictionary of result fields
    pub fields: serde_json::Value,
    ///UUID
    pub id: Option<String>,
    #[serde(rename = "projectId")]
    /**The project that the assay result should be uploaded to. Only users with read access to the project will be able to read the assay result. Leaving this empty will result in only the creator having read access.
*/
    pub project_id: Option<String>,
    #[serde(rename = "schemaId")]
    ///ID of result schema under which to upload this result
    pub schema_id: String,
}
impl std::fmt::Display for AssayResultCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultIdsRequest {
    #[serde(rename = "assayResultIds")]
    pub assay_result_ids: Vec<String>,
}
impl std::fmt::Display for AssayResultIdsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultIdsResponse {
    #[serde(rename = "assayResultIds")]
    pub assay_result_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AssayResultIdsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultSchema(pub BaseAssaySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultSchemasPaginatedList {
    #[serde(rename = "assayResultSchemas")]
    pub assay_result_schemas: Option<Vec<AssayResultSchema>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayResultSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultTransactionCreateResponse {
    pub id: Option<String>,
}
impl std::fmt::Display for AssayResultTransactionCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultsArchive(pub AssayResultIdsRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsBulkCreateRequest {
    #[serde(rename = "assayResults")]
    pub assay_results: Vec<AssayResultCreate>,
}
impl std::fmt::Display for AssayResultsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsBulkGet {
    #[serde(rename = "assayResults")]
    pub assay_results: Option<Vec<AssayResult>>,
}
impl std::fmt::Display for AssayResultsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsCreateResponse {
    #[serde(rename = "assayResults")]
    pub assay_results: Option<Vec<String>>,
}
impl std::fmt::Display for AssayResultsCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsPaginatedList {
    #[serde(rename = "assayResults")]
    pub assay_results: Option<Vec<AssayResult>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayResultsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRun {
    #[serde(rename = "apiURL")]
    ///The canonical url of the Run in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "entryId")]
    pub entry_id: Option<String>,
    pub fields: Option<Fields>,
    pub id: Option<String>,
    #[serde(rename = "isReviewed")]
    pub is_reviewed: Option<bool>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "validationComment")]
    pub validation_comment: Option<String>,
    #[serde(rename = "validationStatus")]
    ///Must be either VALID or INVALID
    pub validation_status: Option<String>,
}
impl std::fmt::Display for AssayRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunCreate {
    ///Object of assay run fields
    pub fields: serde_json::Value,
    ///ID of assay run
    pub id: Option<String>,
    #[serde(rename = "projectId")]
    /**The project that the assay run should be uploaded to. Only users with read access to the project will be able to read the assay run. Leaving this empty will result in only the creator having read access.
*/
    pub project_id: Option<String>,
    #[serde(rename = "schemaId")]
    ///ID of assay schema that assay run conforms to
    pub schema_id: String,
    #[serde(rename = "validationComment")]
    ///Additional information about the validation status
    pub validation_comment: Option<String>,
    #[serde(rename = "validationStatus")]
    ///Must be either VALID or INVALID
    pub validation_status: Option<String>,
}
impl std::fmt::Display for AssayRunCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunNotePart(pub BaseNotePart, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunSchema(pub BaseAssaySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunSchemasPaginatedList {
    #[serde(rename = "assayRunSchemas")]
    pub assay_run_schemas: Option<Vec<AssayRunSchema>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayRunSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunUpdate {
    pub fields: Option<Fields>,
}
impl std::fmt::Display for AssayRunUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunUpdatedFieldsEvent(
    pub EventBase,
    UpdateEventMixin,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub enum AssayRunValidationStatus {
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsArchivalChange {
    #[serde(rename = "assayRunIds")]
    pub assay_run_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AssayRunsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsArchive {
    #[serde(rename = "assayRunIds")]
    pub assay_run_ids: Vec<String>,
    /**The reason for archiving the provided Assay Runs. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for AssayRunsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkCreateRequest {
    #[serde(rename = "assayRuns")]
    pub assay_runs: Vec<AssayRunCreate>,
}
impl std::fmt::Display for AssayRunsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkCreateResponse {
    #[serde(rename = "assayRuns")]
    pub assay_runs: Option<Vec<String>>,
}
impl std::fmt::Display for AssayRunsBulkCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkGet {
    #[serde(rename = "assayRuns")]
    pub assay_runs: Option<Vec<AssayRun>>,
}
impl std::fmt::Display for AssayRunsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsPaginatedList {
    #[serde(rename = "assayRuns")]
    pub assay_runs: Option<Vec<AssayRun>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayRunsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsUnarchive {
    #[serde(rename = "assayRunIds")]
    pub assay_run_ids: Vec<String>,
}
impl std::fmt::Display for AssayRunsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsyncTask {
    /**Present only when status is FAILED for a bulk task. Contains information about the individual errors in the bulk task.
*/
    pub errors: Option<serde_json::Value>,
    ///Present only when status is FAILED. Contains information about the error.
    pub message: Option<String>,
    ///Present only when status is SUCCEEDED. response can be empty if there is no data to be returned.
    pub response: Option<serde_json::Value>,
    ///The current state of the task.
    pub status: String,
}
impl std::fmt::Display for AsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsyncTaskLink {
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
}
impl std::fmt::Display for AsyncTaskLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoAnnotateAaSequences {
    #[serde(rename = "aaSequenceIds")]
    ///Array of AA sequence IDs.
    pub aa_sequence_ids: Vec<String>,
    #[serde(rename = "featureLibraryIds")]
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateAaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoAnnotateDnaSequences {
    #[serde(rename = "dnaSequenceIds")]
    ///Array of DNA sequence IDs.
    pub dna_sequence_ids: Vec<String>,
    #[serde(rename = "featureLibraryIds")]
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateDnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoAnnotateRnaSequences {
    #[serde(rename = "featureLibraryIds")]
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
    #[serde(rename = "rnaSequenceIds")]
    ///Array of RNA sequence IDs.
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateRnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillPartsAsyncTask {}
impl std::fmt::Display for AutofillPartsAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillRnaSequences {
    #[serde(rename = "rnaSequenceIds")]
    ///Array of RNA sequence IDs.
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutofillRnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillSequences {
    #[serde(rename = "dnaSequenceIds")]
    ///Array of DNA sequence IDs.
    pub dna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutofillSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillTranslationsAsyncTask {}
impl std::fmt::Display for AutofillTranslationsAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationFile {
    #[serde(rename = "assayRunId")]
    pub assay_run_id: Option<String>,
    #[serde(rename = "automationFileConfig")]
    pub automation_file_config: Option<serde_json::Value>,
    pub file: Option<Blob>,
    pub id: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for AutomationFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationFileInputsPaginatedList {
    #[serde(rename = "automationInputGenerators")]
    pub automation_input_generators: Option<Vec<AutomationInputGenerator>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AutomationFileInputsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationInputGenerator(pub AutomationFile, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationInputGeneratorCompletedV2BetaEvent(
    pub EventBase,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationInputGeneratorUpdate {
    #[serde(rename = "fileId")]
    ///The ID of the file (blob) associated with the input generator. Set to `null` to remove an existing file from the generator.
    pub file_id: Option<String>,
}
impl std::fmt::Display for AutomationInputGeneratorUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessor(pub AutomationFile, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorArchivalChange {
    #[serde(rename = "automationOutputProcessorIds")]
    pub automation_output_processor_ids: Option<Vec<String>>,
    #[serde(rename = "resultIds")]
    pub result_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AutomationOutputProcessorArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessorCompletedV2BetaEvent(
    pub EventBase,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorCreate {
    #[serde(rename = "assayRunId")]
    pub assay_run_id: String,
    #[serde(rename = "automationFileConfigName")]
    pub automation_file_config_name: String,
    #[serde(rename = "completeWithErrors")]
    ///Specifies whether file processing should complete with errors. False means any error in output file processing will result in no actions being committed. True means that if row-level errors occur, then failing rows and their errors will be saved to errorFile, and actions from successful rows will be committed.
    pub complete_with_errors: Option<bool>,
    #[serde(rename = "fileId")]
    ///The ID of a blob link to process.
    pub file_id: String,
}
impl std::fmt::Display for AutomationOutputProcessorCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorUpdate {
    #[serde(rename = "fileId")]
    ///The ID of a blob link to process.
    pub file_id: String,
}
impl std::fmt::Display for AutomationOutputProcessorUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessorUploadedV2BetaEvent(
    pub EventBase,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsArchive {
    #[serde(rename = "automationOutputProcessorIds")]
    ///Array of automation output processor IDs
    pub automation_output_processor_ids: Vec<String>,
    /**The reason that the output processors are being archived. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: Option<String>,
}
impl std::fmt::Display for AutomationOutputProcessorsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsPaginatedList {
    #[serde(rename = "automationOutputProcessors")]
    pub automation_output_processors: Option<Vec<AutomationOutputProcessor>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for AutomationOutputProcessorsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsUnarchive {
    #[serde(rename = "automationOutputProcessorIds")]
    ///Array of automation output processor IDs
    pub automation_output_processor_ids: Vec<String>,
}
impl std::fmt::Display for AutomationOutputProcessorsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationProgressStats {
    #[serde(rename = "rowsFailed")]
    pub rows_failed: Option<i64>,
    #[serde(rename = "rowsSucceeded")]
    pub rows_succeeded: Option<i64>,
    #[serde(rename = "rowsUnprocessed")]
    pub rows_unprocessed: Option<i64>,
}
impl std::fmt::Display for AutomationProgressStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BadRequestError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequestErrorBulk(pub BadRequestError, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BarcodeValidationResult {
    ///Barcode to validate.
    pub barcode: Option<String>,
    #[serde(rename = "isValid")]
    ///Whether the barcode is valid.
    pub is_valid: Option<bool>,
    ///If barcode is not valid, a message string explaining the error.
    pub message: Option<String>,
}
impl std::fmt::Display for BarcodeValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BarcodeValidationResults {
    #[serde(rename = "validationResults")]
    pub validation_results: Option<Vec<BarcodeValidationResult>>,
}
impl std::fmt::Display for BarcodeValidationResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BarcodesList {
    ///Array of barcodes to validate.
    pub barcodes: Vec<String>,
}
impl std::fmt::Display for BarcodesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseAssaySchema(pub Schema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseError {
    pub message: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "userMessage")]
    pub user_message: Option<String>,
}
impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseNotePart {
    ///All notes have an indentation level - the default is 0 for no indent. For lists, indentation gives notes hierarchy - a bulleted list with children is modeled as one note part with indentation 1 followed by note parts with indentation 2, for example.
    pub indentation: Option<i64>,
    #[serde(rename = "type")]
    ///The type of the note.  Type determines what other fields are present.
    pub type_: Option<String>,
}
impl std::fmt::Display for BaseNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Batch {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "createdAt")]
    ///DateTime at which the the result was created
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "defaultConcentration")]
    pub default_concentration: Option<Measurement>,
    pub entity: Option<serde_json::Value>,
    pub fields: Option<Fields>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchCreate {
    #[serde(rename = "defaultConcentration")]
    pub default_concentration: Option<DefaultConcentrationSummary>,
    #[serde(rename = "entityId")]
    ///API identifier for the entity that the batch will be added to.
    pub entity_id: Option<String>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for BatchCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrInaccessibleResource(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchSchemasList {
    #[serde(rename = "batchSchemas")]
    pub batch_schemas: Option<Vec<BatchSchema>>,
}
impl std::fmt::Display for BatchSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSchemasPaginatedList(pub BatchSchemasList, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchUpdate {
    #[serde(rename = "defaultConcentration")]
    pub default_concentration: Option<DefaultConcentrationSummary>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for BatchUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BatchesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesArchive {
    #[serde(rename = "batchIds")]
    pub batch_ids: Vec<String>,
    /**The reason for archiving the provided Batches. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for BatchesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesBulkGet {
    pub batches: Option<Vec<Batch>>,
}
impl std::fmt::Display for BatchesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesPaginatedList {
    pub batches: Option<Vec<Batch>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for BatchesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesUnarchive {
    #[serde(rename = "batchIds")]
    pub batch_ids: Vec<String>,
}
impl std::fmt::Display for BatchesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingApp(pub BenchlingAppUpdate, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppCreate {
    pub description: Option<String>,
    pub name: String,
}
impl std::fmt::Display for BenchlingAppCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppUpdate {
    pub description: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for BenchlingAppUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsArchivalChange {
    #[serde(rename = "appIds")]
    pub app_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BenchlingAppsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsArchive {
    #[serde(rename = "appIds")]
    ///Array of app IDs
    pub app_ids: Vec<String>,
    ///Reason that apps are being archived. Actual reason enum varies by tenant.
    pub reason: String,
}
impl std::fmt::Display for BenchlingAppsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingAppsPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsUnarchive {
    #[serde(rename = "appIds")]
    ///Array of app IDs
    pub app_ids: Vec<String>,
}
impl std::fmt::Display for BenchlingAppsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Blob {
    ///The universally unique identifier (UUID) for the blob.
    pub id: Option<String>,
    #[serde(rename = "mimeType")]
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    ///Name of the blob
    pub name: Option<String>,
    #[serde(rename = "type")]
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: Option<String>,
    #[serde(rename = "uploadStatus")]
    pub upload_status: Option<String>,
}
impl std::fmt::Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobComplete {
    pub parts: Option<Vec<BlobPart>>,
}
impl std::fmt::Display for BlobComplete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobCreate {
    ///base64 encoded file contents
    pub data64: String,
    /**The MD5 hash of the blob part. Note: this should be the hash of the raw data of the blob part, not the hash of the base64 encoding.
*/
    pub md5: String,
    #[serde(rename = "mimeType")]
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    ///Name of the blob
    pub name: String,
    #[serde(rename = "type")]
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: String,
}
impl std::fmt::Display for BlobCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobMultipartCreate {
    #[serde(rename = "mimeType")]
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    ///Name of the blob
    pub name: String,
    #[serde(rename = "type")]
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: String,
}
impl std::fmt::Display for BlobMultipartCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobPart {
    #[serde(rename = "eTag")]
    pub e_tag: Option<String>,
    #[serde(rename = "partNumber")]
    pub part_number: Option<i64>,
}
impl std::fmt::Display for BlobPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobPartCreate {
    pub data64: String,
    pub md5: String,
    #[serde(rename = "partNumber")]
    /**An integer between 1 to 10,000, inclusive. The part number must be unique per part and indicates the ordering of the part inside the final blob. The part numbers do not need to be consecutive.
*/
    pub part_number: i64,
}
impl std::fmt::Display for BlobPartCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobUrl {
    #[serde(rename = "downloadURL")]
    ///a pre-signed download url.
    pub download_url: Option<String>,
    #[serde(rename = "expiresAt")]
    ///The unix time that the download URL expires at.
    pub expires_at: Option<i64>,
}
impl std::fmt::Display for BlobUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobsBulkGet {
    pub blobs: Option<Vec<Blob>>,
}
impl std::fmt::Display for BlobsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Box {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub barcode: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "emptyContainers")]
    ///The number of containers in the box that have no contents.
    pub empty_containers: Option<i64>,
    #[serde(rename = "emptyPositions")]
    ///The number of empty positions for adding additional containers in the box.
    pub empty_positions: Option<i64>,
    pub fields: Option<Fields>,
    #[serde(rename = "filledPositions")]
    ///The number of containers currently in the box.
    pub filled_positions: Option<i64>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    ///The size of the box (i.e. how many containers it can store).
    pub size: Option<i64>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxContentsPaginatedList {
    pub containers: Option<Vec<ContainerWithCoordinates>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for BoxContentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxCreate {
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "schemaId")]
    pub schema_id: String,
}
impl std::fmt::Display for BoxCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCreationTableNotePart(
    pub BaseNotePart,
    StructuredTableApiIdentifiers,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxSchemasList {
    #[serde(rename = "boxSchemas")]
    pub box_schemas: Option<Vec<BoxSchema>>,
}
impl std::fmt::Display for BoxSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSchemasPaginatedList(pub BoxSchemasList, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxUpdate {
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
}
impl std::fmt::Display for BoxUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesArchivalChange {
    #[serde(rename = "boxIds")]
    pub box_ids: Option<Vec<String>>,
    #[serde(rename = "containerIds")]
    pub container_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BoxesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesArchive {
    #[serde(rename = "boxIds")]
    ///Array of box IDs
    pub box_ids: Vec<String>,
    /**Reason that boxes are being archived.
*/
    pub reason: String,
    #[serde(rename = "shouldRemoveBarcodes")]
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: Option<bool>,
}
impl std::fmt::Display for BoxesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesBulkGet {
    pub boxes: Option<Vec<Box>>,
}
impl std::fmt::Display for BoxesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesPaginatedList {
    pub boxes: Option<Vec<Box>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for BoxesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesUnarchive {
    #[serde(rename = "boxIds")]
    ///Array of box IDs
    pub box_ids: Vec<String>,
}
impl std::fmt::Display for BoxesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateAaSequencesAsyncTask {}
impl std::fmt::Display for BulkCreateAaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateContainersAsyncTask {}
impl std::fmt::Display for BulkCreateContainersAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateCustomEntitiesAsyncTask {}
impl std::fmt::Display for BulkCreateCustomEntitiesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateDnaOligosAsyncTask {}
impl std::fmt::Display for BulkCreateDnaOligosAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateDnaSequencesAsyncTask {}
impl std::fmt::Display for BulkCreateDnaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateFeaturesAsyncTask {}
impl std::fmt::Display for BulkCreateFeaturesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateRnaOligosAsyncTask {}
impl std::fmt::Display for BulkCreateRnaOligosAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateRnaSequencesAsyncTask {}
impl std::fmt::Display for BulkCreateRnaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkRegisterEntitiesAsyncTask {}
impl std::fmt::Display for BulkRegisterEntitiesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateAaSequencesAsyncTask {}
impl std::fmt::Display for BulkUpdateAaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateContainersAsyncTask {}
impl std::fmt::Display for BulkUpdateContainersAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateCustomEntitiesAsyncTask {}
impl std::fmt::Display for BulkUpdateCustomEntitiesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateDnaOligosAsyncTask {}
impl std::fmt::Display for BulkUpdateDnaOligosAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateDnaSequencesAsyncTask {}
impl std::fmt::Display for BulkUpdateDnaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateRnaOligosAsyncTask {}
impl std::fmt::Display for BulkUpdateRnaOligosAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateRnaSequencesAsyncTask {}
impl std::fmt::Display for BulkUpdateRnaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckboxNotePart(pub BaseNotePart, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CheckoutRecord {
    pub assignee: Option<serde_json::Value>,
    pub comment: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for CheckoutRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConflictError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Container {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub barcode: Option<String>,
    #[serde(rename = "checkoutRecord")]
    pub checkout_record: Option<CheckoutRecord>,
    pub contents: Option<Vec<ContainerContent>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "parentStorageSchema")]
    pub parent_storage_schema: Option<SchemaSummary>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: Option<ContainerQuantity>,
    pub schema: Option<SchemaSummary>,
    pub volume: Option<DeprecatedContainerVolumeForResponse>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerBulkUpdateItem(pub ContainerWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerContent {
    pub batch: Option<BatchOrInaccessibleResource>,
    pub concentration: Option<Measurement>,
    pub entity: Option<EntityOrInaccessibleResource>,
}
impl std::fmt::Display for ContainerContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerContentUpdate {
    pub concentration: Measurement,
}
impl std::fmt::Display for ContainerContentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerContentsList {
    pub contents: Option<Vec<ContainerContent>>,
}
impl std::fmt::Display for ContainerContentsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerCreate(pub ContainerWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerLabels {
    pub barcode: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for ContainerLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerQuantity {
    pub units: Option<String>,
    pub value: Option<f64>,
}
impl std::fmt::Display for ContainerQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerSchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerSchemasList {
    #[serde(rename = "containerSchemas")]
    pub container_schemas: Option<Vec<ContainerSchema>>,
}
impl std::fmt::Display for ContainerSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerSchemasPaginatedList(pub ContainerSchemasList, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerTransfer {}
impl std::fmt::Display for ContainerTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerTransferBase {
    #[serde(rename = "sourceBatchId")]
    /**ID of the batch that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_batch_id: Option<String>,
    #[serde(rename = "sourceContainerId")]
    /**ID of the container that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_container_id: Option<String>,
    #[serde(rename = "sourceEntityId")]
    /**ID of the entity that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_entity_id: Option<String>,
    #[serde(rename = "transferQuantity")]
    /**This represents the quantity of the source to be transferred into the destination container. Supports mass, volume, and other quantities. Required in place of transferVolume.
*/
    pub transfer_quantity: Option<ContainerQuantity>,
    #[serde(rename = "transferVolume")]
    /**Deprecated - use transferQuantity instead.
*/
    pub transfer_volume: Option<DeprecatedContainerVolumeForInput>,
}
impl std::fmt::Display for ContainerTransferBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerTransferDestinationContentsItem {
    pub concentration: Option<Measurement>,
    #[serde(rename = "entityId")]
    pub entity_id: String,
}
impl std::fmt::Display for ContainerTransferDestinationContentsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerUpdate(pub ContainerWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerWithCoordinates(pub Container, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerWriteBase {
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    ///ID of containing parent storage, can also specify a coordinate for plates and boxes (e.g. plt_2bAks9dx:a2).
    pub parent_storage_id: Option<String>,
}
impl std::fmt::Display for ContainerWriteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersArchivalChange {
    #[serde(rename = "containerIds")]
    pub container_ids: Option<Vec<String>>,
}
impl std::fmt::Display for ContainersArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersArchive {
    #[serde(rename = "containerIds")]
    ///Array of container IDs
    pub container_ids: Vec<String>,
    /**Reason that containers are being archived.
*/
    pub reason: String,
    #[serde(rename = "shouldRemoveBarcodes")]
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: Option<bool>,
}
impl std::fmt::Display for ContainersArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersBulkCreateRequest {
    pub containers: Vec<ContainerCreate>,
}
impl std::fmt::Display for ContainersBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersBulkUpdateRequest {
    pub containers: Vec<ContainerBulkUpdateItem>,
}
impl std::fmt::Display for ContainersBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersCheckin {
    pub comments: Option<String>,
    #[serde(rename = "containerIds")]
    ///Array of container IDs.
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersCheckin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersCheckout {
    #[serde(rename = "assigneeId")]
    ///User or Team API ID.
    pub assignee_id: String,
    pub comment: Option<String>,
    #[serde(rename = "containerIds")]
    ///Array of container IDs.
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersList {
    pub containers: Option<Vec<Container>>,
}
impl std::fmt::Display for ContainersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainersPaginatedList(pub ContainersList, Pagination);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersUnarchive {
    #[serde(rename = "containerIds")]
    ///Array of container IDs
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateConsensusAlignmentAsyncTask {}
impl std::fmt::Display for CreateConsensusAlignmentAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateEntityIntoRegistry {
    #[serde(rename = "entityRegistryId")]
    /**Entity registry ID to set for the registered entity. Cannot specify both entityRegistryId and namingStrategy at the same time.
*/
    pub entity_registry_id: Option<String>,
    #[serde(rename = "folderId")]
    ///ID of the folder containing the entity. Can be left empty when registryId is present.
    pub folder_id: Option<String>,
    #[serde(rename = "namingStrategy")]
    /**Specifies the behavior for automatically generated names when registering an entity.
- NEW_IDS: Generate new registry IDs
- IDS_FROM_NAMES: Generate registry IDs based on entity names
- DELETE_NAMES: Generate new registry IDs and replace name with registry ID
- SET_FROM_NAME_PARTS: Generate new registry IDs, rename according to name template, and keep old name as alias
- REPLACE_NAMES_FROM_PARTS: Generate new registry IDs, and replace name according to name template
- KEEP_NAMES: Keep existing entity names as registry IDs
- REPLACE_ID_AND_NAME_FROM_PARTS: Generate registry IDs and names according to name template
*/
    pub naming_strategy: Option<String>,
    #[serde(rename = "registryId")]
    /**Registry ID into which entity should be registered. this is the ID of the registry which was configured for a particular organization
To get available registryIds, use the [/registries endpoint](#/Registry/listRegistries)

Required in order for entities to be created directly in the registry.
*/
    pub registry_id: Option<String>,
}
impl std::fmt::Display for CreateEntityIntoRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateNucleotideConsensusAlignmentAsyncTask {}
impl std::fmt::Display for CreateNucleotideConsensusAlignmentAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateNucleotideTemplateAlignmentAsyncTask {}
impl std::fmt::Display for CreateNucleotideTemplateAlignmentAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateTemplateAlignmentAsyncTask {}
impl std::fmt::Display for CreateTemplateAlignmentAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreationOrigin {
    pub application: Option<String>,
    #[serde(rename = "originId")]
    pub origin_id: Option<String>,
    #[serde(rename = "originModalUuid")]
    pub origin_modal_uuid: Option<String>,
    #[serde(rename = "originType")]
    pub origin_type: Option<String>,
}
impl std::fmt::Display for CreationOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "customEntityIds")]
    pub custom_entity_ids: Option<Vec<String>>,
}
impl std::fmt::Display for CustomEntitiesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesArchive {
    #[serde(rename = "customEntityIds")]
    pub custom_entity_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for CustomEntitiesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesBulkCreateRequest {
    #[serde(rename = "customEntities")]
    pub custom_entities: Vec<CustomEntityBulkCreate>,
}
impl std::fmt::Display for CustomEntitiesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesBulkUpdateRequest {
    #[serde(rename = "customEntities")]
    pub custom_entities: Vec<CustomEntityBulkUpdate>,
}
impl std::fmt::Display for CustomEntitiesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesList {
    #[serde(rename = "customEntities")]
    pub custom_entities: Option<Vec<CustomEntity>>,
}
impl std::fmt::Display for CustomEntitiesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesPaginatedList {
    #[serde(rename = "customEntities")]
    pub custom_entities: Option<Vec<CustomEntity>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for CustomEntitiesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesUnarchive {
    #[serde(rename = "customEntityIds")]
    pub custom_entity_ids: Vec<String>,
}
impl std::fmt::Display for CustomEntitiesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntity {
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the Custom Entity in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub authors: Option<Vec<UserSummary>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<serde_json::Value>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for CustomEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntityBaseRequest {
    ///Aliases to add to the custom entity
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the custom entity's authors.
    pub author_ids: Option<Vec<String>>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the custom entity. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Schema fields to set on the custom entity. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that the entity is moved into
    pub folder_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    pub schema_id: Option<String>,
}
impl std::fmt::Display for CustomEntityBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBaseRequestForCreate(
    pub CustomEntityBaseRequest,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBulkCreate(pub CustomEntityCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBulkUpdate(pub CustomEntityBaseRequest);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityCreate(
    pub CustomEntityBaseRequestForCreate,
    CreateEntityIntoRegistry,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntityRequestRegistryFields {
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for CustomEntityRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitySummary {
    #[serde(rename = "entityType")]
    pub entity_type: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CustomEntitySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityUpdate(
    pub CustomEntityBaseRequest,
    CustomEntityRequestRegistryFields,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntityWithEntityType {}
impl std::fmt::Display for CustomEntityWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomField {
    pub value: Option<String>,
}
impl std::fmt::Display for CustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomFields {}
impl std::fmt::Display for CustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultConcentrationSummary {
    ///Units of measurement.
    pub units: Option<String>,
    ///Amount of measurement.
    pub value: Option<f64>,
}
impl std::fmt::Display for DefaultConcentrationSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeprecatedAutomationOutputProcessorsPaginatedList {
    #[serde(rename = "automationOutputProcessors")]
    pub automation_output_processors: Option<Vec<AutomationOutputProcessor>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for DeprecatedAutomationOutputProcessorsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeprecatedContainerVolumeForInput {
    pub units: Option<String>,
    pub value: Option<f64>,
}
impl std::fmt::Display for DeprecatedContainerVolumeForInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecatedContainerVolumeForResponse(
    pub ContainerQuantity,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecatedEntitySchema(pub EntitySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeprecatedEntitySchemasList {
    #[serde(rename = "entitySchemas")]
    pub entity_schemas: Option<Vec<DeprecatedEntitySchema>>,
}
impl std::fmt::Display for DeprecatedEntitySchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignment {}
impl std::fmt::Display for DnaAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignmentBase {
    pub algorithm: String,
    pub files: Vec<serde_json::Value>,
    pub name: Option<String>,
}
impl std::fmt::Display for DnaAlignmentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignmentSummary {
    #[serde(rename = "apiURL")]
    ///The canonical url of the DNA Alignment in the API.
    pub api_url: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the DNA Alignment was created
    pub created_at: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the DNA Alignment was last modified
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "referenceSequenceId")]
    ///The ID of the template or consensus DNA Sequence associated with the DNA Alignment
    pub reference_sequence_id: Option<String>,
    #[serde(rename = "webURL")]
    ///The Benchling web UI url to view the DNA Alignment
    pub web_url: Option<String>,
}
impl std::fmt::Display for DnaAlignmentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignmentsPaginatedList {}
impl std::fmt::Display for DnaAlignmentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaAnnotation(pub SequenceFeatureBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaConsensusAlignmentCreate(pub DnaAlignmentBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligo(pub Oligo, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoBulkUpdate(pub serde_json::Value, DnaOligoUpdate);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoCreate(pub OligoCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoUpdate(pub OligoUpdate);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligoWithEntityType {}
impl std::fmt::Display for DnaOligoWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "dnaOligoIds")]
    pub dna_oligo_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DnaOligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosArchive {
    #[serde(rename = "dnaOligoIds")]
    pub dna_oligo_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for DnaOligosArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosBulkCreateRequest {
    #[serde(rename = "dnaOligos")]
    pub dna_oligos: Option<Vec<DnaOligoCreate>>,
}
impl std::fmt::Display for DnaOligosBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosBulkUpdateRequest {
    #[serde(rename = "dnaOligos")]
    pub dna_oligos: Option<Vec<DnaOligoBulkUpdate>>,
}
impl std::fmt::Display for DnaOligosBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligosPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosUnarchive {
    #[serde(rename = "dnaOligoIds")]
    pub dna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for DnaOligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequence {
    pub aliases: Option<Vec<String>>,
    pub annotations: Option<Vec<DnaAnnotation>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the DNA Sequence in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub bases: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "isCircular")]
    pub is_circular: Option<bool>,
    pub length: Option<i64>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub primers: Option<Vec<Primer>>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub translations: Option<Vec<Translation>>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceBaseRequest {
    ///Aliases to add to the DNA sequence
    pub aliases: Option<Vec<String>>,
    /**Annotations to create on the DNA sequence.
*/
    pub annotations: Option<Vec<DnaAnnotation>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the DNA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    /**Base pairs for the DNA sequence.
*/
    pub bases: Option<String>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the DNA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Fields to set on the DNA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    /**ID of the folder containing the DNA sequence.
*/
    pub folder_id: Option<String>,
    #[serde(rename = "isCircular")]
    /**Whether the DNA sequence is circular or linear.
*/
    pub is_circular: Option<bool>,
    /**Name of the DNA sequence.
*/
    pub name: Option<String>,
    pub primers: Option<Vec<Primer>>,
    #[serde(rename = "schemaId")]
    /**ID of the DNA sequence's schema.
*/
    pub schema_id: Option<String>,
    /**Translations to create on the DNA sequence. Translations are specified by either a combination of 'start' and 'end' fields, or a list of regions. Both cannot be provided.
*/
    pub translations: Option<Vec<Translation>>,
}
impl std::fmt::Display for DnaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBaseRequestForCreate(
    pub DnaSequenceBaseRequest,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBulkCreate(pub DnaSequenceCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBulkUpdate(pub serde_json::Value, DnaSequenceBaseRequest);
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceCreate(
    pub DnaSequenceBaseRequestForCreate,
    CreateEntityIntoRegistry,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceRequestRegistryFields {
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for DnaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceSummary {
    #[serde(rename = "entityType")]
    pub entity_type: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for DnaSequenceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceUpdate(
    pub DnaSequenceBaseRequest,
    DnaSequenceRequestRegistryFields,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceWithEntityType {}
impl std::fmt::Display for DnaSequenceWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "dnaSequenceIds")]
    pub dna_sequence_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DnaSequencesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesArchive {
    #[serde(rename = "dnaSequenceIds")]
    pub dna_sequence_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for DnaSequencesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkCreateRequest {
    #[serde(rename = "dnaSequences")]
    pub dna_sequences: Option<Vec<DnaSequenceBulkCreate>>,
}
impl std::fmt::Display for DnaSequencesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkGet {
    #[serde(rename = "dnaSequences")]
    pub dna_sequences: Option<Vec<DnaSequence>>,
}
impl std::fmt::Display for DnaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkUpdateRequest {
    #[serde(rename = "dnaSequences")]
    pub dna_sequences: Option<Vec<DnaSequenceBulkUpdate>>,
}
impl std::fmt::Display for DnaSequencesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesPaginatedList {
    #[serde(rename = "dnaSequences")]
    pub dna_sequences: Option<Vec<DnaSequence>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for DnaSequencesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesUnarchive {
    #[serde(rename = "dnaSequenceIds")]
    pub dna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for DnaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaTemplateAlignmentCreate(pub DnaAlignmentBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaTemplateAlignmentFile {
    pub data: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for DnaTemplateAlignmentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dropdown(pub DropdownSummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownCreate {
    ///Name of the dropdown
    pub name: String,
    ///Options to set for the dropdown
    pub options: Vec<DropdownOptionCreate>,
    #[serde(rename = "registryId")]
    ///ID of registry in which to create the dropdown. Required if multiple registries exist.
    pub registry_id: Option<String>,
}
impl std::fmt::Display for DropdownCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DropdownFieldDefinition(pub FieldDefinition, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOption {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for DropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionCreate {
    pub name: String,
}
impl std::fmt::Display for DropdownOptionCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionUpdate {
    ///ID of the dropdown option to update, omitted if creating a new option.
    pub id: Option<String>,
    ///Name of the dropdown option.
    pub name: String,
}
impl std::fmt::Display for DropdownOptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsArchivalChange {
    #[serde(rename = "dropdownOptionIds")]
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DropdownOptionsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsArchive {
    #[serde(rename = "dropdownOptionIds")]
    ///Array of dropdown option IDs
    pub dropdown_option_ids: Option<Vec<String>>,
    /**Reason that dropdown options are being archived.
*/
    pub reason: Option<String>,
}
impl std::fmt::Display for DropdownOptionsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsUnarchive {
    #[serde(rename = "dropdownOptionIds")]
    ///Array of dropdown option IDs
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DropdownOptionsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownSummariesPaginatedList {
    pub dropdowns: Option<Vec<DropdownSummary>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for DropdownSummariesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownSummary {
    ///ID of the dropdown
    pub id: Option<String>,
    ///Name of the dropdown
    pub name: Option<String>,
}
impl std::fmt::Display for DropdownSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownUpdate {
    ///Options to set for the dropdown
    pub options: Vec<DropdownOptionUpdate>,
}
impl std::fmt::Display for DropdownUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownsRegistryList {
    pub dropdowns: Option<Vec<DropdownSummary>>,
}
impl std::fmt::Display for DropdownsRegistryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmptyObject {}
impl std::fmt::Display for EmptyObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Entity {}
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Expended,
    Shipped,
    Contaminated,
    Expired,
    Missing,
    Other,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntityLabels {
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for EntityLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntityOrInaccessibleResource {}
impl std::fmt::Display for EntityOrInaccessibleResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityRegisteredEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntitySchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntitySchemasPaginatedList {
    #[serde(rename = "entitySchemas")]
    pub entity_schemas: Option<Vec<EntitySchema>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntitySchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Entries {
    pub entries: Option<Vec<Entry>>,
}
impl std::fmt::Display for Entries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesArchivalChange {
    #[serde(rename = "entryIds")]
    pub entry_ids: Option<Vec<String>>,
}
impl std::fmt::Display for EntriesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesArchive {
    #[serde(rename = "entryIds")]
    ///Array of entry IDs
    pub entry_ids: Vec<String>,
    /**Reason that entries are being archived. One of ["Made in error", "Retired", "Other"].
*/
    pub reason: String,
}
impl std::fmt::Display for EntriesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesPaginatedList {
    pub entries: Option<Vec<Entry>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntriesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesUnarchive {
    #[serde(rename = "entryIds")]
    ///Array of entry IDs
    pub entry_ids: Vec<String>,
}
impl std::fmt::Display for EntriesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Entry {
    #[serde(rename = "apiURL")]
    ///The canonical url of the Entry in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    /**ArchiveRecord Resource if the entry is archived. This is null if the entry is not archived.
*/
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "assignedReviewers")]
    /**Array of users assigned to review the entry, if any.
*/
    pub assigned_reviewers: Option<Vec<UserSummary>>,
    /**Array of UserSummary Resources of the authors of the entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    #[serde(rename = "createdAt")]
    ///DateTime the entry was created at
    pub created_at: Option<String>,
    ///UserSummary Resource of the user who created the entry
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    /**Array of day objects. Each day object has a date field (string) and notes field (array of notes, expand further for details on note types).
*/
    pub days: Option<Vec<EntryDay>>,
    #[serde(rename = "displayId")]
    ///User-friendly ID of the entry
    pub display_id: Option<String>,
    #[serde(rename = "entryTemplateId")]
    ///ID of the Entry Template this Entry was created from
    pub entry_template_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the entry
    pub folder_id: Option<String>,
    ///ID of the entry
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the entry was last modified
    pub modified_at: Option<String>,
    ///Title of the entry
    pub name: Option<String>,
    #[serde(rename = "reviewRecord")]
    ///Review record if set
    pub review_record: Option<serde_json::Value>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    #[serde(rename = "webURL")]
    ///URL of the entry
    pub web_url: Option<String>,
}
impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryById {
    /**Entries are notes that users can take. They're organized by "days" (which are user-configurable) and modeled within each day as a list of "notes." Each note has a type - the simplest is a "text" type, but lists, tables, and external files are also supported.

*Note:* the current Entry resource has a few limitations:
- Formatting information is not yet supported. Header formatting, bolding, and other stylistic information is not presented.
- Data in tables is presented as text always - numeric values will need to be parsed into floats or integers, as appropriate.

Note: Data in Results tables are not accessible through this API call. Results table data can be called through the Results API calls.
*/
    pub entry: Option<Entry>,
}
impl std::fmt::Display for EntryById {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryCreate {
    #[serde(rename = "authorIds")]
    pub author_ids: Option<serde_json::Value>,
    #[serde(rename = "customFields")]
    ///Custom fields to add to the entry
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entryTemplateId")]
    ///ID of the template to clone the entry from
    pub entry_template_id: Option<String>,
    /**Fields to set on the entry. Must correspond with the schema's field definitions.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that will contain the entry
    pub folder_id: String,
    #[serde(rename = "initialTables")]
    /**An array of table API IDs and blob id pairs to seed tables from the template while creating the entry. The entryTemplateId parameter must be set to use this parameter. The table API IDs should be the API Identifiers of the tables in the given template.
- If a template table has one row, the values in that row act as default values for cloned entries.
- If a template table has multiple rows, there is no default value and those rows are added to the cloned entry along with the provided csv data.
- If a table has default values, they will be populated in any respective undefined columns in the csv data.
- If a table has no default values, undefined columns from csv data will be empty.
- If no csv data is provided for a table, the table in the entry will be populated with whatever values are in the respective template table.
*/
    pub initial_tables: Option<Vec<InitialTable>>,
    ///Name of the entry
    pub name: String,
    #[serde(rename = "schemaId")]
    ///ID of the entry's schema
    pub schema_id: Option<String>,
}
impl std::fmt::Display for EntryCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryDay {
    ///A Date string
    pub date: Option<String>,
    pub notes: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for EntryDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryExternalFile {
    #[serde(rename = "downloadURL")]
    /**A short-lived URL that can be used to download the original file.
*/
    pub download_url: Option<String>,
    #[serde(rename = "expiresAt")]
    ///UNIX timestamp when downloadURL expires.
    pub expires_at: Option<i64>,
    ///ID of the external file
    pub id: Option<String>,
    ///Size, in bytes, of the external file
    pub size: Option<i64>,
}
impl std::fmt::Display for EntryExternalFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryExternalFileById {
    #[serde(rename = "externalFile")]
    /**The ExternalFile resource stores metadata about the file. The actual original file can be downloaded by using the 'downloadURL' property.
*/
    pub external_file: Option<EntryExternalFile>,
}
impl std::fmt::Display for EntryExternalFileById {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryLink {
    /**For linked Benchling resources, this will be the ID of that resource (e.g., 'seq_RhYGVnHF'). Omitted for "link" types.
*/
    pub id: Option<String>,
    #[serde(rename = "type")]
    /**The type of resource being linked. For hyperlinks: 'link'. For linked Benchling resources, one of: 'user', 'request', 'entry', 'stage_entry', 'protocol', 'workflow', 'custom_entity', 'aa_sequence', 'dna_sequence', 'batch', 'box', 'container', 'location', 'plate'.
*/
    pub type_: Option<String>,
    #[serde(rename = "webURL")]
    /**Canonical URL of the linked Benchling resource (if you have at least READ authorization for that resource), or the explicit URL provided as hyperlink for "link" types. Note: locations do not currently have a URL.
*/
    pub web_url: Option<String>,
}
impl std::fmt::Display for EntryLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntrySchema {
    ///ID of the entry schema
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Entry Schema was last modified
    pub modified_at: Option<String>,
    ///Name of the entry schema
    pub name: Option<String>,
}
impl std::fmt::Display for EntrySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntrySchemaDetailed(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntrySchemasPaginatedList {
    #[serde(rename = "entrySchemas")]
    pub entry_schemas: Option<Vec<EntrySchemaDetailed>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntrySchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTable {
    #[serde(rename = "columnLabels")]
    /**Array of strings, with one item per column. Defaults to null, if the user is using the default, but is set if the user has given a custom name to the column.
*/
    pub column_labels: Option<Vec<String>>,
    /**Name of the table - defaults to e.g. Table1 but can be renamed.
*/
    pub name: Option<String>,
    ///Array of row objects.
    pub rows: Option<Vec<EntryTableRow>>,
}
impl std::fmt::Display for EntryTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTableCell {
    /**A Link Resource if this cell contained a hyperlink. Otherwise, link will be omitted from the cell object. (Note: storage and user types are not yet supported.)
*/
    pub link: Option<EntryLink>,
    /**The textual content of the cell. If the cell was originally a formula, this will be the evaluated version of the formula.
*/
    pub text: Option<String>,
}
impl std::fmt::Display for EntryTableCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTableRow {
    pub cells: Option<Vec<EntryTableCell>>,
}
impl std::fmt::Display for EntryTableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTemplate {
    #[serde(rename = "apiURL")]
    ///The canonical url of the Entry Template in the API.
    pub api_url: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the template was created at
    pub created_at: Option<String>,
    ///UserSummary Resource of the user who created the template
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    /**Array of day objects. Each day object has a day index (integer) and notes field (array of notes, expand further for details on note types).
*/
    pub days: Option<Vec<EntryTemplateDay>>,
    pub fields: Option<Fields>,
    ///ID of the entry template
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the template was last modified
    pub modified_at: Option<String>,
    ///Title of the template
    pub name: Option<String>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    #[serde(rename = "templateCollectionId")]
    ///ID of the collection that contains the template
    pub template_collection_id: Option<String>,
    #[serde(rename = "webURL")]
    ///URL of the template
    pub web_url: Option<String>,
}
impl std::fmt::Display for EntryTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTemplateDay {
    ///1 indexed day signifier.
    pub day: Option<i64>,
    pub notes: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for EntryTemplateDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTemplatesPaginatedList {
    #[serde(rename = "entryTemplates")]
    pub entry_templates: Option<Vec<EntryTemplate>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntryTemplatesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryUpdate {
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the entry's authors.
    pub author_ids: Option<String>,
    ///Schema fields to set on the entry
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that will contain the entry
    pub folder_id: Option<String>,
    ///New name of the entry
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    ///ID of the schema for the entry
    pub schema_id: Option<String>,
}
impl std::fmt::Display for EntryUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryUpdatedFieldsEvent(pub EventBase, UpdateEventMixin, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryUpdatedReviewRecordEvent(
    pub EventBase,
    UpdateEventMixin,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct Event(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventBase {
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub deprecated: Option<bool>,
    #[serde(rename = "excludedProperties")]
    /**These properties have been dropped from the payload due to size.
*/
    pub excluded_properties: Option<Vec<String>>,
    pub id: Option<String>,
    pub schema: Option<serde_json::Value>,
}
impl std::fmt::Display for EventBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventsPaginatedList {
    pub events: Option<Vec<Event>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for EventsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExecuteSampleGroups {}
impl std::fmt::Display for ExecuteSampleGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportItemRequest {
    ///ID of the item to export
    pub id: String,
}
impl std::fmt::Display for ExportItemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportsAsyncTask {}
impl std::fmt::Display for ExportsAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileNotePart(pub BaseNotePart, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Feature {}
impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureBase {
    ///The color of the annotations generated by the feature. Must be a valid hex string
    pub color: Option<String>,
    #[serde(rename = "featureLibraryId")]
    ///The id of the feature library the feature belongs to
    pub feature_library_id: Option<String>,
    #[serde(rename = "featureType")]
    /**The type of feature, like gene, promoter, etc. Note: This is an arbitrary string, not an enum
*/
    pub feature_type: Option<String>,
    ///The name of the feature
    pub name: Option<String>,
    ///The pattern used for matching during auto-annotation.
    pub pattern: Option<String>,
}
impl std::fmt::Display for FeatureBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureBulkCreate(pub FeatureCreate);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureCreate {}
impl std::fmt::Display for FeatureCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibrariesPaginatedList {}
impl std::fmt::Display for FeatureLibrariesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibrary {}
impl std::fmt::Display for FeatureLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibraryBase {
    ///The description for the feature library
    pub description: Option<String>,
    ///The name of the feature library
    pub name: Option<String>,
}
impl std::fmt::Display for FeatureLibraryBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibraryCreate {}
impl std::fmt::Display for FeatureLibraryCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibraryUpdate {}
impl std::fmt::Display for FeatureLibraryUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureUpdate {}
impl std::fmt::Display for FeatureUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeaturesBulkCreateRequest {
    pub features: Option<Vec<FeatureBulkCreate>>,
}
impl std::fmt::Display for FeaturesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeaturesPaginatedList {}
impl std::fmt::Display for FeaturesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Field {
    #[serde(rename = "displayValue")]
    pub display_value: Option<String>,
    #[serde(rename = "isMulti")]
    pub is_multi: Option<bool>,
    #[serde(rename = "textValue")]
    pub text_value: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /**For single link fields, use the id of the item you want to link (eg. "seq_jdf8BV24").
For multi-link fields, use an array of ids of the items you want to link (eg. ["seq_jdf8BV24"])
*/
    pub value: Option<serde_json::Value>,
}
impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FieldDefinition {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub id: Option<String>,
    #[serde(rename = "isMulti")]
    pub is_multi: Option<bool>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "dna_sequence_link")]
    DnaSequenceLink,
    #[serde(rename = "aa_sequence_link")]
    AaSequenceLink,
    #[serde(rename = "custom_entity_link")]
    CustomEntityLink,
    #[serde(rename = "entity_link")]
    EntityLink,
    #[serde(rename = "mixture_link")]
    MixtureLink,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "part_link")]
    PartLink,
    #[serde(rename = "translation_link")]
    TranslationLink,
    #[serde(rename = "blob_link")]
    BlobLink,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "long_text")]
    LongText,
    #[serde(rename = "batch_link")]
    BatchLink,
    #[serde(rename = "storage_link")]
    StorageLink,
    #[serde(rename = "entry_link")]
    EntryLink,
    #[serde(rename = "assay_request_link")]
    AssayRequestLink,
    #[serde(rename = "assay_result_link")]
    AssayResultLink,
    #[serde(rename = "assay_run_link")]
    AssayRunLink,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "datetime")]
    Datetime,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "json")]
    Json,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Fields {}
impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FindMatchingRegionsAsyncTask {}
impl std::fmt::Display for FindMatchingRegionsAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FloatFieldDefinition(pub FieldDefinition, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Folder {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentFolderId")]
    ///ID of the parent folder, if it exists
    pub parent_folder_id: Option<String>,
    #[serde(rename = "projectId")]
    ///ID of the containing project
    pub project_id: Option<String>,
}
impl std::fmt::Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FolderCreate {
    ///The name of the new folder.
    pub name: String,
    #[serde(rename = "parentFolderId")]
    ///The ID of the parent folder.
    pub parent_folder_id: String,
}
impl std::fmt::Display for FolderCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersArchivalChange {
    #[serde(rename = "aaSequenceIds")]
    pub aa_sequence_ids: Option<Vec<String>>,
    #[serde(rename = "customEntityIds")]
    pub custom_entity_ids: Option<Vec<String>>,
    #[serde(rename = "dnaSequenceIds")]
    pub dna_sequence_ids: Option<Vec<String>>,
    #[serde(rename = "entryIds")]
    pub entry_ids: Option<Vec<String>>,
    #[serde(rename = "folderIds")]
    pub folder_ids: Option<Vec<String>>,
    #[serde(rename = "mixtureIds")]
    pub mixture_ids: Option<Vec<String>>,
    #[serde(rename = "oligoIds")]
    pub oligo_ids: Option<Vec<String>>,
    #[serde(rename = "protocolIds")]
    pub protocol_ids: Option<Vec<String>>,
}
impl std::fmt::Display for FoldersArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersArchive {
    #[serde(rename = "folderIds")]
    ///A list of folder IDs to archive.
    pub folder_ids: Vec<String>,
    /**The reason for archiving the provided folders. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for FoldersArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersPaginatedList {
    pub folders: Option<Vec<Folder>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for FoldersPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersUnarchive {
    #[serde(rename = "folderIds")]
    ///A list of folder IDs to unarchive.
    pub folder_ids: Vec<String>,
}
impl std::fmt::Display for FoldersUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ForbiddenError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for ForbiddenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericEntity {
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "apiURL")]
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    /**Array of UserSummary Resources of the authors of the entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<serde_json::Value>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for GenericEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InaccessibleResource {
    #[serde(rename = "inaccessibleId")]
    pub inaccessible_id: Option<String>,
    #[serde(rename = "type")]
    /**The type of this inaccessible item. Example values: "custom_entity", "container", "plate", "dna_sequence"
*/
    pub type_: Option<String>,
}
impl std::fmt::Display for InaccessibleResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ingredient {
    /**The amount value of this ingredient in its mixture, in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4). One ingredient on this mixture can have an amount value of `"QS"`.
*/
    pub amount: Option<String>,
    #[serde(rename = "catalogIdentifier")]
    pub catalog_identifier: Option<String>,
    #[serde(rename = "componentEntity")]
    pub component_entity: Option<serde_json::Value>,
    #[serde(rename = "componentLotContainer")]
    ///The container representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "storage" format.
    pub component_lot_container: Option<ContainerLabels>,
    #[serde(rename = "componentLotEntity")]
    ///The entity representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "storage" format.
    pub component_lot_entity: Option<EntityLabels>,
    #[serde(rename = "componentLotText")]
    ///Text representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "text" format.
    pub component_lot_text: Option<String>,
    #[serde(rename = "hasParent")]
    pub has_parent: Option<bool>,
    pub notes: Option<String>,
    #[serde(rename = "targetAmount")]
    ///The target amount for this ingredient such that this ingredient's proportion in its mixture would preserve the equivalent ingredient's proportion in the parent mixture. Pair with `units`.
    pub target_amount: Option<String>,
    pub units: Option<String>,
}
impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IngredientMeasurementUnits {
    #[serde(rename = "nL")]
    NL,
    #[serde(rename = "uL")]
    UL,
    #[serde(rename = "mL")]
    ML,
    L,
    #[serde(rename = "mg")]
    Mg,
    #[serde(rename = "g")]
    G,
    Units,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IngredientWriteParams {
    /**The amount value of this ingredient in its mixture, in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4). One ingredient on this mixture can have an amount value of `"QS"`.
*/
    pub amount: Option<String>,
    #[serde(rename = "catalogIdentifier")]
    pub catalog_identifier: Option<String>,
    #[serde(rename = "componentEntityId")]
    ///The entity that uniquely identifies this component.
    pub component_entity_id: String,
    #[serde(rename = "componentLotContainerId")]
    ///The container representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "storage" format.
    pub component_lot_container_id: Option<String>,
    #[serde(rename = "componentLotEntityId")]
    ///The entity representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "storage" format.
    pub component_lot_entity_id: Option<String>,
    #[serde(rename = "componentLotText")]
    ///Text representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "text" format.
    pub component_lot_text: Option<String>,
    pub notes: Option<String>,
    pub units: Option<String>,
}
impl std::fmt::Display for IngredientWriteParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InitialTable {
    #[serde(rename = "csvData")]
    ///blobId of an uploaded csv blob. The CSV should be formatted with column headers of `columnId` which can be found in the [EntryTemplate](#/components/schemas/EntryTemplate). For more information on uploading a blob, [click here](https://docs.benchling.com/docs/uploading-a-blob-to-benchling).
    pub csv_data: Option<String>,
    #[serde(rename = "templateTableID")]
    ///Template table API ID
    pub template_table_id: Option<String>,
}
impl std::fmt::Display for InitialTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerFieldDefinition(pub FieldDefinition, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabAutomationBenchlingAppError {
    pub message: Option<String>,
}
impl std::fmt::Display for LabAutomationBenchlingAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabAutomationTransform {
    #[serde(rename = "apiURL")]
    ///The canonical url of the transform in the API.
    pub api_url: Option<String>,
    #[serde(rename = "blobId")]
    pub blob_id: Option<String>,
    #[serde(rename = "customTransformId")]
    pub custom_transform_id: Option<String>,
    pub errors: Option<Vec<LabAutomationBenchlingAppError>>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the transform was last modified.
    pub modified_at: Option<String>,
    #[serde(rename = "outputProcessorId")]
    pub output_processor_id: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for LabAutomationTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabAutomationTransformUpdate {
    #[serde(rename = "blobId")]
    pub blob_id: Option<String>,
    pub errors: Option<Vec<LabAutomationBenchlingAppError>>,
}
impl std::fmt::Display for LabAutomationTransformUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabelTemplate {
    ///ID of the label template.
    pub id: Option<String>,
    ///Name of the label template.
    pub name: Option<String>,
    #[serde(rename = "zplTemplate")]
    ///The ZPL template that will be filled in and sent to a printer.
    pub zpl_template: Option<String>,
}
impl std::fmt::Display for LabelTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabelTemplatesList {
    #[serde(rename = "labelTemplates")]
    pub label_templates: Option<Vec<LabelTemplate>>,
}
impl std::fmt::Display for LabelTemplatesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflow {
    #[serde(rename = "createdAt")]
    ///DateTime at which the the legacy workflow was created
    pub created_at: Option<String>,
    ///Description of the legacy workflow
    pub description: Option<String>,
    #[serde(rename = "displayId")]
    ///User-friendly ID of the legacy workflow
    pub display_id: Option<String>,
    ///ID of the legacy workflow
    pub id: Option<String>,
    ///Name of the legacy workflow
    pub name: Option<String>,
    #[serde(rename = "projectId")]
    ///ID of the project that contains the legacy workflow
    pub project_id: Option<String>,
}
impl std::fmt::Display for LegacyWorkflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowList {
    pub workflows: Option<Vec<LegacyWorkflow>>,
}
impl std::fmt::Display for LegacyWorkflowList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowPatch {
    ///Description of the legacy workflow
    pub description: Option<String>,
    ///Name of the legacy workflow
    pub name: Option<String>,
    #[serde(rename = "projectId")]
    ///ID of the project that contains the legacy workflow
    pub project_id: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowSample {
    #[serde(rename = "batchId")]
    ///ID of the batch
    pub batch_id: Option<String>,
    #[serde(rename = "containerIds")]
    ///Array of IDs of containers
    pub container_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    ///DateTime at which the the sample was created
    pub created_at: Option<String>,
    ///ID of the sample
    pub id: Option<String>,
    ///Name of the sample
    pub name: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowSampleList {
    pub samples: Option<Vec<LegacyWorkflowSample>>,
}
impl std::fmt::Display for LegacyWorkflowSampleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStage {
    #[serde(rename = "createdAt")]
    ///DateTime at which the the legacy workflow stage was created
    pub created_at: Option<String>,
    ///ID of the legacy workflow stage
    pub id: Option<String>,
    ///Name of the legacy workflow stage
    pub name: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageList {
    #[serde(rename = "workflowStages")]
    pub workflow_stages: Option<Vec<LegacyWorkflowStage>>,
}
impl std::fmt::Display for LegacyWorkflowStageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageRun {
    #[serde(rename = "createdAt")]
    ///DateTime at which the the legacy workflow stage run was created
    pub created_at: Option<String>,
    ///ID of the legacy workflow stage run
    pub id: Option<String>,
    ///Name of the legacy workflow stage run
    pub name: Option<String>,
    ///Status of the legacy workflow stage run
    pub status: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowStageRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageRunList {
    #[serde(rename = "workflowStageRuns")]
    pub workflow_stage_runs: Option<Vec<LegacyWorkflowStageRun>>,
}
impl std::fmt::Display for LegacyWorkflowStageRunList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Location {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub barcode: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationCreate {
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub name: String,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "schemaId")]
    pub schema_id: String,
}
impl std::fmt::Display for LocationCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationSchemasList {
    #[serde(rename = "locationSchemas")]
    pub location_schemas: Option<Vec<LocationSchema>>,
}
impl std::fmt::Display for LocationSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSchemasPaginatedList(pub LocationSchemasList, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationUpdate {
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
}
impl std::fmt::Display for LocationUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsArchivalChange {
    #[serde(rename = "boxIds")]
    pub box_ids: Option<Vec<String>>,
    #[serde(rename = "containerIds")]
    pub container_ids: Option<Vec<String>>,
    #[serde(rename = "locationIds")]
    pub location_ids: Option<Vec<String>>,
    #[serde(rename = "plateIds")]
    pub plate_ids: Option<Vec<String>>,
}
impl std::fmt::Display for LocationsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsArchive {
    #[serde(rename = "locationIds")]
    ///Array of location IDs
    pub location_ids: Vec<String>,
    /**Reason that locations are being archived.
*/
    pub reason: String,
    #[serde(rename = "shouldRemoveBarcodes")]
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: Option<bool>,
}
impl std::fmt::Display for LocationsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsBulkGet {
    pub locations: Option<Vec<Location>>,
}
impl std::fmt::Display for LocationsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsPaginatedList {
    pub locations: Option<Vec<Location>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for LocationsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsUnarchive {
    #[serde(rename = "locationIds")]
    ///Array of location IDs
    pub location_ids: Vec<String>,
}
impl std::fmt::Display for LocationsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTableNotePart(pub StructuredTableApiIdentifiers, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Measurement {
    ///Can only be null if value is also null
    pub units: Option<String>,
    ///Can only be null if units is also null
    pub value: Option<f64>,
}
impl std::fmt::Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Mixture {
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "allowMeasuredIngredients")]
    ///Derived from the mixture's schema.
    pub allow_measured_ingredients: Option<bool>,
    ///The positive numerical amount value of this mixture in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4).
    pub amount: Option<String>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the Mixture in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub authors: Option<Vec<UserSummary>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<serde_json::Value>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    ///Mixtures can have up to one parent mixture field.
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    pub id: Option<String>,
    ///List of ingredients on this mixture.
    pub ingredients: Option<Vec<Ingredient>>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub units: Option<String>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Mixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixtureBulkUpdate(pub MixtureUpdate, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct MixtureCreate(pub MixtureUpdate, CreateEntityIntoRegistry, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum MixtureMeasurementUnits {
    #[serde(rename = "nL")]
    NL,
    #[serde(rename = "uL")]
    UL,
    #[serde(rename = "mL")]
    ML,
    L,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "kg")]
    Kg,
    Units,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixturePrepTableNotePart(
    pub BaseNotePart,
    StructuredTableApiIdentifiers,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixtureUpdate {
    ///Aliases to add to the mixture
    pub aliases: Option<Vec<String>>,
    ///The positive numerical amount value of this mixture in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4).
    pub amount: Option<String>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the mixture's authors.
    pub author_ids: Option<Vec<String>>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the mixture. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    /**Schema fields to set on the mixture. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
If you are setting the parent mixture field here, you must also specify `ingredients`
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that the entity is moved into
    pub folder_id: Option<String>,
    /**Desired final state for the ingredients on this mixture.
Each ingredient you specify will be matched with the existing ingredients on the mixture based on the component entity, and Benchling will create, update, or delete this mixture's ingredients so that the final state of this mixture's ingredients matches your request.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixture (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub ingredients: Option<Vec<IngredientWriteParams>>,
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    pub schema_id: Option<String>,
    pub units: Option<String>,
}
impl std::fmt::Display for MixtureUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixtureWithEntityType {}
impl std::fmt::Display for MixtureWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesArchivalChange {
    #[serde(rename = "mixtureIds")]
    pub mixture_ids: Option<Vec<String>>,
}
impl std::fmt::Display for MixturesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesArchive {
    #[serde(rename = "mixtureIds")]
    pub mixture_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for MixturesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesBulkCreateRequest {
    pub mixtures: Vec<MixtureCreate>,
}
impl std::fmt::Display for MixturesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesBulkUpdateRequest {
    pub mixtures: Option<Vec<MixtureBulkUpdate>>,
}
impl std::fmt::Display for MixturesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesPaginatedList {
    pub mixtures: Option<Vec<Mixture>>,
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for MixturesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesUnarchive {
    #[serde(rename = "mixtureIds")]
    pub mixture_ids: Vec<String>,
}
impl std::fmt::Display for MixturesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Molecule {
    ///Array of aliases.
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the Molecule in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "canonicalizedSmiles")]
    ///The canonicalized chemical structure in SMILES format.
    pub canonicalized_smiles: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the Molecule was created.
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    ///Custom fields set on the Molecule.
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    ///Registry ID of the Molecule if registered.
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the Molecule.
    pub folder_id: Option<String>,
    ///ID of the Molecule.
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Molecule was last modified.
    pub modified_at: Option<String>,
    ///Name of the Molecule.
    pub name: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    ///Registry the Molecule is registered in.
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    ///URL of the Molecule.
    pub web_url: Option<String>,
}
impl std::fmt::Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculeBaseRequest {
    ///Aliases to add to the Molecule.
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the Molecule's authors.
    pub author_ids: Option<Vec<String>>,
    #[serde(rename = "chemicalStructure")]
    /**Chemical structure of the Molecule.
*/
    pub chemical_structure: Option<MoleculeStructure>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the Molecule. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Fields to set on the Molecule. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    /**ID of the folder containing the Molecule.
*/
    pub folder_id: Option<String>,
    /**Name of the Molecule.
*/
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    /**ID of the Molecule's schema.
*/
    pub schema_id: Option<String>,
}
impl std::fmt::Display for MoleculeBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeBaseRequestForCreate(pub MoleculeBaseRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeBulkUpdate(pub serde_json::Value, MoleculeUpdate);
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeCreate(pub MoleculeBaseRequestForCreate, CreateEntityIntoRegistry);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculeStructure {
    #[serde(rename = "structureFormat")]
    pub structure_format: Option<serde_json::Value>,
    ///Chemical structure in SMILES or molfile format.
    pub value: Option<String>,
}
impl std::fmt::Display for MoleculeStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeUpdate(pub serde_json::Value, MoleculeBaseRequest);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "moleculeIds")]
    pub molecule_ids: Option<Vec<String>>,
}
impl std::fmt::Display for MoleculesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesArchive {
    #[serde(rename = "moleculeIds")]
    pub molecule_ids: Vec<String>,
    /**The reason for archiving the provided Molecules. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for MoleculesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesBulkCreateRequest {
    pub molecules: Option<Vec<MoleculeCreate>>,
}
impl std::fmt::Display for MoleculesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesBulkUpdateRequest {
    pub molecules: Option<Vec<MoleculeBulkUpdate>>,
}
impl std::fmt::Display for MoleculesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculesPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesUnarchive {
    #[serde(rename = "moleculeIds")]
    pub molecule_ids: Vec<String>,
}
impl std::fmt::Display for MoleculesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MultipleContainersTransfer {}
impl std::fmt::Display for MultipleContainersTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MultipleContainersTransfersList {
    pub transfers: Vec<MultipleContainersTransfer>,
}
impl std::fmt::Display for MultipleContainersTransfersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum NamingStrategy {
    #[serde(rename = "NEW_IDS")]
    NewIds,
    #[serde(rename = "IDS_FROM_NAMES")]
    IdsFromNames,
    #[serde(rename = "DELETE_NAMES")]
    DeleteNames,
    #[serde(rename = "SET_FROM_NAME_PARTS")]
    SetFromNameParts,
    #[serde(rename = "REPLACE_NAMES_FROM_PARTS")]
    ReplaceNamesFromParts,
    #[serde(rename = "KEEP_NAMES")]
    KeepNames,
    #[serde(rename = "REPLACE_ID_AND_NAME_FROM_PARTS")]
    ReplaceIdAndNameFromParts,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NotFoundError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignment {}
impl std::fmt::Display for NucleotideAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentBase {
    pub algorithm: String,
    pub files: Vec<serde_json::Value>,
    pub name: Option<String>,
}
impl std::fmt::Display for NucleotideAlignmentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentFile {
    pub data: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for NucleotideAlignmentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentSummary {
    #[serde(rename = "apiURL")]
    ///The canonical url of the Alignment in the API.
    pub api_url: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the Alignment was created
    pub created_at: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Alignment was last modified
    pub modified_at: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "referenceSequenceId")]
    ///The ID of the template or consensus Sequence associated with the Alignment
    pub reference_sequence_id: Option<String>,
    #[serde(rename = "webURL")]
    ///The Benchling web UI url to view the Alignment
    pub web_url: Option<String>,
}
impl std::fmt::Display for NucleotideAlignmentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentsPaginatedList {}
impl std::fmt::Display for NucleotideAlignmentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NucleotideConsensusAlignmentCreate(
    pub NucleotideAlignmentBase,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct NucleotideTemplateAlignmentCreate(
    pub NucleotideAlignmentBase,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OAuthBadRequestError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for OAuthBadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OAuthUnauthorizedError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for OAuthUnauthorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Oligo {
    ///Array of aliases
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the Oligo in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    ///Base pairs of the Oligo.
    pub bases: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the Oligo was created.
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    ///Custom fields set on the Oligo.
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    ///Registry ID of the Oligo if registered.
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the Oligo.
    pub folder_id: Option<String>,
    ///ID of the Oligo.
    pub id: Option<String>,
    ///Number of bases in the Oligo.
    pub length: Option<i64>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Oligo was last modified.
    pub modified_at: Option<String>,
    ///Name of the Oligo.
    pub name: Option<String>,
    #[serde(rename = "nucleotideType")]
    ///Nucleotide type of the Oligo.
    pub nucleotide_type: Option<String>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    ///Registry the Oligo is registered in.
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "webURL")]
    ///URL of the Oligo.
    pub web_url: Option<String>,
}
impl std::fmt::Display for Oligo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligoBaseRequest {
    ///Aliases to add to the Oligo
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the Oligo's authors.
    pub author_ids: Option<Vec<String>>,
    /**Base pairs of the oligo.
*/
    pub bases: Option<String>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the Oligo. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Fields to set on the Oligo. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    /**ID of the folder containing the Oligo.
*/
    pub folder_id: Option<String>,
    /**Name of the Oligo.
*/
    pub name: Option<String>,
    #[serde(rename = "schemaId")]
    /**ID of the oligo's schema.
*/
    pub schema_id: Option<String>,
}
impl std::fmt::Display for OligoBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoBaseRequestForCreate(pub OligoBaseRequest, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoCreate(pub OligoBaseRequestForCreate, CreateEntityIntoRegistry);
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoUpdate(pub OligoBaseRequest);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "oligoIds")]
    pub oligo_ids: Option<Vec<String>>,
}
impl std::fmt::Display for OligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosArchive {
    #[serde(rename = "oligoIds")]
    pub oligo_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for OligosArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosBulkCreateRequest {
    pub oligos: Option<Vec<OligoCreate>>,
}
impl std::fmt::Display for OligosBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosBulkGet {
    pub oligos: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for OligosBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OligosPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosUnarchive {
    #[serde(rename = "oligoIds")]
    pub oligo_ids: Vec<String>,
}
impl std::fmt::Display for OligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Organization {
    pub handle: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrganizationSummary {
    pub handle: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for OrganizationSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrganizationsPaginatedList {}
impl std::fmt::Display for OrganizationsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pagination {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
}
impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PartySummary {
    pub handle: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for PartySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Plate {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    ///Barcode of the plate
    pub barcode: Option<String>,
    #[serde(rename = "createdAt")]
    ///DateTime the container was created
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    ///ID of the plate
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the plate was last modified
    pub modified_at: Option<String>,
    ///Name of the plate, defaults to barcode if name is not provided.
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    ///ID of containing parent storage (e.g. loc_k2lNspzS).
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    ///ID of the project if set
    pub project_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
    ///Well contents of the plate, keyed by position string (eg. "A1").
    pub wells: Option<serde_json::Value>,
}
impl std::fmt::Display for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlateCreate {
    pub barcode: Option<String>,
    #[serde(rename = "containerSchemaId")]
    pub container_schema_id: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "schemaId")]
    pub schema_id: String,
    pub wells: Option<serde_json::Value>,
}
impl std::fmt::Display for PlateCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateCreationTableNotePart(
    pub BaseNotePart,
    StructuredTableApiIdentifiers,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateSchema(pub RegistrySchema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlateSchemasList {
    #[serde(rename = "plateSchemas")]
    pub plate_schemas: Option<Vec<PlateSchema>>,
}
impl std::fmt::Display for PlateSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateSchemasPaginatedList(pub PlateSchemasList, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlateUpdate {
    pub fields: Option<Fields>,
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    pub parent_storage_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
}
impl std::fmt::Display for PlateUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesArchivalChange {
    #[serde(rename = "containerIds")]
    pub container_ids: Option<Vec<String>>,
    #[serde(rename = "plateIds")]
    pub plate_ids: Option<Vec<String>>,
}
impl std::fmt::Display for PlatesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesArchive {
    #[serde(rename = "plateIds")]
    ///Array of plate IDs
    pub plate_ids: Vec<String>,
    /**Reason that plates are being archived.
*/
    pub reason: String,
    #[serde(rename = "shouldRemoveBarcodes")]
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: bool,
}
impl std::fmt::Display for PlatesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesBulkGet {
    pub plates: Option<Vec<Plate>>,
}
impl std::fmt::Display for PlatesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    pub plates: Option<Vec<Plate>>,
}
impl std::fmt::Display for PlatesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesUnarchive {
    #[serde(rename = "plateIds")]
    ///Array of plate IDs
    pub plate_ids: Vec<String>,
}
impl std::fmt::Display for PlatesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Primer {
    pub bases: Option<String>,
    #[serde(rename = "bindPosition")]
    pub bind_position: Option<i64>,
    pub color: Option<String>,
    pub end: Option<i64>,
    pub name: Option<String>,
    #[serde(rename = "oligoId")]
    pub oligo_id: Option<String>,
    #[serde(rename = "overhangLength")]
    pub overhang_length: Option<i64>,
    pub start: Option<i64>,
    pub strand: Option<i64>,
}
impl std::fmt::Display for Primer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrintLabels {
    #[serde(rename = "containerIds")]
    /**List of IDs of containers that will have labels printed (one label will be printed per container).
*/
    pub container_ids: Vec<String>,
    #[serde(rename = "labelTemplateId")]
    /**ID of label template to use (same template will be used for all labels printed).
*/
    pub label_template_id: String,
    #[serde(rename = "printerId")]
    /**ID of printer to use (same printer will be used for all labels printed).
*/
    pub printer_id: String,
}
impl std::fmt::Display for PrintLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Printer {
    ///Web address of the printer (either IP address or URL).
    pub address: Option<String>,
    ///Short description of the printer.
    pub description: Option<String>,
    ///ID of the printer.
    pub id: Option<String>,
    ///Name of the printer.
    pub name: Option<String>,
    ///Port to reach the printer at.
    pub port: Option<i64>,
    #[serde(rename = "registryId")]
    ///ID of the registry associated with this printer.
    pub registry_id: Option<String>,
}
impl std::fmt::Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrintersList {
    #[serde(rename = "labelPrinters")]
    pub label_printers: Option<Vec<Printer>>,
}
impl std::fmt::Display for PrintersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub owner: Option<serde_json::Value>,
}
impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsArchivalChange {
    #[serde(rename = "aaSequenceIds")]
    pub aa_sequence_ids: Option<Vec<String>>,
    #[serde(rename = "customEntityIds")]
    pub custom_entity_ids: Option<Vec<String>>,
    #[serde(rename = "dnaSequenceIds")]
    pub dna_sequence_ids: Option<Vec<String>>,
    #[serde(rename = "entryIds")]
    pub entry_ids: Option<Vec<String>>,
    #[serde(rename = "folderIds")]
    pub folder_ids: Option<Vec<String>>,
    #[serde(rename = "mixtureIds")]
    pub mixture_ids: Option<Vec<String>>,
    #[serde(rename = "oligoIds")]
    pub oligo_ids: Option<Vec<String>>,
    #[serde(rename = "projectIds")]
    pub project_ids: Option<Vec<String>>,
    #[serde(rename = "protocolIds")]
    pub protocol_ids: Option<Vec<String>>,
}
impl std::fmt::Display for ProjectsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsArchive {
    #[serde(rename = "projectIds")]
    ///A list of project IDs to archive.
    pub project_ids: Vec<String>,
    /**The reason for archiving the provided projects. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for ProjectsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    pub projects: Option<Vec<Project>>,
}
impl std::fmt::Display for ProjectsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsUnarchive {
    #[serde(rename = "projectIds")]
    ///A list of project IDs to unarchive.
    pub project_ids: Vec<String>,
}
impl std::fmt::Display for ProjectsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegisterEntities {
    #[serde(rename = "entityIds")]
    ///Array of entity IDs
    pub entity_ids: Vec<String>,
    #[serde(rename = "namingStrategy")]
    /**Specifies the behavior for automatically generated names when registering an entity.
- NEW_IDS: Generate new registry IDs
- IDS_FROM_NAMES: Generate registry IDs based on entity names
- DELETE_NAMES: Generate new registry IDs and replace name with registry ID
- SET_FROM_NAME_PARTS: Generate new registry IDs, rename according to name template, and keep old name as alias
- REPLACE_NAMES_FROM_PARTS: Generate new registry IDs, and replace name according to name template
- KEEP_NAMES: Keep existing entity names as registry IDs
- REPLACE_ID_AND_NAME_FROM_PARTS: Generate registry IDs and names according to name template
*/
    pub naming_strategy: String,
}
impl std::fmt::Display for RegisterEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegisteredEntitiesList {
    pub entities: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for RegisteredEntitiesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistrationOrigin {
    #[serde(rename = "originEntryId")]
    pub origin_entry_id: Option<String>,
    #[serde(rename = "registeredAt")]
    pub registered_at: Option<String>,
}
impl std::fmt::Display for RegistrationOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationTableNotePart(
    pub BaseNotePart,
    StructuredTableApiIdentifiers,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistriesList {
    pub registries: Option<Vec<Registry>>,
}
impl std::fmt::Display for RegistriesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Registry {
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Registry was last modified
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub owner: Option<Organization>,
}
impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrySchema(pub Schema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Request(pub RequestBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestBase {}
impl std::fmt::Display for RequestBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreate(pub RequestWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestFulfillment {
    #[serde(rename = "createdAt")]
    ///Date and time the fulfillment was created
    pub created_at: Option<String>,
    #[serde(rename = "entryId")]
    ///ID of the entry this fulfillment was executed in, if any
    pub entry_id: Option<String>,
    ///ID of the request fulfillment
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the Request Fulfillment was last modified
    pub modified_at: Option<String>,
    #[serde(rename = "requestId")]
    ///ID of the request this fulfillment fulfills
    pub request_id: Option<String>,
    #[serde(rename = "sampleGroup")]
    ///The request sample group this fulfillment was executed upon, if any.
    pub sample_group: Option<SampleGroup>,
    ///Status of a sample group
    pub status: Option<String>,
}
impl std::fmt::Display for RequestFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestFulfillmentsPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "requestFulfillments")]
    pub request_fulfillments: Option<Vec<RequestFulfillment>>,
}
impl std::fmt::Display for RequestFulfillmentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestResponse {
    pub results: Option<Vec<AssayResult>>,
    ///Array of samples produced by the request.
    pub samples: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for RequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponseSamplesItemBatch(pub BatchOrInaccessibleResource);
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponseSamplesItemEntity(pub EntityOrInaccessibleResource);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleGroup {
    pub id: Option<String>,
    /**The key for each RequestSample should match one of the samplesSchema[n].name property in the request schema json.
*/
    pub samples: Option<RequestSampleGroupSamples>,
}
impl std::fmt::Display for RequestSampleGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSampleGroupCreate {
    /**The key for each RequestSample should match one of the samplesSchema[n].name property in the request schema json.
*/
    pub samples: RequestSampleGroupSamples,
}
impl std::fmt::Display for RequestSampleGroupCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleGroupSamples {}
impl std::fmt::Display for RequestSampleGroupSamples {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleWithBatch {
    #[serde(rename = "batchId")]
    pub batch_id: String,
    #[serde(rename = "containerId")]
    pub container_id: Option<String>,
}
impl std::fmt::Display for RequestSampleWithBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleWithEntity {
    #[serde(rename = "containerId")]
    pub container_id: Option<String>,
    #[serde(rename = "entityId")]
    pub entity_id: String,
}
impl std::fmt::Display for RequestSampleWithEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSchema(pub Schema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSchemasPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "requestSchemas")]
    pub request_schemas: Option<Vec<RequestSchema>>,
}
impl std::fmt::Display for RequestSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestStatus {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTask {
    pub schema: Option<SchemaSummary>,
}
impl std::fmt::Display for RequestTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTaskBase {
    ///ID of the request task
    pub id: String,
}
impl std::fmt::Display for RequestTaskBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTaskBaseFields {
    /**Schema fields to set on the request task.
Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "sampleGroupIds")]
    ///IDs of all request sample groups now associated with this task.
    pub sample_group_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RequestTaskBaseFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTaskSchema(pub Schema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTaskSchemasPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "requestTaskSchemas")]
    pub request_task_schemas: Option<Vec<RequestTaskSchema>>,
}
impl std::fmt::Display for RequestTaskSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkCreate {
    #[serde(rename = "schemaId")]
    ///The schema id of the task to create
    pub schema_id: String,
}
impl std::fmt::Display for RequestTasksBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkCreateRequest {
    ///The tasks to create
    pub tasks: Vec<RequestTasksBulkCreate>,
}
impl std::fmt::Display for RequestTasksBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkCreateResponse {
    ///The created tasks
    pub tasks: Option<Vec<RequestTask>>,
}
impl std::fmt::Display for RequestTasksBulkCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkUpdateRequest {
    ///The tasks to update
    pub tasks: Vec<RequestTaskBase>,
}
impl std::fmt::Display for RequestTasksBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkUpdateResponse {
    ///The tasks to update
    pub tasks: Option<Vec<RequestTask>>,
}
impl std::fmt::Display for RequestTasksBulkUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTeamAssignee {
    pub team: Option<TeamSummary>,
}
impl std::fmt::Display for RequestTeamAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate(pub RequestWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdatedFieldsEvent(pub EventBase, UpdateEventMixin, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestUserAssignee {
    pub user: Option<UserSummary>,
}
impl std::fmt::Display for RequestUserAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestWriteBase(pub RequestBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestWriteTeamAssignee {
    #[serde(rename = "teamId")]
    pub team_id: String,
}
impl std::fmt::Display for RequestWriteTeamAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestWriteUserAssignee {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl std::fmt::Display for RequestWriteUserAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestsBulkGet {
    pub requests: Option<Vec<Request>>,
}
impl std::fmt::Display for RequestsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestsPaginatedList(pub RequestsBulkGet, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsTableNotePart(
    pub BaseNotePart,
    StructuredTableApiIdentifiers,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaAnnotation(pub SequenceFeatureBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligo(pub Oligo, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoBulkUpdate(pub serde_json::Value, RnaOligoUpdate);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoCreate(pub OligoCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoUpdate(pub OligoUpdate);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligoWithEntityType {}
impl std::fmt::Display for RnaOligoWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosArchivalChange {
    #[serde(rename = "batchIds")]
    pub batch_ids: Option<Vec<String>>,
    #[serde(rename = "rnaOligoIds")]
    pub rna_oligo_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RnaOligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    #[serde(rename = "rnaOligoIds")]
    pub rna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for RnaOligosArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosBulkCreateRequest {
    #[serde(rename = "rnaOligos")]
    pub rna_oligos: Option<Vec<RnaOligoCreate>>,
}
impl std::fmt::Display for RnaOligosBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosBulkUpdateRequest {
    #[serde(rename = "rnaOligos")]
    pub rna_oligos: Option<Vec<RnaOligoBulkUpdate>>,
}
impl std::fmt::Display for RnaOligosBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligosPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosUnarchive {
    #[serde(rename = "rnaOligoIds")]
    pub rna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for RnaOligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequence {
    pub aliases: Option<Vec<String>>,
    pub annotations: Option<Vec<RnaAnnotation>>,
    #[serde(rename = "apiURL")]
    ///The canonical url of the RNA Sequence in the API.
    pub api_url: Option<String>,
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    pub bases: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    pub folder_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "isCircular")]
    pub is_circular: Option<bool>,
    pub length: Option<i64>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub primers: Option<Vec<Primer>>,
    #[serde(rename = "registrationOrigin")]
    pub registration_origin: Option<RegistrationOrigin>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub translations: Option<Vec<Translation>>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for RnaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequenceBaseRequest {
    ///Aliases to add to the RNA sequence
    pub aliases: Option<Vec<String>>,
    /**Annotations to create on the RNA sequence.
*/
    pub annotations: Option<Vec<RnaAnnotation>>,
    #[serde(rename = "authorIds")]
    ///IDs of users to set as the RNA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    /**Base pairs for the RNA sequence.
*/
    pub bases: Option<String>,
    #[serde(rename = "customFields")]
    /**Custom fields to add to the RNA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Fields to set on the RNA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    /**ID of the folder containing the RNA sequence.
*/
    pub folder_id: Option<String>,
    #[serde(rename = "isCircular")]
    /**Whether the RNA sequence is circular or linear. RNA sequences can only be linear
*/
    pub is_circular: Option<bool>,
    /**Name of the RNA sequence.
*/
    pub name: Option<String>,
    pub primers: Option<Vec<Primer>>,
    #[serde(rename = "schemaId")]
    /**ID of the RNA sequence's schema.
*/
    pub schema_id: Option<String>,
    /**Translations to create on the RNA sequence. Translations are specified by either a combination of 'start' and 'end' fields, or a list of regions. Both cannot be provided.
*/
    pub translations: Option<Vec<Translation>>,
}
impl std::fmt::Display for RnaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBaseRequestForCreate(
    pub RnaSequenceBaseRequest,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBulkCreate(pub RnaSequenceCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBulkUpdate(pub serde_json::Value, RnaSequenceBaseRequest);
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceCreate(
    pub RnaSequenceBaseRequestForCreate,
    CreateEntityIntoRegistry,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequenceRequestRegistryFields {
    #[serde(rename = "entityRegistryId")]
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for RnaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceUpdate(
    pub RnaSequenceBaseRequest,
    RnaSequenceRequestRegistryFields,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesArchivalChange {
    #[serde(rename = "rnaSequenceIds")]
    pub rna_sequence_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RnaSequencesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    #[serde(rename = "rnaSequenceIds")]
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for RnaSequencesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkCreateRequest {
    #[serde(rename = "rnaSequences")]
    pub rna_sequences: Option<Vec<RnaSequenceBulkCreate>>,
}
impl std::fmt::Display for RnaSequencesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkGet {
    #[serde(rename = "rnaSequences")]
    pub rna_sequences: Option<Vec<RnaSequence>>,
}
impl std::fmt::Display for RnaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkUpdateRequest {
    #[serde(rename = "rnaSequences")]
    pub rna_sequences: Option<Vec<RnaSequenceBulkUpdate>>,
}
impl std::fmt::Display for RnaSequencesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "rnaSequences")]
    pub rna_sequences: Option<Vec<RnaSequence>>,
}
impl std::fmt::Display for RnaSequencesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesUnarchive {
    #[serde(rename = "rnaSequenceIds")]
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for RnaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SampleGroup {
    pub id: Option<String>,
    pub samples: Option<serde_json::Value>,
}
impl std::fmt::Display for SampleGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum SampleGroupStatus {
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SampleGroupStatusUpdate {
    #[serde(rename = "sampleGroupId")]
    ///The string id of the sample group
    pub sample_group_id: String,
    ///Status of a sample group
    pub status: String,
}
impl std::fmt::Display for SampleGroupStatusUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SampleGroupsStatusUpdate {
    #[serde(rename = "sampleGroups")]
    pub sample_groups: Vec<SampleGroupStatusUpdate>,
}
impl std::fmt::Display for SampleGroupsStatusUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Schema {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "fieldDefinitions")]
    pub field_definitions: Option<Vec<serde_json::Value>>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchemaFieldsQueryParam {}
impl std::fmt::Display for SchemaFieldsQueryParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaLinkFieldDefinition(pub FieldDefinition, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SchemaSummary {
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for SchemaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SequenceFeatureBase {
    ///Hex color code used when displaying this feature in the UI.
    pub color: Option<String>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<Vec<SequenceFeatureCustomField>>,
    pub name: Option<String>,
    pub notes: Option<String>,
}
impl std::fmt::Display for SequenceFeatureBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SequenceFeatureCustomField {
    ///Name of the custom field
    pub name: Option<String>,
    ///Value of the custom field
    pub value: Option<String>,
}
impl std::fmt::Display for SequenceFeatureCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleFieldDefinition(pub FieldDefinition, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleNotePart(pub BaseNotePart, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StageEntry {
    #[serde(rename = "apiURL")]
    ///The canonical url of the Stage Entry in the API.
    pub api_url: Option<String>,
    /**Array of UserSummary Resources of the authors of the stage entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    #[serde(rename = "createdAt")]
    ///DateTime the stage entry was created at
    pub created_at: Option<String>,
    ///UserSummary Resource of the user who created the stage entry
    pub creator: Option<UserSummary>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "displayId")]
    ///User-friendly ID of the stage entry
    pub display_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the stage entry
    pub folder_id: Option<String>,
    ///ID of the stage entry
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the stage entry was last modified
    pub modified_at: Option<String>,
    ///Title of the stage entry
    pub name: Option<String>,
    #[serde(rename = "reviewRecord")]
    ///Review record if set
    pub review_record: Option<serde_json::Value>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    #[serde(rename = "webURL")]
    ///URL of the stage entry
    pub web_url: Option<String>,
    #[serde(rename = "workflowId")]
    ///ID of the parent workflow
    pub workflow_id: Option<String>,
    #[serde(rename = "workflowStageId")]
    ///ID of the associated workflow stage
    pub workflow_stage_id: Option<String>,
}
impl std::fmt::Display for StageEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntryCreatedEvent(pub EventBase);
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntryUpdatedFieldsEvent(
    pub EventBase,
    UpdateEventMixin,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntryUpdatedReviewRecordEvent(
    pub EventBase,
    UpdateEventMixin,
    serde_json::Value,
);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StructuredTableApiIdentifiers {
    #[serde(rename = "apiId")]
    pub api_id: Option<String>,
    pub columns: Option<Vec<StructuredTableColumnInfo>>,
}
impl std::fmt::Display for StructuredTableApiIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StructuredTableColumnInfo {
    #[serde(rename = "columnId")]
    pub column_id: Option<String>,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: Option<bool>,
    pub name: Option<String>,
}
impl std::fmt::Display for StructuredTableColumnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TableNotePart(pub BaseNotePart, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Team {}
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamSummary(pub PartySummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenCreate {
    /**ID of client to request token for. Leave off if client ID and secret are being supplied through Authorization header.
*/
    pub client_id: Option<String>,
    /**Leave off if client ID and secret are being supplied through Authorization header.
*/
    pub client_secret: Option<String>,
    pub grant_type: String,
}
impl std::fmt::Display for TokenCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    ///Number of seconds that token is valid for
    pub expires_in: Option<i64>,
    pub token_type: Option<String>,
}
impl std::fmt::Display for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransfersAsyncTask {}
impl std::fmt::Display for TransfersAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Translation(pub SequenceFeatureBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnregisterEntities {
    #[serde(rename = "entityIds")]
    ///Array of entity IDs
    pub entity_ids: Vec<String>,
    #[serde(rename = "folderId")]
    ///ID of the folder that the entities should be moved to
    pub folder_id: String,
}
impl std::fmt::Display for UnregisterEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateEventMixin {
    /**These properties have been updated, causing this message
*/
    pub updates: Option<Vec<String>>,
}
impl std::fmt::Display for UpdateEventMixin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummary(pub PartySummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserValidation {
    #[serde(rename = "validationComment")]
    ///A string explaining the reason for the provided validation status.
    pub validation_comment: Option<String>,
    #[serde(rename = "validationStatus")]
    /**Valid values for this enum depend on whether it is used to set a value (e.g., in a POST request), or is a return value for an existing result.
Acceptable values for setting a status are 'VALID' or 'INVALID'. Possible return values are 'VALID', 'INVALID', or 'PARTIALLY_VALID' (a result will be partially valid if it has some valid fields and some invalid fields).
*/
    pub validation_status: Option<String>,
}
impl std::fmt::Display for UserValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersPaginatedList(pub Pagination, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WarehouseCredentials {
    #[serde(rename = "expiresAt")]
    /**The time after which new connections using the username/password will not be permitted. Upon expiration, currently open connections are not terminated.
*/
    pub expires_at: Option<String>,
    ///The password to connect to the warehouse.
    pub password: Option<String>,
    ///The username to connect to the warehouse.
    pub username: Option<String>,
}
impl std::fmt::Display for WarehouseCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WarehouseCredentialsCreate {
    #[serde(rename = "expiresIn")]
    /**Duration, in seconds, that credentials should be active for. Must be greater than 0 and less than 3600.
*/
    pub expires_in: i64,
}
impl std::fmt::Display for WarehouseCredentialsCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Well {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    ///Barcode of the well
    pub barcode: Option<String>,
    #[serde(rename = "checkoutRecord")]
    pub checkout_record: Option<CheckoutRecord>,
    ///Array of well contents, each with a batch and concentration
    pub contents: Option<Vec<ContainerContent>>,
    #[serde(rename = "createdAt")]
    ///DateTime the well was created
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    ///ID of the well
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    ///DateTime the well was last modified
    pub modified_at: Option<String>,
    ///Name of the well, defaults to barcode if name is not provided.
    pub name: Option<String>,
    #[serde(rename = "parentStorageId")]
    ///ID of containing parent storage, a plate well with a coordinate (e.g. plt_2bAks9dx:a2).
    pub parent_storage_id: Option<String>,
    #[serde(rename = "parentStorageSchema")]
    pub parent_storage_schema: Option<SchemaSummary>,
    #[serde(rename = "projectId")]
    ///ID of the project if set
    pub project_id: Option<String>,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: Option<ContainerQuantity>,
    pub schema: Option<SchemaSummary>,
    pub volume: Option<DeprecatedContainerVolumeForResponse>,
    #[serde(rename = "webURL")]
    pub web_url: Option<String>,
}
impl std::fmt::Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowList {
    pub workflows: Option<Vec<LegacyWorkflow>>,
}
impl std::fmt::Display for WorkflowList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutput(pub WorkflowOutputSummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowOutputArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputBulkCreate(pub WorkflowOutputCreate);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputBulkUpdate(pub WorkflowOutputWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputCreate(pub WorkflowOutputWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputSchema {
    #[serde(rename = "archiveRecord")]
    pub archive_record: Option<ArchiveRecord>,
    #[serde(rename = "fieldDefinitions")]
    pub field_definitions: Option<Vec<serde_json::Value>>,
    pub name: Option<String>,
    pub prefix: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for WorkflowOutputSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputSummary {
    #[serde(rename = "displayId")]
    ///User-friendly ID of the workflow task group
    pub display_id: Option<String>,
    ///The ID of the workflow output
    pub id: Option<String>,
}
impl std::fmt::Display for WorkflowOutputSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputUpdate(pub WorkflowOutputWriteBase);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputUpdatedFieldsEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputWriteBase {
    pub fields: Option<Fields>,
}
impl std::fmt::Display for WorkflowOutputWriteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsArchivalChange {
    #[serde(rename = "workflowOutputIds")]
    pub workflow_output_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowOutputsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsArchive {
    /**The reason for archiving the provided workflow outputs. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    #[serde(rename = "workflowOutputIds")]
    pub workflow_output_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowOutputsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsBulkCreateRequest {
    #[serde(rename = "workflowOutputs")]
    pub workflow_outputs: Option<Vec<WorkflowOutputBulkCreate>>,
}
impl std::fmt::Display for WorkflowOutputsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsBulkUpdateRequest {
    #[serde(rename = "workflowOutputs")]
    pub workflow_outputs: Option<Vec<WorkflowOutputBulkUpdate>>,
}
impl std::fmt::Display for WorkflowOutputsBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "workflowOutputs")]
    pub workflow_outputs: Option<Vec<WorkflowOutput>>,
}
impl std::fmt::Display for WorkflowOutputsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsUnarchive {
    #[serde(rename = "workflowOutputIds")]
    pub workflow_output_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowOutputsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowPatch {
    ///Description of the workflow
    pub description: Option<String>,
    ///Name of the workflow
    pub name: Option<String>,
    #[serde(rename = "projectId")]
    ///ID of the project that contains the workflow
    pub project_id: Option<String>,
}
impl std::fmt::Display for WorkflowPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowSample {
    #[serde(rename = "batchId")]
    ///ID of the batch
    pub batch_id: Option<String>,
    #[serde(rename = "containerIds")]
    ///Array of IDs of containers
    pub container_ids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    ///DateTime at which the the sample was created
    pub created_at: Option<String>,
    ///ID of the sample
    pub id: Option<String>,
    ///Name of the sample
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowSampleList {
    pub samples: Option<Vec<WorkflowSample>>,
}
impl std::fmt::Display for WorkflowSampleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStage {
    #[serde(rename = "createdAt")]
    ///DateTime at which the the workflow stage was created
    pub created_at: Option<String>,
    ///ID of the workflow stage
    pub id: Option<String>,
    ///Name of the workflow stage
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageList {
    #[serde(rename = "workflowStages")]
    pub workflow_stages: Option<Vec<WorkflowStage>>,
}
impl std::fmt::Display for WorkflowStageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageRun {
    #[serde(rename = "createdAt")]
    ///DateTime at which the the stage run was created
    pub created_at: Option<String>,
    ///ID of the stage run
    pub id: Option<String>,
    ///Name of the stage run
    pub name: Option<String>,
    ///Status of the stage run
    pub status: Option<String>,
}
impl std::fmt::Display for WorkflowStageRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageRunList {
    #[serde(rename = "workflowStageRuns")]
    pub workflow_stage_runs: Option<Vec<WorkflowStageRun>>,
}
impl std::fmt::Display for WorkflowStageRunList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTask(pub WorkflowTaskBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowTaskArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskBase(pub WorkflowTaskSummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskBulkCreate(pub WorkflowTaskCreate);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskBulkUpdate {}
impl std::fmt::Display for WorkflowTaskBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskCreate(pub WorkflowTaskWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskExecutionOrigin {
    #[serde(rename = "entryId")]
    pub entry_id: Option<String>,
    #[serde(rename = "originModalUuid")]
    pub origin_modal_uuid: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for WorkflowTaskExecutionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroup(pub WorkflowTaskGroupBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowTaskGroupArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupBase(pub WorkflowTaskGroupSummary, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupCreate(pub WorkflowTaskGroupWriteBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupCreatedEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupSummary {
    #[serde(rename = "displayId")]
    ///User-friendly ID of the workflow task group
    pub display_id: Option<String>,
    ///The ID of the workflow task group
    pub id: Option<String>,
    ///The name of the workflow task group
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowTaskGroupSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupUpdate(pub WorkflowTaskGroupWriteBase);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupUpdatedWatchersEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupWriteBase {
    #[serde(rename = "folderId")]
    ///ID of the folder that contains the workflow task group
    pub folder_id: Option<String>,
    ///The name of the workflow task group
    pub name: Option<String>,
    #[serde(rename = "watcherIds")]
    ///IDs of the users watching the workflow task group
    pub watcher_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTaskGroupWriteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsArchivalChange {
    #[serde(rename = "workflowTaskGroupIds")]
    pub workflow_task_group_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTaskGroupsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsArchive {
    /**The reason for archiving the provided workflow task groups. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    #[serde(rename = "workflowTaskGroupIds")]
    pub workflow_task_group_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTaskGroupsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "workflowTaskGroups")]
    pub workflow_task_groups: Option<Vec<WorkflowTaskGroup>>,
}
impl std::fmt::Display for WorkflowTaskGroupsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsUnarchive {
    #[serde(rename = "workflowTaskGroupIds")]
    pub workflow_task_group_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTaskGroupsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSchema(pub WorkflowTaskSchemaBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSchemaBase(pub Schema, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskSchemaSummary {
    ///The ID of the workflow task schema
    pub id: Option<String>,
    ///The name of the workflow task schema
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowTaskSchemaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskSchemasPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "workflowTaskSchemas")]
    pub workflow_task_schemas: Option<Vec<WorkflowTaskSchema>>,
}
impl std::fmt::Display for WorkflowTaskSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskStatus {
    #[serde(rename = "displayName")]
    ///The status label
    pub display_name: Option<String>,
    ///The ID of the workflow task status
    pub id: Option<String>,
    #[serde(rename = "statusType")]
    ///The status type
    pub status_type: Option<String>,
}
impl std::fmt::Display for WorkflowTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskStatusLifecycle {
    pub id: Option<String>,
    #[serde(rename = "initialStatus")]
    pub initial_status: Option<WorkflowTaskStatus>,
    pub name: Option<String>,
    pub statuses: Option<Vec<WorkflowTaskStatus>>,
    pub transitions: Option<Vec<WorkflowTaskStatusLifecycleTransition>>,
}
impl std::fmt::Display for WorkflowTaskStatusLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskStatusLifecycleTransition {
    pub from: Option<WorkflowTaskStatus>,
    pub to: Option<WorkflowTaskStatus>,
}
impl std::fmt::Display for WorkflowTaskStatusLifecycleTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskSummary {
    #[serde(rename = "displayId")]
    ///User-friendly ID of the workflow task
    pub display_id: Option<String>,
    ///The ID of the workflow task
    pub id: Option<String>,
}
impl std::fmt::Display for WorkflowTaskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskUpdate {}
impl std::fmt::Display for WorkflowTaskUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedAssigneeEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedFieldsEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedScheduledOnEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedStatusEvent(pub EventBase, serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskWriteBase {
    #[serde(rename = "assigneeId")]
    ///The id of the user assigned to the task
    pub assignee_id: Option<String>,
    pub fields: Option<Fields>,
    #[serde(rename = "scheduledOn")]
    ///The date on which the task is scheduled to be executed
    pub scheduled_on: Option<String>,
}
impl std::fmt::Display for WorkflowTaskWriteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksArchivalChange {
    #[serde(rename = "workflowTaskIds")]
    pub workflow_task_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTasksArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksArchive {
    /**The reason for archiving the provided workflow tasks. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    #[serde(rename = "workflowTaskIds")]
    pub workflow_task_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTasksArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkCopyRequest {
    #[serde(rename = "workflowTaskIds")]
    pub workflow_task_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTasksBulkCopyRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkCreateRequest {
    #[serde(rename = "workflowTasks")]
    pub workflow_tasks: Option<Vec<WorkflowTaskBulkCreate>>,
}
impl std::fmt::Display for WorkflowTasksBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkUpdateRequest {
    #[serde(rename = "workflowTasks")]
    pub workflow_tasks: Option<Vec<WorkflowTaskBulkUpdate>>,
}
impl std::fmt::Display for WorkflowTasksBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksPaginatedList {
    #[serde(rename = "nextToken")]
    pub next_token: Option<String>,
    #[serde(rename = "workflowTasks")]
    pub workflow_tasks: Option<Vec<WorkflowTask>>,
}
impl std::fmt::Display for WorkflowTasksPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksUnarchive {
    #[serde(rename = "workflowTaskIds")]
    pub workflow_task_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTasksUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

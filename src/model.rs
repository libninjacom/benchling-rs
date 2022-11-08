use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkUpdateRequest {
    pub dna_sequences: Option<Vec<DnaSequenceBulkUpdate>>,
}
impl std::fmt::Display for DnaSequencesBulkUpdateRequest {
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
pub struct WorkflowTaskSchemasPaginatedList {
    pub next_token: Option<String>,
    pub workflow_task_schemas: Option<Vec<WorkflowTaskSchema>>,
}
impl std::fmt::Display for WorkflowTaskSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenResponse {
    ///Number of seconds that token is valid for
    pub expires_in: Option<i64>,
    pub access_token: Option<String>,
    pub token_type: Option<String>,
}
impl std::fmt::Display for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaConsensusAlignmentCreate {
    pub new_sequence: serde_json::Value,
    pub sequence_id: String,
    pub dna_alignment_base: DnaAlignmentBase,
}
impl std::fmt::Display for DnaConsensusAlignmentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntitySchema {
    pub containable_type: String,
    pub constraint: Option<serde_json::Value>,
    ///DateTime the Entity Schema was last modified
    pub modified_at: String,
    pub type_: String,
    pub registry_schema: RegistrySchema,
}
impl std::fmt::Display for EntitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConsensusNucleotideAlignmentRequired {
    pub new_sequence: serde_json::Value,
    pub algorithm: String,
    pub sequence_id: String,
    pub files: Vec<serde_json::Value>,
    pub name: String,
}
impl std::fmt::Display for CreateConsensusNucleotideAlignmentRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesArchive {
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
pub struct TokenCreate {
    pub grant_type: String,
    /**ID of client to request token for. Leave off if client ID and secret are being supplied through Authorization header.
*/
    pub client_id: Option<String>,
    /**Leave off if client ID and secret are being supplied through Authorization header.
*/
    pub client_secret: Option<String>,
}
impl std::fmt::Display for TokenCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceWithEntityType {}
impl std::fmt::Display for DnaSequenceWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageRun {
    ///DateTime at which the the legacy workflow stage run was created
    pub created_at: Option<String>,
    ///Status of the legacy workflow stage run
    pub status: Option<String>,
    ///Name of the legacy workflow stage run
    pub name: Option<String>,
    ///ID of the legacy workflow stage run
    pub id: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowStageRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesUnarchive {
    ///Array of plate IDs
    pub plate_ids: Vec<String>,
}
impl std::fmt::Display for PlatesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemGenericCreate {}
impl std::fmt::Display for AppConfigItemGenericCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoBaseRequest {
    /**Name of the Oligo.
*/
    pub name: Option<String>,
    ///IDs of users to set as the Oligo's authors.
    pub author_ids: Option<Vec<String>>,
    /**Fields to set on the Oligo. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    /**ID of the folder containing the Oligo.
*/
    pub folder_id: Option<String>,
    /**ID of the oligo's schema.
*/
    pub schema_id: Option<String>,
    /**Base pairs of the oligo.
*/
    pub bases: Option<String>,
    ///Aliases to add to the Oligo
    pub aliases: Option<Vec<String>>,
    /**Custom fields to add to the Oligo. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
}
impl std::fmt::Display for OligoBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculesPaginatedList {
    pub pagination: Pagination,
    pub molecules: Vec<Molecule>,
}
impl std::fmt::Display for MoleculesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosArchivalChange {
    pub oligo_ids: Option<Vec<String>>,
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for OligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemGenericBulkUpdate {}
impl std::fmt::Display for AppConfigItemGenericBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsPaginatedList {
    pub assay_runs: Option<Vec<AssayRun>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayRunsPaginatedList {
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
pub struct FeatureBase {
    ///The pattern used for matching during auto-annotation.
    pub pattern: Option<String>,
    ///The id of the feature library the feature belongs to
    pub feature_library_id: Option<String>,
    /**The type of feature, like gene, promoter, etc. Note: This is an arbitrary string, not an enum
*/
    pub feature_type: Option<String>,
    ///The color of the annotations generated by the feature. Must be a valid hex string
    pub color: Option<String>,
    ///The name of the feature
    pub name: Option<String>,
}
impl std::fmt::Display for FeatureBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemDateBulkUpdate {}
impl std::fmt::Display for AppConfigItemDateBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOrInaccessibleResource(pub serde_json::Value);
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
pub struct WorkflowPatch {
    ///Name of the workflow
    pub name: Option<String>,
    ///ID of the project that contains the workflow
    pub project_id: Option<String>,
    ///Description of the workflow
    pub description: Option<String>,
}
impl std::fmt::Display for WorkflowPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobPartCreate {
    pub data64: String,
    pub md5: String,
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
pub struct RequestTasksBulkCreateResponse {
    ///The created tasks
    pub tasks: Option<Vec<RequestTask>>,
}
impl std::fmt::Display for RequestTasksBulkCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleNotePart {
    ///The textual contents of the note.
    pub text: String,
    pub base_note_part: BaseNotePart,
    /**Array of links referenced in text via an @-mention, hyperlink, or the drag-n-dropped preview attached to the note.
*/
    pub links: Vec<EntryLink>,
    pub type_: String,
}
impl std::fmt::Display for SimpleNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsUnarchive {
    pub workflow_output_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowOutputsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequence {
    pub schema: Option<SchemaSummary>,
    pub creator: Option<UserSummary>,
    ///Array of aliases
    pub aliases: Option<Vec<String>>,
    pub fields: Option<Fields>,
    ///Custom fields set on the AA sequence.
    pub custom_fields: Option<CustomFields>,
    ///Number of amino acids in the AA sequence.
    pub length: Option<i64>,
    ///Name of the AA sequence.
    pub name: Option<String>,
    ///Registry the AA sequence is registered in.
    pub registry_id: Option<String>,
    ///DateTime the AA sequence was created.
    pub created_at: Option<String>,
    ///ID of the AA sequence.
    pub id: Option<String>,
    ///Amino acids of the AA sequence.
    pub amino_acids: Option<String>,
    ///The canonical url of the AA Sequence in the API.
    pub api_url: Option<String>,
    ///Registry ID of the AA sequence if registered.
    pub entity_registry_id: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub registration_origin: Option<RegistrationOrigin>,
    ///URL of the protein.
    pub web_url: Option<String>,
    ///ID of the folder that contains the AA sequence.
    pub folder_id: Option<String>,
    ///Array of annotation objects on the AA sequence.
    pub annotations: Option<Vec<AaAnnotation>>,
    ///DateTime the AA sequence was last modified.
    pub modified_at: Option<String>,
}
impl std::fmt::Display for AaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDnaOligoRequired {
    pub schema_id: String,
    pub name: String,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub aliases: Vec<String>,
    pub bases: String,
    pub registry_id: String,
    pub author_ids: Vec<String>,
    pub entity_registry_id: String,
    pub naming_strategy: String,
    pub folder_id: String,
}
impl std::fmt::Display for CreateDnaOligoRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Plate {
    pub web_url: Option<String>,
    ///Well contents of the plate, keyed by position string (eg. "A1").
    pub wells: Option<serde_json::Value>,
    ///DateTime the container was created
    pub created_at: Option<String>,
    ///Barcode of the plate
    pub barcode: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    ///DateTime the plate was last modified
    pub modified_at: Option<String>,
    ///ID of the project if set
    pub project_id: Option<String>,
    pub type_: Option<String>,
    ///ID of the plate
    pub id: Option<String>,
    pub fields: Option<Fields>,
    ///ID of containing parent storage (e.g. loc_k2lNspzS).
    pub parent_storage_id: Option<String>,
    ///Name of the plate, defaults to barcode if name is not provided.
    pub name: Option<String>,
    pub creator: Option<UserSummary>,
    pub schema: Option<SchemaSummary>,
}
impl std::fmt::Display for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkCreateResponse {
    pub assay_runs: Option<Vec<String>>,
}
impl std::fmt::Display for AssayRunsBulkCreateResponse {
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
pub struct AaSequenceBaseRequest {
    ///IDs of users to set as the AA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    ///Aliases to add to the AA sequence
    pub aliases: Option<Vec<String>>,
    /**Amino acids for the AA sequence.
*/
    pub amino_acids: Option<String>,
    /**Custom fields to add to the AA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Name of the AA sequence.
*/
    pub name: Option<String>,
    /**Fields to set on the AA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    /**Annotations to create on the AA sequence.
*/
    pub annotations: Option<Vec<AaAnnotation>>,
    /**ID of the folder containing the AA sequence.
*/
    pub folder_id: Option<String>,
    /**ID of the AA sequence's schema.
*/
    pub schema_id: Option<String>,
}
impl std::fmt::Display for AaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesList {
    pub custom_entities: Option<Vec<CustomEntity>>,
}
impl std::fmt::Display for CustomEntitiesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequenceRequestRegistryFields {
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for RnaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityCreate {
    pub create_entity_into_registry: CreateEntityIntoRegistry,
    pub custom_entity_base_request_for_create: CustomEntityBaseRequestForCreate,
}
impl std::fmt::Display for CustomEntityCreate {
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
pub struct BarcodesList {
    ///Array of barcodes to validate.
    pub barcodes: Vec<String>,
}
impl std::fmt::Display for BarcodesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosBulkCreateRequest {
    pub dna_oligos: Option<Vec<DnaOligoCreate>>,
}
impl std::fmt::Display for DnaOligosBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesUnarchive {
    ///Array of entry IDs
    pub entry_ids: Vec<String>,
}
impl std::fmt::Display for EntriesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryTemplatesPaginatedList {
    pub next_token: Option<String>,
    pub entry_templates: Option<Vec<EntryTemplate>>,
}
impl std::fmt::Display for EntryTemplatesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationCreate {
    pub schema_id: String,
    pub parent_storage_id: Option<String>,
    pub fields: Option<Fields>,
    pub barcode: Option<String>,
    pub name: String,
}
impl std::fmt::Display for LocationCreate {
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
pub struct LocationsBulkGet {
    pub locations: Option<Vec<Location>>,
}
impl std::fmt::Display for LocationsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputWriteBase(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub id: Option<String>,
    pub is_multi: Option<bool>,
    pub name: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub is_required: Option<bool>,
    pub type_: Option<String>,
}
impl std::fmt::Display for FieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkCreateRequest {
    pub assay_runs: Vec<AssayRunCreate>,
}
impl std::fmt::Display for AssayRunsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTemplate {
    ///ID of the collection that contains the template
    pub template_collection_id: Option<String>,
    ///DateTime the template was created at
    pub created_at: Option<String>,
    ///The canonical url of the Entry Template in the API.
    pub api_url: Option<String>,
    ///UserSummary Resource of the user who created the template
    pub creator: Option<UserSummary>,
    pub custom_fields: Option<CustomFields>,
    ///ID of the entry template
    pub id: Option<String>,
    ///DateTime the template was last modified
    pub modified_at: Option<String>,
    ///Title of the template
    pub name: Option<String>,
    pub fields: Option<Fields>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    ///URL of the template
    pub web_url: Option<String>,
    /**Array of day objects. Each day object has a day index (integer) and notes field (array of notes, expand further for details on note types).
*/
    pub days: Option<Vec<EntryTemplateDay>>,
}
impl std::fmt::Display for EntryTemplate {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesBulkUpdateRequest {
    pub aa_sequences: Option<Vec<AaSequenceBulkUpdate>>,
}
impl std::fmt::Display for AaSequencesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflow {
    ///DateTime at which the the legacy workflow was created
    pub created_at: Option<String>,
    ///Description of the legacy workflow
    pub description: Option<String>,
    ///ID of the legacy workflow
    pub id: Option<String>,
    ///Name of the legacy workflow
    pub name: Option<String>,
    ///User-friendly ID of the legacy workflow
    pub display_id: Option<String>,
    ///ID of the project that contains the legacy workflow
    pub project_id: Option<String>,
}
impl std::fmt::Display for LegacyWorkflow {
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
pub struct RnaOligoWithEntityType {}
impl std::fmt::Display for RnaOligoWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsBulkCreateRequest {
    pub workflow_outputs: Option<Vec<WorkflowOutputBulkCreate>>,
}
impl std::fmt::Display for WorkflowOutputsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsArchivalChange {
    pub workflow_task_group_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTaskGroupsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerBulkUpdateItem {
    /**Desired volume for a container, well, or transfer. "volume" type keys are deprecated in API requests; use the more permissive "quantity" type key instead.
*/
    pub volume: DeprecatedContainerVolumeForInput,
    pub container_id: String,
    pub container_write_base: ContainerWriteBase,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: ContainerQuantity,
}
impl std::fmt::Display for ContainerBulkUpdateItem {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct EntrySchemaDetailed {
    pub registry_schema: RegistrySchema,
    pub type_: String,
}
impl std::fmt::Display for EntrySchemaDetailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsBulkGet {
    pub assay_results: Option<Vec<AssayResult>>,
}
impl std::fmt::Display for AssayResultsBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSchema {
    pub type_: String,
    pub width: f64,
    pub registry_schema: RegistrySchema,
    pub container_schema: Option<serde_json::Value>,
    pub height: f64,
}
impl std::fmt::Display for BoxSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreationOrigin {
    pub origin_type: Option<String>,
    pub application: Option<String>,
    pub origin_id: Option<String>,
    pub origin_modal_uuid: Option<String>,
}
impl std::fmt::Display for CreationOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BarcodeValidationResults {
    pub validation_results: Option<Vec<BarcodeValidationResult>>,
}
impl std::fmt::Display for BarcodeValidationResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsPaginatedList {
    pub teams: Vec<Team>,
    pub pagination: Pagination,
}
impl std::fmt::Display for TeamsPaginatedList {
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
pub struct WorkflowSample {
    ///Name of the sample
    pub name: Option<String>,
    ///ID of the sample
    pub id: Option<String>,
    ///DateTime at which the the sample was created
    pub created_at: Option<String>,
    ///Array of IDs of containers
    pub container_ids: Option<Vec<String>>,
    ///ID of the batch
    pub batch_id: Option<String>,
}
impl std::fmt::Display for WorkflowSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRnaSequenceRequired {
    pub primers: Vec<Primer>,
    pub author_ids: Vec<String>,
    pub is_circular: bool,
    pub entity_registry_id: String,
    pub bases: String,
    pub annotations: Vec<RnaAnnotation>,
    pub aliases: Vec<String>,
    pub registry_id: String,
    pub custom_fields: CustomFields,
    pub name: String,
    pub translations: Vec<Translation>,
    pub schema_id: String,
    pub folder_id: String,
    pub fields: Fields,
    pub naming_strategy: String,
}
impl std::fmt::Display for CreateRnaSequenceRequired {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessor {
    ///DateTime the Automation Output Processor was created
    pub created_at: String,
    ///Processing progress information.
    pub progress_stats: AutomationProgressStats,
    ///DateTime the Automation Output Processor was last modified
    pub modified_at: String,
    ///The canonical url of the Automation Output Processor in the API.
    pub api_url: String,
    pub archive_record: ArchiveRecord,
    pub id: String,
    pub transforms: Vec<LabAutomationTransform>,
    pub automation_file: AutomationFile,
    pub error_file: Option<Blob>,
    ///Specifies whether file processing should complete with errors. False means any error in output file processing will result in no actions being committed. True means that if row-level errors occur, then failing rows and their errors will be saved to errorFile, and actions from successful rows will be committed.
    pub complete_with_errors: bool,
}
impl std::fmt::Display for AutomationOutputProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerWithCoordinates {
    pub container: Container,
    pub grid_number: f64,
    pub grid_position: String,
}
impl std::fmt::Display for ContainerWithCoordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrganizationSummary {
    pub handle: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for OrganizationSummary {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfigItemApiMixin {
    pub app: Option<serde_json::Value>,
    ///DateTime the app config item was last modified
    pub modified_at: Option<String>,
    ///Array-based representation of config item's location in the tree in order from top to bottom.
    pub path: Option<Vec<String>>,
    pub api_url: Option<String>,
    ///Type of the app config item
    pub type_: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for AppConfigItemApiMixin {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerTransfer {}
impl std::fmt::Display for ContainerTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesPaginatedList {
    pub dna_sequences: Option<Vec<DnaSequence>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for DnaSequencesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSchema {
    ///DateTime the Request Schema was last modified
    pub modified_at: String,
    pub system_name: String,
    pub type_: String,
    pub schema: Schema,
    /**The organization that owns the schema.
*/
    pub organization: serde_json::Value,
}
impl std::fmt::Display for RequestSchema {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoCreate {
    pub create_entity_into_registry: CreateEntityIntoRegistry,
    pub oligo_base_request_for_create: OligoBaseRequestForCreate,
}
impl std::fmt::Display for OligoCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequence {
    pub bases: Option<String>,
    pub created_at: Option<String>,
    pub annotations: Option<Vec<RnaAnnotation>>,
    pub translations: Option<Vec<Translation>>,
    ///The canonical url of the RNA Sequence in the API.
    pub api_url: Option<String>,
    pub entity_registry_id: Option<String>,
    pub folder_id: Option<String>,
    pub name: Option<String>,
    pub registration_origin: Option<RegistrationOrigin>,
    pub fields: Option<Fields>,
    pub modified_at: Option<String>,
    pub id: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub custom_fields: Option<CustomFields>,
    pub is_circular: Option<bool>,
    pub length: Option<i64>,
    pub creator: Option<UserSummary>,
    pub registry_id: Option<String>,
    pub web_url: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub aliases: Option<Vec<String>>,
    pub primers: Option<Vec<Primer>>,
}
impl std::fmt::Display for RnaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowTaskGroupArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
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
pub struct AssayRunSchemasPaginatedList {
    pub assay_run_schemas: Option<Vec<AssayRunSchema>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayRunSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesUnarchive {
    pub dna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for DnaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateCreate {
    pub name: Option<String>,
    pub barcode: Option<String>,
    pub container_schema_id: Option<String>,
    pub fields: Option<Fields>,
    pub parent_storage_id: Option<String>,
    pub project_id: Option<String>,
    pub schema_id: String,
    pub wells: Option<serde_json::Value>,
}
impl std::fmt::Display for PlateCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupUpdatedWatchersEvent {
    pub workflow_task_group: WorkflowTaskGroup,
    pub event_base: EventBase,
    pub event_type: String,
}
impl std::fmt::Display for WorkflowTaskGroupUpdatedWatchersEvent {
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
pub struct BoxesArchivalChange {
    pub box_ids: Option<Vec<String>>,
    pub container_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BoxesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRnaOligoRequired {
    pub registry_id: String,
    pub bases: String,
    pub fields: Fields,
    pub folder_id: String,
    pub author_ids: Vec<String>,
    pub name: String,
    pub naming_strategy: String,
    pub aliases: Vec<String>,
    pub schema_id: String,
    pub custom_fields: CustomFields,
    pub entity_registry_id: String,
}
impl std::fmt::Display for CreateRnaOligoRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerSchemasPaginatedList {
    pub next_token: String,
    pub container_schemas_list: ContainerSchemasList,
}
impl std::fmt::Display for ContainerSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemFloatUpdate {
    pub type_: String,
    pub value: f64,
}
impl std::fmt::Display for AppConfigItemFloatUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub is_multi: Option<bool>,
    pub display_value: Option<String>,
    pub text_value: Option<String>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdate {
    pub default_concentration: Option<DefaultConcentrationSummary>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for BatchUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaAnnotation {
    pub strand: i64,
    pub end: i64,
    pub type_: String,
    pub sequence_feature_base: SequenceFeatureBase,
    pub start: i64,
}
impl std::fmt::Display for DnaAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionUpdate {
    ///Name of the dropdown option.
    pub name: String,
    ///ID of the dropdown option to update, omitted if creating a new option.
    pub id: Option<String>,
}
impl std::fmt::Display for DropdownOptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Mixture {
    pub modified_at: Option<String>,
    pub created_at: Option<String>,
    pub registration_origin: Option<RegistrationOrigin>,
    ///Mixtures can have up to one parent mixture field.
    pub fields: Option<Fields>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub creator: Option<serde_json::Value>,
    pub entity_registry_id: Option<String>,
    ///Derived from the mixture's schema.
    pub allow_measured_ingredients: Option<bool>,
    pub custom_fields: Option<CustomFields>,
    pub registry_id: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub authors: Option<Vec<UserSummary>>,
    ///The positive numerical amount value of this mixture in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4).
    pub amount: Option<String>,
    pub id: Option<String>,
    pub folder_id: Option<String>,
    ///List of ingredients on this mixture.
    pub ingredients: Option<Vec<Ingredient>>,
    pub web_url: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub units: Option<String>,
    ///The canonical url of the Mixture in the API.
    pub api_url: Option<String>,
}
impl std::fmt::Display for Mixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosBulkUpdateRequest {
    pub dna_oligos: Option<Vec<DnaOligoBulkUpdate>>,
}
impl std::fmt::Display for DnaOligosBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemDateUpdate {
    pub value: String,
    pub type_: String,
}
impl std::fmt::Display for AppConfigItemDateUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemDatetimeCreate {}
impl std::fmt::Display for AppConfigItemDatetimeCreate {
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
pub struct SimpleFieldDefinition {
    pub type_: String,
    pub field_definition: FieldDefinition,
}
impl std::fmt::Display for SimpleFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleWithBatch {
    pub batch_id: String,
    pub container_id: Option<String>,
}
impl std::fmt::Display for RequestSampleWithBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxSchemasPaginatedList {
    pub box_schemas_list: BoxSchemasList,
    pub next_token: String,
}
impl std::fmt::Display for BoxSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Well {
    ///ID of containing parent storage, a plate well with a coordinate (e.g. plt_2bAks9dx:a2).
    pub parent_storage_id: Option<String>,
    ///Array of well contents, each with a batch and concentration
    pub contents: Option<Vec<ContainerContent>>,
    pub schema: Option<SchemaSummary>,
    ///Barcode of the well
    pub barcode: Option<String>,
    ///DateTime the well was created
    pub created_at: Option<String>,
    ///DateTime the well was last modified
    pub modified_at: Option<String>,
    ///Name of the well, defaults to barcode if name is not provided.
    pub name: Option<String>,
    pub web_url: Option<String>,
    pub parent_storage_schema: Option<SchemaSummary>,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: Option<ContainerQuantity>,
    ///ID of the project if set
    pub project_id: Option<String>,
    pub creator: Option<UserSummary>,
    pub volume: Option<DeprecatedContainerVolumeForResponse>,
    pub fields: Option<Fields>,
    pub checkout_record: Option<CheckoutRecord>,
    pub archive_record: Option<ArchiveRecord>,
    ///ID of the well
    pub id: Option<String>,
}
impl std::fmt::Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersCheckin {
    pub comments: Option<String>,
    ///Array of container IDs.
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersCheckin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceUpdate {
    pub dna_sequence_request_registry_fields: DnaSequenceRequestRegistryFields,
    pub dna_sequence_base_request: DnaSequenceBaseRequest,
}
impl std::fmt::Display for DnaSequenceUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub archive_record: Option<ArchiveRecord>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub owner: Option<serde_json::Value>,
}
impl std::fmt::Display for Project {
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
pub struct EntryTemplateDay {
    pub notes: Option<Vec<serde_json::Value>>,
    ///1 indexed day signifier.
    pub day: Option<i64>,
}
impl std::fmt::Display for EntryTemplateDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoAnnotateRnaSequences {
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
    ///Array of RNA sequence IDs.
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateRnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResult {
    /**ArchiveRecord Resource if the result is archived. This is null if the result is not archived.
*/
    pub archive_record: Option<ArchiveRecord>,
    ///ID of the project to insert the result into
    pub project_id: Option<String>,
    ///DateTime at which the the result was created
    pub created_at: Option<String>,
    ///Whether or not this result is attached to an accepted entry
    pub is_reviewed: Option<bool>,
    ///Schema that the result belongs to
    pub schema: Option<SchemaSummary>,
    pub validation_comment: Option<String>,
    pub validation_status: Option<String>,
    ///UserSummary Resource of who created the request
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    ///ID of the result
    pub id: Option<String>,
    ///ID of the entry that this result is attached to
    pub entry_id: Option<String>,
    /**Object mapping field names to a UserValidation Resource object for that field. To **set** validation for a result, you *must* use this object.
*/
    pub field_validation: Option<serde_json::Value>,
}
impl std::fmt::Display for AssayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorArchivalChange {
    pub automation_output_processor_ids: Option<Vec<String>>,
    pub result_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AutomationOutputProcessorArchivalChange {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureUpdate {}
impl std::fmt::Display for FeatureUpdate {
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
pub struct WorkflowTaskStatus {
    ///The status type
    pub status_type: Option<String>,
    ///The status label
    pub display_name: Option<String>,
    ///The ID of the workflow task status
    pub id: Option<String>,
}
impl std::fmt::Display for WorkflowTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMoleculeRequired {
    pub entity_registry_id: String,
    pub naming_strategy: String,
    pub author_ids: Vec<String>,
    pub registry_id: String,
    pub fields: Fields,
    pub name: String,
    pub custom_fields: CustomFields,
    pub chemical_structure: MoleculeStructure,
    pub schema_id: String,
    pub aliases: Vec<String>,
    pub folder_id: String,
}
impl std::fmt::Display for CreateMoleculeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMixtureRequired {
    pub fields: Fields,
    pub naming_strategy: String,
    pub schema_id: String,
    pub registry_id: String,
    pub name: String,
    pub custom_fields: CustomFields,
    pub amount: String,
    pub author_ids: Vec<String>,
    pub aliases: Vec<String>,
    pub entity_registry_id: String,
    pub folder_id: String,
    pub ingredients: Vec<IngredientWriteParams>,
}
impl std::fmt::Display for CreateMixtureRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBulkCreate {
    pub aa_sequence_create: AaSequenceCreate,
}
impl std::fmt::Display for AaSequenceBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DeprecatedContainerVolumeForResponse {
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub container_quantity: ContainerQuantity,
}
impl std::fmt::Display for DeprecatedContainerVolumeForResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrySchema {
    pub registry_id: String,
    pub schema: Schema,
    pub prefix: String,
}
impl std::fmt::Display for RegistrySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupUpdate {
    pub workflow_task_group_write_base: WorkflowTaskGroupWriteBase,
}
impl std::fmt::Display for WorkflowTaskGroupUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ForbiddenError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for ForbiddenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnregisterEntities {
    ///Array of entity IDs
    pub entity_ids: Vec<String>,
    ///ID of the folder that the entities should be moved to
    pub folder_id: String,
}
impl std::fmt::Display for UnregisterEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsUnarchive {
    ///Array of automation output processor IDs
    pub automation_output_processor_ids: Vec<String>,
}
impl std::fmt::Display for AutomationOutputProcessorsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OligosUnarchive {
    pub oligo_ids: Vec<String>,
}
impl std::fmt::Display for OligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultCreate {
    ///ID of result schema under which to upload this result
    pub schema_id: String,
    ///UUID
    pub id: Option<String>,
    /**Dictionary mapping field names to UserValidation Resources.
*/
    pub field_validation: Option<serde_json::Value>,
    /**The project that the assay result should be uploaded to. Only users with read access to the project will be able to read the assay result. Leaving this empty will result in only the creator having read access.
*/
    pub project_id: Option<String>,
    ///Dictionary of result fields
    pub fields: serde_json::Value,
}
impl std::fmt::Display for AssayResultCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerCreate {
    pub project_id: Option<String>,
    pub schema_id: String,
    pub barcode: String,
    pub container_write_base: ContainerWriteBase,
}
impl std::fmt::Display for ContainerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaAnnotation {
    pub end: i64,
    pub start: i64,
    pub strand: i64,
    pub sequence_feature_base: SequenceFeatureBase,
    pub type_: String,
}
impl std::fmt::Display for RnaAnnotation {
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
pub struct EntityOrInaccessibleResource {}
impl std::fmt::Display for EntityOrInaccessibleResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTaskSchema {
    pub system_name: String,
    ///DateTime the Request Task Schema was last modified
    pub modified_at: String,
    pub schema: Schema,
    pub type_: String,
    /**The organization that owns the schema.
*/
    pub organization: serde_json::Value,
}
impl std::fmt::Display for RequestTaskSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTasksBulkCreate {
    ///The schema id of the task to create
    pub schema_id: String,
}
impl std::fmt::Display for RequestTasksBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesUnarchive {
    pub batch_ids: Vec<String>,
}
impl std::fmt::Display for BatchesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MoleculesUnarchive {
    pub molecule_ids: Vec<String>,
}
impl std::fmt::Display for MoleculesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequestRequired {
    pub scheduled_on: String,
    pub schema_id: String,
    pub fields: Fields,
    pub sample_groups: Vec<RequestSampleGroupCreate>,
    pub project_id: String,
    pub assignees: Vec<serde_json::Value>,
}
impl std::fmt::Display for CreateRequestRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentFile {
    pub name: Option<String>,
    pub data: Option<String>,
}
impl std::fmt::Display for NucleotideAlignmentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputBulkCreate {
    pub workflow_output_create: WorkflowOutputCreate,
}
impl std::fmt::Display for WorkflowOutputBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTaskBaseFields {
    ///IDs of all request sample groups now associated with this task.
    pub sample_group_ids: Option<Vec<String>>,
    /**Schema fields to set on the request task.
Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
}
impl std::fmt::Display for RequestTaskBaseFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceCreate {
    pub rna_sequence_base_request_for_create: RnaSequenceBaseRequestForCreate,
    pub create_entity_into_registry: CreateEntityIntoRegistry,
}
impl std::fmt::Display for RnaSequenceCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskCreatedEvent {
    pub workflow_task: WorkflowTask,
    pub event_type: String,
    pub event_base: EventBase,
}
impl std::fmt::Display for WorkflowTaskCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSchemasPaginatedList {
    pub batch_schemas_list: BatchSchemasList,
    pub next_token: String,
}
impl std::fmt::Display for BatchSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryExternalFile {
    /**A short-lived URL that can be used to download the original file.
*/
    pub download_url: Option<String>,
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
pub struct ContainerLabels {
    pub name: Option<String>,
    pub barcode: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for ContainerLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBulkUpdate {
    pub dna_sequence_base_request: DnaSequenceBaseRequest,
    pub id: String,
}
impl std::fmt::Display for DnaSequenceBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckboxNotePart {
    /**Indicates whether the checkbox is checked or not.
*/
    pub checked: bool,
    pub type_: String,
    ///The textual contents of the note.
    pub text: String,
    /**Array of links referenced in text via an @-mention, hyperlink, or the drag-n-dropped preview attached to the note.
*/
    pub links: Vec<EntryLink>,
    pub base_note_part: BaseNotePart,
}
impl std::fmt::Display for CheckboxNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersArchivalChange {
    pub oligo_ids: Option<Vec<String>>,
    pub entry_ids: Option<Vec<String>>,
    pub custom_entity_ids: Option<Vec<String>>,
    pub aa_sequence_ids: Option<Vec<String>>,
    pub dna_sequence_ids: Option<Vec<String>>,
    pub folder_ids: Option<Vec<String>>,
    pub protocol_ids: Option<Vec<String>>,
    pub mixture_ids: Option<Vec<String>>,
}
impl std::fmt::Display for FoldersArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersUnarchive {
    ///A list of folder IDs to unarchive.
    pub folder_ids: Vec<String>,
}
impl std::fmt::Display for FoldersUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserAssignee {
    pub user: Option<UserSummary>,
}
impl std::fmt::Display for RequestUserAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskStatusLifecycle {
    pub initial_status: Option<WorkflowTaskStatus>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub transitions: Option<Vec<WorkflowTaskStatusLifecycleTransition>>,
    pub statuses: Option<Vec<WorkflowTaskStatus>>,
}
impl std::fmt::Display for WorkflowTaskStatusLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct FloatFieldDefinition {
    pub field_definition: FieldDefinition,
    pub legal_text_dropdown_id: Option<String>,
    pub numeric_max: Option<f64>,
    pub decimal_precision: Option<f64>,
    pub numeric_min: Option<f64>,
    pub type_: String,
}
impl std::fmt::Display for FloatFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputSchema {
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub type_: Option<String>,
    pub field_definitions: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for WorkflowOutputSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsPaginatedList {
    pub next_token: Option<String>,
    pub projects: Option<Vec<Project>>,
}
impl std::fmt::Display for ProjectsPaginatedList {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsPaginatedList {
    pub next_token: Option<String>,
    pub workflow_outputs: Option<Vec<WorkflowOutput>>,
}
impl std::fmt::Display for WorkflowOutputsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkCreateRequest {
    pub rna_sequences: Option<Vec<RnaSequenceBulkCreate>>,
}
impl std::fmt::Display for RnaSequencesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesUnarchive {
    pub aa_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBaseRequestForCreate {
    pub dna_sequence_base_request: DnaSequenceBaseRequest,
}
impl std::fmt::Display for DnaSequenceBaseRequestForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingAppsPaginatedList {
    pub pagination: Pagination,
    pub apps: Vec<BenchlingApp>,
}
impl std::fmt::Display for BenchlingAppsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessorCompletedV2BetaEvent {
    pub event_base: EventBase,
    pub automation_output_processor: AutomationFile,
    pub event_type: String,
}
impl std::fmt::Display for AutomationOutputProcessorCompletedV2BetaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequestErrorBulk {
    pub error: serde_json::Value,
    pub bad_request_error: BadRequestError,
}
impl std::fmt::Display for BadRequestErrorBulk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestFulfillment {
    ///The request sample group this fulfillment was executed upon, if any.
    pub sample_group: Option<SampleGroup>,
    ///DateTime the Request Fulfillment was last modified
    pub modified_at: Option<String>,
    ///Date and time the fulfillment was created
    pub created_at: Option<String>,
    ///ID of the request fulfillment
    pub id: Option<String>,
    ///Status of a sample group
    pub status: Option<String>,
    ///ID of the entry this fulfillment was executed in, if any
    pub entry_id: Option<String>,
    ///ID of the request this fulfillment fulfills
    pub request_id: Option<String>,
}
impl std::fmt::Display for RequestFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceSummary {
    pub id: Option<String>,
    pub type_: Option<String>,
    pub entity_type: Option<String>,
}
impl std::fmt::Display for AaSequenceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBulkUpdate {
    pub custom_entity_base_request: CustomEntityBaseRequest,
}
impl std::fmt::Display for CustomEntityBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BooleanAppConfigItem {}
impl std::fmt::Display for BooleanAppConfigItem {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationInputGenerator {
    ///DateTime the Automation Input Generator was last modified
    pub modified_at: String,
    ///The canonical url of the Automation Input Generator in the API.
    pub api_url: String,
    pub id: String,
    pub automation_file: AutomationFile,
    ///DateTime the Automation Input Generator was last modified
    pub created_at: String,
}
impl std::fmt::Display for AutomationInputGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCreationTableNotePart {
    pub type_: String,
    pub box_schema_id: String,
    pub base_note_part: BaseNotePart,
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
}
impl std::fmt::Display for BoxCreationTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOligoRequired {
    pub aliases: Vec<String>,
    pub schema_id: String,
    pub registry_id: String,
    pub naming_strategy: String,
    pub author_ids: Vec<String>,
    pub bases: String,
    pub custom_fields: CustomFields,
    pub folder_id: String,
    pub name: String,
    pub fields: Fields,
    pub entity_registry_id: String,
}
impl std::fmt::Display for CreateOligoRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceCreate {
    pub aa_sequence_base_request_for_create: AaSequenceBaseRequestForCreate,
    pub create_entity_into_registry: CreateEntityIntoRegistry,
}
impl std::fmt::Display for AaSequenceCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultIdsResponse {
    pub assay_result_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AssayResultIdsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersArchive {
    /**The reason for archiving the provided folders. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    ///A list of folder IDs to archive.
    pub folder_ids: Vec<String>,
}
impl std::fmt::Display for FoldersArchive {
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
pub struct RnaSequencesArchivalChange {
    pub rna_sequence_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RnaSequencesArchivalChange {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateDnaSequencesAsyncTask {}
impl std::fmt::Display for BulkUpdateDnaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedFieldsEvent {
    pub event_type: String,
    pub workflow_task: WorkflowTask,
    pub event_base: EventBase,
}
impl std::fmt::Display for WorkflowTaskUpdatedFieldsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Oligo {
    pub fields: Option<Fields>,
    ///ID of the folder that contains the Oligo.
    pub folder_id: Option<String>,
    ///DateTime the Oligo was created.
    pub created_at: Option<String>,
    ///Base pairs of the Oligo.
    pub bases: Option<String>,
    ///ID of the Oligo.
    pub id: Option<String>,
    ///Array of aliases
    pub aliases: Option<Vec<String>>,
    ///Registry ID of the Oligo if registered.
    pub entity_registry_id: Option<String>,
    ///Nucleotide type of the Oligo.
    pub nucleotide_type: Option<String>,
    ///URL of the Oligo.
    pub web_url: Option<String>,
    pub schema: Option<SchemaSummary>,
    ///Number of bases in the Oligo.
    pub length: Option<i64>,
    pub creator: Option<UserSummary>,
    pub archive_record: Option<ArchiveRecord>,
    ///Name of the Oligo.
    pub name: Option<String>,
    pub registration_origin: Option<RegistrationOrigin>,
    ///The canonical url of the Oligo in the API.
    pub api_url: Option<String>,
    ///Registry the Oligo is registered in.
    pub registry_id: Option<String>,
    ///Custom fields set on the Oligo.
    pub custom_fields: Option<CustomFields>,
    ///DateTime the Oligo was last modified.
    pub modified_at: Option<String>,
}
impl std::fmt::Display for Oligo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AlignedSequence {
    pub dna_sequence_id: Option<String>,
    pub trim_end: Option<i64>,
    pub trim_start: Option<i64>,
    /**Fraction of bases between trimStart and trimEnd that match the template bases. Only present for Template Alignments; Will be empty for Consensus Alignments.
*/
    pub pairwise_identity: Option<f64>,
    pub bases: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for AlignedSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainersPaginatedList {
    pub containers_list: ContainersList,
    pub pagination: Pagination,
}
impl std::fmt::Display for ContainersPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownCreate {
    ///ID of registry in which to create the dropdown. Required if multiple registries exist.
    pub registry_id: Option<String>,
    ///Options to set for the dropdown
    pub options: Vec<DropdownOptionCreate>,
    ///Name of the dropdown
    pub name: String,
}
impl std::fmt::Display for DropdownCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsBulkUpdateRequest {
    pub workflow_outputs: Option<Vec<WorkflowOutputBulkUpdate>>,
}
impl std::fmt::Display for WorkflowOutputsBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAaSequenceRequired {
    pub entity_registry_id: String,
    pub author_ids: Vec<String>,
    pub registry_id: String,
    pub annotations: Vec<AaAnnotation>,
    pub naming_strategy: String,
    pub folder_id: String,
    pub name: String,
    pub amino_acids: String,
    pub schema_id: String,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub aliases: Vec<String>,
}
impl std::fmt::Display for CreateAaSequenceRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
    pub start: i64,
    pub strand: i64,
    pub amino_acids: String,
    ///The genetic code to use when translating the nucleotide sequence into amino acids.
    pub genetic_code: String,
    pub regions: Vec<serde_json::Value>,
    pub end: i64,
    pub sequence_feature_base: SequenceFeatureBase,
}
impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunSchema {
    pub automation_output_file_configs: Vec<serde_json::Value>,
    ///DateTime the Assay Run Schema was last modified
    pub modified_at: String,
    pub type_: String,
    pub base_assay_schema: BaseAssaySchema,
    pub automation_input_file_configs: Vec<serde_json::Value>,
}
impl std::fmt::Display for AssayRunSchema {
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
pub struct CustomEntitySummary {
    pub id: Option<String>,
    pub entity_type: Option<String>,
    pub type_: Option<String>,
}
impl std::fmt::Display for CustomEntitySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DatetimeAppConfigItem {
    pub type_: String,
    pub base_app_config_item: BaseAppConfigItem,
    pub value: Option<String>,
}
impl std::fmt::Display for DatetimeAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    ///DateTime the Registry was last modified
    pub modified_at: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub owner: Option<Organization>,
}
impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalFileNotePart {
    /**Array of links referenced in the caption via an @-mention, hyperlink, or the drag-n-dropped preview attached to the note.
*/
    pub links: Vec<EntryLink>,
    ///The caption of the file attachment.
    pub text: String,
    pub base_note_part: BaseNotePart,
    /**The ID of the external file. Use the 'Get an external file' endpoint to retrieve metadata about it.
*/
    pub external_file_id: String,
    pub type_: String,
}
impl std::fmt::Display for ExternalFileNotePart {
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
pub struct BulkCreateFeaturesAsyncTask {}
impl std::fmt::Display for BulkCreateFeaturesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAaSequenceRequired {
    pub annotations: Vec<AaAnnotation>,
    pub aliases: Vec<String>,
    pub aa_sequence_id: String,
    pub author_ids: Vec<String>,
    pub folder_id: String,
    pub fields: Fields,
    pub name: String,
    pub schema_id: String,
    pub entity_registry_id: String,
    pub amino_acids: String,
    pub custom_fields: CustomFields,
}
impl std::fmt::Display for UpdateAaSequenceRequired {
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
pub struct BoxContentsPaginatedList {
    pub next_token: Option<String>,
    pub containers: Option<Vec<ContainerWithCoordinates>>,
}
impl std::fmt::Display for BoxContentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsArchivalChange {
    pub oligo_ids: Option<Vec<String>>,
    pub dna_sequence_ids: Option<Vec<String>>,
    pub aa_sequence_ids: Option<Vec<String>>,
    pub mixture_ids: Option<Vec<String>>,
    pub project_ids: Option<Vec<String>>,
    pub folder_ids: Option<Vec<String>>,
    pub custom_entity_ids: Option<Vec<String>>,
    pub protocol_ids: Option<Vec<String>>,
    pub entry_ids: Option<Vec<String>>,
}
impl std::fmt::Display for ProjectsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosArchivalChange {
    pub dna_oligo_ids: Option<Vec<String>>,
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DnaOligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSummary {
    pub party_summary: PartySummary,
}
impl std::fmt::Display for UserSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupWriteBase(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsUnarchive {
    ///Array of dropdown option IDs
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DropdownOptionsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDnaSequenceRequired {
    pub primers: Vec<Primer>,
    pub schema_id: String,
    pub translations: Vec<Translation>,
    pub fields: Fields,
    pub dna_sequence_id: String,
    pub custom_fields: CustomFields,
    pub is_circular: bool,
    pub entity_registry_id: String,
    pub annotations: Vec<DnaAnnotation>,
    pub author_ids: Vec<String>,
    pub aliases: Vec<String>,
    pub bases: String,
    pub name: String,
    pub folder_id: String,
}
impl std::fmt::Display for UpdateDnaSequenceRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBulkUpdate {
    pub aa_sequence_base_request: AaSequenceBaseRequest,
    pub id: String,
}
impl std::fmt::Display for AaSequenceBulkUpdate {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntity {
    pub registration_origin: Option<RegistrationOrigin>,
    pub archive_record: Option<ArchiveRecord>,
    pub registry_id: Option<String>,
    pub folder_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub authors: Option<Vec<UserSummary>>,
    pub aliases: Option<Vec<String>>,
    pub id: Option<String>,
    pub created_at: Option<String>,
    pub creator: Option<serde_json::Value>,
    ///The canonical url of the Custom Entity in the API.
    pub api_url: Option<String>,
    pub custom_fields: Option<CustomFields>,
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
    pub web_url: Option<String>,
    pub modified_at: Option<String>,
}
impl std::fmt::Display for CustomEntity {
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
pub struct BaseAssaySchema {
    pub derived_from: Option<String>,
    pub system_name: String,
    pub schema: Schema,
    pub organization: serde_json::Value,
}
impl std::fmt::Display for BaseAssaySchema {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeCreate {
    pub create_entity_into_registry: CreateEntityIntoRegistry,
    pub molecule_base_request_for_create: MoleculeBaseRequestForCreate,
}
impl std::fmt::Display for MoleculeCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceRequestRegistryFields {
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for DnaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryUpdatedFieldsEvent {
    pub event_base: EventBase,
    pub event_type: String,
    /**Entries are notes that users can take. They're organized by "days" (which are user-configurable) and modeled within each day as a list of "notes." Each note has a type - the simplest is a "text" type, but lists, tables, and external files are also supported.

*Note:* the current Entry resource has a few limitations:
- Formatting information is not yet supported. Header formatting, bolding, and other stylistic information is not presented.
- Data in tables is presented as text always - numeric values will need to be parsed into floats or integers, as appropriate.

Note: Data in Results tables are not accessible through this API call. Results table data can be called through the Results API calls.
*/
    pub entry: Entry,
    pub update_event_mixin: UpdateEventMixin,
}
impl std::fmt::Display for EntryUpdatedFieldsEvent {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSchemasPaginatedList {
    pub next_token: Option<String>,
    pub request_schemas: Option<Vec<RequestSchema>>,
}
impl std::fmt::Display for RequestSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeUpdate {
    pub molecule_base_request: MoleculeBaseRequest,
    pub entity_registry_id: String,
}
impl std::fmt::Display for MoleculeUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItem {}
impl std::fmt::Display for AppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultSchemasPaginatedList {
    pub next_token: Option<String>,
    pub assay_result_schemas: Option<Vec<AssayResultSchema>>,
}
impl std::fmt::Display for AssayResultSchemasPaginatedList {
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
pub struct EntryTable {
    /**Name of the table - defaults to e.g. Table1 but can be renamed.
*/
    pub name: Option<String>,
    /**Array of strings, with one item per column. Defaults to null, if the user is using the default, but is set if the user has given a custom name to the column.
*/
    pub column_labels: Option<Vec<String>>,
    ///Array of row objects.
    pub rows: Option<Vec<EntryTableRow>>,
}
impl std::fmt::Display for EntryTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsPaginatedList {
    pub locations: Option<Vec<Location>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for LocationsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateTemplateNucleotideAlignmentRequired {
    pub name: String,
    pub template_sequence_id: String,
    pub algorithm: String,
    pub files: Vec<serde_json::Value>,
}
impl std::fmt::Display for CreateTemplateNucleotideAlignmentRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemUpdate {}
impl std::fmt::Display for AppConfigItemUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotFoundError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupCreate {
    ///The workflow task schema of tasks in this task group
    pub schema_id: String,
    pub workflow_task_group_write_base: WorkflowTaskGroupWriteBase,
}
impl std::fmt::Display for WorkflowTaskGroupCreate {
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
pub struct CustomEntityBaseRequestForCreate {
    pub custom_entity_base_request: CustomEntityBaseRequest,
}
impl std::fmt::Display for CustomEntityBaseRequestForCreate {
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
pub struct AssayFieldsCreate {}
impl std::fmt::Display for AssayFieldsCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligo {
    pub api_url: String,
    pub nucleotide_type: String,
    pub bases: String,
    pub oligo: Oligo,
}
impl std::fmt::Display for RnaOligo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TableNotePart {
    ///The caption of the table.
    pub text: String,
    pub type_: String,
    pub base_note_part: BaseNotePart,
    /**Array of links referenced in the caption via an @-mention, hyperlink, or the drag-n-dropped preview attached to the note.
*/
    pub links: Vec<EntryLink>,
    pub table: serde_json::Value,
}
impl std::fmt::Display for TableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputCreatedEvent {
    pub event_type: String,
    pub workflow_output: WorkflowOutput,
    pub event_base: EventBase,
}
impl std::fmt::Display for WorkflowOutputCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunCreatedEvent {
    pub event_base: EventBase,
    pub assay_run: AssayRun,
    pub event_type: String,
}
impl std::fmt::Display for AssayRunCreatedEvent {
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
pub struct Fields {}
impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskBulkCreate {
    pub workflow_task_create: WorkflowTaskCreate,
}
impl std::fmt::Display for WorkflowTaskBulkCreate {
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
pub struct TeamSummary {
    pub party_summary: PartySummary,
}
impl std::fmt::Display for TeamSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxUpdate {
    pub name: Option<String>,
    pub parent_storage_id: Option<String>,
    pub fields: Option<Fields>,
    pub project_id: Option<String>,
}
impl std::fmt::Display for BoxUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventBase {
    /**These properties have been dropped from the payload due to size.
*/
    pub excluded_properties: Option<Vec<String>>,
    pub schema: Option<serde_json::Value>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub deprecated: Option<bool>,
}
impl std::fmt::Display for EventBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobMultipartCreate {
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: String,
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    ///Name of the blob
    pub name: String,
}
impl std::fmt::Display for BlobMultipartCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixtureCreate {
    pub create_entity_into_registry: CreateEntityIntoRegistry,
    pub mixture_update: MixtureUpdate,
}
impl std::fmt::Display for MixtureCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlatesArchivalChange {
    pub container_ids: Option<Vec<String>>,
    pub plate_ids: Option<Vec<String>>,
}
impl std::fmt::Display for PlatesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OligosPaginatedList {
    pub pagination: Pagination,
    pub oligos: Vec<Oligo>,
}
impl std::fmt::Display for OligosPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskBulkUpdate {}
impl std::fmt::Display for WorkflowTaskBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputBulkUpdate {
    ///The ID of the workflow output
    pub workflow_output_id: String,
    pub workflow_output_write_base: WorkflowOutputWriteBase,
}
impl std::fmt::Display for WorkflowOutputBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StructuredTableApiIdentifiers {
    pub columns: Option<Vec<StructuredTableColumnInfo>>,
    pub api_id: Option<String>,
}
impl std::fmt::Display for StructuredTableApiIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowTaskArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibrariesPaginatedList {}
impl std::fmt::Display for FeatureLibrariesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryUpdate {
    ///New name of the entry
    pub name: Option<String>,
    ///ID of the schema for the entry
    pub schema_id: Option<String>,
    ///Schema fields to set on the entry
    pub fields: Option<Fields>,
    ///ID of the folder that will contain the entry
    pub folder_id: Option<String>,
    ///IDs of users to set as the entry's authors.
    pub author_ids: Option<String>,
}
impl std::fmt::Display for EntryUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabelTemplate {
    ///Name of the label template.
    pub name: Option<String>,
    ///ID of the label template.
    pub id: Option<String>,
    ///The ZPL template that will be filled in and sent to a printer.
    pub zpl_template: Option<String>,
}
impl std::fmt::Display for LabelTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesArchive {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    ///DateTime the entry was created at
    pub created_at: Option<String>,
    pub fields: Option<Fields>,
    ///Title of the entry
    pub name: Option<String>,
    ///User-friendly ID of the entry
    pub display_id: Option<String>,
    ///ID of the Entry Template this Entry was created from
    pub entry_template_id: Option<String>,
    ///ID of the folder that contains the entry
    pub folder_id: Option<String>,
    pub custom_fields: Option<CustomFields>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    ///Review record if set
    pub review_record: Option<serde_json::Value>,
    /**Array of day objects. Each day object has a date field (string) and notes field (array of notes, expand further for details on note types).
*/
    pub days: Option<Vec<EntryDay>>,
    /**Array of users assigned to review the entry, if any.
*/
    pub assigned_reviewers: Option<Vec<UserSummary>>,
    ///ID of the entry
    pub id: Option<String>,
    /**ArchiveRecord Resource if the entry is archived. This is null if the entry is not archived.
*/
    pub archive_record: Option<ArchiveRecord>,
    ///UserSummary Resource of the user who created the entry
    pub creator: Option<UserSummary>,
    ///DateTime the entry was last modified
    pub modified_at: Option<String>,
    ///URL of the entry
    pub web_url: Option<String>,
    /**Array of UserSummary Resources of the authors of the entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    ///The canonical url of the Entry in the API.
    pub api_url: Option<String>,
}
impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignmentSummary {
    ///The ID of the template or consensus DNA Sequence associated with the DNA Alignment
    pub reference_sequence_id: Option<String>,
    ///DateTime the DNA Alignment was created
    pub created_at: Option<String>,
    ///The canonical url of the DNA Alignment in the API.
    pub api_url: Option<String>,
    ///DateTime the DNA Alignment was last modified
    pub modified_at: Option<String>,
    ///The Benchling web UI url to view the DNA Alignment
    pub web_url: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for DnaAlignmentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputCreate {
    pub workflow_output_write_base: WorkflowOutputWriteBase,
    ///The ID of the workflow task this output belogns to
    pub workflow_task_id: String,
}
impl std::fmt::Display for WorkflowOutputCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoCreate {
    pub oligo_create: OligoCreate,
}
impl std::fmt::Display for RnaOligoCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LookupTableNotePart {
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
    pub type_: String,
}
impl std::fmt::Display for LookupTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsArchive {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkUpdateAaSequencesAsyncTask {}
impl std::fmt::Display for BulkUpdateAaSequencesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeBaseRequest {
    ///Aliases to add to the Molecule.
    pub aliases: Option<Vec<String>>,
    /**Name of the Molecule.
*/
    pub name: Option<String>,
    /**Fields to set on the Molecule. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    /**ID of the Molecule's schema.
*/
    pub schema_id: Option<String>,
    ///IDs of users to set as the Molecule's authors.
    pub author_ids: Option<Vec<String>>,
    /**Chemical structure of the Molecule.
*/
    pub chemical_structure: Option<MoleculeStructure>,
    /**Custom fields to add to the Molecule. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**ID of the folder containing the Molecule.
*/
    pub folder_id: Option<String>,
}
impl std::fmt::Display for MoleculeBaseRequest {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesUnarchive {
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for RnaSequencesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SampleGroupsStatusUpdate {
    pub sample_groups: Vec<SampleGroupStatusUpdate>,
}
impl std::fmt::Display for SampleGroupsStatusUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IntegerAppConfigItem {}
impl std::fmt::Display for IntegerAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainerSchemasList {
    pub container_schemas: Option<Vec<ContainerSchema>>,
}
impl std::fmt::Display for ContainerSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsUnarchive {
    pub workflow_task_group_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTaskGroupsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkGet {
    pub dna_sequences: Option<Vec<DnaSequence>>,
}
impl std::fmt::Display for DnaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestWriteTeamAssignee {
    pub team_id: String,
}
impl std::fmt::Display for RequestWriteTeamAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemCreate {}
impl std::fmt::Display for AppConfigItemCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceWithEntityType {}
impl std::fmt::Display for AaSequenceWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GenericApiIdentifiedAppConfigItem {}
impl std::fmt::Display for GenericApiIdentifiedAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponseSamplesItemBatch {
    pub batch_or_inaccessible_resource: BatchOrInaccessibleResource,
}
impl std::fmt::Display for RequestResponseSamplesItemBatch {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Box {
    pub modified_at: Option<String>,
    ///The size of the box (i.e. how many containers it can store).
    pub size: Option<i64>,
    ///The number of empty positions for adding additional containers in the box.
    pub empty_positions: Option<i64>,
    ///The number of containers in the box that have no contents.
    pub empty_containers: Option<i64>,
    pub parent_storage_id: Option<String>,
    ///The number of containers currently in the box.
    pub filled_positions: Option<i64>,
    pub barcode: Option<String>,
    pub project_id: Option<String>,
    pub web_url: Option<String>,
    pub created_at: Option<String>,
    pub creator: Option<UserSummary>,
    pub schema: Option<SchemaSummary>,
    pub fields: Option<Fields>,
    pub archive_record: Option<ArchiveRecord>,
    pub name: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSchema {
    ///DateTime the Batch Schema was last modified
    pub modified_at: String,
    pub entity_schema_id: String,
    pub registry_schema: RegistrySchema,
}
impl std::fmt::Display for BatchSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesUnarchive {
    pub mixture_ids: Vec<String>,
}
impl std::fmt::Display for MixturesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityUpdate {
    pub custom_entity_base_request: CustomEntityBaseRequest,
    pub custom_entity_request_registry_fields: CustomEntityRequestRegistryFields,
}
impl std::fmt::Display for CustomEntityUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoAnnotateDnaSequences {
    ///Array of DNA sequence IDs.
    pub dna_sequence_ids: Vec<String>,
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateDnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WarehouseCredentials {
    ///The username to connect to the warehouse.
    pub username: Option<String>,
    /**The time after which new connections using the username/password will not be permitted. Upon expiration, currently open connections are not terminated.
*/
    pub expires_at: Option<String>,
    ///The password to connect to the warehouse.
    pub password: Option<String>,
}
impl std::fmt::Display for WarehouseCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRecord {
    pub assignee: Option<serde_json::Value>,
    pub modified_at: Option<String>,
    pub comment: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for CheckoutRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemJsonUpdate {
    pub type_: String,
    pub value: String,
}
impl std::fmt::Display for AppConfigItemJsonUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponseSamplesItemEntity {
    pub entity_or_inaccessible_resource: EntityOrInaccessibleResource,
}
impl std::fmt::Display for RequestResponseSamplesItemEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillSequences {
    ///Array of DNA sequence IDs.
    pub dna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutofillSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemIntegerCreate {}
impl std::fmt::Display for AppConfigItemIntegerCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxSchemasList {
    pub box_schemas: Option<Vec<BoxSchema>>,
}
impl std::fmt::Display for BoxSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericEntity {
    pub modified_at: Option<String>,
    pub custom_fields: Option<CustomFields>,
    pub name: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub api_url: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub fields: Option<Fields>,
    pub registration_origin: Option<RegistrationOrigin>,
    pub registry_id: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub folder_id: Option<String>,
    pub creator: Option<serde_json::Value>,
    pub entity_registry_id: Option<String>,
    pub created_at: Option<String>,
    pub web_url: Option<String>,
    /**Array of UserSummary Resources of the authors of the entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    pub id: Option<String>,
}
impl std::fmt::Display for GenericEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsArchive {
    ///Array of location IDs
    pub location_ids: Vec<String>,
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: Option<bool>,
    /**Reason that locations are being archived.
*/
    pub reason: String,
}
impl std::fmt::Display for LocationsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesBulkCreateRequest {
    pub dna_sequences: Option<Vec<DnaSequenceBulkCreate>>,
}
impl std::fmt::Display for DnaSequencesBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestWriteBase {
    ///Array of assignees
    pub assignees: Vec<serde_json::Value>,
    ///The ID of the project to which the request belongs.
    pub project_id: String,
    /**ID of the user making the request. If unspecified, the requestor is the request creator.
*/
    pub requestor_id: Option<String>,
    pub sample_groups: Vec<RequestSampleGroupCreate>,
    ///A request is an ask to perform a service, e.g. produce a sample or perform assays on a sample. Requests are usually placed to another team or individual who specializes in performing the service.
    pub request_base: RequestBase,
    ///The request's fields
    pub fields: Fields,
    ///Date the request is scheduled to be executed on, in YYYY-MM-DD format.
    pub scheduled_on: String,
}
impl std::fmt::Display for RequestWriteBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkUpdateRequest {
    pub workflow_tasks: Option<Vec<WorkflowTaskBulkUpdate>>,
}
impl std::fmt::Display for WorkflowTasksBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesPaginatedList {
    pub custom_entities: Option<Vec<CustomEntity>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for CustomEntitiesPaginatedList {
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
pub struct AlignedNucleotideSequence {
    pub name: Option<String>,
    /**Fraction of bases between trimStart and trimEnd that match the template bases. Only present for Template Alignments; Will be empty for Consensus Alignments.
*/
    pub pairwise_identity: Option<f64>,
    pub bases: Option<String>,
    pub sequence_id: Option<String>,
    pub trim_start: Option<i64>,
    pub trim_end: Option<i64>,
}
impl std::fmt::Display for AlignedNucleotideSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsCreateResponse {
    pub assay_results: Option<Vec<String>>,
}
impl std::fmt::Display for AssayResultsCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersArchivalChange {
    pub container_ids: Option<Vec<String>>,
}
impl std::fmt::Display for ContainersArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeprecatedAutomationOutputProcessorsPaginatedList {
    pub next_token: Option<String>,
    pub automation_output_processors: Option<Vec<AutomationOutputProcessor>>,
}
impl std::fmt::Display for DeprecatedAutomationOutputProcessorsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRun {
    pub schema: Option<SchemaSummary>,
    pub archive_record: Option<ArchiveRecord>,
    pub validation_comment: Option<String>,
    pub project_id: Option<String>,
    pub creator: Option<UserSummary>,
    pub fields: Option<Fields>,
    pub entry_id: Option<String>,
    pub id: Option<String>,
    ///Must be either VALID or INVALID
    pub validation_status: Option<String>,
    ///The canonical url of the Run in the API.
    pub api_url: Option<String>,
    pub created_at: Option<String>,
    pub is_reviewed: Option<bool>,
}
impl std::fmt::Display for AssayRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaTemplateAlignmentFile {
    pub name: Option<String>,
    pub data: Option<String>,
}
impl std::fmt::Display for DnaTemplateAlignmentFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub fields: Fields,
    pub tasks: Vec<RequestTask>,
    pub creator: serde_json::Value,
    ///User-friendly ID of the request
    pub display_id: String,
    pub request_status: String,
    pub requestor: serde_json::Value,
    ///Array of assignees
    pub assignees: Vec<serde_json::Value>,
    ///URL of the request
    pub web_url: String,
    ///Unique ID for the request
    pub id: String,
    ///Date and time the request was created
    pub created_at: String,
    ///The canonical url of the Request in the API.
    pub api_url: String,
    ///A request is an ask to perform a service, e.g. produce a sample or perform assays on a sample. Requests are usually placed to another team or individual who specializes in performing the service.
    pub request_base: RequestBase,
    pub schema: serde_json::Value,
    ///Date the request is scheduled to be executed on, in YYYY-MM-DD format.
    pub scheduled_on: Option<String>,
    pub sample_groups: Vec<RequestSampleGroup>,
    ///The ID of the project to which the request belongs.
    pub project_id: String,
}
impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBulkUpdate {
    pub id: String,
    pub rna_sequence_base_request: RnaSequenceBaseRequest,
}
impl std::fmt::Display for RnaSequenceBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingApp {
    pub benchling_app_update: BenchlingAppUpdate,
    ///DateTime at which the the app was last modified
    pub modified_at: String,
    pub api_url: String,
    pub archive_record: Option<ArchiveRecord>,
    pub creator: UserSummary,
    ///DateTime at which the the app was created
    pub created_at: String,
    pub id: String,
    pub web_url: String,
}
impl std::fmt::Display for BenchlingApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowSample {
    ///Array of IDs of containers
    pub container_ids: Option<Vec<String>>,
    ///ID of the sample
    pub id: Option<String>,
    ///ID of the batch
    pub batch_id: Option<String>,
    ///DateTime at which the the sample was created
    pub created_at: Option<String>,
    ///Name of the sample
    pub name: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowSample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FloatAppConfigItem {}
impl std::fmt::Display for FloatAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageRunList {
    pub workflow_stage_runs: Option<Vec<WorkflowStageRun>>,
}
impl std::fmt::Display for WorkflowStageRunList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedAppConfigResourceMixin {
    pub linked_resource: Option<LinkedAppConfigResource>,
}
impl std::fmt::Display for LinkedAppConfigResourceMixin {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationInputGeneratorCompletedV2BetaEvent {
    pub automation_input_generator: AutomationFile,
    pub event_type: String,
    pub event_base: EventBase,
}
impl std::fmt::Display for AutomationInputGeneratorCompletedV2BetaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesArchivalChange {
    pub entry_ids: Option<Vec<String>>,
}
impl std::fmt::Display for EntriesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntryLink {
    /**For linked Benchling resources, this will be the ID of that resource (e.g., 'seq_RhYGVnHF'). Omitted for "link" types.
*/
    pub id: Option<String>,
    /**The type of resource being linked. For hyperlinks: 'link'. For linked Benchling resources, one of: 'user', 'request', 'entry', 'stage_entry', 'protocol', 'workflow', 'custom_entity', 'aa_sequence', 'dna_sequence', 'batch', 'box', 'container', 'location', 'plate'.
*/
    pub type_: Option<String>,
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
pub struct AutomationInputGeneratorUpdate {
    ///The ID of the file (blob) associated with the input generator. Set to `null` to remove an existing file from the generator.
    pub file_id: Option<String>,
}
impl std::fmt::Display for AutomationInputGeneratorUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorUpdate {
    ///The ID of a blob link to process.
    pub file_id: String,
}
impl std::fmt::Display for AutomationOutputProcessorUpdate {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestBase {}
impl std::fmt::Display for RequestBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksPaginatedList {
    pub next_token: Option<String>,
    pub workflow_tasks: Option<Vec<WorkflowTask>>,
}
impl std::fmt::Display for WorkflowTasksPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequenceSummary {
    pub entity_type: Option<String>,
    pub type_: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for DnaSequenceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsArchivalChange {
    pub location_ids: Option<Vec<String>>,
    pub plate_ids: Option<Vec<String>>,
    pub box_ids: Option<Vec<String>>,
    pub container_ids: Option<Vec<String>>,
}
impl std::fmt::Display for LocationsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStage {
    ///Name of the legacy workflow stage
    pub name: Option<String>,
    ///DateTime at which the the legacy workflow stage was created
    pub created_at: Option<String>,
    ///ID of the legacy workflow stage
    pub id: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunCreate {
    ///ID of assay schema that assay run conforms to
    pub schema_id: String,
    /**The project that the assay run should be uploaded to. Only users with read access to the project will be able to read the assay run. Leaving this empty will result in only the creator having read access.
*/
    pub project_id: Option<String>,
    ///Must be either VALID or INVALID
    pub validation_status: Option<String>,
    ///ID of assay run
    pub id: Option<String>,
    ///Additional information about the validation status
    pub validation_comment: Option<String>,
    ///Object of assay run fields
    pub fields: serde_json::Value,
}
impl std::fmt::Display for AssayRunCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateCreationTableNotePart {
    pub plate_schema_id: String,
    pub base_note_part: BaseNotePart,
    pub type_: String,
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
}
impl std::fmt::Display for PlateCreationTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum SchemaDependencySubtypes {
    #[serde(rename = "aa_sequence")]
    AaSequence,
    #[serde(rename = "dna_sequence")]
    DnaSequence,
    #[serde(rename = "custom_entity")]
    CustomEntity,
    #[serde(rename = "mixture")]
    Mixture,
    #[serde(rename = "molecule")]
    Molecule,
    #[serde(rename = "dna_oligo")]
    DnaOligo,
    #[serde(rename = "rna_oligo")]
    RnaOligo,
    #[serde(rename = "rna_sequence")]
    RnaSequence,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskCreate {
    pub workflow_task_write_base: WorkflowTaskWriteBase,
    ///The workflow ID
    pub workflow_task_group_id: String,
}
impl std::fmt::Display for WorkflowTaskCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsArchive {
    pub workflow_task_group_ids: Vec<String>,
    /**The reason for archiving the provided workflow task groups. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for WorkflowTaskGroupsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemFloatCreate {}
impl std::fmt::Display for AppConfigItemFloatCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTask {
    pub schema: Option<SchemaSummary>,
}
impl std::fmt::Display for RequestTask {
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
pub struct AppConfigItemsBulkCreateRequest {
    pub app_configuration_items: Vec<AppConfigItemCreate>,
}
impl std::fmt::Display for AppConfigItemsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NucleotideTemplateAlignmentCreate {
    pub nucleotide_alignment_base: NucleotideAlignmentBase,
    pub template_sequence_id: String,
}
impl std::fmt::Display for NucleotideTemplateAlignmentCreate {
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
pub struct LabAutomationTransformUpdate {
    pub blob_id: Option<String>,
    pub errors: Option<Vec<LabAutomationBenchlingAppError>>,
}
impl std::fmt::Display for LabAutomationTransformUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreate {
    ///ID of the request's schema.
    pub schema_id: String,
    pub request_write_base: RequestWriteBase,
}
impl std::fmt::Display for RequestCreate {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeStructure {
    pub structure_format: Option<serde_json::Value>,
    ///Chemical structure in SMILES or molfile format.
    pub value: Option<String>,
}
impl std::fmt::Display for MoleculeStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsBulkGet {
    pub assay_runs: Option<Vec<AssayRun>>,
}
impl std::fmt::Display for AssayRunsBulkGet {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateBlobPartRequired {
    pub md5: String,
    pub blob_id: String,
    pub part_number: i64,
    pub data64: String,
}
impl std::fmt::Display for CreateBlobPartRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PlateSchemasList {
    pub plate_schemas: Option<Vec<PlateSchema>>,
}
impl std::fmt::Display for PlateSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdatedFieldsEvent {
    pub event_base: EventBase,
    pub request: Request,
    pub event_type: String,
    pub update_event_mixin: UpdateEventMixin,
}
impl std::fmt::Display for RequestUpdatedFieldsEvent {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputUpdate {
    pub workflow_output_write_base: WorkflowOutputWriteBase,
}
impl std::fmt::Display for WorkflowOutputUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct TransfersAsyncTask {}
impl std::fmt::Display for TransfersAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSchemaSummary(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligoWithEntityType {}
impl std::fmt::Display for DnaOligoWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationFile {
    pub automation_file_config: Option<serde_json::Value>,
    pub file: Option<Blob>,
    pub id: Option<String>,
    pub assay_run_id: Option<String>,
    pub status: Option<String>,
}
impl std::fmt::Display for AutomationFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixturesBulkCreateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntitiesBulkUpdateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SecureTextAppConfigItem {}
impl std::fmt::Display for SecureTextAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateBlobRequired {
    pub data64: String,
    pub name: String,
    pub md5: String,
    pub type_: String,
}
impl std::fmt::Display for CreateBlobRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSchemasPaginatedList {
    pub next_token: String,
    pub location_schemas_list: LocationSchemasList,
}
impl std::fmt::Display for LocationSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateWorkflowTaskGroupRequired {
    pub folder_id: String,
    pub workflow_task_group_id: String,
    pub watcher_ids: Vec<String>,
    pub name: String,
}
impl std::fmt::Display for UpdateWorkflowTaskGroupRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunUpdatedFieldsEvent {
    pub update_event_mixin: UpdateEventMixin,
    pub event_type: String,
    pub assay_run: AssayRun,
    pub event_base: EventBase,
}
impl std::fmt::Display for AssayRunUpdatedFieldsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsArchivalChange {
    pub assay_run_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AssayRunsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemJsonCreate {}
impl std::fmt::Display for AppConfigItemJsonCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultIdsRequest {
    pub assay_result_ids: Vec<String>,
}
impl std::fmt::Display for AssayResultIdsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRnaSequenceRequired {
    pub is_circular: bool,
    pub primers: Vec<Primer>,
    pub bases: String,
    pub folder_id: String,
    pub translations: Vec<Translation>,
    pub aliases: Vec<String>,
    pub entity_registry_id: String,
    pub author_ids: Vec<String>,
    pub fields: Fields,
    pub custom_fields: CustomFields,
    pub name: String,
    pub schema_id: String,
    pub annotations: Vec<RnaAnnotation>,
    pub rna_sequence_id: String,
}
impl std::fmt::Display for UpdateRnaSequenceRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayRunsArchive {
    /**The reason for archiving the provided Assay Runs. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub assay_run_ids: Vec<String>,
}
impl std::fmt::Display for AssayRunsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsPaginatedList {
    pub next_token: Option<String>,
    pub automation_output_processors: Option<Vec<AutomationOutputProcessor>>,
}
impl std::fmt::Display for AutomationOutputProcessorsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdate {
    pub parent_storage_id: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
}
impl std::fmt::Display for LocationUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntrySchemasPaginatedList {
    pub entry_schemas: Option<Vec<EntrySchemaDetailed>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntrySchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InaccessibleResource {
    pub inaccessible_id: Option<String>,
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
pub struct BatchesArchivalChange {
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BatchesArchivalChange {
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
pub struct FeatureCreate {}
impl std::fmt::Display for FeatureCreate {
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
pub struct AppConfigItemIntegerBulkUpdate {}
impl std::fmt::Display for AppConfigItemIntegerBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsBulkCreateRequest {
    pub assay_results: Vec<AssayResultCreate>,
}
impl std::fmt::Display for AssayResultsBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoUpdate {
    pub oligo_update: OligoUpdate,
}
impl std::fmt::Display for RnaOligoUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRnaOligoRequired {
    pub fields: Fields,
    pub author_ids: Vec<String>,
    pub bases: String,
    pub name: String,
    pub schema_id: String,
    pub aliases: Vec<String>,
    pub oligo_id: String,
    pub custom_fields: CustomFields,
    pub folder_id: String,
}
impl std::fmt::Display for UpdateRnaOligoRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersArchive {
    /**Reason that containers are being archived.
*/
    pub reason: String,
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: Option<bool>,
    ///Array of container IDs
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobCreate {
    ///base64 encoded file contents
    pub data64: String,
    ///Name of the blob
    pub name: String,
    /**The MD5 hash of the blob part. Note: this should be the hash of the raw data of the blob part, not the hash of the base64 encoding.
*/
    pub md5: String,
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: String,
}
impl std::fmt::Display for BlobCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonAppConfigItem {
    pub base_app_config_item: BaseAppConfigItem,
    pub type_: String,
    pub value: Option<String>,
}
impl std::fmt::Display for JsonAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DefaultConcentrationSummary {
    ///Amount of measurement.
    pub value: Option<f64>,
    ///Units of measurement.
    pub units: Option<String>,
}
impl std::fmt::Display for DefaultConcentrationSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesPaginatedList {
    pub next_token: Option<String>,
    pub rna_sequences: Option<Vec<RnaSequence>>,
}
impl std::fmt::Display for RnaSequencesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerSchema {
    ///DateTime the Container Schema was last modified
    pub modified_at: String,
    pub type_: String,
    pub registry_schema: RegistrySchema,
}
impl std::fmt::Display for ContainerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomEntityRequired {
    pub author_ids: Vec<String>,
    pub custom_entity_id: String,
    pub fields: Fields,
    pub folder_id: String,
    pub custom_fields: CustomFields,
    pub schema_id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub entity_registry_id: String,
}
impl std::fmt::Display for UpdateCustomEntityRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingAppUpdate(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntityLabels {
    pub entity_registry_id: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
}
impl std::fmt::Display for EntityLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesArchivalChange {
    pub aa_sequence_ids: Option<Vec<String>>,
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AaSequencesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaOligosArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub dna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for DnaOligosArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocationsUnarchive {
    ///Array of location IDs
    pub location_ids: Vec<String>,
}
impl std::fmt::Display for LocationsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldAppConfigItem {
    pub base_app_config_item: BaseAppConfigItem,
    pub type_: String,
    pub linked_app_config_resource_mixin: LinkedAppConfigResourceMixin,
    pub value: Option<String>,
}
impl std::fmt::Display for FieldAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeBaseRequestForCreate {
    pub molecule_base_request: MoleculeBaseRequest,
}
impl std::fmt::Display for MoleculeBaseRequestForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoUpdate {
    pub oligo_base_request: OligoBaseRequest,
}
impl std::fmt::Display for OligoUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Printer {
    ///ID of the printer.
    pub id: Option<String>,
    ///Short description of the printer.
    pub description: Option<String>,
    ///Name of the printer.
    pub name: Option<String>,
    ///ID of the registry associated with this printer.
    pub registry_id: Option<String>,
    ///Web address of the printer (either IP address or URL).
    pub address: Option<String>,
    ///Port to reach the printer at.
    pub port: Option<i64>,
}
impl std::fmt::Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
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
pub struct Schema {
    pub field_definitions: Option<Vec<serde_json::Value>>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub type_: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
}
impl std::fmt::Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigurationPaginatedList {}
impl std::fmt::Display for AppConfigurationPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct LocationSchemasList {
    pub location_schemas: Option<Vec<LocationSchema>>,
}
impl std::fmt::Display for LocationSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreatedEvent {
    pub event_type: String,
    pub event_base: EventBase,
    pub request: Request,
}
impl std::fmt::Display for RequestCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSummary(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksArchive {
    /**The reason for archiving the provided workflow tasks. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub workflow_task_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTasksArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationOutputProcessorUploadedV2BetaEvent {
    pub event_type: String,
    pub automation_output_processor: AutomationFile,
    pub event_base: EventBase,
}
impl std::fmt::Display for AutomationOutputProcessorUploadedV2BetaEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AigGenerateInputAsyncTask {}
impl std::fmt::Display for AigGenerateInputAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationProgressStats {
    pub rows_failed: Option<i64>,
    pub rows_succeeded: Option<i64>,
    pub rows_unprocessed: Option<i64>,
}
impl std::fmt::Display for AutomationProgressStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseAppConfigItem {}
impl std::fmt::Display for BaseAppConfigItem {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct MixturePrepTableNotePart {
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
    pub mixture_schema_id: String,
    pub base_note_part: BaseNotePart,
    pub type_: String,
}
impl std::fmt::Display for MixturePrepTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsUnarchive {
    ///A list of project IDs to unarchive.
    pub project_ids: Vec<String>,
}
impl std::fmt::Display for ProjectsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerContent {
    pub entity: Option<EntityOrInaccessibleResource>,
    pub batch: Option<BatchOrInaccessibleResource>,
    pub concentration: Option<Measurement>,
}
impl std::fmt::Display for ContainerContent {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct OligoBaseRequestForCreate {
    pub oligo_base_request: OligoBaseRequest,
}
impl std::fmt::Display for OligoBaseRequestForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaLinkFieldDefinition {
    pub type_: String,
    pub schema_id: Option<String>,
    pub field_definition: FieldDefinition,
}
impl std::fmt::Display for SchemaLinkFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligoBulkUpdate {
    pub rna_oligo_update: RnaOligoUpdate,
    pub id: String,
}
impl std::fmt::Display for RnaOligoBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputUpdatedFieldsEvent {
    pub workflow_output: WorkflowOutput,
    pub event_type: String,
    pub event_base: EventBase,
}
impl std::fmt::Display for WorkflowOutputUpdatedFieldsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchSchemasList {
    pub batch_schemas: Option<Vec<BatchSchema>>,
}
impl std::fmt::Display for BatchSchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorCreate {
    pub automation_file_config_name: String,
    ///Specifies whether file processing should complete with errors. False means any error in output file processing will result in no actions being committed. True means that if row-level errors occur, then failing rows and their errors will be saved to errorFile, and actions from successful rows will be committed.
    pub complete_with_errors: Option<bool>,
    ///The ID of a blob link to process.
    pub file_id: String,
    pub assay_run_id: String,
}
impl std::fmt::Display for AutomationOutputProcessorCreate {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntry {
    ///ID of the stage entry
    pub id: Option<String>,
    ///Review record if set
    pub review_record: Option<serde_json::Value>,
    /**Array of UserSummary Resources of the authors of the stage entry. This defaults to the creator but can be manually changed.
*/
    pub authors: Option<Vec<UserSummary>>,
    ///DateTime the stage entry was created at
    pub created_at: Option<String>,
    ///Entry schema if set
    pub schema: Option<serde_json::Value>,
    ///ID of the parent workflow
    pub workflow_id: Option<String>,
    ///DateTime the stage entry was last modified
    pub modified_at: Option<String>,
    ///ID of the associated workflow stage
    pub workflow_stage_id: Option<String>,
    ///ID of the folder that contains the stage entry
    pub folder_id: Option<String>,
    ///Title of the stage entry
    pub name: Option<String>,
    ///UserSummary Resource of the user who created the stage entry
    pub creator: Option<UserSummary>,
    ///URL of the stage entry
    pub web_url: Option<String>,
    pub custom_fields: Option<CustomFields>,
    ///The canonical url of the Stage Entry in the API.
    pub api_url: Option<String>,
    ///User-friendly ID of the stage entry
    pub display_id: Option<String>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for StageEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct IngredientWriteParams {
    ///Text representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "text" format.
    pub component_lot_text: Option<String>,
    pub units: Option<String>,
    ///The container representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "storage" format.
    pub component_lot_container_id: Option<String>,
    /**The amount value of this ingredient in its mixture, in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4). One ingredient on this mixture can have an amount value of `"QS"`.
*/
    pub amount: Option<String>,
    pub notes: Option<String>,
    ///The entity representing the component lot for this ingredient. This is only writable if the mixture schema supports component lots in "storage" format.
    pub component_lot_entity_id: Option<String>,
    pub catalog_identifier: Option<String>,
    ///The entity that uniquely identifies this component.
    pub component_entity_id: String,
}
impl std::fmt::Display for IngredientWriteParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesBulkCreateRequest {
    pub aa_sequences: Option<Vec<AaSequenceBulkCreate>>,
}
impl std::fmt::Display for AaSequencesBulkCreateRequest {
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
pub struct AppConfigItemDatetimeBulkUpdate {}
impl std::fmt::Display for AppConfigItemDatetimeBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageList {
    pub workflow_stages: Option<Vec<WorkflowStage>>,
}
impl std::fmt::Display for WorkflowStageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedScheduledOnEvent {
    pub event_type: String,
    pub workflow_task: WorkflowTask,
    pub event_base: EventBase,
}
impl std::fmt::Display for WorkflowTaskUpdatedScheduledOnEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedStatusEvent {
    pub event_base: EventBase,
    pub event_type: String,
    pub workflow_task: WorkflowTask,
}
impl std::fmt::Display for WorkflowTaskUpdatedStatusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkflowTaskRequired {
    pub fields: Fields,
    pub assignee_id: String,
    pub scheduled_on: String,
    pub workflow_task_group_id: String,
}
impl std::fmt::Display for CreateWorkflowTaskRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Primer {
    pub oligo_id: Option<String>,
    pub strand: Option<i64>,
    pub end: Option<i64>,
    pub bases: Option<String>,
    pub start: Option<i64>,
    pub bind_position: Option<i64>,
    pub color: Option<String>,
    pub overhang_length: Option<i64>,
    pub name: Option<String>,
}
impl std::fmt::Display for Primer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FeatureLibraryBase {
    ///The name of the feature library
    pub name: Option<String>,
    ///The description for the feature library
    pub description: Option<String>,
}
impl std::fmt::Display for FeatureLibraryBase {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerTransferBase {
    /**ID of the batch that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_batch_id: Option<String>,
    /**This represents the quantity of the source to be transferred into the destination container. Supports mass, volume, and other quantities. Required in place of transferVolume.
*/
    pub transfer_quantity: Option<ContainerQuantity>,
    /**Deprecated - use transferQuantity instead.
*/
    pub transfer_volume: Option<DeprecatedContainerVolumeForInput>,
    /**ID of the container that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_container_id: Option<String>,
    /**ID of the entity that will be transferred in. Must specify one of sourceEntityId, sourceBatchId, or sourceContainerId.
*/
    pub source_entity_id: Option<String>,
}
impl std::fmt::Display for ContainerTransferBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextAppConfigItem {}
impl std::fmt::Display for TextAppConfigItem {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskBase {
    pub creator: UserSummary,
    pub outputs: Vec<WorkflowOutputSummary>,
    ///The date on which the task is scheduled to be executed
    pub scheduled_on: Option<String>,
    pub workflow_task_summary: WorkflowTaskSummary,
    pub creation_origin: CreationOrigin,
    pub execution_origin: Option<WorkflowTaskExecutionOrigin>,
    pub status: WorkflowTaskStatus,
    pub fields: Fields,
    pub assignee: Option<UserSummary>,
    ///URL of the workflow task
    pub web_url: String,
    pub workflow_task_group: WorkflowTaskGroupSummary,
    pub cloned_from: Option<WorkflowTaskSummary>,
}
impl std::fmt::Display for WorkflowTaskBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for ConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsArchivalChange {
    pub app_ids: Option<Vec<String>>,
}
impl std::fmt::Display for BenchlingAppsArchivalChange {
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
pub struct AutoAnnotateAaSequences {
    ///Array of AA sequence IDs.
    pub aa_sequence_ids: Vec<String>,
    ///Array of feature library IDs.
    pub feature_library_ids: Vec<String>,
}
impl std::fmt::Display for AutoAnnotateAaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsArchive {
    /**Reason that dropdown options are being archived.
*/
    pub reason: Option<String>,
    ///Array of dropdown option IDs
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DropdownOptionsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntryUpdatedFieldsEvent {
    pub update_event_mixin: UpdateEventMixin,
    pub event_type: String,
    pub event_base: EventBase,
    ///A notebook entry used for execution of one or more stage runs in a legacy workflow.
    pub stage_entry: StageEntry,
}
impl std::fmt::Display for StageEntryUpdatedFieldsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskWriteBase(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkCopyRequest {
    pub workflow_task_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTasksBulkCopyRequest {
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
pub struct PlatesPaginatedList {
    pub next_token: Option<String>,
    pub plates: Option<Vec<Plate>>,
}
impl std::fmt::Display for PlatesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InitialTable {
    ///blobId of an uploaded csv blob. The CSV should be formatted with column headers of `columnId` which can be found in the [EntryTemplate](#/components/schemas/EntryTemplate). For more information on uploading a blob, [click here](https://docs.benchling.com/docs/uploading-a-blob-to-benchling).
    pub csv_data: Option<String>,
    ///Template table API ID
    pub template_table_id: Option<String>,
}
impl std::fmt::Display for InitialTable {
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
pub struct AppConfigItemBooleanCreate {}
impl std::fmt::Display for AppConfigItemBooleanCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MoleculeBulkUpdate {
    pub id: String,
    pub molecule_update: MoleculeUpdate,
}
impl std::fmt::Display for MoleculeBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskGroupsPaginatedList {
    pub next_token: Option<String>,
    pub workflow_task_groups: Option<Vec<WorkflowTaskGroup>>,
}
impl std::fmt::Display for WorkflowTaskGroupsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaAlignmentBase {
    pub algorithm: String,
    pub name: Option<String>,
    pub files: Vec<serde_json::Value>,
}
impl std::fmt::Display for DnaAlignmentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligosPaginatedList {
    pub pagination: Pagination,
    pub dna_oligos: Vec<DnaOligo>,
}
impl std::fmt::Display for DnaOligosPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerUpdate {
    pub container_write_base: ContainerWriteBase,
    /**Desired volume for a container, well, or transfer. "volume" type keys are deprecated in API requests; use the more permissive "quantity" type key instead.
*/
    pub volume: DeprecatedContainerVolumeForInput,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: ContainerQuantity,
    pub project_id: Option<String>,
}
impl std::fmt::Display for ContainerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationTableNotePart {
    pub entity_schema_id: String,
    pub base_note_part: BaseNotePart,
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
    pub type_: String,
}
impl std::fmt::Display for RegistrationTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
pub struct RnaOligosBulkCreateRequest {
    pub rna_oligos: Option<Vec<RnaOligoCreate>>,
}
impl std::fmt::Display for RnaOligosBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceCreate {
    pub create_entity_into_registry: CreateEntityIntoRegistry,
    pub dna_sequence_base_request_for_create: DnaSequenceBaseRequestForCreate,
}
impl std::fmt::Display for DnaSequenceCreate {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBulkCreate {
    pub rna_sequence_create: RnaSequenceCreate,
}
impl std::fmt::Display for RnaSequenceBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreate {
    pub default_concentration: Option<DefaultConcentrationSummary>,
    ///API identifier for the entity that the batch will be added to.
    pub entity_id: Option<String>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for BatchCreate {
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateNucleotideConsensusAlignmentAsyncTask {}
impl std::fmt::Display for CreateNucleotideConsensusAlignmentAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaTemplateAlignmentCreate {
    pub dna_alignment_base: DnaAlignmentBase,
    pub template_sequence_id: String,
}
impl std::fmt::Display for DnaTemplateAlignmentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesArchivalChange {
    pub batch_ids: Option<Vec<String>>,
    pub custom_entity_ids: Option<Vec<String>>,
}
impl std::fmt::Display for CustomEntitiesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub has_parent: Option<bool>,
    ///The target amount for this ingredient such that this ingredient's proportion in its mixture would preserve the equivalent ingredient's proportion in the parent mixture. Pair with `units`.
    pub target_amount: Option<String>,
    ///Text representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "text" format.
    pub component_lot_text: Option<String>,
    pub catalog_identifier: Option<String>,
    pub notes: Option<String>,
    ///The container representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "storage" format.
    pub component_lot_container: Option<ContainerLabels>,
    ///The entity representing the component lot for this ingredient. This is only present if the mixture schema supports component lots in "storage" format.
    pub component_lot_entity: Option<EntityLabels>,
    pub component_entity: Option<serde_json::Value>,
    pub units: Option<String>,
    /**The amount value of this ingredient in its mixture, in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4). One ingredient on this mixture can have an amount value of `"QS"`.
*/
    pub amount: Option<String>,
}
impl std::fmt::Display for Ingredient {
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
pub struct OligosArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub oligo_ids: Vec<String>,
}
impl std::fmt::Display for OligosArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemJsonBulkUpdate {}
impl std::fmt::Display for AppConfigItemJsonBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksBulkCreateRequest {
    pub workflow_tasks: Option<Vec<WorkflowTaskBulkCreate>>,
}
impl std::fmt::Display for WorkflowTasksBulkCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequence {
    pub custom_fields: Option<CustomFields>,
    pub primers: Option<Vec<Primer>>,
    pub schema: Option<SchemaSummary>,
    pub aliases: Option<Vec<String>>,
    pub translations: Option<Vec<Translation>>,
    pub name: Option<String>,
    pub web_url: Option<String>,
    pub annotations: Option<Vec<DnaAnnotation>>,
    pub registry_id: Option<String>,
    pub registration_origin: Option<RegistrationOrigin>,
    ///The canonical url of the DNA Sequence in the API.
    pub api_url: Option<String>,
    pub entity_registry_id: Option<String>,
    pub is_circular: Option<bool>,
    pub length: Option<i64>,
    pub modified_at: Option<String>,
    pub folder_id: Option<String>,
    pub creator: Option<UserSummary>,
    pub bases: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub created_at: Option<String>,
    pub fields: Option<Fields>,
    pub id: Option<String>,
}
impl std::fmt::Display for DnaSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesPaginatedList {
    pub next_token: Option<String>,
    pub mixtures: Option<Vec<Mixture>>,
}
impl std::fmt::Display for MixturesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDnaOligoRequired {
    pub custom_fields: CustomFields,
    pub folder_id: String,
    pub schema_id: String,
    pub aliases: Vec<String>,
    pub author_ids: Vec<String>,
    pub fields: Fields,
    pub bases: String,
    pub name: String,
    pub oligo_id: String,
}
impl std::fmt::Display for UpdateDnaOligoRequired {
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
pub struct BoxesArchive {
    ///Array of box IDs
    pub box_ids: Vec<String>,
    /**Reason that boxes are being archived.
*/
    pub reason: String,
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
pub struct AppConfigItemDatetimeUpdate {
    pub value: String,
    pub type_: String,
}
impl std::fmt::Display for AppConfigItemDatetimeUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerWriteBase(pub serde_json::Value);
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
pub struct PlatesArchive {
    ///Array of plate IDs
    pub plate_ids: Vec<String>,
    /**Reason that plates are being archived.
*/
    pub reason: String,
    /**Remove barcodes. Removing barcodes from archived storage that contain items will also remove barcodes from the contained items.
*/
    pub should_remove_barcodes: bool,
}
impl std::fmt::Display for PlatesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
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
pub struct RnaOligosUnarchive {
    pub rna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for RnaOligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownOptionsArchivalChange {
    pub dropdown_option_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DropdownOptionsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUnauthorizedError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for OAuthUnauthorizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStage {
    ///ID of the workflow stage
    pub id: Option<String>,
    ///DateTime at which the the workflow stage was created
    pub created_at: Option<String>,
    ///Name of the workflow stage
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Container {
    pub project_id: Option<String>,
    pub web_url: Option<String>,
    pub parent_storage_schema: Option<SchemaSummary>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub parent_storage_id: Option<String>,
    pub id: Option<String>,
    pub schema: Option<SchemaSummary>,
    pub creator: Option<UserSummary>,
    ///Quantity of a container, well, or transfer. Supports mass, volume, and other quantities.
    pub quantity: Option<ContainerQuantity>,
    pub checkout_record: Option<CheckoutRecord>,
    pub archive_record: Option<ArchiveRecord>,
    pub contents: Option<Vec<ContainerContent>>,
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
    pub volume: Option<DeprecatedContainerVolumeForResponse>,
}
impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksUnarchive {
    pub workflow_task_ids: Vec<String>,
}
impl std::fmt::Display for WorkflowTasksUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchRequestRequired {
    pub fields: Fields,
    pub project_id: String,
    pub assignees: Vec<serde_json::Value>,
    pub request_id: String,
    pub scheduled_on: String,
    pub sample_groups: Vec<RequestSampleGroupCreate>,
    pub request_status: String,
}
impl std::fmt::Display for PatchRequestRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenchlingAppsUnarchive {
    ///Array of app IDs
    pub app_ids: Vec<String>,
}
impl std::fmt::Display for BenchlingAppsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersCheckout {
    ///User or Team API ID.
    pub assignee_id: String,
    pub comment: Option<String>,
    ///Array of container IDs.
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersCheckout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContainersUnarchive {
    ///Array of container IDs
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for ContainersUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBaseRequest {
    /**Custom fields to add to the RNA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**ID of the RNA sequence's schema.
*/
    pub schema_id: Option<String>,
    ///Aliases to add to the RNA sequence
    pub aliases: Option<Vec<String>>,
    /**Whether the RNA sequence is circular or linear. RNA sequences can only be linear
*/
    pub is_circular: Option<bool>,
    /**Translations to create on the RNA sequence. Translations are specified by either a combination of 'start' and 'end' fields, or a list of regions. Both cannot be provided.
*/
    pub translations: Option<Vec<Translation>>,
    /**ID of the folder containing the RNA sequence.
*/
    pub folder_id: Option<String>,
    ///IDs of users to set as the RNA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    pub primers: Option<Vec<Primer>>,
    /**Annotations to create on the RNA sequence.
*/
    pub annotations: Option<Vec<RnaAnnotation>>,
    /**Base pairs for the RNA sequence.
*/
    pub bases: Option<String>,
    /**Fields to set on the RNA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    /**Name of the RNA sequence.
*/
    pub name: Option<String>,
}
impl std::fmt::Display for RnaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskUpdatedAssigneeEvent {
    pub event_base: EventBase,
    pub event_type: String,
    pub workflow_task: WorkflowTask,
}
impl std::fmt::Display for WorkflowTaskUpdatedAssigneeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntitiesArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub custom_entity_ids: Vec<String>,
}
impl std::fmt::Display for CustomEntitiesArchive {
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
pub struct DnaSequenceBulkCreate {
    pub dna_sequence_create: DnaSequenceCreate,
}
impl std::fmt::Display for DnaSequenceBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTeamAssignee {
    pub team: Option<TeamSummary>,
}
impl std::fmt::Display for RequestTeamAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoCreate {
    pub oligo_create: OligoCreate,
}
impl std::fmt::Display for DnaOligoCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaAnnotation {
    pub name: Option<String>,
    pub color: Option<String>,
    pub type_: Option<String>,
    pub id: Option<String>,
    pub end: Option<i64>,
    pub start: Option<i64>,
}
impl std::fmt::Display for AaAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDnaConsensusAlignmentRequired {
    pub files: Vec<serde_json::Value>,
    pub name: String,
    pub sequence_id: String,
    pub new_sequence: serde_json::Value,
    pub algorithm: String,
}
impl std::fmt::Display for CreateDnaConsensusAlignmentRequired {
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
pub struct AaSequencesBulkGet {
    pub aa_sequences: Option<Vec<AaSequence>>,
}
impl std::fmt::Display for AaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCreatedEvent {
    /**Entries are notes that users can take. They're organized by "days" (which are user-configurable) and modeled within each day as a list of "notes." Each note has a type - the simplest is a "text" type, but lists, tables, and external files are also supported.

*Note:* the current Entry resource has a few limitations:
- Formatting information is not yet supported. Header formatting, bolding, and other stylistic information is not presented.
- Data in tables is presented as text always - numeric values will need to be parsed into floats or integers, as appropriate.

Note: Data in Results tables are not accessible through this API call. Results table data can be called through the Results API calls.
*/
    pub entry: Entry,
    pub event_type: String,
    pub event_base: EventBase,
}
impl std::fmt::Display for EntryCreatedEvent {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct StageEntryCreatedEvent {
    pub event_base: EventBase,
}
impl std::fmt::Display for StageEntryCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
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
pub struct UsersPaginatedList {
    pub users: Vec<User>,
    pub pagination: Pagination,
}
impl std::fmt::Display for UsersPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrintLabels {
    /**ID of label template to use (same template will be used for all labels printed).
*/
    pub label_template_id: String,
    /**ID of printer to use (same printer will be used for all labels printed).
*/
    pub printer_id: String,
    /**List of IDs of containers that will have labels printed (one label will be printed per container).
*/
    pub container_ids: Vec<String>,
}
impl std::fmt::Display for PrintLabels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    pub name: Option<String>,
    pub fields: Option<Fields>,
    pub web_url: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    pub id: Option<String>,
    pub default_concentration: Option<Measurement>,
    pub schema: Option<SchemaSummary>,
    pub creator: Option<UserSummary>,
    ///DateTime at which the the result was created
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub entity: Option<serde_json::Value>,
}
impl std::fmt::Display for Batch {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryTableCell {
    /**The textual content of the cell. If the cell was originally a formula, this will be the evaluated version of the formula.
*/
    pub text: Option<String>,
    /**A Link Resource if this cell contained a hyperlink. Otherwise, link will be omitted from the cell object. (Note: storage and user types are not yet supported.)
*/
    pub link: Option<EntryLink>,
}
impl std::fmt::Display for EntryTableCell {
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
pub struct MoleculesArchivalChange {
    pub batch_ids: Option<Vec<String>>,
    pub molecule_ids: Option<Vec<String>>,
}
impl std::fmt::Display for MoleculesArchivalChange {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoBulkUpdate {
    pub dna_oligo_update: DnaOligoUpdate,
    pub id: String,
}
impl std::fmt::Display for DnaOligoBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeprecatedEntitySchemasList {
    pub entity_schemas: Option<Vec<DeprecatedEntitySchema>>,
}
impl std::fmt::Display for DeprecatedEntitySchemasList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthBadRequestError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for OAuthBadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Organization {
    pub name: Option<String>,
    pub id: Option<String>,
    pub handle: Option<String>,
}
impl std::fmt::Display for Organization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PrintersList {
    pub label_printers: Option<Vec<Printer>>,
}
impl std::fmt::Display for PrintersList {
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
pub struct StageEntryUpdatedReviewRecordEvent {
    ///A notebook entry used for execution of one or more stage runs in a legacy workflow.
    pub entry: StageEntry,
    pub update_event_mixin: UpdateEventMixin,
    pub event_type: String,
    pub event_base: EventBase,
}
impl std::fmt::Display for StageEntryUpdatedReviewRecordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LinkedAppConfigResourceSummary {
    ///The API ID of the linked resource
    pub id: Option<String>,
    ///The name of the resource in Benchling
    pub name: Option<String>,
}
impl std::fmt::Display for LinkedAppConfigResourceSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EventsPaginatedList {
    pub events: Option<Vec<Event>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for EventsPaginatedList {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceUpdate {
    pub aa_sequence_request_registry_fields: AaSequenceRequestRegistryFields,
    pub aa_sequence_base_request: AaSequenceBaseRequest,
}
impl std::fmt::Display for AaSequenceUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BoxesUnarchive {
    ///Array of box IDs
    pub box_ids: Vec<String>,
}
impl std::fmt::Display for BoxesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderCreate(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct IntegerFieldDefinition {
    pub numeric_max: Option<f64>,
    pub type_: String,
    pub numeric_min: Option<f64>,
    pub field_definition: FieldDefinition,
}
impl std::fmt::Display for IntegerFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LinkedAppConfigResource {}
impl std::fmt::Display for LinkedAppConfigResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateUpdate {
    pub name: Option<String>,
    pub project_id: Option<String>,
    pub parent_storage_id: Option<String>,
    pub fields: Option<Fields>,
}
impl std::fmt::Display for PlateUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesArchive {
    pub rna_sequence_ids: Vec<String>,
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for RnaSequencesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageList {
    pub workflow_stages: Option<Vec<LegacyWorkflowStage>>,
}
impl std::fmt::Display for LegacyWorkflowStageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SequenceFeatureBase {
    pub custom_fields: Option<Vec<SequenceFeatureCustomField>>,
    ///Hex color code used when displaying this feature in the UI.
    pub color: Option<String>,
    pub name: Option<String>,
    pub notes: Option<String>,
}
impl std::fmt::Display for SequenceFeatureBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StructuredTableColumnInfo {
    pub column_id: Option<String>,
    pub is_read_only: Option<bool>,
    pub name: Option<String>,
}
impl std::fmt::Display for StructuredTableColumnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateSchema {
    pub container_schema: Option<serde_json::Value>,
    pub height: f64,
    pub width: f64,
    pub type_: String,
    pub plate_type: String,
    pub registry_schema: RegistrySchema,
}
impl std::fmt::Display for PlateSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobUrl {
    ///a pre-signed download url.
    pub download_url: Option<String>,
    ///The unix time that the download URL expires at.
    pub expires_at: Option<i64>,
}
impl std::fmt::Display for BlobUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserValidation {
    /**Valid values for this enum depend on whether it is used to set a value (e.g., in a POST request), or is a return value for an existing result.
Acceptable values for setting a status are 'VALID' or 'INVALID'. Possible return values are 'VALID', 'INVALID', or 'PARTIALLY_VALID' (a result will be partially valid if it has some valid fields and some invalid fields).
*/
    pub validation_status: Option<String>,
    ///A string explaining the reason for the provided validation status.
    pub validation_comment: Option<String>,
}
impl std::fmt::Display for UserValidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesPaginatedList {
    pub batches: Option<Vec<Batch>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for BatchesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsArchive {
    pub workflow_output_ids: Vec<String>,
    /**The reason for archiving the provided workflow outputs. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
}
impl std::fmt::Display for WorkflowOutputsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemBooleanBulkUpdate {}
impl std::fmt::Display for AppConfigItemBooleanBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaSequenceBaseRequest {
    pub primers: Option<Vec<Primer>>,
    /**Translations to create on the DNA sequence. Translations are specified by either a combination of 'start' and 'end' fields, or a list of regions. Both cannot be provided.
*/
    pub translations: Option<Vec<Translation>>,
    /**ID of the DNA sequence's schema.
*/
    pub schema_id: Option<String>,
    ///IDs of users to set as the DNA sequence's authors.
    pub author_ids: Option<Vec<String>>,
    /**ID of the folder containing the DNA sequence.
*/
    pub folder_id: Option<String>,
    /**Base pairs for the DNA sequence.
*/
    pub bases: Option<String>,
    /**Name of the DNA sequence.
*/
    pub name: Option<String>,
    /**Fields to set on the DNA sequence. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    /**Annotations to create on the DNA sequence.
*/
    pub annotations: Option<Vec<DnaAnnotation>>,
    ///Aliases to add to the DNA sequence
    pub aliases: Option<Vec<String>>,
    /**Custom fields to add to the DNA sequence. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    /**Whether the DNA sequence is circular or linear.
*/
    pub is_circular: Option<bool>,
}
impl std::fmt::Display for DnaSequenceBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowStageRun {
    ///DateTime at which the the stage run was created
    pub created_at: Option<String>,
    ///Status of the stage run
    pub status: Option<String>,
    ///ID of the stage run
    pub id: Option<String>,
    ///Name of the stage run
    pub name: Option<String>,
}
impl std::fmt::Display for WorkflowStageRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestTaskSchemasPaginatedList {
    pub request_task_schemas: Option<Vec<RequestTaskSchema>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for RequestTaskSchemasPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemBulkUpdate {}
impl std::fmt::Display for AppConfigItemBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AaSequenceBaseRequestForCreate {
    pub aa_sequence_base_request: AaSequenceBaseRequest,
}
impl std::fmt::Display for AaSequenceBaseRequestForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunNotePart {
    pub type_: String,
    pub base_note_part: BaseNotePart,
    pub assay_run_id: Option<String>,
    pub assay_run_schema_id: String,
}
impl std::fmt::Display for AssayRunNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectsArchive {
    /**The reason for archiving the provided projects. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    ///A list of project IDs to archive.
    pub project_ids: Vec<String>,
}
impl std::fmt::Display for ProjectsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate {
    pub request_write_base: RequestWriteBase,
    pub request_status: String,
}
impl std::fmt::Display for RequestUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateDnaTemplateAlignmentRequired {
    pub files: Vec<serde_json::Value>,
    pub template_sequence_id: String,
    pub name: String,
    pub algorithm: String,
}
impl std::fmt::Display for CreateDnaTemplateAlignmentRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DropdownFieldDefinition {
    pub type_: String,
    pub field_definition: FieldDefinition,
    pub dropdown_id: Option<String>,
}
impl std::fmt::Display for DropdownFieldDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Blob {
    ///The universally unique identifier (UUID) for the blob.
    pub id: Option<String>,
    pub upload_status: Option<String>,
    ///Name of the blob
    pub name: Option<String>,
    ///eg. application/jpeg
    pub mime_type: Option<String>,
    /**One of RAW_FILE or VISUALIZATION. If VISUALIZATION, the blob may be displayed as an image preview.
*/
    pub type_: Option<String>,
}
impl std::fmt::Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemIntegerUpdate {
    pub value: i64,
    pub type_: String,
}
impl std::fmt::Display for AppConfigItemIntegerUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BoxCreate {
    pub name: Option<String>,
    pub schema_id: String,
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub parent_storage_id: Option<String>,
    pub project_id: Option<String>,
}
impl std::fmt::Display for BoxCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligo {
    pub api_url: String,
    pub oligo: Oligo,
    pub bases: String,
}
impl std::fmt::Display for DnaOligo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTask {
    ///The method by which the task of the workflow is executed
    pub execution_type: String,
    pub workflow_task_base: WorkflowTaskBase,
}
impl std::fmt::Display for WorkflowTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntitySchemaAppConfigItem {
    pub linked_app_config_resource_mixin: LinkedAppConfigResourceMixin,
    pub type_: String,
    pub base_app_config_item: BaseAppConfigItem,
    pub value: Option<String>,
    pub subtype: String,
}
impl std::fmt::Display for EntitySchemaAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupCreatedEvent {
    pub event_base: EventBase,
    pub event_type: String,
    pub workflow_task_group: WorkflowTaskGroup,
}
impl std::fmt::Display for WorkflowTaskGroupCreatedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixturesBulkUpdateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BulkCreateCustomEntitiesAsyncTask {}
impl std::fmt::Display for BulkCreateCustomEntitiesAsyncTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSchema {
    pub registry_schema: RegistrySchema,
    pub type_: String,
}
impl std::fmt::Display for LocationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBaseRequest {
    pub schema_id: Option<String>,
    ///IDs of users to set as the custom entity's authors.
    pub author_ids: Option<Vec<String>>,
    /**Schema fields to set on the custom entity. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub fields: Option<Fields>,
    ///Aliases to add to the custom entity
    pub aliases: Option<Vec<String>>,
    /**Custom fields to add to the custom entity. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
    ///ID of the folder that the entity is moved into
    pub folder_id: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for CustomEntityBaseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkflowOutputArchiveReason {
    #[serde(rename = "Made in error")]
    MadeInError,
    Retired,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroup {
    pub workflow_task_group_base: WorkflowTaskGroupBase,
    ///The method by which the workflow is executed
    pub execution_type: String,
}
impl std::fmt::Display for WorkflowTaskGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsyncTaskLink {
    pub task_id: Option<String>,
}
impl std::fmt::Display for AsyncTaskLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub rna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for RnaOligosArchive {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultsArchive {
    ///The reason for archiving the provided results. Accepted reasons may differ based on tenant configuration
    pub reason: String,
    pub assay_result_ids_request: AssayResultIdsRequest,
}
impl std::fmt::Display for AssayResultsArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryUpdatedReviewRecordEvent {
    /**Entries are notes that users can take. They're organized by "days" (which are user-configurable) and modeled within each day as a list of "notes." Each note has a type - the simplest is a "text" type, but lists, tables, and external files are also supported.

*Note:* the current Entry resource has a few limitations:
- Formatting information is not yet supported. Header formatting, bolding, and other stylistic information is not presented.
- Data in tables is presented as text always - numeric values will need to be parsed into floats or integers, as appropriate.

Note: Data in Results tables are not accessible through this API call. Results table data can be called through the Results API calls.
*/
    pub entry: Entry,
    pub event_base: EventBase,
    pub update_event_mixin: UpdateEventMixin,
    pub event_type: String,
}
impl std::fmt::Display for EntryUpdatedReviewRecordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutput {
    pub fields: Fields,
    ///URL of the workflow output
    pub web_url: String,
    pub workflow_task_group: WorkflowTaskGroupSummary,
    pub task: WorkflowTaskSummary,
    pub workflow_output_summary: WorkflowOutputSummary,
    pub creation_origin: CreationOrigin,
}
impl std::fmt::Display for WorkflowOutput {
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
pub struct RnaOligosArchivalChange {
    pub rna_oligo_ids: Option<Vec<String>>,
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for RnaOligosArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestWriteUserAssignee {
    pub user_id: String,
}
impl std::fmt::Display for RequestWriteUserAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabAutomationTransform {
    pub custom_transform_id: Option<String>,
    ///The canonical url of the transform in the API.
    pub api_url: Option<String>,
    pub errors: Option<Vec<LabAutomationBenchlingAppError>>,
    pub id: Option<String>,
    ///DateTime the transform was last modified.
    pub modified_at: Option<String>,
    pub output_processor_id: Option<String>,
    pub status: Option<String>,
    pub blob_id: Option<String>,
}
impl std::fmt::Display for LabAutomationTransform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WarehouseCredentialsCreate {
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
pub struct CreateEntityIntoRegistry {
    ///ID of the folder containing the entity. Can be left empty when registryId is present.
    pub folder_id: Option<String>,
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
    /**Entity registry ID to set for the registered entity. Cannot specify both entityRegistryId and namingStrategy at the same time.
*/
    pub entity_registry_id: Option<String>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryExternalFileById {
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
pub struct AppConfigItemDateCreate {}
impl std::fmt::Display for AppConfigItemDateCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseError {
    pub type_: Option<String>,
    pub user_message: Option<String>,
    pub message: Option<String>,
}
impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSchemaBase {
    ///ID of the default folder for creating workflow task groups
    pub default_creation_folder_id: Option<String>,
    ///ID of the default folder for workflow task execution entries
    pub default_entry_execution_folder_id: Option<String>,
    ///The ID of the template of the entries tasks of this schema will be executed into.
    pub entry_template_id: Option<String>,
    ///The prefix for the displayId of task groups containing tasks of this schema
    pub task_group_prefix: String,
    pub default_responsible_team: Option<TeamSummary>,
    pub workflow_output_schema: Option<WorkflowOutputSchema>,
    pub schema: Schema,
    ///The prefix for the displayId of tasks of this schema.
    pub prefix: String,
    ///Whether or not tasks of this schema can be created with a non-null assignee.
    pub can_set_assignee_on_task_creation: bool,
    pub status_lifecycle: WorkflowTaskStatusLifecycle,
}
impl std::fmt::Display for WorkflowTaskSchemaBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DropdownSummariesPaginatedList {
    pub dropdowns: Option<Vec<DropdownSummary>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for DropdownSummariesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FoldersPaginatedList {
    pub folders: Option<Vec<Folder>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for FoldersPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlobPart {
    pub part_number: Option<i64>,
    pub e_tag: Option<String>,
}
impl std::fmt::Display for BlobPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Feature {}
impl std::fmt::Display for Feature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssayResultsPaginatedList {
    pub assay_results: Option<Vec<AssayResult>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for AssayResultsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntityRequestRegistryFields {
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for CustomEntityRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LabelTemplatesList {
    pub label_templates: Option<Vec<LabelTemplate>>,
}
impl std::fmt::Display for LabelTemplatesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixtureUpdate {
    ///Aliases to add to the mixture
    pub aliases: Option<Vec<String>>,
    pub units: Option<String>,
    pub entity_registry_id: Option<String>,
    ///IDs of users to set as the mixture's authors.
    pub author_ids: Option<Vec<String>>,
    ///ID of the folder that the entity is moved into
    pub folder_id: Option<String>,
    /**Schema fields to set on the mixture. Must correspond with the schema's field definitions. Every field should have its name as a key, mapping to an object with information about the value of the field.
If you are setting the parent mixture field here, you must also specify `ingredients`
*/
    pub fields: Option<Fields>,
    /**Desired final state for the ingredients on this mixture.
Each ingredient you specify will be matched with the existing ingredients on the mixture based on the component entity, and Benchling will create, update, or delete this mixture's ingredients so that the final state of this mixture's ingredients matches your request.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixture (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub ingredients: Option<Vec<IngredientWriteParams>>,
    pub name: Option<String>,
    ///The positive numerical amount value of this mixture in string format (to preserve full precision). Pair with `units`. Supports scientific notation (1.23e4).
    pub amount: Option<String>,
    pub schema_id: Option<String>,
    /**Custom fields to add to the mixture. Every field should have its name as a key, mapping to an object with information about the value of the field.
*/
    pub custom_fields: Option<CustomFields>,
}
impl std::fmt::Display for MixtureUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DropdownOption {
    pub name: Option<String>,
    pub id: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
}
impl std::fmt::Display for DropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntityBulkCreate {
    pub custom_entity_create: CustomEntityCreate,
}
impl std::fmt::Display for CustomEntityBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemBulkUpdateMixin {
    pub id: String,
}
impl std::fmt::Display for AppConfigItemBulkUpdateMixin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContainerRequired {
    pub parent_storage_id: String,
    pub fields: Fields,
    pub barcode: String,
    pub schema_id: String,
    pub name: String,
}
impl std::fmt::Display for CreateContainerRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemGenericUpdate {
    pub type_: String,
    pub value: String,
}
impl std::fmt::Display for AppConfigItemGenericUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemFloatBulkUpdate {}
impl std::fmt::Display for AppConfigItemFloatBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DnaOligoUpdate {
    pub oligo_update: OligoUpdate,
}
impl std::fmt::Display for DnaOligoUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentSummary {
    ///The Benchling web UI url to view the Alignment
    pub web_url: Option<String>,
    ///The canonical url of the Alignment in the API.
    pub api_url: Option<String>,
    pub id: Option<String>,
    ///DateTime the Alignment was last modified
    pub modified_at: Option<String>,
    ///The ID of the template or consensus Sequence associated with the Alignment
    pub reference_sequence_id: Option<String>,
    ///DateTime the Alignment was created
    pub created_at: Option<String>,
    pub name: Option<String>,
}
impl std::fmt::Display for NucleotideAlignmentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkGet {
    pub rna_sequences: Option<Vec<RnaSequence>>,
}
impl std::fmt::Display for RnaSequencesBulkGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureBulkCreate {
    ///Inputs for a new feature
    pub feature_create: FeatureCreate,
}
impl std::fmt::Display for FeatureBulkCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaSequencesBulkUpdateRequest {
    pub rna_sequences: Option<Vec<RnaSequenceBulkUpdate>>,
}
impl std::fmt::Display for RnaSequencesBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BadRequestError {
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for BadRequestError {
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
pub struct DnaSequencesArchivalChange {
    pub dna_sequence_ids: Option<Vec<String>>,
    pub batch_ids: Option<Vec<String>>,
}
impl std::fmt::Display for DnaSequencesArchivalChange {
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
pub struct CustomEntitiesUnarchive {
    pub custom_entity_ids: Vec<String>,
}
impl std::fmt::Display for CustomEntitiesUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NucleotideConsensusAlignmentCreate {
    pub sequence_id: String,
    pub new_sequence: serde_json::Value,
    pub nucleotide_alignment_base: NucleotideAlignmentBase,
}
impl std::fmt::Display for NucleotideConsensusAlignmentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestsPaginatedList {
    pub requests_bulk_get: RequestsBulkGet,
    pub next_token: String,
}
impl std::fmt::Display for RequestsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowOutputSummary(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
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
pub struct EntitySchemasPaginatedList {
    pub entity_schemas: Option<Vec<EntitySchema>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntitySchemasPaginatedList {
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
pub struct ContainersList {
    pub containers: Option<Vec<Container>>,
}
impl std::fmt::Display for ContainersList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskSchema {
    pub workflow_task_schema_base: WorkflowTaskSchemaBase,
    ///The method by which instances of this schema are executed
    pub execution_type: String,
}
impl std::fmt::Display for WorkflowTaskSchema {
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
pub struct AutomationFileInputsPaginatedList {
    pub next_token: Option<String>,
    pub automation_input_generators: Option<Vec<AutomationInputGenerator>>,
}
impl std::fmt::Display for AutomationFileInputsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultsTableNotePart {
    pub base_note_part: BaseNotePart,
    pub assay_result_schema_id: String,
    pub structured_table_api_identifiers: StructuredTableApiIdentifiers,
    pub type_: String,
}
impl std::fmt::Display for ResultsTableNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemsBulkUpdateRequest {
    pub app_configuration_items: Vec<AppConfigItemBulkUpdate>,
}
impl std::fmt::Display for AppConfigItemsBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutomationOutputProcessorsArchive {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchlingAppCreate(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RnaOligosBulkUpdateRequest {
    pub rna_oligos: Option<Vec<RnaOligoBulkUpdate>>,
}
impl std::fmt::Display for RnaOligosBulkUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaOligosPaginatedList {
    pub rna_oligos: Vec<RnaOligo>,
    pub pagination: Pagination,
}
impl std::fmt::Display for RnaOligosPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMoleculeRequired {
    pub author_ids: Vec<String>,
    pub folder_id: String,
    pub entity_registry_id: String,
    pub molecule_id: String,
    pub schema_id: String,
    pub custom_fields: CustomFields,
    pub name: String,
    pub aliases: Vec<String>,
    pub fields: Fields,
    pub chemical_structure: MoleculeStructure,
}
impl std::fmt::Display for UpdateMoleculeRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SampleGroupStatusUpdate {
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
pub struct Team {}
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesPaginatedList {
    pub entries: Option<Vec<Entry>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for EntriesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceBaseRequestForCreate {
    pub rna_sequence_base_request: RnaSequenceBaseRequest,
}
impl std::fmt::Display for RnaSequenceBaseRequestForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesPaginatedList {
    pub aa_sequences: Option<Vec<AaSequence>>,
    pub next_token: Option<String>,
}
impl std::fmt::Display for AaSequencesPaginatedList {
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
pub struct DnaOligosUnarchive {
    pub dna_oligo_ids: Vec<String>,
}
impl std::fmt::Display for DnaOligosUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PartySummary {
    pub id: Option<String>,
    pub name: Option<String>,
    pub handle: Option<String>,
}
impl std::fmt::Display for PartySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlateSchemasPaginatedList {
    pub plate_schemas_list: PlateSchemasList,
    pub next_token: String,
}
impl std::fmt::Display for PlateSchemasPaginatedList {
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
pub struct AssayRunsUnarchive {
    pub assay_run_ids: Vec<String>,
}
impl std::fmt::Display for AssayRunsUnarchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RnaSequenceUpdate {
    pub rna_sequence_base_request: RnaSequenceBaseRequest,
    pub rna_sequence_request_registry_fields: RnaSequenceRequestRegistryFields,
}
impl std::fmt::Display for RnaSequenceUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AssayRunValidationStatus {
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOligoRequired {
    pub oligo_id: String,
    pub author_ids: Vec<String>,
    pub bases: String,
    pub fields: Fields,
    pub custom_fields: CustomFields,
    pub aliases: Vec<String>,
    pub name: String,
    pub folder_id: String,
    pub schema_id: String,
}
impl std::fmt::Display for UpdateOligoRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowStageRunList {
    pub workflow_stage_runs: Option<Vec<LegacyWorkflowStageRun>>,
}
impl std::fmt::Display for LegacyWorkflowStageRunList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomEntityRequired {
    pub custom_fields: CustomFields,
    pub naming_strategy: String,
    pub schema_id: String,
    pub author_ids: Vec<String>,
    pub registry_id: String,
    pub entity_registry_id: String,
    pub name: String,
    pub folder_id: String,
    pub aliases: Vec<String>,
    pub fields: Fields,
}
impl std::fmt::Display for CreateCustomEntityRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CustomEntityWithEntityType {}
impl std::fmt::Display for CustomEntityWithEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BatchesArchive {
    /**The reason for archiving the provided Batches. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub batch_ids: Vec<String>,
}
impl std::fmt::Display for BatchesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerTransferDestinationContentsItem {
    pub concentration: Option<Measurement>,
    pub entity_id: String,
}
impl std::fmt::Display for ContainerTransferDestinationContentsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestFulfillmentsPaginatedList {
    pub next_token: Option<String>,
    pub request_fulfillments: Option<Vec<RequestFulfillment>>,
}
impl std::fmt::Display for RequestFulfillmentsPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayResultSchema {
    pub base_assay_schema: BaseAssaySchema,
    ///DateTime the Assay Result Schema was last modified
    pub modified_at: String,
    pub type_: String,
}
impl std::fmt::Display for AssayResultSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTaskExecutionOrigin {
    pub entry_id: Option<String>,
    pub origin_modal_uuid: Option<String>,
    pub type_: Option<String>,
}
impl std::fmt::Display for WorkflowTaskExecutionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContainerRequired {
    pub volume: DeprecatedContainerVolumeForInput,
    pub fields: Fields,
    pub parent_storage_id: String,
    pub quantity: ContainerQuantity,
    pub name: String,
    pub container_id: String,
}
impl std::fmt::Display for UpdateContainerRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NucleotideAlignmentBase {
    pub files: Vec<serde_json::Value>,
    pub name: Option<String>,
    pub algorithm: String,
}
impl std::fmt::Display for NucleotideAlignmentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequencesArchive {
    /**The reason for archiving the provided entities. Accepted reasons may differ based on tenant configuration.
*/
    pub reason: String,
    pub aa_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AaSequencesArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Molecule {
    pub creator: Option<UserSummary>,
    ///The canonical url of the Molecule in the API.
    pub api_url: Option<String>,
    pub fields: Option<Fields>,
    ///URL of the Molecule.
    pub web_url: Option<String>,
    ///Name of the Molecule.
    pub name: Option<String>,
    ///Registry the Molecule is registered in.
    pub registry_id: Option<String>,
    pub schema: Option<SchemaSummary>,
    ///Array of aliases.
    pub aliases: Option<Vec<String>>,
    ///Registry ID of the Molecule if registered.
    pub entity_registry_id: Option<String>,
    ///ID of the folder that contains the Molecule.
    pub folder_id: Option<String>,
    pub archive_record: Option<ArchiveRecord>,
    ///DateTime the Molecule was last modified.
    pub modified_at: Option<String>,
    ///The canonicalized chemical structure in SMILES format.
    pub canonicalized_smiles: Option<String>,
    ///DateTime the Molecule was created.
    pub created_at: Option<String>,
    ///Custom fields set on the Molecule.
    pub custom_fields: Option<CustomFields>,
    pub registration_origin: Option<RegistrationOrigin>,
    ///ID of the Molecule.
    pub id: Option<String>,
}
impl std::fmt::Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomEntitiesBulkCreateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntryCreate {
    /**Fields to set on the entry. Must correspond with the schema's field definitions.
*/
    pub fields: Option<Fields>,
    ///ID of the entry's schema
    pub schema_id: Option<String>,
    ///Custom fields to add to the entry
    pub custom_fields: Option<CustomFields>,
    ///ID of the folder that will contain the entry
    pub folder_id: String,
    /**An array of table API IDs and blob id pairs to seed tables from the template while creating the entry. The entryTemplateId parameter must be set to use this parameter. The table API IDs should be the API Identifiers of the tables in the given template.
- If a template table has one row, the values in that row act as default values for cloned entries.
- If a template table has multiple rows, there is no default value and those rows are added to the cloned entry along with the provided csv data.
- If a table has default values, they will be populated in any respective undefined columns in the csv data.
- If a table has no default values, undefined columns from csv data will be empty.
- If no csv data is provided for a table, the table in the entry will be populated with whatever values are in the respective template table.
*/
    pub initial_tables: Option<Vec<InitialTable>>,
    pub author_ids: Option<serde_json::Value>,
    ///Name of the entry
    pub name: String,
    ///ID of the template to clone the entry from
    pub entry_template_id: Option<String>,
}
impl std::fmt::Display for EntryCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssayRunUpdate {
    pub fields: Option<Fields>,
}
impl std::fmt::Display for AssayRunUpdate {
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
pub struct CreateWorkflowTaskGroupRequired {
    pub name: String,
    pub watcher_ids: Vec<String>,
    pub folder_id: String,
    pub schema_id: String,
}
impl std::fmt::Display for CreateWorkflowTaskGroupRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MixturesArchivalChange {
    pub mixture_ids: Option<Vec<String>>,
}
impl std::fmt::Display for MixturesArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AaSequenceRequestRegistryFields {
    pub entity_registry_id: Option<String>,
}
impl std::fmt::Display for AaSequenceRequestRegistryFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemCreateMixin {
    ///App id to which this config item belongs.
    pub app_id: String,
    ///Array-based representation of config item's location in the tree in order from top to bottom.
    pub path: Vec<String>,
}
impl std::fmt::Display for AppConfigItemCreateMixin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutofillRnaSequences {
    ///Array of RNA sequence IDs.
    pub rna_sequence_ids: Vec<String>,
}
impl std::fmt::Display for AutofillRnaSequences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BarcodeValidationResult {
    ///Barcode to validate.
    pub barcode: Option<String>,
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
pub struct BoxesPaginatedList {
    pub next_token: Option<String>,
    pub boxes: Option<Vec<Box>>,
}
impl std::fmt::Display for BoxesPaginatedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityRegisteredEvent {
    pub event_base: EventBase,
    pub entity: GenericEntity,
    pub event_type: String,
}
impl std::fmt::Display for EntityRegisteredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntrySchema {
    ///ID of the entry schema
    pub id: Option<String>,
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
pub struct DeprecatedEntitySchema {
    pub entity_schema: EntitySchema,
    pub type_: String,
}
impl std::fmt::Display for DeprecatedEntitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LegacyWorkflowPatch {
    ///Name of the legacy workflow
    pub name: Option<String>,
    ///ID of the project that contains the legacy workflow
    pub project_id: Option<String>,
    ///Description of the legacy workflow
    pub description: Option<String>,
}
impl std::fmt::Display for LegacyWorkflowPatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Location(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EntriesArchive {
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
pub struct RegistrationOrigin {
    pub origin_entry_id: Option<String>,
    pub registered_at: Option<String>,
}
impl std::fmt::Display for RegistrationOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestSampleWithEntity {
    pub container_id: Option<String>,
    pub entity_id: String,
}
impl std::fmt::Display for RequestSampleWithEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowOutputsArchivalChange {
    pub workflow_output_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowOutputsArchivalChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DnaSequencesArchive {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupBase {
    pub workflow_task_group_summary: WorkflowTaskGroupSummary,
    ///URL of the workflow task group
    pub web_url: String,
    pub responsible_team: Option<TeamSummary>,
    pub workflow_task_schema: WorkflowTaskSchemaSummary,
    ///The outputs of the workflow task group
    pub outputs: Vec<WorkflowOutputSummary>,
    pub creation_origin: CreationOrigin,
    ///The users watching the workflow task group
    pub watchers: Vec<UserSummary>,
    pub creator: UserSummary,
    pub folder: Folder,
    ///The input tasks to the workflow task group
    pub workflow_tasks: Vec<WorkflowTaskSummary>,
}
impl std::fmt::Display for WorkflowTaskGroupBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowTaskGroupSummary(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegisterEntities {
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
    ///Array of entity IDs
    pub entity_ids: Vec<String>,
}
impl std::fmt::Display for RegisterEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Dropdown {
    pub dropdown_summary: DropdownSummary,
    pub archive_record: Option<ArchiveRecord>,
    ///Array of dropdown options
    pub options: Vec<DropdownOption>,
}
impl std::fmt::Display for Dropdown {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct DateAppConfigItem {
    pub type_: String,
    pub base_app_config_item: BaseAppConfigItem,
    pub value: Option<String>,
}
impl std::fmt::Display for DateAppConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDnaSequenceRequired {
    pub name: String,
    pub translations: Vec<Translation>,
    pub schema_id: String,
    pub annotations: Vec<DnaAnnotation>,
    pub bases: String,
    pub is_circular: bool,
    pub entity_registry_id: String,
    pub author_ids: Vec<String>,
    pub naming_strategy: String,
    pub fields: Fields,
    pub custom_fields: CustomFields,
    pub primers: Vec<Primer>,
    pub aliases: Vec<String>,
    pub folder_id: String,
    pub registry_id: String,
}
impl std::fmt::Display for CreateDnaSequenceRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MixtureBulkUpdate {
    pub id: String,
    pub mixture_update: MixtureUpdate,
}
impl std::fmt::Display for MixtureBulkUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorkflowTasksArchivalChange {
    pub workflow_task_ids: Option<Vec<String>>,
}
impl std::fmt::Display for WorkflowTasksArchivalChange {
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
pub struct BaseNotePart {
    ///The type of the note.  Type determines what other fields are present.
    pub type_: Option<String>,
    ///All notes have an indentation level - the default is 0 for no indent. For lists, indentation gives notes hierarchy - a bulleted list with children is modeled as one note part with indentation 1 followed by note parts with indentation 2, for example.
    pub indentation: Option<i64>,
}
impl std::fmt::Display for BaseNotePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfigItemBooleanUpdate {
    pub value: bool,
    pub type_: String,
}
impl std::fmt::Display for AppConfigItemBooleanUpdate {
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

//! [`BenchlingClient`](struct.BenchlingClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct BenchlingClient {
    pub(crate) client: httpclient::Client,
    authentication: BenchlingAuthentication,
}
impl BenchlingClient {
    pub fn from_env() -> Self {
        let url = "/api/v2".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: BenchlingAuthentication::from_env(),
        }
    }
}
impl BenchlingClient {
    pub fn new(url: &str, authentication: BenchlingAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(
        mut self,
        authentication: BenchlingAuthentication,
    ) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            BenchlingAuthentication::BasicApiKeyAuth { basic_api_key_auth } => {
                r = r.basic_auth(basic_api_key_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///List AA sequences
    pub fn list_aa_sequences(&self) -> request::ListAaSequencesRequest {
        request::ListAaSequencesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            amino_acids: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create an AA sequence
    pub fn create_aa_sequence(
        &self,
        args: request::CreateAaSequenceRequired,
    ) -> request::CreateAaSequenceRequest {
        request::CreateAaSequenceRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            amino_acids: args.amino_acids.to_owned(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get an AA sequence
    pub fn get_aa_sequence(
        &self,
        aa_sequence_id: &str,
    ) -> request::GetAaSequenceRequest {
        request::GetAaSequenceRequest {
            client: &self,
            aa_sequence_id: aa_sequence_id.to_owned(),
        }
    }
    ///Update an AA sequence
    pub fn update_aa_sequence(
        &self,
        args: request::UpdateAaSequenceRequired,
    ) -> request::UpdateAaSequenceRequest {
        request::UpdateAaSequenceRequest {
            client: &self,
            aa_sequence_id: args.aa_sequence_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            amino_acids: args.amino_acids.to_owned(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
        }
    }
    ///Archive AA sequences
    pub fn archive_aa_sequences(
        &self,
        aa_sequence_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveAaSequencesRequest {
        request::ArchiveAaSequencesRequest {
            client: &self,
            aa_sequence_ids: aa_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Auto-annotate AA sequences with matching features from specified Feature Libraries
    pub fn auto_annotate_aa_sequences(
        &self,
        aa_sequence_ids: &[&str],
        feature_library_ids: &[&str],
    ) -> request::AutoAnnotateAaSequencesRequest {
        request::AutoAnnotateAaSequencesRequest {
            client: &self,
            aa_sequence_ids: aa_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
            feature_library_ids: feature_library_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    /**Bulk Create AA sequences

Bulk Create AA sequences. Limit of 1000 AA Sequences per request.*/
    pub fn bulk_create_aa_sequences(&self) -> request::BulkCreateAaSequencesRequest {
        request::BulkCreateAaSequencesRequest {
            client: &self,
            aa_sequences: None,
        }
    }
    ///Bulk get AA sequences by ID
    pub fn bulk_get_aa_sequences(
        &self,
        aa_sequence_ids: &str,
    ) -> request::BulkGetAaSequencesRequest {
        request::BulkGetAaSequencesRequest {
            client: &self,
            aa_sequence_ids: aa_sequence_ids.to_owned(),
        }
    }
    ///Bulk Update AA sequences
    pub fn bulk_update_aa_sequences(&self) -> request::BulkUpdateAaSequencesRequest {
        request::BulkUpdateAaSequencesRequest {
            client: &self,
            aa_sequences: None,
        }
    }
    ///Unarchive AA sequences
    pub fn unarchive_aa_sequences(
        &self,
        aa_sequence_ids: &[&str],
    ) -> request::UnarchiveAaSequencesRequest {
        request::UnarchiveAaSequencesRequest {
            client: &self,
            aa_sequence_ids: aa_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Get app configuration items
    pub fn list_app_configuration_items(
        &self,
    ) -> request::ListAppConfigurationItemsRequest {
        request::ListAppConfigurationItemsRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
            app_id: None,
            ids: None,
            sort: None,
        }
    }
    ///Create app configuration item
    pub fn create_app_configuration_item(
        &self,
    ) -> request::CreateAppConfigurationItemRequest {
        request::CreateAppConfigurationItemRequest {
            client: &self,
        }
    }
    ///Get app configuration item
    pub fn get_app_configuration_item_by_id(
        &self,
        item_id: &str,
    ) -> request::GetAppConfigurationItemByIdRequest {
        request::GetAppConfigurationItemByIdRequest {
            client: &self,
            item_id: item_id.to_owned(),
        }
    }
    ///Update app configuration item
    pub fn update_app_configuration_item(
        &self,
        item_id: &str,
    ) -> request::UpdateAppConfigurationItemRequest {
        request::UpdateAppConfigurationItemRequest {
            client: &self,
            item_id: item_id.to_owned(),
        }
    }
    /**Bulk Create app configuration items. Limit of 1000 App Config Items per request.

Bulk Create app configuration items*/
    pub fn bulk_create_app_configuration_items(
        &self,
        app_configuration_items: Vec<AppConfigItemCreate>,
    ) -> request::BulkCreateAppConfigurationItemsRequest {
        request::BulkCreateAppConfigurationItemsRequest {
            client: &self,
            app_configuration_items,
        }
    }
    /**Bulk Update app configuration items. Limit of 1000 App Config Items per request.

Bulk Update app configuration items*/
    pub fn bulk_update_app_configuration_items(
        &self,
        app_configuration_items: Vec<AppConfigItemBulkUpdate>,
    ) -> request::BulkUpdateAppConfigurationItemsRequest {
        request::BulkUpdateAppConfigurationItemsRequest {
            client: &self,
            app_configuration_items,
        }
    }
    ///List apps
    pub fn list_benchling_apps(&self) -> request::ListBenchlingAppsRequest {
        request::ListBenchlingAppsRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            ids: None,
            modified_at: None,
            name: None,
            name_includes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            member_of: None,
            admin_of: None,
        }
    }
    ///Create an app
    pub fn create_benchling_app(
        &self,
        name: &str,
    ) -> request::CreateBenchlingAppRequest {
        request::CreateBenchlingAppRequest {
            client: &self,
            description: None,
            name: name.to_owned(),
        }
    }
    ///Get an app by ID
    pub fn get_benchling_app_by_id(
        &self,
        app_id: &str,
    ) -> request::GetBenchlingAppByIdRequest {
        request::GetBenchlingAppByIdRequest {
            client: &self,
            app_id: app_id.to_owned(),
        }
    }
    ///Update an app
    pub fn patch_benchling_app(
        &self,
        app_id: &str,
    ) -> request::PatchBenchlingAppRequest {
        request::PatchBenchlingAppRequest {
            client: &self,
            app_id: app_id.to_owned(),
            description: None,
            name: None,
        }
    }
    ///Archive apps
    pub fn archive_benchling_apps(
        &self,
        app_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveBenchlingAppsRequest {
        request::ArchiveBenchlingAppsRequest {
            client: &self,
            app_ids: app_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Unarchive apps
    pub fn unarchive_benchling_apps(
        &self,
        app_ids: &[&str],
    ) -> request::UnarchiveBenchlingAppsRequest {
        request::UnarchiveBenchlingAppsRequest {
            client: &self,
            app_ids: app_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List assay result schemas
    pub fn list_assay_result_schemas(&self) -> request::ListAssayResultSchemasRequest {
        request::ListAssayResultSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a Result schema by ID
    pub fn get_result_schema(&self, schema_id: &str) -> request::GetResultSchemaRequest {
        request::GetResultSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List results
    pub fn list_assay_results(
        &self,
        schema_id: &str,
    ) -> request::ListAssayResultsRequest {
        request::ListAssayResultsRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
            created_at_lt: None,
            created_at_gt: None,
            created_at_lte: None,
            created_at_gte: None,
            min_created_time: None,
            max_created_time: None,
            sort: None,
            next_token: None,
            page_size: None,
            entity_ids: None,
            storage_ids: None,
            assay_run_ids: None,
            ids: None,
        }
    }
    ///Create 1 or more results.
    pub fn create_assay_results(
        &self,
        assay_results: Vec<AssayResultCreate>,
    ) -> request::CreateAssayResultsRequest {
        request::CreateAssayResultsRequest {
            client: &self,
            assay_results,
        }
    }
    ///Get a result
    pub fn get_assay_result(
        &self,
        assay_result_id: &str,
    ) -> request::GetAssayResultRequest {
        request::GetAssayResultRequest {
            client: &self,
            assay_result_id: assay_result_id.to_owned(),
        }
    }
    /**Archive 1 or more results.

**Only results that have not been added to a Notebook Entry can be Archived.**
Once results are attached to a notebook entry, they are tracked in the history of that notebook entry, and cannot be archived.
*/
    pub fn archive_assay_results(
        &self,
        assay_result_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveAssayResultsRequest {
        request::ArchiveAssayResultsRequest {
            client: &self,
            assay_result_ids: assay_result_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Gets multiple results specified by a list of IDs.

Up to 200 IDs can be specified at once.*/
    pub fn bulk_get_assay_results(
        &self,
        assay_result_ids: &str,
    ) -> request::BulkGetAssayResultsRequest {
        request::BulkGetAssayResultsRequest {
            client: &self,
            assay_result_ids: assay_result_ids.to_owned(),
        }
    }
    ///Unarchive 1 or more results.
    pub fn unarchive_assay_results(
        &self,
        assay_result_ids: &[&str],
    ) -> request::UnarchiveAssayResultsRequest {
        request::UnarchiveAssayResultsRequest {
            client: &self,
            assay_result_ids: assay_result_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List assay run schemas
    pub fn list_assay_run_schemas(&self) -> request::ListAssayRunSchemasRequest {
        request::ListAssayRunSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a Run schema by ID
    pub fn get_run_schema(&self, schema_id: &str) -> request::GetRunSchemaRequest {
        request::GetRunSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List runs
    pub fn list_assay_runs(&self, schema_id: &str) -> request::ListAssayRunsRequest {
        request::ListAssayRunsRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
            min_created_time: None,
            max_created_time: None,
            next_token: None,
            page_size: None,
            ids: None,
        }
    }
    ///Create 1 or more runs.
    pub fn create_assay_runs(
        &self,
        assay_runs: Vec<AssayRunCreate>,
    ) -> request::CreateAssayRunsRequest {
        request::CreateAssayRunsRequest {
            client: &self,
            assay_runs,
        }
    }
    ///Get a run
    pub fn get_assay_run(&self, assay_run_id: &str) -> request::GetAssayRunRequest {
        request::GetAssayRunRequest {
            client: &self,
            assay_run_id: assay_run_id.to_owned(),
        }
    }
    ///Update a run
    pub fn update_assay_run(
        &self,
        assay_run_id: &str,
    ) -> request::UpdateAssayRunRequest {
        request::UpdateAssayRunRequest {
            client: &self,
            assay_run_id: assay_run_id.to_owned(),
            fields: None,
        }
    }
    ///list AutomationInputGenerators by Run
    pub fn list_automation_input_generators(
        &self,
        assay_run_id: &str,
    ) -> request::ListAutomationInputGeneratorsRequest {
        request::ListAutomationInputGeneratorsRequest {
            client: &self,
            assay_run_id: assay_run_id.to_owned(),
            next_token: None,
            modified_at: None,
        }
    }
    /**list AutomationOutputProcessors by Run

Deprecated in favor of '/automation-output-processors'. For each output config in the run config, will create an empty automationOutputProcessor for the run if one doesn't exist.*/
    pub fn list_automation_output_processors_deprecated(
        &self,
        assay_run_id: &str,
    ) -> request::ListAutomationOutputProcessorsDeprecatedRequest {
        request::ListAutomationOutputProcessorsDeprecatedRequest {
            client: &self,
            assay_run_id: assay_run_id.to_owned(),
            next_token: None,
        }
    }
    /**Archive Assay Runs

Archive assay runs that are not embedded in a notebook entry*/
    pub fn archive_assay_runs(
        &self,
        assay_run_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveAssayRunsRequest {
        request::ArchiveAssayRunsRequest {
            client: &self,
            assay_run_ids: assay_run_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Bulk get runs by ID
    pub fn bulk_get_assay_runs(
        &self,
        assay_run_ids: &str,
    ) -> request::BulkGetAssayRunsRequest {
        request::BulkGetAssayRunsRequest {
            client: &self,
            assay_run_ids: assay_run_ids.to_owned(),
        }
    }
    /**Unarchive Assay Runs

Unarchive assay runs*/
    pub fn unarchive_assay_runs(
        &self,
        assay_run_ids: &[&str],
    ) -> request::UnarchiveAssayRunsRequest {
        request::UnarchiveAssayRunsRequest {
            client: &self,
            assay_run_ids: assay_run_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Get a Lab Automation Transform step
    pub fn get_lab_automation_transform(
        &self,
        transform_id: &str,
    ) -> request::GetLabAutomationTransformRequest {
        request::GetLabAutomationTransformRequest {
            client: &self,
            transform_id: transform_id.to_owned(),
        }
    }
    ///Update a Lab Automation Transform step
    pub fn update_lab_automation_transform(
        &self,
        transform_id: &str,
    ) -> request::UpdateLabAutomationTransformRequest {
        request::UpdateLabAutomationTransformRequest {
            client: &self,
            transform_id: transform_id.to_owned(),
            blob_id: None,
            errors: None,
        }
    }
    ///Get an Automation Input Generator
    pub fn get_automation_input_generator(
        &self,
        input_generator_id: &str,
    ) -> request::GetAutomationInputGeneratorRequest {
        request::GetAutomationInputGeneratorRequest {
            client: &self,
            input_generator_id: input_generator_id.to_owned(),
        }
    }
    ///Update an Automation Input Generator
    pub fn update_automation_input_generator(
        &self,
        input_generator_id: &str,
    ) -> request::UpdateAutomationInputGeneratorRequest {
        request::UpdateAutomationInputGeneratorRequest {
            client: &self,
            input_generator_id: input_generator_id.to_owned(),
            file_id: None,
        }
    }
    ///Generate Input with an Automation Input Generator
    pub fn generate_input_with_automation_input_generator(
        &self,
        input_generator_id: &str,
    ) -> request::GenerateInputWithAutomationInputGeneratorRequest {
        request::GenerateInputWithAutomationInputGeneratorRequest {
            client: &self,
            input_generator_id: input_generator_id.to_owned(),
        }
    }
    /**List non-empty Automation Output Processors

List Automation Output Processors which have an attached file*/
    pub fn list_automation_output_processors(
        &self,
    ) -> request::ListAutomationOutputProcessorsRequest {
        request::ListAutomationOutputProcessorsRequest {
            client: &self,
            assay_run_id: None,
            automation_file_config_name: None,
            archive_reason: None,
            modified_at: None,
            next_token: None,
        }
    }
    ///Create Automation Output Processor
    pub fn create_automation_output_processor(
        &self,
        assay_run_id: &str,
        automation_file_config_name: &str,
        file_id: &str,
    ) -> request::CreateAutomationOutputProcessorRequest {
        request::CreateAutomationOutputProcessorRequest {
            client: &self,
            assay_run_id: assay_run_id.to_owned(),
            automation_file_config_name: automation_file_config_name.to_owned(),
            complete_with_errors: None,
            file_id: file_id.to_owned(),
        }
    }
    ///Get an Automation Output Processor
    pub fn get_automation_output_processor(
        &self,
        output_processor_id: &str,
    ) -> request::GetAutomationOutputProcessorRequest {
        request::GetAutomationOutputProcessorRequest {
            client: &self,
            output_processor_id: output_processor_id.to_owned(),
        }
    }
    ///Update an Automation Output Processor
    pub fn update_automation_output_processor(
        &self,
        output_processor_id: &str,
        file_id: &str,
    ) -> request::UpdateAutomationOutputProcessorRequest {
        request::UpdateAutomationOutputProcessorRequest {
            client: &self,
            output_processor_id: output_processor_id.to_owned(),
            file_id: file_id.to_owned(),
        }
    }
    ///Process Output with an Automation Output Processor
    pub fn process_output_with_automation_output_processor(
        &self,
        output_processor_id: &str,
    ) -> request::ProcessOutputWithAutomationOutputProcessorRequest {
        request::ProcessOutputWithAutomationOutputProcessorRequest {
            client: &self,
            output_processor_id: output_processor_id.to_owned(),
        }
    }
    ///Archive Automation Output Processors and linked Results
    pub fn archive_automation_output_processors(
        &self,
        automation_output_processor_ids: &[&str],
    ) -> request::ArchiveAutomationOutputProcessorsRequest {
        request::ArchiveAutomationOutputProcessorsRequest {
            client: &self,
            automation_output_processor_ids: automation_output_processor_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
            reason: None,
        }
    }
    ///Unarchive Automation Output Processors and linked Results
    pub fn unarchive_automation_output_processors(
        &self,
        automation_output_processor_ids: &[&str],
    ) -> request::UnarchiveAutomationOutputProcessorsRequest {
        request::UnarchiveAutomationOutputProcessorsRequest {
            client: &self,
            automation_output_processor_ids: automation_output_processor_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///List batch schemas
    pub fn list_batch_schemas(&self) -> request::ListBatchSchemasRequest {
        request::ListBatchSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a batch schema by ID
    pub fn get_batch_schema(&self, schema_id: &str) -> request::GetBatchSchemaRequest {
        request::GetBatchSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List batches
    pub fn list_batches(&self) -> request::ListBatchesRequest {
        request::ListBatchesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            ids: None,
            creator_ids: None,
        }
    }
    ///Create a batch
    pub fn create_batch(&self) -> request::CreateBatchRequest {
        request::CreateBatchRequest {
            client: &self,
            default_concentration: None,
            entity_id: None,
            fields: None,
        }
    }
    ///Get a batch
    pub fn get_batch(&self, batch_id: &str) -> request::GetBatchRequest {
        request::GetBatchRequest {
            client: &self,
            batch_id: batch_id.to_owned(),
        }
    }
    ///Update a batch
    pub fn update_batch(&self, batch_id: &str) -> request::UpdateBatchRequest {
        request::UpdateBatchRequest {
            client: &self,
            batch_id: batch_id.to_owned(),
            default_concentration: None,
            fields: None,
        }
    }
    ///Archive Batches
    pub fn archive_batches(
        &self,
        batch_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveBatchesRequest {
        request::ArchiveBatchesRequest {
            client: &self,
            batch_ids: batch_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Bulk get batches

Batches can be queried by their IDs or their names. Querying by name requires specifying a registryId since batch names are not necessarily unique across registries. Batches must be registered to query by name.
*/
    pub fn bulk_get_batches(&self) -> request::BulkGetBatchesRequest {
        request::BulkGetBatchesRequest {
            client: &self,
            batch_ids: None,
            batch_names: None,
            registry_id: None,
        }
    }
    ///Unarchive Batches
    pub fn unarchive_batches(
        &self,
        batch_ids: &[&str],
    ) -> request::UnarchiveBatchesRequest {
        request::UnarchiveBatchesRequest {
            client: &self,
            batch_ids: batch_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Upload single-part blob


This endpoint uploads a blob in a single API call.

Blobs larger than 10MB should be uploaded in [multiple parts](#/Blobs/createMultipartBlob). The data64 parameter is the base64-encoded part contents, and the md5 parameter is the hex-encoded MD5 hash of the part contents. For example, given the string hello, data64 is aGVsbG8= and md5 is 5d41402abc4b2a76b9719d911017c592.
*/
    pub fn create_blob(
        &self,
        args: request::CreateBlobRequired,
    ) -> request::CreateBlobRequest {
        request::CreateBlobRequest {
            client: &self,
            data64: args.data64.to_owned(),
            md5: args.md5.to_owned(),
            mime_type: None,
            name: args.name.to_owned(),
            type_: args.type_.to_owned(),
        }
    }
    ///Get a Blob
    pub fn get_blob(&self, blob_id: &str) -> request::GetBlobRequest {
        request::GetBlobRequest {
            client: &self,
            blob_id: blob_id.to_owned(),
        }
    }
    ///Get a Blob's download url
    pub fn get_blob_url(&self, blob_id: &str) -> request::GetBlobUrlRequest {
        request::GetBlobUrlRequest {
            client: &self,
            blob_id: blob_id.to_owned(),
        }
    }
    /**Upload a part of a multi-part blob


Upload a part of the blob. This part must be at least 5MB, unless it's the last or only part. It's recommended to keep the part size around 10MB.

The data64 parameter is the base64-encoded part contents, and the md5 parameter is the hex-encoded MD5 hash of the part contents. For example, given the string hello, data64 is aGVsbG8= and md5 is 5d41402abc4b2a76b9719d911017c592.

## Multipart Upload

If a blob is larger than 10MB, it should be uploaded in multiple parts using the following endpoints:
- [Start a multi-part blob upload](#/Blobs/createMultipartBlob)
- [Upload a blob part](#/Blobs/createBlobPart)
- [Complete a blob upload](#/Blobs/completeMultipartBlob)

Each part has a *partNumber* and an *eTag*. The part number can be any number between 1 to 10,000, inclusive - this number should be unique and identifies the order of the part in the final blob. The eTag of a part is returned in the API response - this eTag must be specified when completing the upload in order to ensure the server has received all the expected parts.
*/
    pub fn create_blob_part(
        &self,
        args: request::CreateBlobPartRequired,
    ) -> request::CreateBlobPartRequest {
        request::CreateBlobPartRequest {
            client: &self,
            blob_id: args.blob_id.to_owned(),
            data64: args.data64.to_owned(),
            md5: args.md5.to_owned(),
            part_number: args.part_number,
        }
    }
    ///Abort multi-part blob upload
    pub fn abort_multipart_blob(
        &self,
        blob_id: &str,
    ) -> request::AbortMultipartBlobRequest {
        request::AbortMultipartBlobRequest {
            client: &self,
            blob_id: blob_id.to_owned(),
        }
    }
    /**Complete multi-part blob upload


Combine blob parts into a single blob.

## Multipart Upload

If a blob is larger than 10MB, it should be uploaded in multiple parts using the following endpoints:
- [Start a multi-part blob upload](#/Blobs/createMultipartBlob)
- [Upload a blob part](#/Blobs/createBlobPart)
- [Complete a blob upload](#/Blobs/completeMultipartBlob)

Each part must be at least 5MB in size, except for the last part. We recommend keeping each part to under 10MB when uploading.

Each part has a *partNumber* and an *eTag*. The part number can be any number between 1 to 10,000, inclusive - this number should be unique and identifies the order of the part in the final blob. The eTag of a part is returned in the API response - this eTag must be specified when completing the upload in order to ensure the server has received all the expected parts.
*/
    pub fn complete_multipart_blob(
        &self,
        blob_id: &str,
    ) -> request::CompleteMultipartBlobRequest {
        request::CompleteMultipartBlobRequest {
            client: &self,
            blob_id: blob_id.to_owned(),
            parts: None,
        }
    }
    ///Bulk get Blobs by UUID
    pub fn bulk_get_blobs(&self) -> request::BulkGetBlobsRequest {
        request::BulkGetBlobsRequest {
            client: &self,
            blob_ids: None,
        }
    }
    /**Initiate multi-part blob upload


Blobs may be uploaded using multi-part upload. This endpoint initiates the upload process - blob parts can then be uploaded in multiple blob parts.

## Multipart Upload

If a blob is larger than 10MB, it should be uploaded in multiple parts using the following endpoints:
- [Start a multi-part blob upload](#/Blobs/createMultipartBlob)
- [Upload a blob part](#/Blobs/createBlobPart)
- [Complete a blob upload](#/Blobs/completeMultipartBlob)

Each part must be at least 5MB in size, except for the last part. We recommend keeping each part to under 10MB when uploading.

Each part has a *partNumber* and an *eTag*. The part number can be any number between 1 to 10,000, inclusive - this number should be unique and identifies the order of the part in the final blob. The eTag of a part is returned in the API response - this eTag must be specified when completing the upload in order to ensure the server has received all the expected parts.
*/
    pub fn create_multipart_blob(
        &self,
        name: &str,
        type_: &str,
    ) -> request::CreateMultipartBlobRequest {
        request::CreateMultipartBlobRequest {
            client: &self,
            mime_type: None,
            name: name.to_owned(),
            type_: type_.to_owned(),
        }
    }
    ///List box schemas
    pub fn list_box_schemas(&self) -> request::ListBoxSchemasRequest {
        request::ListBoxSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
        }
    }
    ///Get a box schema by ID
    pub fn get_box_schema(&self, schema_id: &str) -> request::GetBoxSchemaRequest {
        request::GetBoxSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List boxes
    pub fn list_boxes(&self) -> request::ListBoxesRequest {
        request::ListBoxesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            schema_id: None,
            schema_fields: None,
            modified_at: None,
            name: None,
            name_includes: None,
            empty_positions: None,
            empty_positions_gte: None,
            empty_positions_gt: None,
            empty_positions_lte: None,
            empty_positions_lt: None,
            empty_containers: None,
            empty_containers_gte: None,
            empty_containers_gt: None,
            empty_containers_lte: None,
            empty_containers_lt: None,
            ancestor_storage_id: None,
            storage_contents_id: None,
            storage_contents_ids: None,
            archive_reason: None,
            ids: None,
            barcodes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
        }
    }
    ///Create a box
    pub fn create_box(&self, schema_id: &str) -> request::CreateBoxRequest {
        request::CreateBoxRequest {
            client: &self,
            barcode: None,
            fields: None,
            name: None,
            parent_storage_id: None,
            project_id: None,
            schema_id: schema_id.to_owned(),
        }
    }
    ///Get a box
    pub fn get_box(&self, box_id: &str) -> request::GetBoxRequest {
        request::GetBoxRequest {
            client: &self,
            box_id: box_id.to_owned(),
        }
    }
    ///Update a box
    pub fn update_box(&self, box_id: &str) -> request::UpdateBoxRequest {
        request::UpdateBoxRequest {
            client: &self,
            box_id: box_id.to_owned(),
            fields: None,
            name: None,
            parent_storage_id: None,
            project_id: None,
        }
    }
    ///List a box's contents
    pub fn list_box_contents(&self, box_id: &str) -> request::ListBoxContentsRequest {
        request::ListBoxContentsRequest {
            client: &self,
            box_id: box_id.to_owned(),
            page_size: None,
            next_token: None,
        }
    }
    /**Archive boxes

Archive boxes and any containers of the boxes*/
    pub fn archive_boxes(
        &self,
        box_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveBoxesRequest {
        request::ArchiveBoxesRequest {
            client: &self,
            box_ids: box_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
            should_remove_barcodes: None,
        }
    }
    ///BulkGet boxes
    pub fn bulk_get_boxes(&self) -> request::BulkGetBoxesRequest {
        request::BulkGetBoxesRequest {
            client: &self,
            box_ids: None,
            barcodes: None,
        }
    }
    /**Unarchive boxes

Unarchive boxes and the containers that were archived along with them*/
    pub fn unarchive_boxes(&self, box_ids: &[&str]) -> request::UnarchiveBoxesRequest {
        request::UnarchiveBoxesRequest {
            client: &self,
            box_ids: box_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List container schemas
    pub fn list_container_schemas(&self) -> request::ListContainerSchemasRequest {
        request::ListContainerSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a container schema by ID
    pub fn get_container_schema(
        &self,
        schema_id: &str,
    ) -> request::GetContainerSchemaRequest {
        request::GetContainerSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List containers
    pub fn list_containers(&self) -> request::ListContainersRequest {
        request::ListContainersRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            schema_id: None,
            schema_fields: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ancestor_storage_id: None,
            storage_contents_id: None,
            storage_contents_ids: None,
            archive_reason: None,
            checkout_status: None,
            ids: None,
            barcodes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
        }
    }
    ///Create a new container
    pub fn create_container(
        &self,
        args: request::CreateContainerRequired,
    ) -> request::CreateContainerRequest {
        request::CreateContainerRequest {
            client: &self,
            fields: args.fields,
            name: args.name.to_owned(),
            parent_storage_id: args.parent_storage_id.to_owned(),
            barcode: args.barcode.to_owned(),
            project_id: None,
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///get a container by id
    pub fn get_container(&self, container_id: &str) -> request::GetContainerRequest {
        request::GetContainerRequest {
            client: &self,
            container_id: container_id.to_owned(),
        }
    }
    ///update a container
    pub fn update_container(
        &self,
        args: request::UpdateContainerRequired,
    ) -> request::UpdateContainerRequest {
        request::UpdateContainerRequest {
            client: &self,
            container_id: args.container_id.to_owned(),
            fields: args.fields,
            name: args.name.to_owned(),
            parent_storage_id: args.parent_storage_id.to_owned(),
            project_id: None,
            quantity: args.quantity,
            volume: args.volume,
        }
    }
    ///List a container's contents
    pub fn list_container_contents(
        &self,
        container_id: &str,
    ) -> request::ListContainerContentsRequest {
        request::ListContainerContentsRequest {
            client: &self,
            container_id: container_id.to_owned(),
        }
    }
    ///Get a container content
    pub fn get_container_content(
        &self,
        container_id: &str,
        containable_id: &str,
    ) -> request::GetContainerContentRequest {
        request::GetContainerContentRequest {
            client: &self,
            container_id: container_id.to_owned(),
            containable_id: containable_id.to_owned(),
        }
    }
    ///Delete a container content
    pub fn delete_container_content(
        &self,
        container_id: &str,
        containable_id: &str,
    ) -> request::DeleteContainerContentRequest {
        request::DeleteContainerContentRequest {
            client: &self,
            container_id: container_id.to_owned(),
            containable_id: containable_id.to_owned(),
        }
    }
    ///Update a container content
    pub fn update_container_content(
        &self,
        container_id: &str,
        containable_id: &str,
        concentration: Measurement,
    ) -> request::UpdateContainerContentRequest {
        request::UpdateContainerContentRequest {
            client: &self,
            container_id: container_id.to_owned(),
            containable_id: containable_id.to_owned(),
            concentration,
        }
    }
    /**Transfer into container

Transfers a volume of an entity, batch, or container into a destination container.
Transfering a volume is cumulative with the existing destination container's contents. To transfer an entire container's contents, the sourceContainerId should be specified. To otherwise transfer multiple entities within a container, you can make multiple calls to this endpoint, specifying a single entity with each call.
*/
    pub fn transfer_into_container(
        &self,
        destination_container_id: &str,
    ) -> request::TransferIntoContainerRequest {
        request::TransferIntoContainerRequest {
            client: &self,
            destination_container_id: destination_container_id.to_owned(),
        }
    }
    ///Archive containers
    pub fn archive_containers(
        &self,
        container_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveContainersRequest {
        request::ArchiveContainersRequest {
            client: &self,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
            should_remove_barcodes: None,
        }
    }
    /**Bulk create containers. Limit of 1000 containers per request.

Bulk create containers*/
    pub fn bulk_create_containers(
        &self,
        containers: Vec<ContainerCreate>,
    ) -> request::BulkCreateContainersRequest {
        request::BulkCreateContainersRequest {
            client: &self,
            containers,
        }
    }
    /**Bulk get a set of containers

Bulk get a set of containers. Provide either containerIds or barcodes, not both.*/
    pub fn bulk_get_containers(&self) -> request::BulkGetContainersRequest {
        request::BulkGetContainersRequest {
            client: &self,
            container_ids: None,
            barcodes: None,
        }
    }
    ///Bulk update containers
    pub fn bulk_update_containers(
        &self,
        containers: Vec<ContainerBulkUpdateItem>,
    ) -> request::BulkUpdateContainersRequest {
        request::BulkUpdateContainersRequest {
            client: &self,
            containers,
        }
    }
    /**Check in containers

Check in containers to signify that they are available for use.*/
    pub fn checkin_containers(
        &self,
        container_ids: &[&str],
    ) -> request::CheckinContainersRequest {
        request::CheckinContainersRequest {
            client: &self,
            comments: None,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Check out containers

Check out containers to signify that they are in use.*/
    pub fn checkout_containers(
        &self,
        assignee_id: &str,
        container_ids: &[&str],
    ) -> request::CheckoutContainersRequest {
        request::CheckoutContainersRequest {
            client: &self,
            assignee_id: assignee_id.to_owned(),
            comment: None,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Print labels
    pub fn print_labels(
        &self,
        container_ids: &[&str],
        label_template_id: &str,
        printer_id: &str,
    ) -> request::PrintLabelsRequest {
        request::PrintLabelsRequest {
            client: &self,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
            label_template_id: label_template_id.to_owned(),
            printer_id: printer_id.to_owned(),
        }
    }
    /**Reserve containers

Reserve containers to signify that someone plans to use the containers.*/
    pub fn reserve_containers(
        &self,
        assignee_id: &str,
        container_ids: &[&str],
    ) -> request::ReserveContainersRequest {
        request::ReserveContainersRequest {
            client: &self,
            assignee_id: assignee_id.to_owned(),
            comment: None,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Unarchive containers
    pub fn unarchive_containers(
        &self,
        container_ids: &[&str],
    ) -> request::UnarchiveContainersRequest {
        request::UnarchiveContainersRequest {
            client: &self,
            container_ids: container_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List custom entities
    pub fn list_custom_entities(&self) -> request::ListCustomEntitiesRequest {
        request::ListCustomEntitiesRequest {
            client: &self,
            next_token: None,
            page_size: None,
            sort: None,
            modified_at: None,
            name: None,
            returning: None,
            name_includes: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            entity_registry_ids_any_of: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create a custom entity
    pub fn create_custom_entity(
        &self,
        args: request::CreateCustomEntityRequired,
    ) -> request::CreateCustomEntityRequest {
        request::CreateCustomEntityRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get a custom entity
    pub fn get_custom_entity(
        &self,
        custom_entity_id: &str,
    ) -> request::GetCustomEntityRequest {
        request::GetCustomEntityRequest {
            client: &self,
            custom_entity_id: custom_entity_id.to_owned(),
        }
    }
    ///Update a custom entity
    pub fn update_custom_entity(
        &self,
        args: request::UpdateCustomEntityRequired,
    ) -> request::UpdateCustomEntityRequest {
        request::UpdateCustomEntityRequest {
            client: &self,
            custom_entity_id: args.custom_entity_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
        }
    }
    ///Archive custom entities
    pub fn archive_custom_entities(
        &self,
        custom_entity_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveCustomEntitiesRequest {
        request::ArchiveCustomEntitiesRequest {
            client: &self,
            custom_entity_ids: custom_entity_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Bulk Create custom entities

Bulk Create custom entities. Limit of 2500 custom entities per request.*/
    pub fn bulk_create_custom_entities(
        &self,
        custom_entities: Vec<CustomEntityBulkCreate>,
    ) -> request::BulkCreateCustomEntitiesRequest {
        request::BulkCreateCustomEntitiesRequest {
            client: &self,
            custom_entities,
        }
    }
    ///Bulk get custom entities by ID
    pub fn bulk_get_custom_entities(
        &self,
        custom_entity_ids: &str,
    ) -> request::BulkGetCustomEntitiesRequest {
        request::BulkGetCustomEntitiesRequest {
            client: &self,
            custom_entity_ids: custom_entity_ids.to_owned(),
        }
    }
    ///Bulk Update custom entities
    pub fn bulk_update_custom_entities(
        &self,
        custom_entities: Vec<CustomEntityBulkUpdate>,
    ) -> request::BulkUpdateCustomEntitiesRequest {
        request::BulkUpdateCustomEntitiesRequest {
            client: &self,
            custom_entities,
        }
    }
    ///Unarchive custom entities
    pub fn unarchive_custom_entities(
        &self,
        custom_entity_ids: &[&str],
    ) -> request::UnarchiveCustomEntitiesRequest {
        request::UnarchiveCustomEntitiesRequest {
            client: &self,
            custom_entity_ids: custom_entity_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List DNA Alignments
    pub fn list_dna_alignments(&self) -> request::ListDnaAlignmentsRequest {
        request::ListDnaAlignmentsRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ids: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            sequence_ids: None,
        }
    }
    ///Get a DNA Alignment
    pub fn get_dna_alignment(
        &self,
        dna_alignment_id: &str,
    ) -> request::GetDnaAlignmentRequest {
        request::GetDnaAlignmentRequest {
            client: &self,
            dna_alignment_id: dna_alignment_id.to_owned(),
        }
    }
    ///Delete a DNA Alignment
    pub fn delete_dna_alignment(
        &self,
        dna_alignment_id: &str,
    ) -> request::DeleteDnaAlignmentRequest {
        request::DeleteDnaAlignmentRequest {
            client: &self,
            dna_alignment_id: dna_alignment_id.to_owned(),
        }
    }
    ///Create a consensus DNA alignment
    pub fn create_dna_consensus_alignment(
        &self,
        args: request::CreateDnaConsensusAlignmentRequired,
    ) -> request::CreateDnaConsensusAlignmentRequest {
        request::CreateDnaConsensusAlignmentRequest {
            client: &self,
            algorithm: args.algorithm.to_owned(),
            files: args.files,
            name: args.name.to_owned(),
            new_sequence: args.new_sequence,
            sequence_id: args.sequence_id.to_owned(),
        }
    }
    ///Create a template DNA alignment
    pub fn create_dna_template_alignment(
        &self,
        args: request::CreateDnaTemplateAlignmentRequired,
    ) -> request::CreateDnaTemplateAlignmentRequest {
        request::CreateDnaTemplateAlignmentRequest {
            client: &self,
            algorithm: args.algorithm.to_owned(),
            files: args.files,
            name: args.name.to_owned(),
            template_sequence_id: args.template_sequence_id.to_owned(),
        }
    }
    ///List DNA Oligos
    pub fn list_dna_oligos(&self) -> request::ListDnaOligosRequest {
        request::ListDnaOligosRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            bases: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create a DNA Oligo
    pub fn create_dna_oligo(
        &self,
        args: request::CreateDnaOligoRequired,
    ) -> request::CreateDnaOligoRequest {
        request::CreateDnaOligoRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get a DNA Oligo
    pub fn get_dna_oligo(&self, oligo_id: &str) -> request::GetDnaOligoRequest {
        request::GetDnaOligoRequest {
            client: &self,
            oligo_id: oligo_id.to_owned(),
        }
    }
    ///Update a DNA Oligo
    pub fn update_dna_oligo(
        &self,
        args: request::UpdateDnaOligoRequired,
    ) -> request::UpdateDnaOligoRequest {
        request::UpdateDnaOligoRequest {
            client: &self,
            oligo_id: args.oligo_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Archive DNA Oligos
    pub fn archive_dna_oligos(
        &self,
        dna_oligo_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveDnaOligosRequest {
        request::ArchiveDnaOligosRequest {
            client: &self,
            dna_oligo_ids: dna_oligo_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Bulk Create DNA Oligos

Bulk Create DNA Oligos. Limit of 1000 DNA Oligos per request.*/
    pub fn bulk_create_dna_oligos(&self) -> request::BulkCreateDnaOligosRequest {
        request::BulkCreateDnaOligosRequest {
            client: &self,
            dna_oligos: None,
        }
    }
    ///Bulk Update DNA Oligos
    pub fn bulk_update_dna_oligos(&self) -> request::BulkUpdateDnaOligosRequest {
        request::BulkUpdateDnaOligosRequest {
            client: &self,
            dna_oligos: None,
        }
    }
    ///Unarchive DNA Oligos
    pub fn unarchive_dna_oligos(
        &self,
        dna_oligo_ids: &[&str],
    ) -> request::UnarchiveDnaOligosRequest {
        request::UnarchiveDnaOligosRequest {
            client: &self,
            dna_oligo_ids: dna_oligo_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List DNA sequences
    pub fn list_dna_sequences(&self) -> request::ListDnaSequencesRequest {
        request::ListDnaSequencesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            bases: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create a DNA sequence
    pub fn create_dna_sequence(
        &self,
        args: request::CreateDnaSequenceRequired,
    ) -> request::CreateDnaSequenceRequest {
        request::CreateDnaSequenceRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            is_circular: args.is_circular,
            name: args.name.to_owned(),
            primers: args.primers,
            schema_id: args.schema_id.to_owned(),
            translations: args.translations,
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get a DNA sequence
    pub fn get_dna_sequence(
        &self,
        dna_sequence_id: &str,
    ) -> request::GetDnaSequenceRequest {
        request::GetDnaSequenceRequest {
            client: &self,
            dna_sequence_id: dna_sequence_id.to_owned(),
        }
    }
    ///Update a DNA sequence
    pub fn update_dna_sequence(
        &self,
        args: request::UpdateDnaSequenceRequired,
    ) -> request::UpdateDnaSequenceRequest {
        request::UpdateDnaSequenceRequest {
            client: &self,
            dna_sequence_id: args.dna_sequence_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            is_circular: args.is_circular,
            name: args.name.to_owned(),
            primers: args.primers,
            schema_id: args.schema_id.to_owned(),
            translations: args.translations,
            entity_registry_id: args.entity_registry_id.to_owned(),
        }
    }
    ///Archive DNA sequences
    pub fn archive_dna_sequences(
        &self,
        dna_sequence_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveDnaSequencesRequest {
        request::ArchiveDnaSequencesRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Auto-annotate DNA sequences with matching features from specified Feature Libraries
    pub fn auto_annotate_dna_sequences(
        &self,
        dna_sequence_ids: &[&str],
        feature_library_ids: &[&str],
    ) -> request::AutoAnnotateDnaSequencesRequest {
        request::AutoAnnotateDnaSequencesRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
            feature_library_ids: feature_library_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///Autofill DNA sequence parts
    pub fn autofill_dna_sequence_parts(
        &self,
        dna_sequence_ids: &[&str],
    ) -> request::AutofillDnaSequencePartsRequest {
        request::AutofillDnaSequencePartsRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Autofill DNA sequence translations
    pub fn autofill_dna_sequence_translations(
        &self,
        dna_sequence_ids: &[&str],
    ) -> request::AutofillDnaSequenceTranslationsRequest {
        request::AutofillDnaSequenceTranslationsRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Bulk Create DNA sequences

Bulk Create DNA sequences. Limit of 1000 DNA Sequences per request.*/
    pub fn bulk_create_dna_sequences(&self) -> request::BulkCreateDnaSequencesRequest {
        request::BulkCreateDnaSequencesRequest {
            client: &self,
            dna_sequences: None,
        }
    }
    ///Bulk get DNA sequences by ID
    pub fn bulk_get_dna_sequences(
        &self,
        dna_sequence_ids: &str,
    ) -> request::BulkGetDnaSequencesRequest {
        request::BulkGetDnaSequencesRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.to_owned(),
        }
    }
    ///Bulk Update DNA sequences
    pub fn bulk_update_dna_sequences(&self) -> request::BulkUpdateDnaSequencesRequest {
        request::BulkUpdateDnaSequencesRequest {
            client: &self,
            dna_sequences: None,
        }
    }
    ///Unarchive DNA sequences
    pub fn unarchive_dna_sequences(
        &self,
        dna_sequence_ids: &[&str],
    ) -> request::UnarchiveDnaSequencesRequest {
        request::UnarchiveDnaSequencesRequest {
            client: &self,
            dna_sequence_ids: dna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List dropdowns
    pub fn list_dropdowns(&self) -> request::ListDropdownsRequest {
        request::ListDropdownsRequest {
            client: &self,
            next_token: None,
            page_size: None,
        }
    }
    ///Create a dropdown
    pub fn create_dropdown(
        &self,
        name: &str,
        options: Vec<DropdownOptionCreate>,
    ) -> request::CreateDropdownRequest {
        request::CreateDropdownRequest {
            client: &self,
            name: name.to_owned(),
            options,
            registry_id: None,
        }
    }
    ///Get a dropdown
    pub fn get_dropdown(&self, dropdown_id: &str) -> request::GetDropdownRequest {
        request::GetDropdownRequest {
            client: &self,
            dropdown_id: dropdown_id.to_owned(),
        }
    }
    ///Update a dropdown
    pub fn update_dropdown(
        &self,
        dropdown_id: &str,
        options: Vec<DropdownOptionUpdate>,
    ) -> request::UpdateDropdownRequest {
        request::UpdateDropdownRequest {
            client: &self,
            dropdown_id: dropdown_id.to_owned(),
            options,
        }
    }
    /**Archive dropdown options

Archive options belonging to a dropdown*/
    pub fn archive_dropdown_options(
        &self,
        dropdown_id: &str,
    ) -> request::ArchiveDropdownOptionsRequest {
        request::ArchiveDropdownOptionsRequest {
            client: &self,
            dropdown_id: dropdown_id.to_owned(),
            dropdown_option_ids: None,
            reason: None,
        }
    }
    /**Unarchive dropdown options

Unarchive options belonging to a dropdown*/
    pub fn unarchive_dropdown_options(
        &self,
        dropdown_id: &str,
    ) -> request::UnarchiveDropdownOptionsRequest {
        request::UnarchiveDropdownOptionsRequest {
            client: &self,
            dropdown_id: dropdown_id.to_owned(),
            dropdown_option_ids: None,
        }
    }
    ///Get an entity's batches
    pub fn get_enitity_batches(
        &self,
        entity_id: &str,
    ) -> request::GetEnitityBatchesRequest {
        request::GetEnitityBatchesRequest {
            client: &self,
            entity_id: entity_id.to_owned(),
        }
    }
    ///List entity schemas
    pub fn list_entity_schemas(&self) -> request::ListEntitySchemasRequest {
        request::ListEntitySchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get an entity schema by ID
    pub fn get_entity_schema(&self, schema_id: &str) -> request::GetEntitySchemaRequest {
        request::GetEntitySchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    /**List entries

List notebook entries*/
    pub fn list_entries(&self) -> request::ListEntriesRequest {
        request::ListEntriesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            project_id: None,
            archive_reason: None,
            review_status: None,
            mentioned_in: None,
            mentions: None,
            ids: None,
            schema_id: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            assigned_reviewer_ids_any_of: None,
            creator_ids: None,
            author_ids_any_of: None,
            display_ids: None,
        }
    }
    ///Create a notebook entry
    pub fn create_entry(
        &self,
        folder_id: &str,
        name: &str,
    ) -> request::CreateEntryRequest {
        request::CreateEntryRequest {
            client: &self,
            author_ids: None,
            custom_fields: None,
            entry_template_id: None,
            fields: None,
            folder_id: folder_id.to_owned(),
            initial_tables: None,
            name: name.to_owned(),
            schema_id: None,
        }
    }
    ///Get a notebook entry by ID
    pub fn get_entry(&self, entry_id: &str) -> request::GetEntryRequest {
        request::GetEntryRequest {
            client: &self,
            entry_id: entry_id.to_owned(),
        }
    }
    ///Update a notebook entry's metadata
    pub fn update_entry(&self, entry_id: &str) -> request::UpdateEntryRequest {
        request::UpdateEntryRequest {
            client: &self,
            entry_id: entry_id.to_owned(),
            author_ids: None,
            fields: None,
            folder_id: None,
            name: None,
            schema_id: None,
        }
    }
    /**Retrieves the metadata for an external file. Use the 'downloadURL' to download the actual file.


Retrieves the metadata for an external file. Use the 'downloadURL' to download the actual file. (Expand the schema view for details)
*/
    pub fn get_external_file_metadata(
        &self,
        entry_id: &str,
        external_file_id: &str,
    ) -> request::GetExternalFileMetadataRequest {
        request::GetExternalFileMetadataRequest {
            client: &self,
            entry_id: entry_id.to_owned(),
            external_file_id: external_file_id.to_owned(),
        }
    }
    ///Archive notebook entries
    pub fn archive_entries(
        &self,
        entry_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveEntriesRequest {
        request::ArchiveEntriesRequest {
            client: &self,
            entry_ids: entry_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Get notebook entries using entry IDs or display IDs
    pub fn bulk_get_entries(&self) -> request::BulkGetEntriesRequest {
        request::BulkGetEntriesRequest {
            client: &self,
            entry_ids: None,
            display_ids: None,
        }
    }
    ///Unarchive notebook entries
    pub fn unarchive_entries(
        &self,
        entry_ids: &[&str],
    ) -> request::UnarchiveEntriesRequest {
        request::UnarchiveEntriesRequest {
            client: &self,
            entry_ids: entry_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List entry schemas
    pub fn list_entry_schemas(&self) -> request::ListEntrySchemasRequest {
        request::ListEntrySchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get an Entry schema by ID
    pub fn get_entry_schema(&self, schema_id: &str) -> request::GetEntrySchemaRequest {
        request::GetEntrySchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List entry templates
    pub fn list_entry_templates(&self) -> request::ListEntryTemplatesRequest {
        request::ListEntryTemplatesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            modified_at: None,
            name: None,
            template_collection_id: None,
            ids: None,
            schema_id: None,
        }
    }
    ///Get a notebook template entry by ID
    pub fn get_entry_template(
        &self,
        entry_template_id: &str,
    ) -> request::GetEntryTemplateRequest {
        request::GetEntryTemplateRequest {
            client: &self,
            entry_template_id: entry_template_id.to_owned(),
        }
    }
    /**List Events

List Events

## Event Sort Order

Events in Benchling are assigned a stable sort order that reflects when the event was processed (not created). The createdAt time is not the stable sorted order of events. For this reason event createdAt time may appear out of order.
*/
    pub fn list_events(&self) -> request::ListEventsRequest {
        request::ListEventsRequest {
            client: &self,
            page_size: None,
            next_token: None,
            created_at_gte: None,
            starting_after: None,
            event_types: None,
            poll: None,
        }
    }
    /**Export Item

This endpoint launches a [long-running task](#/Tasks/getTask) and returns the Task ID of the launched task.
The task response contains a link to download the exported item from Amazon S3. The download is a ZIP file that contains the exported PDFs.
*/
    pub fn export_item(&self, id: &str) -> request::ExportItemRequest {
        request::ExportItemRequest {
            client: &self,
            id: id.to_owned(),
        }
    }
    ///List Feature Libraries
    pub fn list_feature_libraries(&self) -> request::ListFeatureLibrariesRequest {
        request::ListFeatureLibrariesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ids: None,
            names_any_of: None,
        }
    }
    ///Create a Feature Library
    pub fn create_feature_library(&self) -> request::CreateFeatureLibraryRequest {
        request::CreateFeatureLibraryRequest {
            client: &self,
        }
    }
    ///Get a feature library by ID
    pub fn get_feature_library(
        &self,
        feature_library_id: &str,
    ) -> request::GetFeatureLibraryRequest {
        request::GetFeatureLibraryRequest {
            client: &self,
            feature_library_id: feature_library_id.to_owned(),
        }
    }
    /**Update a feature library

Update a feature library. Note: Features cannot be updated from this endpoint.
Use the /features* endpoints to add or modify features.
*/
    pub fn update_feature_library(
        &self,
        feature_library_id: &str,
    ) -> request::UpdateFeatureLibraryRequest {
        request::UpdateFeatureLibraryRequest {
            client: &self,
            feature_library_id: feature_library_id.to_owned(),
        }
    }
    ///List Features
    pub fn list_features(&self) -> request::ListFeaturesRequest {
        request::ListFeaturesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            name: None,
            ids: None,
            names_any_of_case_sensitive: None,
            feature_library_id: None,
            feature_type: None,
            match_type: None,
        }
    }
    ///Create a Feature
    pub fn create_feature(&self) -> request::CreateFeatureRequest {
        request::CreateFeatureRequest {
            client: &self,
        }
    }
    ///Get a feature by ID
    pub fn get_feature(&self, feature_id: &str) -> request::GetFeatureRequest {
        request::GetFeatureRequest {
            client: &self,
            feature_id: feature_id.to_owned(),
        }
    }
    ///Update a feature
    pub fn update_feature(&self, feature_id: &str) -> request::UpdateFeatureRequest {
        request::UpdateFeatureRequest {
            client: &self,
            feature_id: feature_id.to_owned(),
        }
    }
    ///Bulk create Features
    pub fn bulk_create_features(&self) -> request::BulkCreateFeaturesRequest {
        request::BulkCreateFeaturesRequest {
            client: &self,
            features: None,
        }
    }
    ///List folders
    pub fn list_folders(&self) -> request::ListFoldersRequest {
        request::ListFoldersRequest {
            client: &self,
            next_token: None,
            page_size: None,
            sort: None,
            archive_reason: None,
            name_includes: None,
            parent_folder_id: None,
            project_id: None,
            ids: None,
            name: None,
            section: None,
        }
    }
    ///Create folder
    pub fn create_folder(
        &self,
        name: &str,
        parent_folder_id: &str,
    ) -> request::CreateFolderRequest {
        request::CreateFolderRequest {
            client: &self,
            name: name.to_owned(),
            parent_folder_id: parent_folder_id.to_owned(),
        }
    }
    ///Get a folder by ID
    pub fn get_folder(&self, folder_id: &str) -> request::GetFolderRequest {
        request::GetFolderRequest {
            client: &self,
            folder_id: folder_id.to_owned(),
        }
    }
    /**Archive folders

Archives folders and their contents*/
    pub fn archive_folders(
        &self,
        folder_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveFoldersRequest {
        request::ArchiveFoldersRequest {
            client: &self,
            folder_ids: folder_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Unarchive folders

Unarchives folders and the contents that were archived along with them*/
    pub fn unarchive_folders(
        &self,
        folder_ids: &[&str],
    ) -> request::UnarchiveFoldersRequest {
        request::UnarchiveFoldersRequest {
            client: &self,
            folder_ids: folder_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List legacy workflow stage run input samples
    pub fn list_legacy_workflow_stage_run_input_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListLegacyWorkflowStageRunInputSamplesRequest {
        request::ListLegacyWorkflowStageRunInputSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List legacy workflow stage run output samples
    pub fn list_legacy_workflow_stage_run_output_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListLegacyWorkflowStageRunOutputSamplesRequest {
        request::ListLegacyWorkflowStageRunOutputSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List legacy workflow stage run registered samples
    pub fn list_legacy_workflow_stage_run_registered_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListLegacyWorkflowStageRunRegisteredSamplesRequest {
        request::ListLegacyWorkflowStageRunRegisteredSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List legacy workflow stage runs
    pub fn list_legacy_workflow_stage_runs(
        &self,
        stage_id: &str,
    ) -> request::ListLegacyWorkflowStageRunsRequest {
        request::ListLegacyWorkflowStageRunsRequest {
            client: &self,
            stage_id: stage_id.to_owned(),
        }
    }
    ///List legacy workflows
    pub fn list_legacy_workflows(&self) -> request::ListLegacyWorkflowsRequest {
        request::ListLegacyWorkflowsRequest {
            client: &self,
        }
    }
    /**Update legacy workflow

Update workflow metadata*/
    pub fn update_legacy_workflow_metadata(
        &self,
        legacy_workflow_id: &str,
    ) -> request::UpdateLegacyWorkflowMetadataRequest {
        request::UpdateLegacyWorkflowMetadataRequest {
            client: &self,
            legacy_workflow_id: legacy_workflow_id.to_owned(),
            description: None,
            name: None,
            project_id: None,
        }
    }
    ///List legacy workflow stages
    pub fn list_legacy_workflow_stages(
        &self,
        legacy_workflow_id: &str,
    ) -> request::ListLegacyWorkflowStagesRequest {
        request::ListLegacyWorkflowStagesRequest {
            client: &self,
            legacy_workflow_id: legacy_workflow_id.to_owned(),
        }
    }
    ///List location schemas
    pub fn list_location_schemas(&self) -> request::ListLocationSchemasRequest {
        request::ListLocationSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
        }
    }
    ///Get a location schema by ID
    pub fn get_location_schema(
        &self,
        schema_id: &str,
    ) -> request::GetLocationSchemaRequest {
        request::GetLocationSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List locations
    pub fn list_locations(&self) -> request::ListLocationsRequest {
        request::ListLocationsRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            schema_id: None,
            schema_fields: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ancestor_storage_id: None,
            archive_reason: None,
            ids: None,
            barcodes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
        }
    }
    ///Create a location
    pub fn create_location(
        &self,
        name: &str,
        schema_id: &str,
    ) -> request::CreateLocationRequest {
        request::CreateLocationRequest {
            client: &self,
            barcode: None,
            fields: None,
            name: name.to_owned(),
            parent_storage_id: None,
            schema_id: schema_id.to_owned(),
        }
    }
    ///Get a location by ID
    pub fn get_location(&self, location_id: &str) -> request::GetLocationRequest {
        request::GetLocationRequest {
            client: &self,
            location_id: location_id.to_owned(),
        }
    }
    ///Update a location
    pub fn update_location(&self, location_id: &str) -> request::UpdateLocationRequest {
        request::UpdateLocationRequest {
            client: &self,
            location_id: location_id.to_owned(),
            fields: None,
            name: None,
            parent_storage_id: None,
        }
    }
    ///Archive locations
    pub fn archive_locations(
        &self,
        location_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveLocationsRequest {
        request::ArchiveLocationsRequest {
            client: &self,
            location_ids: location_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
            should_remove_barcodes: None,
        }
    }
    ///BulkGet locations
    pub fn bulk_get_locations(&self) -> request::BulkGetLocationsRequest {
        request::BulkGetLocationsRequest {
            client: &self,
            location_ids: None,
            barcodes: None,
        }
    }
    ///Unarchive locations
    pub fn unarchive_locations(
        &self,
        location_ids: &[&str],
    ) -> request::UnarchiveLocationsRequest {
        request::UnarchiveLocationsRequest {
            client: &self,
            location_ids: location_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List mixtures
    pub fn list_mixtures(&self) -> request::ListMixturesRequest {
        request::ListMixturesRequest {
            client: &self,
            next_token: None,
            page_size: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            entity_registry_ids_any_of: None,
            ingredient_component_entity_ids: None,
            ingredient_component_entity_ids_any_of: None,
            author_ids_any_of: None,
        }
    }
    /**Create a mixture

Create a mixture.
To create a new child mixture (eg. a prep) from a parent mixture (eg. a recipe), set the parent mixture field and specify the desired final state for your ingredients.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixture (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub fn create_mixture(
        &self,
        args: request::CreateMixtureRequired,
    ) -> request::CreateMixtureRequest {
        request::CreateMixtureRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            amount: args.amount.to_owned(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            custom_fields: args.custom_fields,
            entity_registry_id: args.entity_registry_id.to_owned(),
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            ingredients: args.ingredients,
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            units: None,
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get a mixture
    pub fn get_mixture(&self, mixture_id: &str) -> request::GetMixtureRequest {
        request::GetMixtureRequest {
            client: &self,
            mixture_id: mixture_id.to_owned(),
        }
    }
    /**Update a mixture

Update a mixture.
To change the parent mixture, set the parent mixture field and specify the desired final state for your ingredients.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixture (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub fn update_mixture(&self, mixture_id: &str) -> request::UpdateMixtureRequest {
        request::UpdateMixtureRequest {
            client: &self,
            mixture_id: mixture_id.to_owned(),
            aliases: None,
            amount: None,
            author_ids: None,
            custom_fields: None,
            entity_registry_id: None,
            fields: None,
            folder_id: None,
            ingredients: None,
            name: None,
            schema_id: None,
            units: None,
        }
    }
    ///Archive mixtures
    pub fn archive_mixtures(
        &self,
        mixture_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveMixturesRequest {
        request::ArchiveMixturesRequest {
            client: &self,
            mixture_ids: mixture_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Bulk Create mixtures

Create multiple mixtures. Limit of 1000 mixtures per request.
To create new child mixtures (eg. a prep) from parent mixtures (eg. a recipe), set the parent mixture field and specify the desired final state for your ingredients.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixtures (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub fn bulk_create_mixtures(
        &self,
        mixtures: Vec<MixtureCreate>,
    ) -> request::BulkCreateMixturesRequest {
        request::BulkCreateMixturesRequest {
            client: &self,
            mixtures,
        }
    }
    /**Bulk Update mixtures

Update multiple mixtures.
To change the parent mixture on your specified mixtures, set the parent mixture field and specify the desired final state for your ingredients.
Benchling will recognize that any ingredients you specify that match ingredients on the parent mixtures (based on component entity) are inherited. This can be seen on the returned `ingredients[i].hasParent` attribute.
*/
    pub fn bulk_update_mixtures(&self) -> request::BulkUpdateMixturesRequest {
        request::BulkUpdateMixturesRequest {
            client: &self,
            mixtures: None,
        }
    }
    ///Unarchive mixtures
    pub fn unarchive_mixtures(
        &self,
        mixture_ids: &[&str],
    ) -> request::UnarchiveMixturesRequest {
        request::UnarchiveMixturesRequest {
            client: &self,
            mixture_ids: mixture_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**List Molecules

List molecules*/
    pub fn list_molecules(&self) -> request::ListMoleculesRequest {
        request::ListMoleculesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            author_ids_any_of: None,
            chemical_substructure_mol: None,
            chemical_substructure_smiles: None,
        }
    }
    ///Create a Molecule
    pub fn create_molecule(
        &self,
        args: request::CreateMoleculeRequired,
    ) -> request::CreateMoleculeRequest {
        request::CreateMoleculeRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            chemical_structure: args.chemical_structure,
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get a Molecule
    pub fn get_molecule(&self, molecule_id: &str) -> request::GetMoleculeRequest {
        request::GetMoleculeRequest {
            client: &self,
            molecule_id: molecule_id.to_owned(),
        }
    }
    ///Update a Molecule
    pub fn update_molecule(
        &self,
        args: request::UpdateMoleculeRequired,
    ) -> request::UpdateMoleculeRequest {
        request::UpdateMoleculeRequest {
            client: &self,
            molecule_id: args.molecule_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            chemical_structure: args.chemical_structure,
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Archive Molecules
    pub fn archive_molecules(
        &self,
        molecule_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveMoleculesRequest {
        request::ArchiveMoleculesRequest {
            client: &self,
            molecule_ids: molecule_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    ///Bulk Create Molecules
    pub fn bulk_create_molecules(&self) -> request::BulkCreateMoleculesRequest {
        request::BulkCreateMoleculesRequest {
            client: &self,
            molecules: None,
        }
    }
    ///Bulk Update Molecules
    pub fn bulk_update_molecules(&self) -> request::BulkUpdateMoleculesRequest {
        request::BulkUpdateMoleculesRequest {
            client: &self,
            molecules: None,
        }
    }
    ///Unarchive Molecules
    pub fn unarchive_molecules(
        &self,
        molecule_ids: &[&str],
    ) -> request::UnarchiveMoleculesRequest {
        request::UnarchiveMoleculesRequest {
            client: &self,
            molecule_ids: molecule_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List Nucleotide Alignments
    pub fn list_nucleotide_alignments(
        &self,
    ) -> request::ListNucleotideAlignmentsRequest {
        request::ListNucleotideAlignmentsRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ids: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            sequence_ids: None,
        }
    }
    ///Get a Nucleotide Alignment
    pub fn get_nucleotide_alignment(
        &self,
        alignment_id: &str,
    ) -> request::GetNucleotideAlignmentRequest {
        request::GetNucleotideAlignmentRequest {
            client: &self,
            alignment_id: alignment_id.to_owned(),
        }
    }
    ///Delete a Nucleotide Alignment
    pub fn delete_nucleotide_alignment(
        &self,
        alignment_id: &str,
    ) -> request::DeleteNucleotideAlignmentRequest {
        request::DeleteNucleotideAlignmentRequest {
            client: &self,
            alignment_id: alignment_id.to_owned(),
        }
    }
    ///Create a consensus Nucleotide Alignment
    pub fn create_consensus_nucleotide_alignment(
        &self,
        args: request::CreateConsensusNucleotideAlignmentRequired,
    ) -> request::CreateConsensusNucleotideAlignmentRequest {
        request::CreateConsensusNucleotideAlignmentRequest {
            client: &self,
            algorithm: args.algorithm.to_owned(),
            files: args.files,
            name: args.name.to_owned(),
            new_sequence: args.new_sequence,
            sequence_id: args.sequence_id.to_owned(),
        }
    }
    ///Create a template Nucleotide Alignment
    pub fn create_template_nucleotide_alignment(
        &self,
        args: request::CreateTemplateNucleotideAlignmentRequired,
    ) -> request::CreateTemplateNucleotideAlignmentRequest {
        request::CreateTemplateNucleotideAlignmentRequest {
            client: &self,
            algorithm: args.algorithm.to_owned(),
            files: args.files,
            name: args.name.to_owned(),
            template_sequence_id: args.template_sequence_id.to_owned(),
        }
    }
    ///List Oligos
    pub fn list_oligos(&self) -> request::ListOligosRequest {
        request::ListOligosRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            bases: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
        }
    }
    ///Create an Oligo
    pub fn create_oligo(
        &self,
        args: request::CreateOligoRequired,
    ) -> request::CreateOligoRequest {
        request::CreateOligoRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get an Oligo
    pub fn get_oligo(&self, oligo_id: &str) -> request::GetOligoRequest {
        request::GetOligoRequest {
            client: &self,
            oligo_id: oligo_id.to_owned(),
        }
    }
    ///Update an Oligo
    pub fn update_oligo(
        &self,
        args: request::UpdateOligoRequired,
    ) -> request::UpdateOligoRequest {
        request::UpdateOligoRequest {
            client: &self,
            oligo_id: args.oligo_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Archive Oligos
    pub fn archive_oligos(
        &self,
        oligo_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveOligosRequest {
        request::ArchiveOligosRequest {
            client: &self,
            oligo_ids: oligo_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Bulk Create DNA Oligos

Bulk Create DNA Oligos
Please migrate to [Bulk Create DNA Oligos](#/DNA%20Oligos/bulkCreateDNAOligos) so that we can support RNA Oligos.
*/
    pub fn bulk_create_oligos(&self) -> request::BulkCreateOligosRequest {
        request::BulkCreateOligosRequest {
            client: &self,
            oligos: None,
        }
    }
    ///Bulk get Oligos by ID
    pub fn bulk_get_oligos(&self, oligo_ids: &str) -> request::BulkGetOligosRequest {
        request::BulkGetOligosRequest {
            client: &self,
            oligo_ids: oligo_ids.to_owned(),
        }
    }
    ///Unarchive Oligos
    pub fn unarchive_oligos(
        &self,
        oligo_ids: &[&str],
    ) -> request::UnarchiveOligosRequest {
        request::UnarchiveOligosRequest {
            client: &self,
            oligo_ids: oligo_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**List organizations

Returns all organizations that the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of the organization
*/
    pub fn list_organizations(&self) -> request::ListOrganizationsRequest {
        request::ListOrganizationsRequest {
            client: &self,
            ids: None,
            name: None,
            name_includes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            modified_at: None,
            has_members: None,
            has_admins: None,
            page_size: None,
            next_token: None,
            sort: None,
        }
    }
    /**Get an organization by ID

Returns an organization by ID if the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of the organization
*/
    pub fn get_organization(
        &self,
        organization_id: &str,
    ) -> request::GetOrganizationRequest {
        request::GetOrganizationRequest {
            client: &self,
            organization_id: organization_id.to_owned(),
        }
    }
    ///List plate schemas
    pub fn list_plate_schemas(&self) -> request::ListPlateSchemasRequest {
        request::ListPlateSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
        }
    }
    ///Get a plate schema by ID
    pub fn get_plate_schema(&self, schema_id: &str) -> request::GetPlateSchemaRequest {
        request::GetPlateSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List plates
    pub fn list_plates(&self) -> request::ListPlatesRequest {
        request::ListPlatesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            schema_id: None,
            schema_fields: None,
            modified_at: None,
            name: None,
            name_includes: None,
            ancestor_storage_id: None,
            storage_contents_id: None,
            storage_contents_ids: None,
            archive_reason: None,
            ids: None,
            barcodes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            returning: None,
            creator_ids: None,
        }
    }
    ///Create a plate
    pub fn create_plate(&self, schema_id: &str) -> request::CreatePlateRequest {
        request::CreatePlateRequest {
            client: &self,
            returning: None,
            barcode: None,
            container_schema_id: None,
            fields: None,
            name: None,
            parent_storage_id: None,
            project_id: None,
            schema_id: schema_id.to_owned(),
            wells: None,
        }
    }
    ///Get a plate
    pub fn get_plate(&self, plate_id: &str) -> request::GetPlateRequest {
        request::GetPlateRequest {
            client: &self,
            plate_id: plate_id.to_owned(),
            returning: None,
        }
    }
    ///Update a plate
    pub fn update_plate(&self, plate_id: &str) -> request::UpdatePlateRequest {
        request::UpdatePlateRequest {
            client: &self,
            plate_id: plate_id.to_owned(),
            returning: None,
            fields: None,
            name: None,
            parent_storage_id: None,
            project_id: None,
        }
    }
    /**Archive plates

Archive plates and any containers of the plates*/
    pub fn archive_plates(
        &self,
        plate_ids: &[&str],
        reason: &str,
        should_remove_barcodes: bool,
    ) -> request::ArchivePlatesRequest {
        request::ArchivePlatesRequest {
            client: &self,
            plate_ids: plate_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
            should_remove_barcodes,
        }
    }
    ///BulkGet plates
    pub fn bulk_get_plates(&self) -> request::BulkGetPlatesRequest {
        request::BulkGetPlatesRequest {
            client: &self,
            plate_ids: None,
            barcodes: None,
            returning: None,
        }
    }
    /**Unarchive plates

Unarchive plates and the containers that were archived along with them*/
    pub fn unarchive_plates(
        &self,
        plate_ids: &[&str],
    ) -> request::UnarchivePlatesRequest {
        request::UnarchivePlatesRequest {
            client: &self,
            plate_ids: plate_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List projects
    pub fn list_projects(&self) -> request::ListProjectsRequest {
        request::ListProjectsRequest {
            client: &self,
            next_token: None,
            page_size: None,
            sort: None,
            archive_reason: None,
            ids: None,
            name: None,
        }
    }
    ///Get a project by ID
    pub fn get_project(&self, project_id: &str) -> request::GetProjectRequest {
        request::GetProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
        }
    }
    /**Archive projects

Archives projects and their contents*/
    pub fn archive_projects(
        &self,
        project_ids: &[&str],
        reason: &str,
    ) -> request::ArchiveProjectsRequest {
        request::ArchiveProjectsRequest {
            client: &self,
            project_ids: project_ids.iter().map(|&x| x.to_owned()).collect(),
            reason: reason.to_owned(),
        }
    }
    /**Unarchive projects

Unarchives projects and the contents that were archived along with them*/
    pub fn unarchive_projects(
        &self,
        project_ids: &[&str],
    ) -> request::UnarchiveProjectsRequest {
        request::UnarchiveProjectsRequest {
            client: &self,
            project_ids: project_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List registries
    pub fn list_registries(&self) -> request::ListRegistriesRequest {
        request::ListRegistriesRequest {
            client: &self,
            name: None,
            modified_at: None,
        }
    }
    ///Get registry
    pub fn get_registry(&self, registry_id: &str) -> request::GetRegistryRequest {
        request::GetRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List batch schemas by registry

List batch schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_batch_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListBatchSchemasByRegistryRequest {
        request::ListBatchSchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List box schemas by registry

List box schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_box_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListBoxSchemasByRegistryRequest {
        request::ListBoxSchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List container schemas by registry

List container schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_container_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListContainerSchemasByRegistryRequest {
        request::ListContainerSchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List dropdowns for a given registry

List dropdowns by registry*/
    pub fn list_dropdowns_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListDropdownsByRegistryRequest {
        request::ListDropdownsByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List entity schemas by registry

List entity schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_entity_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListEntitySchemasByRegistryRequest {
        request::ListEntitySchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    ///List printers
    pub fn list_printers(&self, registry_id: &str) -> request::ListPrintersRequest {
        request::ListPrintersRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            name: None,
        }
    }
    ///List label templates
    pub fn list_label_templates(
        &self,
        registry_id: &str,
    ) -> request::ListLabelTemplatesRequest {
        request::ListLabelTemplatesRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            name: None,
        }
    }
    /**List location schemas by registry

List location schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_location_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListLocationSchemasByRegistryRequest {
        request::ListLocationSchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    /**List plate schemas by registry

List plate schemas by registry. Deprecated - use Schemas endpoints instead.*/
    pub fn list_plate_schemas_by_registry(
        &self,
        registry_id: &str,
    ) -> request::ListPlateSchemasByRegistryRequest {
        request::ListPlateSchemasByRegistryRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
        }
    }
    ///Bulk get registered entities
    pub fn bulk_get_registered_entities(
        &self,
        registry_id: &str,
        entity_registry_ids: &str,
    ) -> request::BulkGetRegisteredEntitiesRequest {
        request::BulkGetRegisteredEntitiesRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            entity_registry_ids: entity_registry_ids.to_owned(),
        }
    }
    /**Register entities

Attempts to move entities into the registry. Limit of 2500 entities per request. This endpoint will first check that the entities are all valid to be moved into the registry, given the namingStrategy. If any entities fail validation, no files will be moved and errors describing invalid entities is returned. If all entities pass validation, the entities are moved into the registry.
*/
    pub fn register_entities(
        &self,
        registry_id: &str,
        entity_ids: &[&str],
        naming_strategy: &str,
    ) -> request::RegisterEntitiesRequest {
        request::RegisterEntitiesRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            entity_ids: entity_ids.iter().map(|&x| x.to_owned()).collect(),
            naming_strategy: naming_strategy.to_owned(),
        }
    }
    /**Unregister entities

Unregisters entities and moves them to a folder*/
    pub fn unregister_entities(
        &self,
        registry_id: &str,
        entity_ids: &[&str],
        folder_id: &str,
    ) -> request::UnregisterEntitiesRequest {
        request::UnregisterEntitiesRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            entity_ids: entity_ids.iter().map(|&x| x.to_owned()).collect(),
            folder_id: folder_id.to_owned(),
        }
    }
    /**Validate barcodes

Validate barcodes on storage objects.*/
    pub fn validate_barcodes(
        &self,
        registry_id: &str,
        barcodes: &[&str],
    ) -> request::ValidateBarcodesRequest {
        request::ValidateBarcodesRequest {
            client: &self,
            registry_id: registry_id.to_owned(),
            barcodes: barcodes.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List Request Fulfillments
    pub fn list_request_fulfillments(
        &self,
        entry_id: &str,
    ) -> request::ListRequestFulfillmentsRequest {
        request::ListRequestFulfillmentsRequest {
            client: &self,
            entry_id: entry_id.to_owned(),
            modified_at: None,
            next_token: None,
            page_size: None,
        }
    }
    ///Get a request's fulfillment
    pub fn get_request_fulfillment(
        &self,
        request_fulfillment_id: &str,
    ) -> request::GetRequestFulfillmentRequest {
        request::GetRequestFulfillmentRequest {
            client: &self,
            request_fulfillment_id: request_fulfillment_id.to_owned(),
        }
    }
    ///List request schemas
    pub fn list_request_schemas(&self) -> request::ListRequestSchemasRequest {
        request::ListRequestSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a Request schema by ID
    pub fn get_request_schema(
        &self,
        schema_id: &str,
    ) -> request::GetRequestSchemaRequest {
        request::GetRequestSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List request task schemas
    pub fn list_request_task_schemas(&self) -> request::ListRequestTaskSchemasRequest {
        request::ListRequestTaskSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a Request Task schema by ID
    pub fn get_request_task_schema(
        &self,
        schema_id: &str,
    ) -> request::GetRequestTaskSchemaRequest {
        request::GetRequestTaskSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List requests
    pub fn list_requests(&self, schema_id: &str) -> request::ListRequestsRequest {
        request::ListRequestsRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
            request_status: None,
            min_created_time: None,
            max_created_time: None,
            next_token: None,
            page_size: None,
        }
    }
    ///Create a request
    pub fn create_request(
        &self,
        args: request::CreateRequestRequired,
    ) -> request::CreateRequestRequest {
        request::CreateRequestRequest {
            client: &self,
            assignees: args.assignees,
            fields: args.fields,
            project_id: args.project_id.to_owned(),
            requestor_id: None,
            sample_groups: args.sample_groups,
            scheduled_on: args.scheduled_on.to_owned(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Get a request by ID
    pub fn get_request(&self, request_id: &str) -> request::GetRequestRequest {
        request::GetRequestRequest {
            client: &self,
            request_id: request_id.to_owned(),
        }
    }
    ///Update a request
    pub fn patch_request(
        &self,
        args: request::PatchRequestRequired,
    ) -> request::PatchRequestRequest {
        request::PatchRequestRequest {
            client: &self,
            request_id: args.request_id.to_owned(),
            assignees: args.assignees,
            fields: args.fields,
            project_id: args.project_id.to_owned(),
            requestor_id: None,
            sample_groups: args.sample_groups,
            scheduled_on: args.scheduled_on.to_owned(),
            request_status: args.request_status.to_owned(),
        }
    }
    ///Get a request's response
    pub fn get_request_response(
        &self,
        request_id: &str,
    ) -> request::GetRequestResponseRequest {
        request::GetRequestResponseRequest {
            client: &self,
            request_id: request_id.to_owned(),
        }
    }
    ///Create tasks for a request
    pub fn bulk_create_request_tasks(
        &self,
        request_id: &str,
        tasks: Vec<RequestTasksBulkCreate>,
    ) -> request::BulkCreateRequestTasksRequest {
        request::BulkCreateRequestTasksRequest {
            client: &self,
            request_id: request_id.to_owned(),
            tasks,
        }
    }
    ///Bulk update tasks for a request
    pub fn bulk_update_request_tasks(
        &self,
        request_id: &str,
        tasks: Vec<RequestTaskBase>,
    ) -> request::BulkUpdateRequestTasksRequest {
        request::BulkUpdateRequestTasksRequest {
            client: &self,
            request_id: request_id.to_owned(),
            tasks,
        }
    }
    ///Update the status of sample groups in a request
    pub fn execute_requests_sample_groups(
        &self,
        request_id: &str,
        sample_groups: Vec<SampleGroupStatusUpdate>,
    ) -> request::ExecuteRequestsSampleGroupsRequest {
        request::ExecuteRequestsSampleGroupsRequest {
            client: &self,
            request_id: request_id.to_owned(),
            sample_groups,
        }
    }
    /**Bulk get requests

Bulk get requests by API ID or display ID*/
    pub fn bulk_get_requests(&self) -> request::BulkGetRequestsRequest {
        request::BulkGetRequestsRequest {
            client: &self,
            request_ids: None,
            display_ids: None,
        }
    }
    /**Create a transaction

Transactions allow results to be upload in multiple requests. This endpoint lets you create a transaction. You can then upload results to the transaction, abort the transaction, or commit the transaction.
*/
    pub fn create_assay_results_transaction(
        &self,
    ) -> request::CreateAssayResultsTransactionRequest {
        request::CreateAssayResultsTransactionRequest {
            client: &self,
        }
    }
    ///Create results in a transaction
    pub fn create_assay_results_in_transaction(
        &self,
        transaction_id: &str,
        assay_results: Vec<AssayResultCreate>,
    ) -> request::CreateAssayResultsInTransactionRequest {
        request::CreateAssayResultsInTransactionRequest {
            client: &self,
            transaction_id: transaction_id.to_owned(),
            assay_results,
        }
    }
    /**Abort a transaction

Aborting a transaction will discard all uploaded results.*/
    pub fn abort_assay_results_transaction(
        &self,
        transaction_id: &str,
    ) -> request::AbortAssayResultsTransactionRequest {
        request::AbortAssayResultsTransactionRequest {
            client: &self,
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**Commit a transaction

Committing a transaction will cause all results that have been uploaded to be saved and visible to others.*/
    pub fn commit_assay_results_transaction(
        &self,
        transaction_id: &str,
    ) -> request::CommitAssayResultsTransactionRequest {
        request::CommitAssayResultsTransactionRequest {
            client: &self,
            transaction_id: transaction_id.to_owned(),
        }
    }
    ///List RNA Oligos
    pub fn list_rna_oligos(&self) -> request::ListRnaOligosRequest {
        request::ListRnaOligosRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            bases: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create an RNA Oligo
    pub fn create_rna_oligo(
        &self,
        args: request::CreateRnaOligoRequired,
    ) -> request::CreateRnaOligoRequest {
        request::CreateRnaOligoRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get an RNA Oligo
    pub fn get_rna_oligo(&self, oligo_id: &str) -> request::GetRnaOligoRequest {
        request::GetRnaOligoRequest {
            client: &self,
            oligo_id: oligo_id.to_owned(),
        }
    }
    ///Update an RNA Oligo
    pub fn update_rna_oligo(
        &self,
        args: request::UpdateRnaOligoRequired,
    ) -> request::UpdateRnaOligoRequest {
        request::UpdateRnaOligoRequest {
            client: &self,
            oligo_id: args.oligo_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Archive RNA Oligos
    pub fn archive_rna_oligos(
        &self,
        reason: &str,
        rna_oligo_ids: &[&str],
    ) -> request::ArchiveRnaOligosRequest {
        request::ArchiveRnaOligosRequest {
            client: &self,
            reason: reason.to_owned(),
            rna_oligo_ids: rna_oligo_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Bulk Create RNA Oligos

Bulk Create RNA Oligos. Limit of 1000 RNA Oligos per request.*/
    pub fn bulk_create_rna_oligos(&self) -> request::BulkCreateRnaOligosRequest {
        request::BulkCreateRnaOligosRequest {
            client: &self,
            rna_oligos: None,
        }
    }
    ///Bulk Update RNA Oligos
    pub fn bulk_update_rna_oligos(&self) -> request::BulkUpdateRnaOligosRequest {
        request::BulkUpdateRnaOligosRequest {
            client: &self,
            rna_oligos: None,
        }
    }
    ///Unarchive RNA Oligos
    pub fn unarchive_rna_oligos(
        &self,
        rna_oligo_ids: &[&str],
    ) -> request::UnarchiveRnaOligosRequest {
        request::UnarchiveRnaOligosRequest {
            client: &self,
            rna_oligo_ids: rna_oligo_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List RNA sequences
    pub fn list_rna_sequences(&self) -> request::ListRnaSequencesRequest {
        request::ListRnaSequencesRequest {
            client: &self,
            page_size: None,
            next_token: None,
            sort: None,
            modified_at: None,
            name: None,
            name_includes: None,
            bases: None,
            folder_id: None,
            mentioned_in: None,
            project_id: None,
            registry_id: None,
            schema_id: None,
            schema_fields: None,
            archive_reason: None,
            mentions: None,
            ids: None,
            entity_registry_ids_any_of: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            creator_ids: None,
            author_ids_any_of: None,
        }
    }
    ///Create an RNA sequence
    pub fn create_rna_sequence(
        &self,
        args: request::CreateRnaSequenceRequired,
    ) -> request::CreateRnaSequenceRequest {
        request::CreateRnaSequenceRequest {
            client: &self,
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            is_circular: args.is_circular,
            name: args.name.to_owned(),
            primers: args.primers,
            schema_id: args.schema_id.to_owned(),
            translations: args.translations,
            entity_registry_id: args.entity_registry_id.to_owned(),
            naming_strategy: args.naming_strategy.to_owned(),
            registry_id: args.registry_id.to_owned(),
        }
    }
    ///Get an RNA sequence
    pub fn get_rna_sequence(
        &self,
        rna_sequence_id: &str,
    ) -> request::GetRnaSequenceRequest {
        request::GetRnaSequenceRequest {
            client: &self,
            rna_sequence_id: rna_sequence_id.to_owned(),
        }
    }
    ///Update an RNA sequence
    pub fn update_rna_sequence(
        &self,
        args: request::UpdateRnaSequenceRequired,
    ) -> request::UpdateRnaSequenceRequest {
        request::UpdateRnaSequenceRequest {
            client: &self,
            rna_sequence_id: args.rna_sequence_id.to_owned(),
            aliases: args.aliases.iter().map(|&x| x.to_owned()).collect(),
            annotations: args.annotations,
            author_ids: args.author_ids.iter().map(|&x| x.to_owned()).collect(),
            bases: args.bases.to_owned(),
            custom_fields: args.custom_fields,
            fields: args.fields,
            folder_id: args.folder_id.to_owned(),
            is_circular: args.is_circular,
            name: args.name.to_owned(),
            primers: args.primers,
            schema_id: args.schema_id.to_owned(),
            translations: args.translations,
            entity_registry_id: args.entity_registry_id.to_owned(),
        }
    }
    /**Archive RNA Sequences

Archive RNA Sequences. RNA sequences that are already registered will not be removed from the registry.*/
    pub fn archive_rna_sequences(
        &self,
        reason: &str,
        rna_sequence_ids: &[&str],
    ) -> request::ArchiveRnaSequencesRequest {
        request::ArchiveRnaSequencesRequest {
            client: &self,
            reason: reason.to_owned(),
            rna_sequence_ids: rna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Auto-annotate RNA sequences with matching features from specified Feature Libraries

Auto-annotate RNA sequences with matching features from specified Feature Libraries. U/T bases are treated as interchangeable in both features and sequences.*/
    pub fn auto_annotate_rna_sequences(
        &self,
        feature_library_ids: &[&str],
        rna_sequence_ids: &[&str],
    ) -> request::AutoAnnotateRnaSequencesRequest {
        request::AutoAnnotateRnaSequencesRequest {
            client: &self,
            feature_library_ids: feature_library_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
            rna_sequence_ids: rna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Autofill RNA sequence parts

Autofill parts from matching RNA Sequences with linked schemas.*/
    pub fn autofill_rna_sequence_parts(
        &self,
        rna_sequence_ids: &[&str],
    ) -> request::AutofillRnaSequencePartsRequest {
        request::AutofillRnaSequencePartsRequest {
            client: &self,
            rna_sequence_ids: rna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Autofill RNA sequence translations from Amino Acid sequences with matching schemas

Autofill RNA sequence translations*/
    pub fn autofill_rna_sequence_translations(
        &self,
        rna_sequence_ids: &[&str],
    ) -> request::AutofillRnaSequenceTranslationsRequest {
        request::AutofillRnaSequenceTranslationsRequest {
            client: &self,
            rna_sequence_ids: rna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Bulk Create RNA sequences

Bulk Create RNA sequences. Limit of 1000 RNA Sequences per request.*/
    pub fn bulk_create_rna_sequences(&self) -> request::BulkCreateRnaSequencesRequest {
        request::BulkCreateRnaSequencesRequest {
            client: &self,
            rna_sequences: None,
        }
    }
    ///Bulk get RNA sequences by ID
    pub fn bulk_get_rna_sequences(
        &self,
        rna_sequence_ids: &str,
    ) -> request::BulkGetRnaSequencesRequest {
        request::BulkGetRnaSequencesRequest {
            client: &self,
            rna_sequence_ids: rna_sequence_ids.to_owned(),
        }
    }
    ///Bulk Update RNA sequences
    pub fn bulk_update_rna_sequences(&self) -> request::BulkUpdateRnaSequencesRequest {
        request::BulkUpdateRnaSequencesRequest {
            client: &self,
            rna_sequences: None,
        }
    }
    ///Unarchive RNA sequences
    pub fn unarchive_rna_sequences(
        &self,
        rna_sequence_ids: &[&str],
    ) -> request::UnarchiveRnaSequencesRequest {
        request::UnarchiveRnaSequencesRequest {
            client: &self,
            rna_sequence_ids: rna_sequence_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Get a task by id
    pub fn get_task(&self, task_id: &str) -> request::GetTaskRequest {
        request::GetTaskRequest {
            client: &self,
            task_id: task_id.to_owned(),
        }
    }
    /**List teams

Returns all teams that the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of the team's organization
*/
    pub fn list_teams(&self) -> request::ListTeamsRequest {
        request::ListTeamsRequest {
            client: &self,
            ids: None,
            name: None,
            name_includes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            modified_at: None,
            mentioned_in: None,
            organization_id: None,
            has_members: None,
            has_admins: None,
            page_size: None,
            next_token: None,
            sort: None,
        }
    }
    /**Get a team by ID

Returns a team by ID if the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of the team's organization
*/
    pub fn get_team(&self, team_id: &str) -> request::GetTeamRequest {
        request::GetTeamRequest {
            client: &self,
            team_id: team_id.to_owned(),
        }
    }
    /**Generate a token for usage with authenticating via OAuth2 in subsequent API calls.

Generate a token*/
    pub fn generate_token(&self) -> request::GenerateTokenRequest {
        request::GenerateTokenRequest {
            client: &self,
        }
    }
    /**Transfers into containers

Transfers a volume of an entity, batch, or container into a destination container. Limit of 5000 transfers per request. Concentration of all contents in the destination container will be automatically updated based on the previous volume & concentrations of the contents in that container, the concentration of the contents being transferred in, the volume of the contents being transferred in, and the final volume of the container. If no concentration is specified, the concentration will not be tracked.
*/
    pub fn transfer_into_containers(
        &self,
        transfers: Vec<MultipleContainersTransfer>,
    ) -> request::TransferIntoContainersRequest {
        request::TransferIntoContainersRequest {
            client: &self,
            transfers,
        }
    }
    /**List users

Returns all users that the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of the user's organizations
*/
    pub fn list_users(&self) -> request::ListUsersRequest {
        request::ListUsersRequest {
            client: &self,
            ids: None,
            name: None,
            name_includes: None,
            names_any_of: None,
            names_any_of_case_sensitive: None,
            modified_at: None,
            member_of: None,
            admin_of: None,
            handles: None,
            password_last_changed_at: None,
            page_size: None,
            next_token: None,
            sort: None,
        }
    }
    /**Get a user by ID

Returns a user by ID if the caller has permission to view. The following roles have view permission:
  - tenant admins
  - members of any of the user's organizations
*/
    pub fn get_user(&self, user_id: &str) -> request::GetUserRequest {
        request::GetUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
        }
    }
    /**Create Benchling Warehouse credentials

Allows for programmatically generating credentials to connect to the Benchling warehouse. You must have a warehouse configured to access this endpoint.
The credentials will authenticate as the same user calling the API.
Note that expiresIn is required - only temporary credentials are currently allowed.
*/
    pub fn create_warehouse_credentials(
        &self,
        expires_in: i64,
    ) -> request::CreateWarehouseCredentialsRequest {
        request::CreateWarehouseCredentialsRequest {
            client: &self,
            expires_in,
        }
    }
    ///List workflow outputs
    pub fn list_workflow_outputs(&self) -> request::ListWorkflowOutputsRequest {
        request::ListWorkflowOutputsRequest {
            client: &self,
            ids: None,
            workflow_task_group_ids: None,
            workflow_task_ids: None,
            schema_id: None,
            watcher_ids: None,
            responsible_team_ids: None,
            creation_origin_ids: None,
            linked_item_ids_any_of: None,
            linked_item_ids_all_of: None,
            linked_item_ids_none_of: None,
            schema_fields: None,
            name: None,
            name_includes: None,
            creator_ids: None,
            modified_at: None,
            next_token: None,
            page_size: None,
            display_ids: None,
            archive_reason: None,
        }
    }
    ///Create a new workflow output
    pub fn create_workflow_output(
        &self,
        fields: Fields,
        workflow_task_id: &str,
    ) -> request::CreateWorkflowOutputRequest {
        request::CreateWorkflowOutputRequest {
            client: &self,
            fields,
            workflow_task_id: workflow_task_id.to_owned(),
        }
    }
    ///Get a workflow output
    pub fn get_workflow_output(
        &self,
        workflow_output_id: &str,
    ) -> request::GetWorkflowOutputRequest {
        request::GetWorkflowOutputRequest {
            client: &self,
            workflow_output_id: workflow_output_id.to_owned(),
        }
    }
    ///Update a workflow output
    pub fn update_workflow_output(
        &self,
        workflow_output_id: &str,
        fields: Fields,
    ) -> request::UpdateWorkflowOutputRequest {
        request::UpdateWorkflowOutputRequest {
            client: &self,
            workflow_output_id: workflow_output_id.to_owned(),
            fields,
        }
    }
    ///Archive one or more workflow outputs
    pub fn archive_workflow_outputs(
        &self,
        reason: &str,
        workflow_output_ids: &[&str],
    ) -> request::ArchiveWorkflowOutputsRequest {
        request::ArchiveWorkflowOutputsRequest {
            client: &self,
            reason: reason.to_owned(),
            workflow_output_ids: workflow_output_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///Bulk create new workflow outputs
    pub fn bulk_create_workflow_outputs(
        &self,
    ) -> request::BulkCreateWorkflowOutputsRequest {
        request::BulkCreateWorkflowOutputsRequest {
            client: &self,
            workflow_outputs: None,
        }
    }
    ///Bulk update workflow outputs
    pub fn bulk_update_workflow_outputs(
        &self,
    ) -> request::BulkUpdateWorkflowOutputsRequest {
        request::BulkUpdateWorkflowOutputsRequest {
            client: &self,
            workflow_outputs: None,
        }
    }
    ///Unarchive one or more workflow outputs
    pub fn unarchive_workflow_outputs(
        &self,
        workflow_output_ids: &[&str],
    ) -> request::UnarchiveWorkflowOutputsRequest {
        request::UnarchiveWorkflowOutputsRequest {
            client: &self,
            workflow_output_ids: workflow_output_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///List stage run input samples
    pub fn list_stage_run_input_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListStageRunInputSamplesRequest {
        request::ListStageRunInputSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List stage run output samples
    pub fn list_stage_run_output_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListStageRunOutputSamplesRequest {
        request::ListStageRunOutputSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List stage run registered samples
    pub fn list_stage_run_registered_samples(
        &self,
        stage_run_id: &str,
    ) -> request::ListStageRunRegisteredSamplesRequest {
        request::ListStageRunRegisteredSamplesRequest {
            client: &self,
            stage_run_id: stage_run_id.to_owned(),
        }
    }
    ///List workflow stage runs
    pub fn list_workflow_stage_runs(
        &self,
        stage_id: &str,
    ) -> request::ListWorkflowStageRunsRequest {
        request::ListWorkflowStageRunsRequest {
            client: &self,
            stage_id: stage_id.to_owned(),
        }
    }
    ///List workflow task groups
    pub fn list_workflow_task_groups(&self) -> request::ListWorkflowTaskGroupsRequest {
        request::ListWorkflowTaskGroupsRequest {
            client: &self,
            ids: None,
            schema_id: None,
            folder_id: None,
            project_id: None,
            mentioned_in: None,
            watcher_ids: None,
            execution_types: None,
            responsible_team_ids: None,
            status_ids_any_of: None,
            status_ids_none_of: None,
            status_ids_only: None,
            name: None,
            name_includes: None,
            creator_ids: None,
            modified_at: None,
            next_token: None,
            page_size: None,
            display_ids: None,
            archive_reason: None,
        }
    }
    /**Create a new workflow task group

Create a new workflow task group. If no name is specified, uses the workflow schema name and a unique incrementor separated by a single whitespace.*/
    pub fn create_workflow_task_group(
        &self,
        args: request::CreateWorkflowTaskGroupRequired,
    ) -> request::CreateWorkflowTaskGroupRequest {
        request::CreateWorkflowTaskGroupRequest {
            client: &self,
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            watcher_ids: args.watcher_ids.iter().map(|&x| x.to_owned()).collect(),
            schema_id: args.schema_id.to_owned(),
        }
    }
    ///Get a workflow task group
    pub fn get_workflow_task_group(
        &self,
        workflow_task_group_id: &str,
    ) -> request::GetWorkflowTaskGroupRequest {
        request::GetWorkflowTaskGroupRequest {
            client: &self,
            workflow_task_group_id: workflow_task_group_id.to_owned(),
        }
    }
    ///Update a workflow task group
    pub fn update_workflow_task_group(
        &self,
        args: request::UpdateWorkflowTaskGroupRequired,
    ) -> request::UpdateWorkflowTaskGroupRequest {
        request::UpdateWorkflowTaskGroupRequest {
            client: &self,
            workflow_task_group_id: args.workflow_task_group_id.to_owned(),
            folder_id: args.folder_id.to_owned(),
            name: args.name.to_owned(),
            watcher_ids: args.watcher_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///Archive one or more workflows
    pub fn archive_workflow_task_groups(
        &self,
        reason: &str,
        workflow_task_group_ids: &[&str],
    ) -> request::ArchiveWorkflowTaskGroupsRequest {
        request::ArchiveWorkflowTaskGroupsRequest {
            client: &self,
            reason: reason.to_owned(),
            workflow_task_group_ids: workflow_task_group_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///Unarchive one or more workflows
    pub fn unarchive_workflow_task_groups(
        &self,
        workflow_task_group_ids: &[&str],
    ) -> request::UnarchiveWorkflowTaskGroupsRequest {
        request::UnarchiveWorkflowTaskGroupsRequest {
            client: &self,
            workflow_task_group_ids: workflow_task_group_ids
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    ///List workflow task schemas
    pub fn list_workflow_task_schemas(&self) -> request::ListWorkflowTaskSchemasRequest {
        request::ListWorkflowTaskSchemasRequest {
            client: &self,
            next_token: None,
            page_size: None,
            modified_at: None,
        }
    }
    ///Get a workflow task schema
    pub fn get_workflow_task_schema(
        &self,
        schema_id: &str,
    ) -> request::GetWorkflowTaskSchemaRequest {
        request::GetWorkflowTaskSchemaRequest {
            client: &self,
            schema_id: schema_id.to_owned(),
        }
    }
    ///List workflow tasks
    pub fn list_workflow_tasks(&self) -> request::ListWorkflowTasksRequest {
        request::ListWorkflowTasksRequest {
            client: &self,
            ids: None,
            workflow_task_group_ids: None,
            schema_id: None,
            status_ids: None,
            assignee_ids: None,
            watcher_ids: None,
            responsible_team_ids: None,
            execution_origin_ids: None,
            execution_types: None,
            linked_item_ids_any_of: None,
            linked_item_ids_all_of: None,
            linked_item_ids_none_of: None,
            schema_fields: None,
            name: None,
            name_includes: None,
            creator_ids: None,
            scheduled_on: None,
            scheduled_on_lt: None,
            scheduled_on_lte: None,
            scheduled_on_gte: None,
            scheduled_on_gt: None,
            modified_at: None,
            next_token: None,
            page_size: None,
            display_ids: None,
            archive_reason: None,
        }
    }
    ///Create a new workflow task
    pub fn create_workflow_task(
        &self,
        args: request::CreateWorkflowTaskRequired,
    ) -> request::CreateWorkflowTaskRequest {
        request::CreateWorkflowTaskRequest {
            client: &self,
            assignee_id: args.assignee_id.to_owned(),
            fields: args.fields,
            scheduled_on: args.scheduled_on.to_owned(),
            workflow_task_group_id: args.workflow_task_group_id.to_owned(),
        }
    }
    ///Get a workflow task
    pub fn get_workflow_task(
        &self,
        workflow_task_id: &str,
    ) -> request::GetWorkflowTaskRequest {
        request::GetWorkflowTaskRequest {
            client: &self,
            workflow_task_id: workflow_task_id.to_owned(),
        }
    }
    ///Update a workflow task
    pub fn update_workflow_task(
        &self,
        workflow_task_id: &str,
    ) -> request::UpdateWorkflowTaskRequest {
        request::UpdateWorkflowTaskRequest {
            client: &self,
            workflow_task_id: workflow_task_id.to_owned(),
        }
    }
    /**Creates a new workflow task with the same fields and assignee as the provided task and creates a relationship between the two tasks


Creates a new workflow task based on the provided task*/
    pub fn copy_workflow_task(
        &self,
        workflow_task_id: &str,
    ) -> request::CopyWorkflowTaskRequest {
        request::CopyWorkflowTaskRequest {
            client: &self,
            workflow_task_id: workflow_task_id.to_owned(),
        }
    }
    ///Archive one or more workflow tasks
    pub fn archive_workflow_tasks(
        &self,
        reason: &str,
        workflow_task_ids: &[&str],
    ) -> request::ArchiveWorkflowTasksRequest {
        request::ArchiveWorkflowTasksRequest {
            client: &self,
            reason: reason.to_owned(),
            workflow_task_ids: workflow_task_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Bulk creates new workflow tasks where each new task has the same fields and assignee as one of the provided tasks and creates a relationship between the provided task and its copy


Bulk creates new workflow tasks based on the provided tasks*/
    pub fn bulk_copy_workflow_tasks(&self) -> request::BulkCopyWorkflowTasksRequest {
        request::BulkCopyWorkflowTasksRequest {
            client: &self,
            workflow_task_ids: None,
        }
    }
    ///Create one or more workflow tasks
    pub fn bulk_create_workflow_tasks(&self) -> request::BulkCreateWorkflowTasksRequest {
        request::BulkCreateWorkflowTasksRequest {
            client: &self,
            workflow_tasks: None,
        }
    }
    /**Update one or more workflow task

Update one or more workflow tasks*/
    pub fn bulk_update_workflow_tasks(&self) -> request::BulkUpdateWorkflowTasksRequest {
        request::BulkUpdateWorkflowTasksRequest {
            client: &self,
            workflow_tasks: None,
        }
    }
    ///Unarchive one or more workflow tasks
    pub fn unarchive_workflow_tasks(
        &self,
        workflow_task_ids: &[&str],
    ) -> request::UnarchiveWorkflowTasksRequest {
        request::UnarchiveWorkflowTasksRequest {
            client: &self,
            workflow_task_ids: workflow_task_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List workflows
    pub fn list_workflows(&self) -> request::ListWorkflowsRequest {
        request::ListWorkflowsRequest {
            client: &self,
        }
    }
    /**Update workflow

Update workflow metadata*/
    pub fn update_workflow_metadata(
        &self,
        workflow_id: &str,
    ) -> request::UpdateWorkflowMetadataRequest {
        request::UpdateWorkflowMetadataRequest {
            client: &self,
            workflow_id: workflow_id.to_owned(),
            description: None,
            name: None,
            project_id: None,
        }
    }
    ///List workflow stages
    pub fn list_workflow_stages(
        &self,
        workflow_id: &str,
    ) -> request::ListWorkflowStagesRequest {
        request::ListWorkflowStagesRequest {
            client: &self,
            workflow_id: workflow_id.to_owned(),
        }
    }
}
pub enum BenchlingAuthentication {
    BasicApiKeyAuth { basic_api_key_auth: String },
}
impl BenchlingAuthentication {
    pub fn from_env() -> Self {
        Self::BasicApiKeyAuth {
            basic_api_key_auth: std::env::var("BENCHLING_BASIC_API_KEY_AUTH")
                .expect("Environment variable BENCHLING_BASIC_API_KEY_AUTH is not set."),
        }
    }
}

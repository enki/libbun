use std::collections::BTreeMap;

use durable_execution_core::{
    AdmittedDurableExecutionEventObservationGrantV1,
    DurableExecutionEventObservationActionMintRequestV1,
    DurableExecutionEventObservationGrantMintRequestV1,
    DurableExecutionEventObservationGrantProjectionV1,
    DurableExecutionEventObservationSourceGrantMintRequestV1, DurableExecutionEventReplayBatch,
    EventAppendOccurredAtClock,
    admit_durable_execution_event_observation_grant_v1,
    mint_durable_execution_event_observation_grant_v1,
};
use serde_json::{Value, json};
use swarm_capability_linker_core::{
    ProviderValue, SwarmInteger, WideIntegerJsonProjectionDecodeV1,
    provider_value_to_canonical_json_v1, wide_integer_json_projection_decode_v1,
};
use swarm_provider_value_model::ProviderValueObject;
use swarm_rust_sdk_static_provider_host::{
    SelectedProviderBoundaryExecutionResultForProviderHostOwner,
    SelectedProviderBoundaryHostRequest,
};
use swarm_rust_sdk_static_provider_listing::{
    SwarmEventProductSessionOperation, swarm_event_product_session_operation_for_provider_id,
};

mod append_authority;
mod capture_handle_store;
mod journal_core;
mod product_api;
mod source_refs;
mod source_registry;
mod store_binding;
mod subscriptions;

use durable_direct_run_event_product_api_model::DirectRunEventProductApiAdmissionOperation;
use durable_direct_run_rust_sdk_event_frame_projection::{
    direct_run_event_observation_cursor_projection_v1,
    direct_run_event_observation_stream_read_result_from_replay_batch,
};
pub(in crate::direct_run) use subscriptions::DirectRunEventSubscriptionRef;
use subscriptions::{
    DirectRunEventSubscriptionRegistryEntryReadAuthority,
    admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1,
};

use append_authority::{
    DirectRunEventPublicationTransaction, DirectRunSelectedSwarmEventPublishOperation,
    direct_run_event_swarm_event_publish_transaction_for_append,
};
use capture_handle_store::{
    direct_run_ss_test_event_capture_handle_store_insert_start_index,
    direct_run_ss_test_event_capture_handle_store_lookup_start_index,
};
pub use journal_core::install_direct_run_durable_external_postgres_event_product_api_session_from_tokio_client_provider_for_root;
use journal_core::{
    DirectRunEventPublicationContinuationKind, DirectRunEventPublicationReceipt,
    DirectRunEventPublicationResult, DirectRunEventPublicationRuntimeCompletion,
    direct_run_event_publication_parking_handle_with_post_publication_terminal,
};
pub(in crate::direct_run) use journal_core::{
    DirectRunEventPublicationError, DirectRunEventPublicationParkingHandle,
    DirectRunEventPublicationParkingSummary, block_on_direct_run_event_publication_resume,
};
pub(in crate::direct_run) use product_api::direct_run_external_postgres_event_product_session_requirement_for_root;
pub(in crate::direct_run) use product_api::direct_run_external_postgres_event_product_session_requirement_json_v1;
use product_api::{
    direct_run_event_product_route_ack_subscription_ref,
    direct_run_event_product_route_rust_sdk_control_observation_stream_add_grant,
    direct_run_event_product_route_rust_sdk_control_observation_stream_close,
    direct_run_event_product_route_rust_sdk_control_observation_stream_source_control,
    direct_run_event_product_route_rust_sdk_open_observation_stream,
    direct_run_event_product_route_rust_sdk_read_observation_cursor,
    direct_run_event_product_route_rust_sdk_read_observation_source_registry,
    direct_run_event_product_route_rust_sdk_read_observation_stream,
    direct_run_event_product_route_ss_test_event_capture_head_read,
    direct_run_event_product_route_ss_test_event_capture_range_read,
};

pub(in crate::direct_run) struct DirectRunEventProductOwner;

type DirectRunTerminalEventJsonForbidden =
    swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary;
type DirectRunEventProductPublicProjectionValue = Value;

pub(in crate::direct_run) struct DirectRunEventPublicationAppendReceiptFacts {
    pub(in crate::direct_run) retention_epoch: String,
    pub(in crate::direct_run) min_global_seq: u64,
    pub(in crate::direct_run) max_global_seq: u64,
    pub(in crate::direct_run) event_count: u64,
}

pub(in crate::direct_run) struct DirectRunTerminalEventPublicationOutcome;

#[must_use = "event-publication backend-output observations must be carried to the reporting boundary or deliberately consumed"]
#[derive(Debug, PartialEq, Eq)]
pub(in crate::direct_run) struct DirectRunEventPublicationBackendOutputDrainObservationBundle {
    observations: Vec<Value>,
}

pub(in crate::direct_run) enum DirectRunTerminalEventPublicationRuntimeOutcome {
    Completed(DirectRunTerminalEventPublicationOutcome),
    RequiresDurableResume(DirectRunEventPublicationParkingHandle),
}

impl DirectRunEventPublicationBackendOutputDrainObservationBundle {
    pub(in crate::direct_run) fn empty_for_direct_run_event_publication_owner_v1() -> Self {
        Self {
            observations: Vec::new(),
        }
    }

    pub(in crate::direct_run) fn from_provider_drive_output_effect_drain_receipts_for_direct_run_event_publication_owner_v1(
        receipts: crate::ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
    ) -> Self {
        Self::from_provider_drive_output_effect_drain_receipts_and_process_output_records_for_direct_run_event_publication_owner_v1(receipts)
            .0
    }

    pub(in crate::direct_run) fn from_provider_drive_output_effect_drain_receipts_and_process_output_records_for_direct_run_event_publication_owner_v1(
        receipts: crate::ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
    ) -> (
        Self,
        crate::ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    ) {
        let (observations, process_output_records) = receipts
            .into_swarm_io_stream_observations_and_process_output_records_for_direct_run_output_settlement_owner_v1();
        (Self { observations }, process_output_records)
    }

    pub(in crate::direct_run) fn into_observations_for_process_session_public_output_owner_v1(
        self,
    ) -> Vec<Value> {
        self.observations
    }

    pub(in crate::direct_run) fn is_empty_for_process_session_public_output_owner_v1(
        &self,
    ) -> bool {
        self.observations.is_empty()
    }

    pub(in crate::direct_run) fn from_process_child_terminal_observations_for_process_kernel_owner_v1(
        observations: Vec<Value>,
    ) -> Self {
        Self { observations }
    }

    pub(in crate::direct_run) fn extend_for_direct_run_process_session_result_owner_v1(
        &mut self,
        other: Self,
    ) {
        self.observations.extend(other.observations);
    }
}

impl DirectRunEventProductOwner {
    pub(in crate::direct_run) fn require_launch_product_store_preserve_binding(
        child_root_scope_id: &str,
        operation: DirectRunEventStoreBindingAccessOperation,
    ) -> Result<(), String> {
        store_binding::require_direct_run_event_store_binding_for_launch_product_store_preserve(
            child_root_scope_id,
            operation,
        )
    }

    pub(in crate::direct_run::event) fn classify_publication_transaction_for_runtime_completion(
        transaction: DirectRunEventPublicationTransaction,
        continuation_kind: DirectRunEventPublicationContinuationKind,
    ) -> DirectRunEventPublicationResult<DirectRunEventPublicationRuntimeCompletion> {
        journal_core::classify_direct_run_event_publication_transaction_for_runtime_completion(
            transaction,
            continuation_kind,
        )
    }

    pub(in crate::direct_run::event) fn complete_publication_transaction_for_current_runtime(
        transaction: DirectRunEventPublicationTransaction,
        continuation_kind: DirectRunEventPublicationContinuationKind,
    ) -> DirectRunEventPublicationResult<DirectRunEventPublicationReceipt> {
        journal_core::classify_direct_run_event_publication_transaction_for_runtime_completion(
            transaction,
            continuation_kind,
        )?
        .into_completed_for_current_runtime()
    }

    pub(in crate::direct_run::event) fn drive_publication_runtime_completion_with_installed_driver(
        completion: DirectRunEventPublicationRuntimeCompletion,
    ) -> DirectRunEventPublicationResult<DirectRunEventPublicationReceipt> {
        completion.drive_with_installed_durable_product_session()
    }

    pub(in crate::direct_run::event) fn primary_append_receipt_facts(
        publication_receipt: &DirectRunEventPublicationReceipt,
    ) -> DirectRunEventPublicationResult<DirectRunEventPublicationAppendReceiptFacts> {
        let receipt = publication_receipt.primary_append_receipt()?;
        Ok(receipt.facts_for_direct_run_event_publication_owner_v1())
    }

    pub(in crate::direct_run) fn publication_error_diagnostic_value(
        error: &DirectRunEventPublicationError,
    ) -> Value {
        error.diagnostic_value()
    }

    pub(in crate::direct_run) fn rust_sdk_mint_object_source_ref_for_provider_effect(
        object_kind: &str,
        object_id: &str,
    ) -> Result<String, String> {
        source_refs::direct_run_event_object_source_ref_for_kind(
            object_kind,
            object_id,
            "rust_sdk_mint_object_source_ref_provider_execution",
        )
    }

    /// Kernel-owned product-session event route (`direct_run.provider_execution_authority`).
    /// The provider-resume drive sends every host-admitted request whose provider
    /// satisfies `swarm_event_provider_requires_product_session_boundary` here:
    /// the static provider host map cannot execute this family because the
    /// journal store binding is minted per-run inside the kernel. Selected work
    /// reaches the append wall or settles a typed fault.
    pub(in crate::direct_run) fn execute_selected_product_session_provider_effect_for_direct_run_provider_resume_owner_v1(
        root_scope_id: &str,
        session_id: &str,
        node_id: &str,
        request: SelectedProviderBoundaryHostRequest,
        run_occurred_at_clock: &EventAppendOccurredAtClock,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let Some(operation) =
            swarm_event_product_session_operation_for_provider_id(request.provider_id())
        else {
            return Err(json!({
                "kind": "direct_run_product_session_event_operation_admission_drift",
                "reason": "the product-session boundary predicate admitted this provider to the kernel-owned event route, but the capability SDK catalogue names no finite product-session operation for it",
                "provider_id": request.provider_id(),
            })
            .to_string());
        };
        match operation {
            SwarmEventProductSessionOperation::PublishEvent => {
                Self::execute_rust_sdk_publish_event_provider_effect(
                    root_scope_id,
                    session_id,
                    node_id,
                    request,
                    run_occurred_at_clock,
                )
            }
            SwarmEventProductSessionOperation::SsTestEventCaptureHeadRead => {
                Self::execute_ss_test_event_capture_provider_effect(root_scope_id, request)
            }
            SwarmEventProductSessionOperation::SsTestEventCaptureRangeRead => {
                Self::execute_ss_test_event_read_provider_effect(root_scope_id, request)
            }
            SwarmEventProductSessionOperation::MintObservationGrant => {
                Self::execute_swarm_event_mint_observation_grant_provider_effect(request)
            }
            SwarmEventProductSessionOperation::ReadObservationStream => {
                Self::execute_ss_test_event_read_observation_stream_provider_effect(request)
            }
            SwarmEventProductSessionOperation::ControlObservationStream => {
                Self::execute_ss_test_event_control_observation_stream_provider_effect(request)
            }
            SwarmEventProductSessionOperation::ReadObservationSourceRegistry => {
                Self::execute_ss_test_event_read_observation_source_registry_provider_effect(request)
            }
            SwarmEventProductSessionOperation::ReadObservationCursor => {
                Self::execute_ss_test_event_read_observation_cursor_provider_effect(request)
            }
            SwarmEventProductSessionOperation::OpenObservationStream => {
                Self::execute_ss_test_event_open_observation_stream_provider_effect(
                    root_scope_id,
                    request,
                )
            }
            other => Err(json!({
                "kind": "direct_run_product_session_event_operation_consumption_pending",
                "reason": "this operation is admitted to the kernel-owned EventJournalProductSession route, but its finite kernel consumption is not built yet; selected work settles as this typed fault instead of a static-executor installation wall",
                "operation": other.as_str(),
                "provider_id": request.provider_id(),
                "backend_session_authority": "EventJournalProductSession",
                "public_host_route": "direct_run.provider_execution_authority",
            })
            .to_string()),
        }
    }

    fn execute_rust_sdk_publish_event_provider_effect(
        root_scope_id: &str,
        session_id: &str,
        node_id: &str,
        request: SelectedProviderBoundaryHostRequest,
        run_occurred_at_clock: &EventAppendOccurredAtClock,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let selected_publish = swarm_event_publish_append_command_from_provider_input(
            root_scope_id,
            session_id,
            node_id,
            request.provider_input(),
            run_occurred_at_clock,
        )?;
        // R41054: the receipt is the checked-law intent echo
        // (swarm.event.publish.receipt.v1); it must be minted from the same
        // admitted input the journal command consumes, before that input moves.
        let receipt_output = swarm_event_publish_receipt_output_for_direct_run_event_owner_v1(
            request.provider_input(),
        )?;
        let transaction =
            direct_run_event_swarm_event_publish_transaction_for_append(selected_publish);
        let publication_receipt = Self::complete_publication_transaction_for_current_runtime(
            transaction,
            DirectRunEventPublicationContinuationKind::SwarmEventPublishPrimitive,
        )
        .map_err(|error| Self::publication_error_diagnostic_value(&error).to_string())?;
        let _facts = Self::primary_append_receipt_facts(&publication_receipt)
            .map_err(|error| Self::publication_error_diagnostic_value(&error).to_string())?;
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(receipt_output)
            .map_err(|error| error.to_string())
    }

    // R41079 seam-2 (FIX C): `event.capture()` snapshots the journal HEAD —
    // the max-available global seq H, NOT H+1. A publish after the capture
    // lands at H+1, and a strictly-after read (`seq > H`) then includes it.
    //
    // #200 SEAL (ADR-2072 :98 affine handle / :161 body-local start index): the
    // captured HEAD is recorded as the IMMUTABLE start index inside the sealed
    // body-local capture-handle store, keyed by a freshly minted opaque
    // capture_id. The returned handle carries ONLY that capture_id — the cursor
    // (after_global_seq) and schema are dropped so a caller can neither read nor
    // forge the replay window. The paired read resolves the window from the store,
    // never from caller input.
    fn execute_ss_test_event_capture_provider_effect(
        root_scope_id: &str,
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let head_global_seq =
            direct_run_event_product_route_ss_test_event_capture_head_read(root_scope_id)?;
        let capture_id =
            direct_run_ss_test_event_capture_handle_store_insert_start_index(head_global_seq)?;
        let capture = ProviderValue::Object(
            BTreeMap::from([("capture_id".to_owned(), ProviderValue::String(capture_id))]).into(),
        );
        let output = swarm_ss_test_event_ok_result_carrier(capture);
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    // R41079 seam-2: `event.read({ capture })` replays the events strictly after
    // the paired capture window and settles the captured-event batch. The read
    // routes the grant-free ss-test range-read; the sealed replay batch emits
    // its own final observation (root-admitted) which this owner transports into
    // the provider ABI return value, wrapped in the std.Result Ok carrier.
    fn execute_ss_test_event_read_provider_effect(
        root_scope_id: &str,
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        // #200 SEAL: the read consumes ONLY the opaque capture_id off the handle
        // and resolves the IMMUTABLE start index from the sealed body-local store.
        // A caller-supplied after_global_seq (forged or otherwise) is never read,
        // so the replay window cannot be widened. An unknown/foreign capture_id
        // settles a typed fault (rider b) rather than a default cursor.
        let capture_id =
            swarm_ss_test_event_read_capture_id_from_provider_input(request.provider_input())?;
        let start_index = match direct_run_ss_test_event_capture_handle_store_lookup_start_index(
            &capture_id,
        )? {
            Some(start_index) => start_index,
            None => {
                return Err(swarm_ss_test_event_read_input_fault(
                    "capture.capture_id",
                    "event.read capture handle was not minted by event.capture() in this test body; it may be forged or from another body",
                ));
            }
        };
        let batch = direct_run_event_product_route_ss_test_event_capture_range_read(
            root_scope_id,
            start_index,
        )?;
        let mut observation = batch
            .consume_into_direct_run_ss_test_captured_event_batch_observation_v1(root_scope_id)
            .map_err(|error| error.to_string())?;
        // The batch observation carries a raw-cursor capture_id derived from the
        // start index; overwrite it with the opaque handle capture_id so the
        // caller sees the same sealed id it captured (batch.capture_id ==
        // handle.capture_id) and never the underlying cursor.
        if let Value::Object(fields) = &mut observation {
            fields.insert("capture_id".to_owned(), Value::String(capture_id));
        }
        let output = swarm_ss_test_event_ok_result_carrier(
            provider_value_from_ss_test_event_observation_json(&observation)?,
        );
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    // R41120: mintObservationGrant records the sealed grant KERNEL-SIDE. durable-execution-core owns
    // the mint + the volatile grantRef->grant registry + admit-by-ref; this host was input-admission
    // only. Decode the mint request from provider input, delegate to the grant TYPE owner to validate,
    // derive the content-addressed grantRef, and RECORD the sealed grant, then transport the owner-final
    // projection (OBS, no authority) to the provider ABI. open's admit-by-ref later locates THIS record
    // (both run kernel-side under the ADR-2116 current-thread substrate — same thread_local).
    fn execute_swarm_event_mint_observation_grant_provider_effect(
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let mint_request = swarm_event_mint_observation_grant_request_from_provider_input(
            request.provider_input(),
        )?;
        let projection =
            mint_durable_execution_event_observation_grant_v1(mint_request).map_err(|error| {
                json!({
                    "kind": "direct_run_event_observation_grant_mint_admission_failed",
                    "reason": error.to_string(),
                })
                .to_string()
            })?;
        let output = provider_value_from_event_observation_grant_projection(projection);
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    // R41120: openObservationStream mints the observation-stream handle. Decode the passed grant's opaque
    // grantRef, admit-by-ref against durable-execution-core's record (registry presence = non-forgeability;
    // a forged/unrecorded grantRef fails typed), route the landed open executor to create the sealed
    // subscription, and emit the handle OBS. The streamRef it returns is the token the R41116 read/control
    // arms admit downstream (round-trip closed).
    fn execute_ss_test_event_open_observation_stream_provider_effect(
        root_scope_id: &str,
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let provider_input = request.provider_input();
        let observation_grant =
            swarm_event_observation_admit_grant_from_provider_input(provider_input)?;
        let consumer_key =
            swarm_event_observation_required_string_from_input(provider_input, "consumerKey")?;
        let consumer_instance_id = swarm_event_observation_required_string_from_input(
            provider_input,
            "consumerInstanceId",
        )?;
        let default_replay_limit =
            swarm_event_observation_replay_limit_from_provider_input(provider_input)?;
        let after_global_seq =
            swarm_event_observation_after_global_seq_optional_from_provider_input(provider_input)?;
        let root_source_ref = observation_grant
            .root_source_ref_for_direct_run_event_product_owner_v1()
            .to_owned();
        let subscription_ref = Self::rust_sdk_open_observation_stream(
            root_scope_id,
            observation_grant,
            consumer_key.clone(),
            consumer_instance_id.clone(),
            default_replay_limit,
            after_global_seq,
        )?;
        let stream_ref = subscription_ref
            .subscription_id_for_direct_run_event_product_owner_v1()
            .to_owned();
        let handle = ProviderValue::Object(
            BTreeMap::from([
                (
                    "schema".to_owned(),
                    ProviderValue::String("swarm.event.observation_stream_handle.v1".to_owned()),
                ),
                ("streamRef".to_owned(), ProviderValue::String(stream_ref)),
                (
                    "rootSourceRef".to_owned(),
                    ProviderValue::String(root_source_ref),
                ),
                (
                    "consumerKey".to_owned(),
                    ProviderValue::String(consumer_key),
                ),
                (
                    "consumerInstanceId".to_owned(),
                    ProviderValue::String(consumer_instance_id),
                ),
            ])
            .into(),
        );
        let output = swarm_ss_test_event_ok_result_carrier(handle);
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    // R41116 (a-subset): the streamRef-driven observation-stream reads. Each
    // decodes the opaque streamRef handle from provider_input, LOCATES the sealed
    // subscription via admit_..._from_provider_stream_ref (registry presence =
    // non-forgeability; unrecorded streamRef fails typed), routes the landed
    // executor, and transports the owner-minted final OBS to the provider ABI.
    // Open + control/add_observation_grant stay consumption_pending (grant-gated,
    // R41120). In production all 20 files die at Open first, so these are proven
    // by cfg(test) unit tests via the test-grant builder; corpus proof rides
    // R41120's landing.
    fn execute_ss_test_event_read_observation_stream_provider_effect(
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let stream_ref = swarm_ss_test_event_observation_stream_ref_from_provider_input(
            request.provider_input(),
        )?;
        let max_frames = swarm_ss_test_event_observation_max_frames_from_provider_input(
            request.provider_input(),
        )?;
        // waitMs is a contracted optional delivery HINT (ADR-2128 note 160): it must cross the
        // finite unsigned-integer admission aperture and must not be parsed-and-ignored. This
        // synchronous read has no wait authority to consult, so the validated value is discarded.
        let _wait_ms =
            swarm_ss_test_event_observation_wait_ms_from_provider_input(request.provider_input())?;
        let subscription_ref =
            admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                &stream_ref,
                DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                    DirectRunEventProductApiAdmissionOperation::RustSdkReadObservationStream,
                ),
            )?;
        let (batch, acked_global_seq, consumer_instance_id) =
            direct_run_event_product_route_rust_sdk_read_observation_stream(
                subscription_ref,
                max_frames,
            )?;
        let observation = direct_run_event_observation_stream_read_result_from_replay_batch(
            &stream_ref,
            &consumer_instance_id,
            acked_global_seq,
            &batch,
        )?;
        let output = swarm_ss_test_event_ok_result_carrier(
            provider_value_from_ss_test_event_observation_json(&observation)?,
        );
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    fn execute_ss_test_event_read_observation_source_registry_provider_effect(
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let stream_ref = swarm_ss_test_event_observation_stream_ref_from_provider_input(
            request.provider_input(),
        )?;
        let subscription_ref =
            admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                &stream_ref,
                DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                    DirectRunEventProductApiAdmissionOperation::RustSdkReadObservationSourceRegistry,
                ),
            )?;
        let projection = direct_run_event_product_route_rust_sdk_read_observation_source_registry(
            subscription_ref,
        )?;
        let output = swarm_ss_test_event_ok_result_carrier(
            provider_value_from_ss_test_event_observation_json(&projection)?,
        );
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    fn execute_ss_test_event_read_observation_cursor_provider_effect(
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let stream_ref = swarm_ss_test_event_observation_stream_ref_from_provider_input(
            request.provider_input(),
        )?;
        let subscription_ref =
            admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                &stream_ref,
                DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                    DirectRunEventProductApiAdmissionOperation::RustSdkReadObservationCursor,
                ),
            )?;
        let cursor_global_seq =
            direct_run_event_product_route_rust_sdk_read_observation_cursor(subscription_ref)?;
        let observation =
            direct_run_event_observation_cursor_projection_v1(&stream_ref, Some(cursor_global_seq));
        let output = swarm_ss_test_event_ok_result_carrier(
            provider_value_from_ss_test_event_observation_json(&observation)?,
        );
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    fn execute_ss_test_event_control_observation_stream_provider_effect(
        request: SelectedProviderBoundaryHostRequest,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, String> {
        let stream_ref = swarm_ss_test_event_observation_stream_ref_from_provider_input(
            request.provider_input(),
        )?;
        let operation = swarm_ss_test_event_observation_operation_from_provider_input(
            request.provider_input(),
        )?;
        // R41120: add_observation_grant locates the subscription by streamRef (R41116 admit-by-streamRef),
        // admits the passed grant by ref (R41120 producer; forged/unrecorded fails typed), applies the
        // landed add-grant route (records the source into the subscription), and emits the control receipt.
        if operation == "add_observation_grant" {
            let subscription_ref =
                admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                    &stream_ref,
                    DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                        DirectRunEventProductApiAdmissionOperation::RustSdkControlObservationStream,
                    ),
                )?;
            let observation_grant =
                swarm_event_observation_admit_grant_from_provider_input(request.provider_input())?;
            Self::rust_sdk_control_observation_stream_add_grant(
                subscription_ref,
                observation_grant,
            )?;
            let receipt = ProviderValue::Object(
                BTreeMap::from([
                    (
                        "schema".to_owned(),
                        ProviderValue::String(
                            "swarm.event.observation_stream_control_receipt.v1".to_owned(),
                        ),
                    ),
                    ("streamRef".to_owned(), ProviderValue::String(stream_ref)),
                    (
                        "operation".to_owned(),
                        ProviderValue::String("add_observation_grant".to_owned()),
                    ),
                    ("accepted".to_owned(), ProviderValue::Bool(true)),
                ])
                .into(),
            );
            let output = swarm_ss_test_event_ok_result_carrier(receipt);
            return request
                .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
                .map_err(|error| error.to_string());
        }
        let subscription_ref =
            admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                &stream_ref,
                DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                    DirectRunEventProductApiAdmissionOperation::RustSdkControlObservationStream,
                ),
            )?;
        let projection = match operation.as_str() {
            "close_stream" => {
                direct_run_event_product_route_rust_sdk_control_observation_stream_close(
                    subscription_ref,
                )?
            }
            "mask_source" | "unmask_source" | "remove_source" => {
                let source_ref = swarm_ss_test_event_observation_source_ref_from_provider_input(
                    request.provider_input(),
                )?;
                let source_operation: &'static str = match operation.as_str() {
                    "mask_source" => "mask_source",
                    "unmask_source" => "unmask_source",
                    _ => "remove_source",
                };
                direct_run_event_product_route_rust_sdk_control_observation_stream_source_control(
                    subscription_ref,
                    source_ref,
                    source_operation,
                )?
            }
            _ => {
                return Err(swarm_ss_test_event_read_input_fault(
                    "operation",
                    "control_observation_stream operation must be close_stream, mask_source, unmask_source, remove_source, or add_observation_grant",
                ));
            }
        };
        let output = swarm_ss_test_event_ok_result_carrier(
            provider_value_from_ss_test_event_observation_json(&projection)?,
        );
        request
            .into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)
            .map_err(|error| error.to_string())
    }

    pub(in crate::direct_run) fn external_postgres_session_requirement_for_publication_resume(
        requirement_query: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
        _root_scope_id: &str,
    ) -> Result<swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary, String>
    {
        direct_run_external_postgres_event_product_session_requirement_for_root(
            requirement_query,
            _root_scope_id,
            DirectRunEventStoreBindingAccessOperation::EventPublicationResumeEffectExternalPostgresSessionRequirementCheck,
        )
    }

    pub(in crate::direct_run::event) fn terminal_event_publication_transaction(
        terminal: DirectRunTerminalEventJsonForbidden,
        _run_result: DirectRunTerminalEventJsonForbidden,
    ) -> Result<Option<DirectRunEventPublicationTransaction>, String> {
        match terminal {}
    }

    pub(in crate::direct_run) fn complete_terminal_event_publication_for_current_runtime(
        terminal: DirectRunTerminalEventJsonForbidden,
        _run_result: DirectRunTerminalEventJsonForbidden,
    ) -> Result<Option<DirectRunTerminalEventPublicationOutcome>, String> {
        match terminal {}
    }

    pub(in crate::direct_run) fn classify_terminal_event_publication_for_runtime_completion(
        terminal: DirectRunTerminalEventJsonForbidden,
        _run_result: DirectRunTerminalEventJsonForbidden,
    ) -> Result<Option<DirectRunTerminalEventPublicationRuntimeOutcome>, String> {
        match terminal {}
    }

    pub(in crate::direct_run::event) fn terminal_event_publication_outcome_from_receipt(
        _publication_receipt: DirectRunEventPublicationReceipt,
    ) -> DirectRunTerminalEventPublicationOutcome {
        DirectRunTerminalEventPublicationOutcome
    }

    pub(in crate::direct_run) fn parking_handle_with_post_publication_terminal(
        handle: DirectRunEventPublicationParkingHandle,
        terminal: DirectRunTerminalEventJsonForbidden,
    ) -> DirectRunEventPublicationParkingHandle {
        direct_run_event_publication_parking_handle_with_post_publication_terminal(handle, terminal)
    }

    // R41049: complete_live_primitive_transition_persist_receipts_for_current_runtime
    // deleted as dead carriage (zero live callers; the receipt product lost its
    // only mint in the 49290fca0 poison campaign). Future seam recorded in
    // docs/working/repair-queue-history/R41049.md.
    pub(in crate::direct_run) fn rust_sdk_open_observation_stream(
        root_scope_id: &str,
        observation_grant: AdmittedDurableExecutionEventObservationGrantV1,
        consumer_key: String,
        consumer_instance_id: String,
        default_replay_limit: Option<usize>,
        after_global_seq: Option<u64>,
    ) -> Result<DirectRunEventSubscriptionRef, String> {
        direct_run_event_product_route_rust_sdk_open_observation_stream(
            root_scope_id,
            observation_grant,
            consumer_key,
            consumer_instance_id,
            default_replay_limit,
            after_global_seq,
        )
    }

    pub(in crate::direct_run) fn rust_sdk_read_observation_stream(
        subscription_ref: DirectRunEventSubscriptionRef,
        max_frames: Option<usize>,
    ) -> Result<(DurableExecutionEventReplayBatch, u64, String), String> {
        direct_run_event_product_route_rust_sdk_read_observation_stream(
            subscription_ref,
            max_frames,
        )
    }

    pub(in crate::direct_run) fn rust_sdk_control_observation_stream_add_grant(
        subscription_ref: DirectRunEventSubscriptionRef,
        observation_grant: AdmittedDurableExecutionEventObservationGrantV1,
    ) -> Result<DirectRunEventProductPublicProjectionValue, String> {
        direct_run_event_product_route_rust_sdk_control_observation_stream_add_grant(
            subscription_ref,
            observation_grant,
        )
    }

    pub(in crate::direct_run) fn rust_sdk_control_observation_stream_source_control(
        subscription_ref: DirectRunEventSubscriptionRef,
        source_ref: String,
        operation: &'static str,
    ) -> Result<DirectRunEventProductPublicProjectionValue, String> {
        direct_run_event_product_route_rust_sdk_control_observation_stream_source_control(
            subscription_ref,
            source_ref,
            operation,
        )
    }

    pub(in crate::direct_run) fn rust_sdk_control_observation_stream_close(
        subscription_ref: DirectRunEventSubscriptionRef,
    ) -> Result<DirectRunEventProductPublicProjectionValue, String> {
        direct_run_event_product_route_rust_sdk_control_observation_stream_close(subscription_ref)
    }

    pub(in crate::direct_run) fn rust_sdk_read_observation_source_registry(
        subscription_ref: DirectRunEventSubscriptionRef,
    ) -> Result<DirectRunEventProductPublicProjectionValue, String> {
        direct_run_event_product_route_rust_sdk_read_observation_source_registry(subscription_ref)
    }

    pub(in crate::direct_run) fn rust_sdk_read_observation_cursor(
        subscription_ref: DirectRunEventSubscriptionRef,
    ) -> Result<u64, String> {
        direct_run_event_product_route_rust_sdk_read_observation_cursor(subscription_ref)
    }

    pub(in crate::direct_run) fn rust_sdk_ack_observation_cursor(
        subscription_ref: DirectRunEventSubscriptionRef,
        acked_global_seq: u64,
    ) -> Result<(u64, u64), String> {
        direct_run_event_product_route_ack_subscription_ref(subscription_ref, acked_global_seq)
    }
}

fn swarm_event_publish_input_fault(field: &str, reason: &str) -> String {
    json!({
        "kind": "direct_run_swarm_event_publish_input_forbidden",
        "reason": reason,
        "field": field,
    })
    .to_string()
}

fn swarm_ss_test_event_read_input_fault(field: &str, reason: &str) -> String {
    json!({
        "kind": "direct_run_ss_test_event_read_input_forbidden",
        "reason": reason,
        "field": field,
    })
    .to_string()
}

// Wraps an ss-test event provider output in the closed-sum `std.Result::Ok`
// carrier the same way publishEvent settles its receipt, so the checked call's
// result-carrier resume dual-writes the bound local.
fn swarm_ss_test_event_ok_result_carrier(payload: ProviderValue) -> ProviderValue {
    ProviderValue::Object(
        BTreeMap::from([
            (
                swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD.to_owned(),
                ProviderValue::String("std.Result".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD.to_owned(),
                ProviderValue::String("Ok".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD.to_owned(),
                ProviderValue::Bool(true),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD.to_owned(),
                payload,
            ),
        ])
        .into(),
    )
}

// #200 SEAL: `event.read({ capture })` decodes ONLY the opaque capture_id off
// the handle. The forgeable `after_global_seq` extraction is deleted — the read
// no longer consumes any caller-supplied replay cursor; the window is resolved
// from the sealed body-local store keyed by this capture_id.
fn swarm_ss_test_event_read_capture_id_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<String, String> {
    let ProviderValue::Array(arguments) = provider_input else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "event.read provider input must be a single-argument call ABI array",
        ));
    };
    let [ProviderValue::Object(fields)] = arguments.as_slice() else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "event.read provider input must carry exactly one object argument",
        ));
    };
    let Some(ProviderValue::Object(capture)) = fields.get("capture") else {
        return Err(swarm_ss_test_event_read_input_fault(
            "capture",
            "event.read requires the capture handle minted by event.capture()",
        ));
    };
    match capture.get("capture_id") {
        Some(ProviderValue::String(capture_id)) if !capture_id.is_empty() => Ok(capture_id.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            "capture.capture_id",
            "event.read capture handle must carry the opaque capture_id minted by event.capture()",
        )),
    }
}

// R41116: single-argument call-ABI object for the streamRef-driven observation
// stream ops (read/control/registry/cursor).
// R41120 mint decode: parse the provider ABI into the durable-execution-core mint request. This is
// structural parsing only (types + presence); the grant TYPE owner (durable-execution-core mint)
// validates governance (governed topics/operations, observable source kinds, root/issuer/action-target
// presence, non-empty + duplicate-source-ref) and seals. No authority is minted here.
fn swarm_event_mint_observation_grant_request_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<DurableExecutionEventObservationGrantMintRequestV1, String> {
    let ProviderValue::Array(arguments) = provider_input else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "mintObservationGrant provider input must be a single-argument call ABI array",
        ));
    };
    let [ProviderValue::Object(fields)] = arguments.as_slice() else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "mintObservationGrant provider input must carry exactly one object argument",
        ));
    };
    let root_source_ref =
        swarm_event_mint_observation_grant_required_string(fields, "rootSourceRef")?;
    let issued_by_source_ref =
        swarm_event_mint_observation_grant_required_string(fields, "issuedBySourceRef")?;
    let Some(ProviderValue::Array(sources)) = fields.get("sources") else {
        return Err(swarm_ss_test_event_read_input_fault(
            "sources",
            "mintObservationGrant provider input requires a sources array",
        ));
    };
    let mut source_requests = Vec::with_capacity(sources.len());
    for source in sources {
        let ProviderValue::Object(source_fields) = source else {
            return Err(swarm_ss_test_event_read_input_fault(
                "sources",
                "mintObservationGrant sources must be objects",
            ));
        };
        let source_ref =
            swarm_event_mint_observation_grant_required_string(source_fields, "sourceRef")?;
        let source_kind =
            swarm_event_mint_observation_grant_required_string(source_fields, "sourceKind")?;
        let topics = swarm_event_mint_observation_grant_string_array(source_fields, "topics")?;
        let operations =
            swarm_event_mint_observation_grant_string_array(source_fields, "operations")?;
        let Some(ProviderValue::Bool(include_descendants)) =
            source_fields.get("includeDescendants")
        else {
            return Err(swarm_ss_test_event_read_input_fault(
                "includeDescendants",
                "mintObservationGrant source includeDescendants must be a boolean",
            ));
        };
        let replay_policy =
            swarm_event_mint_observation_grant_required_string(source_fields, "replayPolicy")?;
        let Some(ProviderValue::Array(action_requests_input)) = source_fields.get("actionRequests")
        else {
            return Err(swarm_ss_test_event_read_input_fault(
                "actionRequests",
                "mintObservationGrant source actionRequests must be an array",
            ));
        };
        let mut action_requests = Vec::with_capacity(action_requests_input.len());
        for action in action_requests_input {
            let ProviderValue::Object(action_fields) = action else {
                return Err(swarm_ss_test_event_read_input_fault(
                    "actionRequests",
                    "mintObservationGrant actionRequests must be objects",
                ));
            };
            action_requests.push(
                DurableExecutionEventObservationActionMintRequestV1::for_static_provider_host_owner_v1(
                    swarm_event_mint_observation_grant_required_string(action_fields, "actionFamily")?,
                    swarm_event_mint_observation_grant_required_string(action_fields, "operation")?,
                    swarm_event_mint_observation_grant_required_string(
                        action_fields,
                        "targetSourceRef",
                    )?,
                ),
            );
        }
        source_requests.push(
            DurableExecutionEventObservationSourceGrantMintRequestV1::for_static_provider_host_owner_v1(
                source_ref,
                source_kind,
                topics,
                operations,
                *include_descendants,
                replay_policy,
                action_requests,
            ),
        );
    }
    Ok(
        DurableExecutionEventObservationGrantMintRequestV1::for_static_provider_host_owner_v1(
            root_source_ref,
            issued_by_source_ref,
            source_requests,
        ),
    )
}

fn swarm_event_mint_observation_grant_required_string(
    fields: &ProviderValueObject,
    field_name: &'static str,
) -> Result<String, String> {
    match fields.get(field_name) {
        Some(ProviderValue::String(value)) => Ok(value.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            field_name,
            "mintObservationGrant provider input requires this string field",
        )),
    }
}

fn swarm_event_mint_observation_grant_string_array(
    fields: &ProviderValueObject,
    field_name: &'static str,
) -> Result<Vec<String>, String> {
    let Some(ProviderValue::Array(values)) = fields.get(field_name) else {
        return Err(swarm_ss_test_event_read_input_fault(
            field_name,
            "mintObservationGrant provider input requires this string array field",
        ));
    };
    let mut strings = Vec::with_capacity(values.len());
    for value in values {
        let ProviderValue::String(value) = value else {
            return Err(swarm_ss_test_event_read_input_fault(
                field_name,
                "mintObservationGrant provider input array field must contain strings",
            ));
        };
        strings.push(value.clone());
    }
    Ok(strings)
}

// Encode the owner-final grant projection into the provider ABI value. Byte-identical to the pre-R41120
// host into_provider_value shape (schema, grantRef, rootSourceRef, issuedBySourceRef, sources[...]) so
// grant_projection + registry_delivery_ref stay green through the host->kernel routing move.
fn provider_value_from_event_observation_grant_projection(
    projection: DurableExecutionEventObservationGrantProjectionV1,
) -> ProviderValue {
    ProviderValue::Object(
        BTreeMap::from([
            (
                "schema".to_owned(),
                ProviderValue::String(projection.schema),
            ),
            (
                "grantRef".to_owned(),
                ProviderValue::String(projection.grant_ref),
            ),
            (
                "rootSourceRef".to_owned(),
                ProviderValue::String(projection.root_source_ref),
            ),
            (
                "issuedBySourceRef".to_owned(),
                ProviderValue::String(projection.issued_by_source_ref),
            ),
            (
                "sources".to_owned(),
                ProviderValue::Array(
                    projection
                        .sources
                        .into_iter()
                        .map(|source| {
                            ProviderValue::Object(
                                BTreeMap::from([
                                    (
                                        "sourceRef".to_owned(),
                                        ProviderValue::String(source.source_ref),
                                    ),
                                    (
                                        "sourceKind".to_owned(),
                                        ProviderValue::String(source.source_kind),
                                    ),
                                    (
                                        "topics".to_owned(),
                                        ProviderValue::Array(
                                            source
                                                .topics
                                                .into_iter()
                                                .map(ProviderValue::String)
                                                .collect(),
                                        ),
                                    ),
                                    (
                                        "operations".to_owned(),
                                        ProviderValue::Array(
                                            source
                                                .operations
                                                .into_iter()
                                                .map(ProviderValue::String)
                                                .collect(),
                                        ),
                                    ),
                                    (
                                        "includeDescendants".to_owned(),
                                        ProviderValue::Bool(source.include_descendants),
                                    ),
                                    (
                                        "replayPolicy".to_owned(),
                                        ProviderValue::String(source.replay_policy),
                                    ),
                                    (
                                        "actionRefs".to_owned(),
                                        ProviderValue::Array(
                                            source
                                                .action_refs
                                                .into_iter()
                                                .map(|action_ref| {
                                                    ProviderValue::Object(
                                                        BTreeMap::from([
                                                            (
                                                                "actionRef".to_owned(),
                                                                ProviderValue::String(
                                                                    action_ref.action_ref,
                                                                ),
                                                            ),
                                                            (
                                                                "actionFamily".to_owned(),
                                                                ProviderValue::String(
                                                                    action_ref.action_family,
                                                                ),
                                                            ),
                                                            (
                                                                "operation".to_owned(),
                                                                ProviderValue::String(
                                                                    action_ref.operation,
                                                                ),
                                                            ),
                                                            (
                                                                "targetSourceRef".to_owned(),
                                                                ProviderValue::String(
                                                                    action_ref.target_source_ref,
                                                                ),
                                                            ),
                                                        ])
                                                        .into(),
                                                    )
                                                })
                                                .collect(),
                                        ),
                                    ),
                                ])
                                .into(),
                            )
                        })
                        .collect(),
                ),
            ),
        ])
        .into(),
    )
}

// R41120 open/add_grant: decode the opaque grantRef off the passed observationGrant object and admit it by
// ref against durable-execution-core's record. The other grant fields the .ss passes are IGNORED — only the
// recorded ref admits; a forged/unrecorded ref fails typed (non-forgeability).
fn swarm_event_observation_admit_grant_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<AdmittedDurableExecutionEventObservationGrantV1, String> {
    let fields = swarm_ss_test_event_observation_input_fields(provider_input)?;
    let Some(ProviderValue::Object(grant_fields)) = fields.get("observationGrant") else {
        return Err(swarm_ss_test_event_read_input_fault(
            "observationGrant",
            "requires the observationGrant object minted by mintObservationGrant",
        ));
    };
    let Some(ProviderValue::String(grant_ref)) = grant_fields.get("grantRef") else {
        return Err(swarm_ss_test_event_read_input_fault(
            "observationGrant.grantRef",
            "the observationGrant must carry the opaque grantRef",
        ));
    };
    admit_durable_execution_event_observation_grant_v1(grant_ref).map_err(|error| {
        json!({
            "kind": "direct_run_event_observation_grant_not_recorded",
            "reason": error.to_string(),
        })
        .to_string()
    })
}

fn swarm_event_observation_required_string_from_input(
    provider_input: &ProviderValue,
    field_name: &'static str,
) -> Result<String, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get(field_name) {
        Some(ProviderValue::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            field_name,
            "openObservationStream requires this non-empty string field",
        )),
    }
}

fn swarm_event_observation_replay_limit_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<Option<usize>, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("replayLimit") {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::Integer(value)) => value.to_usize().map(Some).ok_or_else(|| {
            swarm_ss_test_event_read_input_fault(
                "replayLimit",
                "openObservationStream replayLimit must be a whole non-negative number",
            )
        }),
        Some(ProviderValue::Number(value)) => {
            let value = value.as_f64();
            if value.fract() != 0.0 || value < 0.0 || value >= usize::MAX as f64 {
                return Err(swarm_ss_test_event_read_input_fault(
                    "replayLimit",
                    "openObservationStream replayLimit must be a whole non-negative number",
                ));
            }
            Ok(Some(value as usize))
        }
        _ => Err(swarm_ss_test_event_read_input_fault(
            "replayLimit",
            "openObservationStream replayLimit must be a whole non-negative number",
        )),
    }
}

fn swarm_event_observation_after_global_seq_optional_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<Option<u64>, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("afterGlobalSeq") {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::Integer(value)) => value.to_u64().map(Some).ok_or_else(|| {
            swarm_ss_test_event_read_input_fault(
                "afterGlobalSeq",
                "openObservationStream afterGlobalSeq must be a whole non-negative number",
            )
        }),
        Some(ProviderValue::Number(value)) => {
            let value = value.as_f64();
            if value.fract() != 0.0 || value < 0.0 || value >= u64::MAX as f64 {
                return Err(swarm_ss_test_event_read_input_fault(
                    "afterGlobalSeq",
                    "openObservationStream afterGlobalSeq must be a whole non-negative number",
                ));
            }
            Ok(Some(value as u64))
        }
        _ => Err(swarm_ss_test_event_read_input_fault(
            "afterGlobalSeq",
            "openObservationStream afterGlobalSeq must be a whole non-negative number",
        )),
    }
}

fn swarm_ss_test_event_observation_input_fields(
    provider_input: &ProviderValue,
) -> Result<&ProviderValueObject, String> {
    let ProviderValue::Array(arguments) = provider_input else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "observation stream provider input must be a single-argument call ABI array",
        ));
    };
    let [ProviderValue::Object(fields)] = arguments.as_slice() else {
        return Err(swarm_ss_test_event_read_input_fault(
            "input",
            "observation stream provider input must carry exactly one object argument",
        ));
    };
    Ok(fields)
}

fn swarm_ss_test_event_observation_stream_ref_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<String, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("streamRef") {
        Some(ProviderValue::String(value)) => Ok(value.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            "streamRef",
            "observation stream op requires the streamRef handle minted by openObservationStream",
        )),
    }
}

fn swarm_ss_test_event_observation_operation_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<String, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("operation") {
        Some(ProviderValue::String(value)) => Ok(value.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            "operation",
            "control_observation_stream requires a string operation",
        )),
    }
}

fn swarm_ss_test_event_observation_source_ref_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<String, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("sourceRef") {
        Some(ProviderValue::String(value)) => Ok(value.clone()),
        _ => Err(swarm_ss_test_event_read_input_fault(
            "sourceRef",
            "control_observation_stream source control requires a sourceRef",
        )),
    }
}

fn swarm_ss_test_event_observation_max_frames_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<Option<usize>, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("maxFrames") {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::Integer(value)) => value.to_usize().map(Some).ok_or_else(|| {
            swarm_ss_test_event_read_input_fault(
                "maxFrames",
                "readObservationStream maxFrames must be a whole non-negative number",
            )
        }),
        Some(ProviderValue::Number(value)) => {
            let value = value.as_f64();
            if value.fract() != 0.0 || value < 0.0 || value >= usize::MAX as f64 {
                return Err(swarm_ss_test_event_read_input_fault(
                    "maxFrames",
                    "readObservationStream maxFrames must be a whole non-negative number",
                ));
            }
            Ok(Some(value as usize))
        }
        _ => Err(swarm_ss_test_event_read_input_fault(
            "maxFrames",
            "readObservationStream maxFrames must be a whole non-negative number",
        )),
    }
}

fn swarm_ss_test_event_observation_wait_ms_from_provider_input(
    provider_input: &ProviderValue,
) -> Result<Option<u64>, String> {
    match swarm_ss_test_event_observation_input_fields(provider_input)?.get("waitMs") {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::Integer(value)) => value.to_u64().map(Some).ok_or_else(|| {
            swarm_ss_test_event_read_input_fault(
                "waitMs",
                "readObservationStream waitMs must be a whole non-negative number",
            )
        }),
        Some(ProviderValue::Number(value)) => {
            let value = value.as_f64();
            if value.fract() != 0.0 || value < 0.0 || value >= u64::MAX as f64 {
                return Err(swarm_ss_test_event_read_input_fault(
                    "waitMs",
                    "readObservationStream waitMs must be a whole non-negative number",
                ));
            }
            Ok(Some(value as u64))
        }
        _ => Err(swarm_ss_test_event_read_input_fault(
            "waitMs",
            "readObservationStream waitMs must be a whole non-negative number",
        )),
    }
}

// Transports the sealed replay batch's final captured-event observation into the
// provider ABI return value. The input is already a finished observation minted
// by the durable-execution EventJournal owner (root-admitted); this is OBS
// transport only, not authority reconstruction.
fn provider_value_from_ss_test_event_observation_json(
    value: &Value,
) -> Result<ProviderValue, String> {
    provider_value_from_ss_test_event_observation_json_scoped(value, false)
}

// Rung N5: TRUE iff this object is an event-fact projection whose
// `eventSchemaVersion` marks the payload as the v3 value-schema envelope —
// only then may the `payload` subtree carry (and lawfully decode) the reserved
// wide-integer projection. v2 payloads transport VERBATIM: the version field
// discriminates; never silent reinterpretation. The version is OBS/schema
// carried on the frame itself, not authority.
fn event_fact_object_payload_carries_v3_value_schema(
    entries: &serde_json::Map<String, Value>,
) -> bool {
    matches!(
        entries.get("eventSchemaVersion"),
        Some(Value::Number(version)) if version.as_u64().is_some_and(|version| version >= 3)
    )
}

fn provider_value_from_ss_test_event_observation_json_scoped(
    value: &Value,
    wide_integer_projection_scope: bool,
) -> Result<ProviderValue, String> {
    match value {
        Value::Null => Ok(ProviderValue::Null),
        Value::Bool(value) => Ok(ProviderValue::Bool(*value)),
        // Rung N5 decode canonicalization (ruling clause 6: integral any
        // magnitude -> exact Integer): i64/u64 stay exact; a u64-range value
        // must never round through f64 (the pre-N5 silent-rounding hole);
        // integral f64 canonicalizes to the same Integer arm; non-integral
        // finite -> Number; non-finite refuses typed.
        Value::Number(number) => {
            if let Some(value) = number.as_i64() {
                Ok(ProviderValue::Integer(SwarmInteger::from_i64(value)))
            } else if let Some(value) = number.as_u64() {
                Ok(ProviderValue::Integer(SwarmInteger::from_u64(value)))
            } else if let Some(value) = number.as_f64() {
                ProviderValue::number_from_f64_canonical_v1(value).ok_or_else(|| {
                    swarm_ss_test_event_read_input_fault(
                        "events",
                        "event.read captured-event observation carried a non-finite number",
                    )
                })
            } else {
                Err(swarm_ss_test_event_read_input_fault(
                    "events",
                    "event.read captured-event observation carried an unrepresentable number",
                ))
            }
        }
        Value::String(value) => Ok(ProviderValue::String(value.clone())),
        Value::Array(values) => values
            .iter()
            .map(|value| {
                provider_value_from_ss_test_event_observation_json_scoped(
                    value,
                    wide_integer_projection_scope,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|values| ProviderValue::Array(values.into())),
        Value::Object(entries) => {
            if wide_integer_projection_scope {
                // Inside a v3 payload the reserved wide-integer projection
                // re-mints the EXACT integer (v3 admission reserves the kind,
                // so the shape provably originated from the integer encoder);
                // a malformed body is corruption and refuses typed.
                match wide_integer_json_projection_decode_v1(entries) {
                    WideIntegerJsonProjectionDecodeV1::Integer(value) => {
                        return Ok(ProviderValue::Integer(value));
                    }
                    WideIntegerJsonProjectionDecodeV1::Malformed => {
                        return Err(swarm_ss_test_event_read_input_fault(
                            "events",
                            "event.read captured-event payload carried a malformed wide-integer projection; refusing corrupted decode",
                        ));
                    }
                    WideIntegerJsonProjectionDecodeV1::NotAWideIntegerProjection => {}
                }
            }
            let payload_scope_enabled = event_fact_object_payload_carries_v3_value_schema(entries);
            entries
                .iter()
                .map(|(key, value)| {
                    let child_scope = wide_integer_projection_scope
                        || (key == "payload" && payload_scope_enabled);
                    provider_value_from_ss_test_event_observation_json_scoped(value, child_scope)
                        .map(|value| (key.clone(), value))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()
                .map(|fields| ProviderValue::Object(fields.into()))
        }
    }
}

fn require_swarm_event_publish_string(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<String, String> {
    match fields.get(field) {
        Some(ProviderValue::String(value)) if !value.trim().is_empty() => Ok(value.clone()),
        Some(ProviderValue::String(_)) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires a non-empty string",
        )),
        Some(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires a string",
        )),
        None => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires this field",
        )),
    }
}

fn optional_swarm_event_publish_string(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<Option<String>, String> {
    match fields.get(field) {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::String(value)) if !value.trim().is_empty() => Ok(Some(value.clone())),
        Some(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires a non-empty string when this field is present",
        )),
    }
}

fn swarm_event_publish_string_array(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<Vec<String>, String> {
    match fields.get(field) {
        Some(ProviderValue::Array(values)) => values
            .iter()
            .map(|value| match value {
                ProviderValue::String(value) if !value.trim().is_empty() => Ok(value.clone()),
                _ => Err(swarm_event_publish_input_fault(
                    field,
                    "publishEvent requires an array of non-empty strings",
                )),
            })
            .collect(),
        Some(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires an array of strings",
        )),
        None => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires this field",
        )),
    }
}

fn require_swarm_event_publish_json_value(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<Value, String> {
    let Some(value) = fields.get(field) else {
        return Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires this field",
        ));
    };
    let canonical = provider_value_to_canonical_json_v1(value).map_err(|error| {
        swarm_event_publish_input_fault(
            field,
            &format!("publishEvent field failed canonical JSON admission: {error}"),
        )
    })?;
    serde_json::from_str(&canonical).map_err(|error| {
        swarm_event_publish_input_fault(
            field,
            &format!("publishEvent field failed canonical JSON decode: {error}"),
        )
    })
}

fn optional_swarm_event_publish_event_index(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<Option<u64>, String> {
    match fields.get(field) {
        None | Some(ProviderValue::Null) => Ok(None),
        Some(ProviderValue::Integer(value)) => value.to_u64().map(Some).ok_or_else(|| {
            swarm_event_publish_input_fault(
                field,
                "publishEvent requires a non-negative integer when this field is present",
            )
        }),
        Some(ProviderValue::Number(value)) => {
            let value = value.as_f64();
            // u64::MAX as f64 rounds UP to exactly 2^64, so the boundary
            // comparison must be `>=`: exactly-2^64 is out of range, and
            // every integral f64 strictly below 2^64 converts exactly —
            // out-of-range integers must not saturate into source-local
            // sequence authority (R41096).
            if value.fract() != 0.0 || value < 0.0 || value >= u64::MAX as f64 {
                return Err(swarm_event_publish_input_fault(
                    field,
                    "publishEvent requires a whole number within the u64 boundary when this field is present",
                ));
            }
            Ok(Some(value as u64))
        }
        Some(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent requires an integer when this field is present",
        )),
    }
}

/// Admits one publishEvent provider input into the write-side publish append
/// command. The authored `payloadTrackHTypeContract` is deliberately not read:
/// the event journal owner mints the compiler-owned contract, so null or
/// invalid authored contracts are replaced by construction.
fn swarm_event_publish_append_command_from_provider_input(
    root_scope_id: &str,
    session_id: &str,
    node_id: &str,
    provider_input: &ProviderValue,
    run_occurred_at_clock: &EventAppendOccurredAtClock,
) -> Result<DirectRunSelectedSwarmEventPublishOperation, String> {
    let ProviderValue::Array(arguments) = provider_input else {
        return Err(swarm_event_publish_input_fault(
            "input",
            "publishEvent provider input must be a single-argument call ABI array",
        ));
    };
    let [ProviderValue::Object(fields)] = arguments.as_slice() else {
        return Err(swarm_event_publish_input_fault(
            "input",
            "publishEvent provider input must carry exactly one object argument",
        ));
    };
    // The finite event intent field map: publishEvent public contract keys
    // must be admitted here before event state can be touched. The authored
    // TSON contract types this input as jsonValue, so this admission is the
    // only finite enumeration; unknown keys refuse instead of being silently
    // ignored (R41096). payloadTrackHTypeContract is admitted-and-ignored:
    // the event journal owner replaces authored contracts by construction.
    const ADMITTED_PUBLISH_EVENT_FIELDS: [&str; 15] = [
        "sourceRef",
        "sourceKind",
        "parentSourceRefs",
        "topic",
        "operation",
        "entityType",
        "entityId",
        "scope",
        "aspects",
        "payload",
        "correlationId",
        "causationId",
        "mutationId",
        "eventIndex",
        "payloadTrackHTypeContract",
    ];
    for field in fields.keys() {
        if !ADMITTED_PUBLISH_EVENT_FIELDS.contains(&field.as_str()) {
            return Err(swarm_event_publish_input_fault(
                field,
                "publishEvent public contract keys must be admitted by the finite event intent field map before event state can be touched",
            ));
        }
    }
    let source_ref = require_swarm_event_publish_source_ref(fields, "sourceRef")?;
    let parent_source_refs = swarm_event_publish_source_ref_array(fields, "parentSourceRefs")?;
    let scope = require_swarm_event_publish_json_value(fields, "scope")?;
    refuse_swarm_event_publish_reserved_engine_scope_keys(&scope)?;
    DirectRunSelectedSwarmEventPublishOperation::admit_for_direct_run_event_owner_v1(
        root_scope_id.to_owned(),
        session_id.to_owned(),
        node_id.to_owned(),
        source_ref,
        require_swarm_event_publish_string(fields, "sourceKind")?,
        parent_source_refs,
        require_swarm_event_publish_string(fields, "topic")?,
        require_swarm_event_publish_string(fields, "operation")?,
        require_swarm_event_publish_string(fields, "entityType")?,
        require_swarm_event_publish_string(fields, "entityId")?,
        scope,
        swarm_event_publish_string_array(fields, "aspects")?,
        require_swarm_event_publish_json_value(fields, "payload")?,
        require_swarm_event_publish_string(fields, "correlationId")?,
        optional_swarm_event_publish_string(fields, "causationId")?,
        optional_swarm_event_publish_string(fields, "mutationId")?,
        optional_swarm_event_publish_event_index(fields, "eventIndex")?,
        run_occurred_at_clock.clone(),
    )
}

fn require_swarm_event_publish_source_ref(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<String, String> {
    let source_ref = require_swarm_event_publish_string(fields, field)?;
    source_refs::admit_direct_run_event_object_source_ref_for_event_source_owner(
        field,
        &source_ref,
    )
    .map_err(|reason| swarm_event_publish_input_fault(field, &reason))?;
    Ok(source_ref)
}

fn swarm_event_publish_source_ref_array(
    fields: &ProviderValueObject,
    field: &'static str,
) -> Result<Vec<String>, String> {
    let source_refs = swarm_event_publish_string_array(fields, field)?;
    for source_ref in &source_refs {
        source_refs::admit_direct_run_event_object_source_ref_for_event_source_owner(
            field, source_ref,
        )
        .map_err(|reason| swarm_event_publish_input_fault(field, &reason))?;
    }
    Ok(source_refs)
}

fn refuse_swarm_event_publish_reserved_engine_scope_keys(scope: &Value) -> Result<(), String> {
    let Some(fields) = scope.as_object() else {
        return Ok(());
    };
    for field in ["rootScopeId", "sessionId"] {
        if fields.contains_key(field) {
            return Err(swarm_event_publish_input_fault(
                "scope",
                "publishEvent authored scope cannot carry engine scope authority keys",
            ));
        }
    }
    Ok(())
}

fn duplicate_swarm_event_publish_echo_value_for_direct_run_event_owner_v1(
    field: &'static str,
    value: &ProviderValue,
) -> Result<ProviderValue, String> {
    match value {
        ProviderValue::Null => Ok(ProviderValue::Null),
        ProviderValue::Bool(value) => Ok(ProviderValue::Bool(*value)),
        ProviderValue::Integer(value) => Ok(ProviderValue::Integer(value.clone())),
        ProviderValue::Number(value) => Ok(ProviderValue::Number(*value)),
        ProviderValue::String(value) => Ok(ProviderValue::String(value.clone())),
        ProviderValue::Bytes(payload) => Ok(ProviderValue::Bytes(payload.clone())),
        ProviderValue::Array(values) => values
            .iter()
            .map(|value| {
                duplicate_swarm_event_publish_echo_value_for_direct_run_event_owner_v1(field, value)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|values| ProviderValue::Array(values.into())),
        ProviderValue::Object(fields) => fields
            .iter()
            .map(|(key, value)| {
                duplicate_swarm_event_publish_echo_value_for_direct_run_event_owner_v1(field, value)
                    .map(|value| (key.clone(), value))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()
            .map(|fields| ProviderValue::Object(fields.into())),
        ProviderValue::HostResourceHandle(_)
        | ProviderValue::LiveOperationHandle(_)
        | ProviderValue::LiveStreamHandle(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent intent cargo cannot carry runtime handles; the receipt echo refuses to duplicate handle authority",
        )),
        ProviderValue::CurrentProcess(_)
        | ProviderValue::ProcessCheckpoint(_)
        | ProviderValue::ProcessPlanSnapshot(_) => Err(swarm_event_publish_input_fault(
            field,
            "publishEvent intent cargo cannot carry process execution authority; the receipt echo refuses to duplicate process authority",
        )),
    }
}

/// R41054: the checked-law receipt product — the compiler-owned contract
/// (`packages/event/src/event.contract.ts` `SwarmEventPublishReceipt`) echoes
/// the ADMITTED publish intent in-band:
/// `{ schema: "swarm.event.publish.receipt.v1", accepted: true, eventIntent }`.
/// The authored `payloadTrackHTypeContract` is deliberately absent from the
/// echo: it was replaced by the owner-minted contract at journal admission and
/// neither the authored value nor the internal contract projection may ride
/// back out. Wrapped in the std.Result Ok resume carrier like every provider
/// output on this route.
fn swarm_event_publish_receipt_output_for_direct_run_event_owner_v1(
    provider_input: &ProviderValue,
) -> Result<ProviderValue, String> {
    let ProviderValue::Array(arguments) = provider_input else {
        return Err(swarm_event_publish_input_fault(
            "input",
            "publishEvent provider input must be a single-argument call ABI array",
        ));
    };
    let [ProviderValue::Object(fields)] = arguments.as_slice() else {
        return Err(swarm_event_publish_input_fault(
            "input",
            "publishEvent provider input must carry exactly one object argument",
        ));
    };
    let mut event_intent = BTreeMap::from([(
        "schema".to_owned(),
        ProviderValue::String("swarm.event.publish_intent.v1".to_owned()),
    )]);
    const REQUIRED_ECHO_FIELDS: [&str; 10] = [
        "sourceRef",
        "sourceKind",
        "parentSourceRefs",
        "topic",
        "operation",
        "entityType",
        "entityId",
        "scope",
        "aspects",
        "payload",
    ];
    for field in REQUIRED_ECHO_FIELDS {
        let Some(value) = fields.get(field) else {
            return Err(swarm_event_publish_input_fault(
                field,
                "publishEvent requires this field",
            ));
        };
        event_intent.insert(
            field.to_owned(),
            duplicate_swarm_event_publish_echo_value_for_direct_run_event_owner_v1(
                "eventIntent",
                value,
            )?,
        );
    }
    const OPTIONAL_ECHO_FIELDS: [&str; 4] =
        ["correlationId", "causationId", "mutationId", "eventIndex"];
    for field in OPTIONAL_ECHO_FIELDS {
        match fields.get(field) {
            None | Some(ProviderValue::Null) => {}
            Some(value) => {
                event_intent.insert(
                    field.to_owned(),
                    duplicate_swarm_event_publish_echo_value_for_direct_run_event_owner_v1(
                        "eventIntent",
                        value,
                    )?,
                );
            }
        }
    }
    let receipt = ProviderValue::Object(
        BTreeMap::from([
            (
                "schema".to_owned(),
                ProviderValue::String("swarm.event.publish.receipt.v1".to_owned()),
            ),
            ("accepted".to_owned(), ProviderValue::Bool(true)),
            (
                "eventIntent".to_owned(),
                ProviderValue::Object(event_intent.into()),
            ),
        ])
        .into(),
    );
    Ok(ProviderValue::Object(
        BTreeMap::from([
            (
                swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD.to_owned(),
                ProviderValue::String("std.Result".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD.to_owned(),
                ProviderValue::String("Ok".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD.to_owned(),
                ProviderValue::Bool(true),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD.to_owned(),
                receipt,
            ),
        ])
        .into(),
    ))
}

pub(in crate::direct_run) use source_refs::direct_run_event_retention_epoch_for_root;
pub(in crate::direct_run) use source_registry::direct_run_volatile_event_source_memory_summary;
pub(in crate::direct_run) use store_binding::{
    DirectRunEventStoreBindingAccessOperation,
    bind_direct_run_event_store_binding_preserving_existing_facts,
    direct_run_event_store_binding_admission_from_in_memory_product_api,
    direct_run_event_store_binding_admission_from_provider_required,
};
pub(in crate::direct_run) use subscriptions::direct_run_volatile_event_subscription_memory_summary;

pub(in crate::direct_run) fn register_direct_run_event_child_root_observation_for_child_launch(
    parent_root_scope_id: &str,
    child_root_scope_id: &str,
) -> Result<(), String> {
    journal_core::register_direct_run_event_child_root_observation(
        parent_root_scope_id,
        child_root_scope_id,
    )
}

// R41049 VERDICT (bindlaw81, ratified — see R41049.md): the live-primitive persist-receipt
// lane is dead carriage; production never minted the receipt. Deleted with this provenance
// (R41040 kernel window):
// - the cfg(test) drain-observations helper (built observations exclusively from the
//   receipt-lane batch fixture + the retired persist-success admission), and
// - the cfg(test) receipt test module (live_primitive_persist_receipts_append_to_
//   in_memory_event_journal / ..._has_no_backend_drain / ..._external_postgres_carries_
//   backend_drain + its fixture installer and receipt spine) — all exercised the
//   receipt-batch append transaction in bindlaw81's live-surface deletion set.
// The PublishEvent lane (R41045/R41054) in this file is live law and is untouched.

#[cfg(test)]
mod observation_stream_provider_input_tests {
    use super::*;

    fn single_arg(fields: BTreeMap<String, ProviderValue>) -> ProviderValue {
        ProviderValue::Array(vec![ProviderValue::Object(fields.into())].into())
    }

    // Mirrors event_observation_frame_projection.test.ss:
    //   readObservationStream({ streamRef: stream.streamRef, maxFrames: 16 }).
    #[test]
    fn stream_ref_and_max_frames_decode() {
        let input = single_arg(BTreeMap::from([
            (
                "streamRef".to_owned(),
                ProviderValue::String("sub-123".to_owned()),
            ),
            (
                "maxFrames".to_owned(),
                ProviderValue::Integer(SwarmInteger::from_i64(16)),
            ),
        ]));
        assert_eq!(
            swarm_ss_test_event_observation_stream_ref_from_provider_input(&input).unwrap(),
            "sub-123",
        );
        assert_eq!(
            swarm_ss_test_event_observation_max_frames_from_provider_input(&input).unwrap(),
            Some(16),
        );
    }

    #[test]
    fn max_frames_absent_is_none() {
        let input = single_arg(BTreeMap::from([(
            "streamRef".to_owned(),
            ProviderValue::String("sub-1".to_owned()),
        )]));
        assert_eq!(
            swarm_ss_test_event_observation_max_frames_from_provider_input(&input).unwrap(),
            None,
        );
    }

    #[test]
    fn missing_stream_ref_is_typed_fault() {
        let input = single_arg(BTreeMap::from([(
            "maxFrames".to_owned(),
            ProviderValue::Integer(SwarmInteger::from_i64(4)),
        )]));
        let err = swarm_ss_test_event_observation_stream_ref_from_provider_input(&input)
            .expect_err("missing streamRef must fault");
        assert!(err.contains("streamRef"), "fault names the field: {err}");
    }

    // Mirrors controlObservationStream({ streamRef, operation, sourceRef }).
    #[test]
    fn control_operation_and_source_ref_decode() {
        let input = single_arg(BTreeMap::from([
            (
                "streamRef".to_owned(),
                ProviderValue::String("sub-9".to_owned()),
            ),
            (
                "operation".to_owned(),
                ProviderValue::String("mask_source".to_owned()),
            ),
            (
                "sourceRef".to_owned(),
                ProviderValue::String("src-x".to_owned()),
            ),
        ]));
        assert_eq!(
            swarm_ss_test_event_observation_operation_from_provider_input(&input).unwrap(),
            "mask_source",
        );
        assert_eq!(
            swarm_ss_test_event_observation_source_ref_from_provider_input(&input).unwrap(),
            "src-x",
        );
    }

    // Non-forgeability anchor (Authority Gate condition 1): a forged/unrecorded
    // streamRef locates nothing in the empty thread-local registry and dies with
    // the typed registry-absence fault — admit-by-streamRef never mints authority
    // from the string; the registry entry IS the authority.
    #[test]
    fn unrecorded_stream_ref_admission_fails_typed() {
        let result =
            admit_direct_run_event_product_subscription_ref_from_provider_stream_ref_for_direct_run_event_product_owner_v1(
                "forged-stream-ref-absent-from-registry",
                DirectRunEventSubscriptionRegistryEntryReadAuthority::for_product_api_admission(
                    DirectRunEventProductApiAdmissionOperation::RustSdkReadObservationStream,
                ),
            );
        let err = result.expect_err("unrecorded streamRef must fail typed");
        assert!(
            err.contains("not present in the event-product owner registry"),
            "registry-absence fault: {err}",
        );
    }
}

#[cfg(test)]
mod ss_test_event_read_capture_handle_seal_tests {
    use super::*;

    fn single_arg(fields: BTreeMap<String, ProviderValue>) -> ProviderValue {
        ProviderValue::Array(vec![ProviderValue::Object(fields.into())].into())
    }

    // #200 FORGERY-CLOSED: the read decode consumes ONLY the opaque capture_id.
    // A caller-supplied after_global_seq riding alongside it is IGNORED — the
    // read can no longer widen the replay window from caller input.
    #[test]
    fn read_decode_reads_capture_id_and_ignores_forged_after_global_seq() {
        let input = single_arg(BTreeMap::from([(
            "capture".to_owned(),
            ProviderValue::Object(
                BTreeMap::from([
                    (
                        "capture_id".to_owned(),
                        ProviderValue::String("ss-test-fixture:swarm-test-capture:0".to_owned()),
                    ),
                    // A forged cursor the old extraction would have replayed from.
                    (
                        "after_global_seq".to_owned(),
                        ProviderValue::Integer(SwarmInteger::from_u64(999_999)),
                    ),
                ])
                .into(),
            ),
        )]));
        assert_eq!(
            swarm_ss_test_event_read_capture_id_from_provider_input(&input).unwrap(),
            "ss-test-fixture:swarm-test-capture:0",
            "read consumes the opaque capture_id, never the forged cursor",
        );
    }

    // A handle missing the opaque capture_id is a typed fault (no default cursor).
    #[test]
    fn read_decode_missing_capture_id_is_typed_fault() {
        let input = single_arg(BTreeMap::from([(
            "capture".to_owned(),
            ProviderValue::Object(
                BTreeMap::from([(
                    "after_global_seq".to_owned(),
                    ProviderValue::Integer(SwarmInteger::from_u64(3)),
                )])
                .into(),
            ),
        )]));
        let err = swarm_ss_test_event_read_capture_id_from_provider_input(&input)
            .expect_err("a handle without capture_id must fault");
        assert!(
            err.contains("capture.capture_id"),
            "fault names the sealed field: {err}",
        );
    }

    // #200 rider b (firing negative): a read whose capture_id was never minted by
    // event.capture() in this body resolves to NO start index, and the read owner
    // settles the existing typed input-forbidden fault — never a default/empty
    // cursor. This ties the store miss to the exact fault the read owner emits for
    // its `None` branch (selected work cannot disappear).
    #[test]
    fn unknown_capture_id_read_settles_typed_fault_not_default() {
        let unminted = "ss-test-fixture:swarm-test-capture:unminted-in-this-body";
        assert_eq!(
            direct_run_ss_test_event_capture_handle_store_lookup_start_index(unminted).unwrap(),
            None,
            "an unminted capture_id has no start index (no default cursor)",
        );
        let fault = swarm_ss_test_event_read_input_fault(
            "capture.capture_id",
            "event.read capture handle was not minted by event.capture() in this test body; it may be forged or from another body",
        );
        assert!(
            fault.contains("direct_run_ss_test_event_read_input_forbidden"),
            "unknown capture_id settles the existing typed input-forbidden fault: {fault}",
        );
        assert!(
            fault.contains("capture.capture_id"),
            "the typed fault names the sealed field",
        );
    }
}

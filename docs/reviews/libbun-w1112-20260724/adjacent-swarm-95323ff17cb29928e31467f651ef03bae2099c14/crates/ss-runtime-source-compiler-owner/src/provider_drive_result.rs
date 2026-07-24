#![allow(dead_code)]

use std::fmt;

use crate::PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1;
use crate::provider_messages::ProviderResult;
use crate::{
    CapabilityContractFingerprint, CapabilityContractIdentity, CapabilityContractProjection,
    CapabilitySdkError, CapabilitySdkResult, ProviderParkReceipt, RustSdkProviderBinding,
};
use serde::Serialize;
use swarm_capability_contract_tson::AdmittedCapabilityContractTson;
use swarm_capability_linker_core::{
    CapabilityContractOutputTypeContractAuthorityProduct, CapabilityTypeContractError,
};
use swarm_provider_value_model::ProviderValue;
use swarm_rust_sdk_static_provider_host::{
    HostAdmittedTypedProviderRequest, ProviderHostResourceReleaseTransferSetV1,
    RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1,
    RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1,
    SelectedProviderBoundaryExecutionResultForProviderHostOwner,
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProviderDriveResultKind {
    Ready,
    Parked,
}

impl ProviderDriveResultKind {
    pub(crate) const fn tag(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Parked => "parked",
        }
    }
}

enum ProviderDriveResultPayload {
    Ready {
        result: ProviderResult,
    },
    Parked {
        settlement: ProviderDriveParkSettlementForProviderHostOwner,
    },
}

pub struct ProviderDriveResult {
    payload: ProviderDriveResultPayload,
}

#[derive(Debug, PartialEq, Error)]
pub enum ProviderDriveSessionExecutionCommitFault {
    #[error("provider-drive session commit requires a ready provider result")]
    NotReady,
    #[error("provider-drive session commit failed: {source}")]
    SessionExecution {
        #[from]
        source: crate::ProcessSessionRunError,
    },
}

#[derive(PartialEq)]
pub(crate) struct ProviderDriveParkSettlementForProviderHostOwner {
    receipt: ProviderParkReceipt,
}

pub(crate) struct ProviderReadyBoundaryReceiptShapeProjectionV1<'a> {
    payload: ProviderReadyBoundaryReceiptShapeProjectionPayloadV1<'a>,
}

#[derive(Serialize)]
#[serde(tag = "receipt_kind", rename_all = "snake_case")]
enum ProviderReadyBoundaryReceiptShapeProjectionPayloadV1<'a> {
    Boundary {
        schema: &'a str,
        boundary_present: bool,
        details_product_present: bool,
    },
    Deadline {
        schema: &'a str,
        owner_class: &'static str,
        deadline_id_present: bool,
        reached_at_present: bool,
    },
    Cancellation {
        schema: &'a str,
        owner_class: &'static str,
        cancellation_id_present: bool,
        cancelled_at_present: bool,
    },
    LivenessWait {
        schema: &'a str,
        blocker_kind_present: bool,
    },
}

struct ProviderHostOwnerReadyOutputPayload {
    ready_output: swarm_capability_model::ProviderReadyBoundaryOutput,
    output_effect_drain_receipts: ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
    host_resource_releases: ProviderHostResourceReleaseTransferSetV1,
}

pub(crate) struct ProviderDriveReadyOutputForProviderHostOwner {
    output: ProviderHostOwnerReadyOutputPayload,
}

pub struct ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner {
    observations: Vec<serde_json::Value>,
    process_output_records: ProviderDriveProcessOutputRecordProductForProviderHostOwner,
}

pub struct ProviderDriveProcessOutputRecordProductForProviderHostOwner {
    records: Vec<RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1>,
}

#[derive(PartialEq)]
pub(crate) enum ProviderDriveBoundaryDetailsForProviderHostOwner {}

/// #139-P loaded-native park-receipt schema. The route-descriptor-free
/// successor of the (now-deleted) RustSdk `park_receipt_schema` static, minted
/// directly for the loaded-native (dlopen) provider live-operation park lane.
/// This is a fixed observation-tag constant identifying the park kind; it feeds
/// no authority selection; the future common selected invocation owns native
/// provider execution, not this string.
pub(crate) const LOADED_NATIVE_PROVIDER_PARK_RECEIPT_SCHEMA: &str =
    "swarm.provider_drive_result.loaded_native_provider_park_receipt.v1";

#[derive(PartialEq)]
pub(crate) struct ProviderDriveParkDetailsForProviderHostOwner {
    host_id: String,
    provider_execution_domain: String,
    provider_id: String,
    contract: CapabilityContractIdentity,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    contract_hash: String,
    park_ref: String,
}

impl fmt::Debug for ProviderDriveReadyOutputForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderDriveReadyOutputForProviderHostOwner")
            .field("provider_output", &"redacted")
            .field("authority", &"provider-host-owner-output")
            .finish()
    }
}

impl ProviderDriveReadyOutputForProviderHostOwner {
    fn from_selected_provider_boundary_execution_result_for_provider_host_owner_v1(
        result: SelectedProviderBoundaryExecutionResultForProviderHostOwner,
    ) -> Result<Self, CapabilitySdkError> {
        let (ready_output, output_effect_drain_receipts, host_resource_releases) = result
            .into_provider_ready_boundary_output_and_effect_drain_receipts_for_provider_drive_result_owner_v1()?;
        Ok(Self {
            output: ProviderHostOwnerReadyOutputPayload {
                ready_output,
                output_effect_drain_receipts: ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner::from_static_provider_host_receipts_for_provider_drive_result_owner_v1(
                    output_effect_drain_receipts,
                ),
                host_resource_releases,
            },
        })
    }

    fn into_ready_boundary_output_effect_drain_receipts_and_host_resource_releases_for_provider_drive_result_owner_v1(
        self,
    ) -> (
        swarm_capability_model::ProviderReadyBoundaryOutput,
        ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
        ProviderHostResourceReleaseTransferSetV1,
    ) {
        (
            self.output.ready_output,
            self.output.output_effect_drain_receipts,
            self.output.host_resource_releases,
        )
    }
}

impl ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner {
    fn empty_for_provider_drive_result_owner_v1() -> Self {
        Self {
            observations: Vec::new(),
            process_output_records: ProviderDriveProcessOutputRecordProductForProviderHostOwner {
                records: Vec::new(),
            },
        }
    }

    fn from_static_provider_host_receipts_for_provider_drive_result_owner_v1(
        receipts: Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
    ) -> Self {
        let mut observations = Vec::new();
        let mut process_output_records = Vec::new();
        for receipt in receipts {
            let (observation, process_output_record) = receipt
                .into_direct_run_output_drain_observation_and_process_output_record_for_provider_drive_result_owner_v1();
            observations.push(observation);
            process_output_records.push(process_output_record);
        }
        Self {
            observations,
            process_output_records: ProviderDriveProcessOutputRecordProductForProviderHostOwner {
                records: process_output_records,
            },
        }
    }

    pub fn into_swarm_io_stream_observations_for_direct_run_output_settlement_owner_v1(
        self,
    ) -> Vec<serde_json::Value> {
        self
            .into_swarm_io_stream_observations_and_process_output_records_for_direct_run_output_settlement_owner_v1()
            .0
    }

    pub fn into_swarm_io_stream_observations_and_process_output_records_for_direct_run_output_settlement_owner_v1(
        self,
    ) -> (
        Vec<serde_json::Value>,
        ProviderDriveProcessOutputRecordProductForProviderHostOwner,
    ) {
        (self.observations, self.process_output_records)
    }
}

impl ProviderDriveProcessOutputRecordProductForProviderHostOwner {
    pub fn extend_for_direct_run_process_child_owner_v1(&mut self, other: Self) {
        self.records.extend(other.records);
    }

    /// Project the exact emitted child-output bytes into the body-local test
    /// observation sink without consuming the reporting product.  The static
    /// provider record owner performs the projection; this owner exposes no
    /// stream/text getters and retains the original product for terminal
    /// reporting.
    pub fn body_local_process_output_observations_for_direct_run_process_child_owner_v1(
        &self,
    ) -> swarm_rust_sdk_static_provider_host::RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1
    {
        swarm_rust_sdk_static_provider_host::RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1::from_exact_process_output_records_for_provider_drive_result_owner_v1(
            &self.records,
        )
    }

    pub fn into_body_local_process_output_observations_for_direct_run_ss_test_owner_v1(
        self,
    ) -> Vec<serde_json::Value> {
        self.records
            .into_iter()
            .map(|record| {
                record
                    .into_body_local_process_output_observation_for_provider_drive_result_owner_v1()
            })
            .collect()
    }
}

impl ProviderDriveBoundaryDetailsForProviderHostOwner {
    pub(crate) fn details_product_present_for_provider_drive_result_owner_v1(&self) -> bool {
        match *self {}
    }
}

impl ProviderDriveParkDetailsForProviderHostOwner {
    pub(crate) fn from_host_admitted_typed_provider_request_for_provider_drive_result_owner_v1(
        request: HostAdmittedTypedProviderRequest,
    ) -> Result<Self, String> {
        let host_id = request.host().host_id().to_owned();
        let provider_execution_domain = request.host().provider_execution_domain().to_owned();
        let provider_id = request.provider_id().to_owned();
        let contract = request.contract().duplicate_for_capability_model_owner();
        let output_type_contract =
            request.into_output_type_contract_for_provider_drive_result_owner_v1();
        let contract_hash =
            required_contract_hash_for_provider_drive_result_owner_v1(&contract, "park_details")?;

        let park_ref = format!(
            "swarm.provider_drive_result.park_ref.v1:{host_id}:{provider_execution_domain}:{provider_id}:{contract_hash}"
        );

        Ok(Self {
            host_id,
            provider_execution_domain,
            provider_id,
            contract,
            output_type_contract,
            contract_hash,
            park_ref,
        })
    }

    pub(crate) fn schema_for_provider_drive_result_owner_v1(&self) -> &str {
        LOADED_NATIVE_PROVIDER_PARK_RECEIPT_SCHEMA
    }

    pub(crate) fn provider_id_for_provider_drive_result_owner_v1(&self) -> &str {
        self.provider_id.as_str()
    }

    pub(crate) fn contract_for_provider_drive_result_owner_v1(
        &self,
    ) -> &CapabilityContractIdentity {
        &self.contract
    }

    pub(crate) fn output_type_contract_product_present_for_provider_drive_result_owner_v1(
        &self,
    ) -> bool {
        let _ = &self.output_type_contract;
        true
    }

    pub(crate) fn park_ref_for_provider_drive_result_owner_v1(&self) -> &str {
        self.park_ref.as_str()
    }

    pub(crate) fn into_contract_tson_for_provider_drive_result_owner_v1(
        self,
    ) -> AdmittedCapabilityContractTson {
        AdmittedCapabilityContractTson::admit_from_capability_linker_output_type_contract_for_contract_tson_owner_v1(
            self.contract,
            self.output_type_contract,
        )
    }

    pub(crate) fn details_product_present_for_provider_drive_result_owner_v1(&self) -> bool {
        !self.host_id.is_empty()
            && !self.provider_execution_domain.is_empty()
            && !self.contract_hash.is_empty()
            && !self.park_ref.is_empty()
            && self.output_type_contract_product_present_for_provider_drive_result_owner_v1()
    }
}

pub(crate) struct ProviderDriveResultWireProjectionV1<'a> {
    payload: ProviderDriveResultWireProjectionPayloadV1<'a>,
}

impl Serialize for ProviderDriveResultWireProjectionV1<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.payload.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "drive_result", rename_all = "snake_case")]
enum ProviderDriveResultWireProjectionPayloadV1<'a> {
    Ready {
        schema: &'a str,
        status: crate::provider_messages::ProviderResultStatus,
    },
    Parked {
        schema: &'a str,
        provider_id: &'a str,
        contract: CapabilityContractProjection,
        owner_class: &'static str,
        park_ref_present: bool,
        details_product_present: bool,
    },
}

#[derive(Serialize)]
pub(crate) struct ProviderDriveResultParkedShapeProjectionV1<'a> {
    provider_id: &'a str,
    contract: CapabilityContractProjection,
    owner_class: &'static str,
    park_ref_present: bool,
    details_product_present: bool,
}

#[derive(Debug, PartialEq)]
pub struct ProviderDriveParkedContinuationForDirectRunLiveOperationOwnerV1 {
    continuation: crate::provider_messages::ProviderContinuationRef,
}

impl fmt::Debug for ProviderDriveResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_struct("ProviderDriveResult");
        debug
            .field("kind", &"redacted")
            .field("status", &"redacted")
            .field("owner_class", &"redacted")
            .field("hidden_provider_result_authority", &"redacted")
            .finish()
    }
}

impl ProviderDriveResult {
    /// Infallibly commit a kernel-internal plain output after the contract owner
    /// has completed every fallible classification while retaining the exact
    /// contract, selected boundary half, and concrete output together.
    pub(crate) fn ready_from_preflighted_kernel_internal_plain_output_for_provider_drive_result_owner_v1(
        settlement: PreflightedKernelInternalPlainOutputSettlementForDirectRunOwnerV1,
    ) -> Self {
        let ready_output =
            settlement.consume_into_ready_output_for_provider_drive_result_owner_v1();
        let ready = ProviderDriveReadyOutputForProviderHostOwner {
            output: ProviderHostOwnerReadyOutputPayload {
                ready_output,
                output_effect_drain_receipts:
                    ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner::empty_for_provider_drive_result_owner_v1(),
                host_resource_releases:
                    ProviderHostResourceReleaseTransferSetV1::empty_for_kernel_internal_provider_route_owner_v1(),
            },
        };
        Self {
            payload: ProviderDriveResultPayload::Ready {
                result:
                    ProviderResult::completed_from_provider_host_owner_output_for_provider_drive_result_owner_v1(
                        ready,
                    ),
            },
        }
    }

    pub fn ready_from_rust_sdk_static_provider_execution_result_for_provider_drive_result_owner_v1(
        result: SelectedProviderBoundaryExecutionResultForProviderHostOwner,
    ) -> Result<Self, CapabilitySdkError> {
        let output =
            ProviderDriveReadyOutputForProviderHostOwner::from_selected_provider_boundary_execution_result_for_provider_host_owner_v1(
                result,
            )?;
        Ok(Self {
            payload: ProviderDriveResultPayload::Ready {
                result:
                    ProviderResult::completed_from_provider_host_owner_output_for_provider_drive_result_owner_v1(
                        output,
                    ),
            },
        })
    }

    pub(crate) fn parked_from_provider_host_owner_settlement_for_provider_drive_result_owner_v1(
        settlement: ProviderDriveParkSettlementForProviderHostOwner,
    ) -> Self {
        Self {
            payload: ProviderDriveResultPayload::Parked { settlement },
        }
    }

    pub fn parked_liveness_from_host_admitted_typed_provider_request_for_provider_drive_result_owner_v1(
        request: HostAdmittedTypedProviderRequest,
    ) -> Result<Self, String> {
        let settlement =
            ProviderDriveParkSettlementForProviderHostOwner::from_host_admitted_typed_provider_request_for_provider_drive_result_owner_v1(
                request,
            )?;
        Ok(
            Self::parked_from_provider_host_owner_settlement_for_provider_drive_result_owner_v1(
                settlement,
            ),
        )
    }

    pub(crate) fn kind(&self) -> ProviderDriveResultKind {
        match self.payload {
            ProviderDriveResultPayload::Ready { .. } => ProviderDriveResultKind::Ready,
            ProviderDriveResultPayload::Parked { .. } => ProviderDriveResultKind::Parked,
        }
    }

    pub fn into_parked_provider_continuation_for_direct_run_live_operation_owner_v1(
        self,
        operation: &'static str,
    ) -> Result<ProviderDriveParkedContinuationForDirectRunLiveOperationOwnerV1, String> {
        match self.payload {
            ProviderDriveResultPayload::Parked { settlement } => {
                Ok(ProviderDriveParkedContinuationForDirectRunLiveOperationOwnerV1 {
                    continuation: settlement
                        .receipt
                        .into_provider_continuation_ref_for_provider_drive_result_owner_v1(),
                })
            }
            ProviderDriveResultPayload::Ready { .. } => Err(serde_json::json!({
                "kind": "provider_drive_result_provider_continuation_requires_parked_result",
                "reason": "live-operation source-start continuation authority must be minted by consuming a typed ProviderDriveResult::Parked product",
                "operation": operation,
                "actual_kind": ProviderDriveResultKind::Ready.tag(),
            })
            .to_string()),
        }
    }

    pub fn commit_ready_into_session_execution_kernel_and_drive_to_direct_run_result_product_v1(
        self,
        session: &mut crate::ProcessSessionV0,
        boundary_context: &'static str,
    ) -> Result<
        (
            crate::DirectRunProcessSessionRunResultProductV1,
            ProviderDriveOutputEffectDrainReceiptBundleForProviderHostOwner,
        ),
        ProviderDriveSessionExecutionCommitFault,
    > {
        match self.payload {
            ProviderDriveResultPayload::Ready { result } => {
                let (ready_output, output_effect_drain_receipts, host_resource_releases) = result
                    .into_output_for_provider_drive_result_owner_v1()
                    .into_ready_boundary_output_effect_drain_receipts_and_host_resource_releases_for_provider_drive_result_owner_v1();
                let result_product = session
                    .commit_provider_ready_boundary_output_and_drive_to_direct_run_result_product_for_direct_run_owner_v1(
                        ready_output,
                        host_resource_releases,
                        boundary_context,
                    )?;
                Ok((result_product, output_effect_drain_receipts))
            }
            ProviderDriveResultPayload::Parked { settlement: _ } => {
                Err(ProviderDriveSessionExecutionCommitFault::NotReady)
            }
        }
    }

    pub(crate) fn wire_projection_v1(&self) -> ProviderDriveResultWireProjectionV1<'_> {
        match &self.payload {
            ProviderDriveResultPayload::Ready { result } => ProviderDriveResultWireProjectionV1 {
                payload: ProviderDriveResultWireProjectionPayloadV1::Ready {
                    schema: result.schema(),
                    status: result.status(),
                },
            },
            ProviderDriveResultPayload::Parked { settlement } => {
                let receipt = settlement.receipt();
                ProviderDriveResultWireProjectionV1 {
                    payload: ProviderDriveResultWireProjectionPayloadV1::Parked {
                        schema: receipt.schema(),
                        provider_id: receipt.provider_id(),
                        contract: receipt.contract().projection(),
                        owner_class: receipt.owner_class().tag(),
                        park_ref_present: receipt.park_ref_present_projection_v1(),
                        details_product_present: receipt
                            .park_details_product_present_for_provider_drive_result_owner_v1(),
                    },
                }
            }
        }
    }

    pub(crate) fn ready_boundary_receipt_shape_projection_v1(
        &self,
    ) -> Option<ProviderReadyBoundaryReceiptShapeProjectionV1<'_>> {
        match &self.payload {
            ProviderDriveResultPayload::Ready { result } => {
                result
                    .receipt()
                    .map(|receipt| ProviderReadyBoundaryReceiptShapeProjectionV1 {
                        payload: match receipt.kind() {
                            crate::provider_messages::BoundaryReceiptKind::Boundary => {
                                let boundary = receipt
                                    .as_boundary()
                                    .expect("boundary receipt kind must expose boundary payload");
                                ProviderReadyBoundaryReceiptShapeProjectionPayloadV1::Boundary {
                                    schema: boundary.schema(),
                                    boundary_present: !boundary.boundary().is_empty(),
                                    details_product_present: boundary
                                        .boundary_details_product_present_for_provider_drive_result_owner_v1(),
                                }
                            }
                            crate::provider_messages::BoundaryReceiptKind::Deadline => {
                                let receipt = receipt
                                    .as_deadline()
                                    .expect("boundary receipt kind must expose deadline payload");
                                ProviderReadyBoundaryReceiptShapeProjectionPayloadV1::Deadline {
                                    schema: receipt.schema(),
                                    owner_class: receipt.owner_class().tag(),
                                    deadline_id_present: !receipt.deadline_id().is_empty(),
                                    reached_at_present: !receipt.reached_at().is_empty(),
                                }
                            }
                            crate::provider_messages::BoundaryReceiptKind::Cancellation => {
                                let receipt = receipt.as_cancellation().expect(
                                    "boundary receipt kind must expose cancellation payload",
                                );
                                ProviderReadyBoundaryReceiptShapeProjectionPayloadV1::Cancellation {
                                    schema: receipt.schema(),
                                    owner_class: receipt.owner_class().tag(),
                                    cancellation_id_present: !receipt.cancellation_id().is_empty(),
                                    cancelled_at_present: !receipt.cancelled_at().is_empty(),
                                }
                            }
                            crate::provider_messages::BoundaryReceiptKind::LivenessWait => {
                                let receipt = receipt.as_liveness_wait().expect(
                                    "boundary receipt kind must expose liveness-wait payload",
                                );
                                ProviderReadyBoundaryReceiptShapeProjectionPayloadV1::LivenessWait {
                                    schema: receipt.schema(),
                                    blocker_kind_present: !receipt.blocker_kind().is_empty(),
                                }
                            }
                        },
                    })
            }
            ProviderDriveResultPayload::Parked { .. } => None,
        }
    }

    pub(crate) fn parked_shape_projection_v1(
        &self,
    ) -> Option<ProviderDriveResultParkedShapeProjectionV1<'_>> {
        match &self.payload {
            ProviderDriveResultPayload::Parked { settlement } => {
                let receipt = settlement.receipt();
                Some(ProviderDriveResultParkedShapeProjectionV1 {
                    provider_id: receipt.provider_id(),
                    contract: receipt.contract().projection(),
                    owner_class: receipt.owner_class().tag(),
                    park_ref_present: receipt.park_ref_present_projection_v1(),
                    details_product_present: receipt
                        .park_details_product_present_for_provider_drive_result_owner_v1(),
                })
            }
            ProviderDriveResultPayload::Ready { .. } => None,
        }
    }

    pub fn diagnostic_descriptor_for_libswarm_diagnostic_owner_v1(&self) -> serde_json::Value {
        match self.kind() {
            ProviderDriveResultKind::Ready => {
                let receipt_shape = self
                    .ready_boundary_receipt_shape_projection_v1()
                    .map(|receipt| serde_json::json!(receipt));
                serde_json::json!({
                    "drive_result": "ready",
                    "wire_projection": self.wire_projection_v1(),
                    "receipt_present": receipt_shape.is_some(),
                    "receipt_shape": receipt_shape.unwrap_or(serde_json::Value::Null),
                })
            }
            ProviderDriveResultKind::Parked => {
                let receipt = self
                    .parked_shape_projection_v1()
                    .expect("parked provider drive result must project a receipt shape");
                let receipt = serde_json::json!(receipt);
                serde_json::json!({
                    "drive_result": "parked",
                    "receipt_shape": provider_drive_result_diagnostic_value_shape(&receipt),
                })
            }
        }
    }
}

fn provider_drive_result_diagnostic_value_shape(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Null => serde_json::json!({"shape": "null"}),
        serde_json::Value::Bool(_) => serde_json::json!({"shape": "bool"}),
        serde_json::Value::Number(_) => serde_json::json!({"shape": "number"}),
        serde_json::Value::String(value) => {
            serde_json::json!({"shape": "string", "length": value.len()})
        }
        serde_json::Value::Array(items) => {
            serde_json::json!({"shape": "array", "length": items.len()})
        }
        serde_json::Value::Object(object) => serde_json::json!({
            "shape": "object",
            "field_count": object.len(),
            "keys": object.keys().take(8).cloned().collect::<Vec<_>>(),
        }),
    }
}

impl ProviderDriveParkSettlementForProviderHostOwner {
    pub(crate) fn from_host_admitted_typed_provider_request_for_provider_drive_result_owner_v1(
        request: HostAdmittedTypedProviderRequest,
    ) -> Result<Self, String> {
        let details =
            ProviderDriveParkDetailsForProviderHostOwner::from_host_admitted_typed_provider_request_for_provider_drive_result_owner_v1(
                request,
            )?;
        let receipt =
            ProviderParkReceipt::from_provider_drive_park_details_for_provider_drive_result_owner_v1(
                details,
            );
        Ok(Self { receipt })
    }

    fn receipt(&self) -> &ProviderParkReceipt {
        &self.receipt
    }
}

impl ProviderDriveParkedContinuationForDirectRunLiveOperationOwnerV1 {
    pub fn require_provider_reference_for_direct_run_live_operation_owner_v1(
        &self,
        provider_reference_id: &str,
    ) -> Result<(), String> {
        self.continuation
            .require_provider_reference_for_direct_run_live_primitive_source_advance_owner_v1(
                provider_reference_id,
            )
    }

    pub fn contract_identity_for_durable_native_provider_loader_owner_v1(
        &self,
    ) -> &CapabilityContractIdentity {
        self.continuation
            .contract_identity_for_durable_native_provider_loader_owner_v1()
    }

    pub fn require_contract_for_durable_native_provider_loader_owner_v1(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> Result<(), String> {
        self.continuation
            .require_contract_for_durable_native_provider_loader_owner_v1(contract)
    }

    pub fn into_admitted_contract_tson_for_durable_native_provider_loader_owner_v1(
        self,
    ) -> swarm_capability_contract_tson::AdmittedCapabilityContractTson {
        self.continuation
            .into_admitted_contract_tson_for_durable_native_provider_loader_owner_v1()
    }
}

impl Serialize for ProviderReadyBoundaryReceiptShapeProjectionV1<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.payload.serialize(serializer)
    }
}

fn require_same_provider_contract(
    label: &str,
    provider: &RustSdkProviderBinding,
    observed_provider_id: &str,
    observed_contract: &CapabilityContractIdentity,
) -> CapabilitySdkResult<()> {
    if provider.provider_id() != observed_provider_id {
        return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
            format!(
                "{label} provider id mismatch: expected {}, observed {observed_provider_id}",
                provider.provider_id()
            ),
        ));
    }
    if provider.contract().package_specifier() != observed_contract.package_specifier()
        || provider.contract().export_name() != observed_contract.export_name()
        || provider
            .contract()
            .fingerprint()
            .map(CapabilityContractFingerprint::as_str)
            != observed_contract
                .fingerprint()
                .map(CapabilityContractFingerprint::as_str)
    {
        return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
            format!(
                "{label} contract mismatch for provider {}",
                provider.provider_id()
            ),
        ));
    }
    Ok(())
}

fn required_contract_hash_for_provider_drive_result_owner_v1(
    contract: &CapabilityContractIdentity,
    label: &'static str,
) -> Result<String, String> {
    contract
        .fingerprint()
        .map(|fingerprint| fingerprint.as_str().to_owned())
        .ok_or_else(|| {
            serde_json::json!({
                "kind": "provider_drive_result_contract_hash_missing",
                "reason": "provider-drive parked-liveness handoff requires exact contract fingerprint authority",
                "label": label,
                "package_specifier": contract.package_specifier(),
                "export_name": contract.export_name(),
            })
            .to_string()
        })
}

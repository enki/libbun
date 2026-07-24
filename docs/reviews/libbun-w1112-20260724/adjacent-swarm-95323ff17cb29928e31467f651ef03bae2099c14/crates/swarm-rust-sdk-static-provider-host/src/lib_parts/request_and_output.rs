#[derive(PartialEq, Eq)]
struct TypedProviderRequest {
    provider_id: String,
    contract: CapabilityContractIdentity,
    operation: AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    input: ProviderValue,
    invocation: Option<AdmittedProviderOperationInvocation>,
}

impl fmt::Debug for TypedProviderRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedProviderRequest")
            .field("provider_id", &self.provider_id)
            .field("contract_fingerprint", &self.contract.fingerprint())
            .field("provider_input", &"redacted")
            .field("hidden_provider_request_authority", &"redacted")
            .finish()
    }
}

impl TypedProviderRequest {
    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        &self.contract
    }

    fn input(&self) -> &ProviderValue {
        &self.input
    }

    fn operation_for_static_provider_host_owner_v1(
        &self,
    ) -> &AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1 {
        &self.operation
    }

    fn diagnostic_value(&self) -> CapabilitySdkResult<Value> {
        let input_canonical_json =
            provider_value_to_canonical_json_v1(&self.input).map_err(|source| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                    "typed provider request input canonicalization failed for {}: {source}",
                    self.provider_id
                ))
            })?;
        Ok(json!({
            "schema": "swarm.rust_sdk_static_provider_host.typed_provider_request.diagnostic.v1",
            "provider_id": self.provider_id,
            "contract": self.contract.projection(),
            "input_canonical_json": input_canonical_json,
        }))
    }
}

impl TypedProviderOutputSettlementAuthority {
    fn from_typed_request_for_provider_host_owner_v1(request: TypedProviderRequest) -> Self {
        let TypedProviderRequest {
            provider_id,
            contract,
            operation: _,
            output_type_contract,
            input: _,
            invocation,
        } = request;
        Self {
            provider_id,
            contract,
            output_type_contract,
            invocation,
        }
    }

    fn into_ready_output_for_provider_host_owner_v1(
        self,
        output: ProviderValue,
        output_effect_drain_receipts: Vec<
            RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1,
        >,
    ) -> CapabilitySdkResult<RustSdkStaticProviderReadyOutputForProviderHostOwner> {
        let Self {
            provider_id,
            contract,
            output_type_contract,
            invocation,
        } = self;
        let invocation_result = invocation
            .map(|invocation| {
                AdmittedProviderOperationInvocationResult::admit_for_static_provider_host_owner_v1(
                    invocation, &output,
                )
            })
            .transpose()?;
        let output_fingerprint = provider_output_fingerprint_v1(&provider_id, &contract, &output)?;
        Ok(RustSdkStaticProviderReadyOutputForProviderHostOwner {
            contract_output: RustSdkStaticProviderReadyContractOutputForProviderHostOwner {
                provider_id,
                contract,
                output_type_contract,
                invocation_result,
                output_effect_drain_receipts,
                output_fingerprint,
                output,
            },
        })
    }
}

#[derive(PartialEq)]
pub struct HostAdmittedTypedProviderRequest {
    host: ProviderHostRequestAdmission,
    binding: RustSdkStaticProviderBinding,
    request: TypedProviderRequest,
}

/// Authored implementation cargo returned by one already-selected installed
/// static-provider executor.  It carries no request, target, output-settlement,
/// or selected-boundary authority: only the provider host can consume it with
/// the retained admitted request and mint the execution result.
#[must_use = "static-provider implementation output must return to the provider host for settlement"]
pub struct RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
    output: ProviderValue,
    output_effect_drain_receipts:
        Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
    host_resource_releases: ProviderHostResourceReleaseTransferSetV1,
}

// compiler-custody: symbol=SelectedProviderBoundaryHostRequest disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
#[must_use = "a selected provider-boundary request must be consumed into exactly one selected execution result"]
pub struct SelectedProviderBoundaryHostRequest {
    request: SelectedProviderBoundaryTypedRequest,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
}

// compiler-custody: symbol=SelectedProviderBoundaryTypedRequest disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
enum SelectedProviderBoundaryTypedRequest {
    RustSdk(HostAdmittedTypedProviderRequest),
    ManifestResolvedExternal {
        call_authority: ManifestResolvedExternalProviderCallAuthority,
        request: TypedProviderRequest,
    },
}

// compiler-custody: symbol=SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
#[must_use = "selected provider-boundary routing must be consumed by exactly one host domain"]
pub enum SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1 {
    RustSdk(SelectedProviderBoundaryHostRequest),
    ManifestResolvedExternal(DurableExternalProviderInvocationAuthority),
}

// compiler-custody: symbol=DurableExternalProviderInvocationAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
#[must_use = "a durable external provider invocation is move-only and must settle through its correlated output authority"]
pub struct DurableExternalProviderInvocationAuthority {
    call_authority: ManifestResolvedExternalProviderCallAuthority,
    provider_input: ProviderValue,
    output_settlement: DurableExternalProviderOutputSettlementAuthority,
}

// compiler-custody: symbol=DurableExternalProviderOutputSettlementAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
#[must_use = "durable external provider output settlement must consume exactly one backend output"]
pub struct DurableExternalProviderOutputSettlementAuthority {
    request: TypedProviderOutputSettlementAuthority,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
}

// compiler-custody: symbol=TypedProviderOutputSettlementAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
struct TypedProviderOutputSettlementAuthority {
    provider_id: String,
    contract: CapabilityContractIdentity,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    invocation: Option<AdmittedProviderOperationInvocation>,
}

#[derive(PartialEq, Eq)]
pub struct AdmittedProviderOperationInvocation {
    inner: AdmittedProviderOperationInvocationInner,
}

#[derive(PartialEq, Eq)]
enum AdmittedProviderOperationInvocationInner {
    SwarmEventMintObjectSourceRef {
        input: AdmittedMintObjectSourceRefInput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoPrint {
        input: AdmittedSwarmIoPrintInput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoError {
        input: AdmittedSwarmIoPrintInput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoReadLine {
        input: AdmittedSwarmIoReadLineInput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
}

#[derive(PartialEq, Eq)]
pub struct AdmittedMintObjectSourceRefInput {
    object_kind: String,
    object_id: String,
}

#[derive(PartialEq, Eq)]
pub struct AdmittedSwarmIoPrintInput {
    rendered_value: String,
}

#[derive(PartialEq, Eq)]
pub struct AdmittedSwarmIoReadLineInput {
    prompt: Option<String>,
}

pub(crate) struct AdmittedProviderOperationInvocationResult {
    inner: AdmittedProviderOperationInvocationResultInner,
}

enum AdmittedProviderOperationInvocationResultInner {
    SwarmEventMintObjectSourceRef {
        provider_id: String,
        output: AdmittedMintObjectSourceRefOutput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoPrint {
        provider_id: String,
        output: AdmittedSwarmIoPrintOutput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoError {
        provider_id: String,
        output: AdmittedSwarmIoPrintOutput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
    SwarmIoReadLine {
        provider_id: String,
        output: AdmittedSwarmIoReadLineOutput,
        provider_route_contract: CapabilityContractIdentity,
        provider_host_kind: &'static str,
        operation_export: String,
    },
}

#[derive(PartialEq, Eq)]
pub(crate) struct AdmittedMintObjectSourceRefOutput {
    object_kind: String,
    object_id: String,
    source_ref: String,
}

#[derive(PartialEq, Eq)]
pub(crate) struct AdmittedSwarmIoPrintOutput;

#[derive(PartialEq, Eq)]
pub(crate) struct AdmittedSwarmIoReadLineOutput {
    line: Option<String>,
    eof: bool,
}

pub struct RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1 {
    inner: RustSdkStaticProviderOutputEffectDrainReceiptInner,
}

pub struct RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1 {
    _inner: RustSdkStaticProviderProcessOutputRecordInner,
}

/// Exact child-output observations admitted back into one body-local
/// `@swarm/test` executor.  The provider-drive product remains the reporting
/// authority; this sealed projection carries only the emitted stream bytes
/// needed by capture-window storage and exposes no record fields.
pub struct RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1 {
    records: Vec<RustSdkStaticProviderBodyLocalProcessOutputObservationForProviderHostOwnerV1>,
}

/// Closed host-set selection for a child-output observation.  Ordinary run
/// sessions may omit the test family; an installed test family must consume
/// the exact observation set or fail typed.
pub enum RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1 {
    ObservedByBodyLocalStaticTestExecutor,
    BodyLocalStaticTestExecutorAbsent,
}

struct RustSdkStaticProviderBodyLocalProcessOutputObservationForProviderHostOwnerV1 {
    stream: RustSdkStaticProviderOutputEffectDrainStream,
    text: String,
}

enum RustSdkStaticProviderOutputEffectDrainReceiptInner {
    SwarmIoStream {
        provider_id: String,
        stream: RustSdkStaticProviderOutputEffectDrainStream,
        operation_export: String,
        rendered_value: String,
        emitted_text: String,
    },
}

enum RustSdkStaticProviderProcessOutputRecordInner {
    SwarmIoStream {
        provider_id: String,
        stream: RustSdkStaticProviderOutputEffectDrainStream,
        operation_export: String,
        rendered_value: String,
        emitted_text: String,
    },
}

#[derive(Clone, Copy)]
enum RustSdkStaticProviderOutputEffectDrainStream {
    Stdout,
    Stderr,
}

pub struct RustSdkStaticProviderOutputForSessionRuntimeOwnerV1 {
    inner: RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1,
}

enum RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1 {
    ClosedSum(RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner),
    InvocationResult(AdmittedProviderOperationInvocationResult),
    PlainContractOutput(RustSdkStaticProviderContractOutputForProviderHostOwner),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProviderHostContext {
    host_id: String,
}

#[derive(PartialEq, Eq)]
pub struct ProviderHostRequestAdmission {
    host_id: String,
    host_kind: String,
    provider_execution_domain: String,
    provider_count: usize,
}

pub struct RustSdkStaticProviderReadyOutputForProviderHostOwner {
    contract_output: RustSdkStaticProviderReadyContractOutputForProviderHostOwner,
}

pub struct RustSdkStaticProviderReadyContractOutputForProviderHostOwner {
    provider_id: String,
    contract: CapabilityContractIdentity,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    invocation_result: Option<AdmittedProviderOperationInvocationResult>,
    output_effect_drain_receipts:
        Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
    output_fingerprint: String,
    output: ProviderValue,
}

pub(crate) struct RustSdkStaticProviderContractOutputForProviderHostOwner {
    provider_id: String,
    contract: CapabilityContractIdentity,
    output_fingerprint: String,
    output: ProviderValue,
}

pub struct RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
    provider_id: String,
    contract: CapabilityContractIdentity,
    closed_sum_output_type: CapabilityContractClosedSumOutputTypeForProviderHostOwner,
    output_body: RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner,
}

#[derive(PartialEq, Eq)]
struct RustSdkStaticProviderClosedSumVariantForProviderHostOwner {
    variant: String,
    payload: Option<ProviderValue>,
}

#[derive(PartialEq, Eq)]
struct RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner {
    inner: RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner,
}

#[derive(PartialEq, Eq)]
enum RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner {
    Ok(ProviderValue),
    Err(ProviderValue),
}

enum RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner {
    Authored(RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner),
    StdResult(RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner),
}

struct RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner {
    provider_id: String,
    contract: CapabilityContractIdentity,
    output_fingerprint: String,
    output: ProviderValue,
}

pub struct RustSdkStaticProviderExecutionResultForProviderHostOwner {
    payload: RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner,
}

/// Opaque finite source for the ordered settlement stream of one mesh remote-
/// provider operation. The source retains the provider-host execution result;
/// no output value, fingerprint, JSON document, or caller-authored selector can
/// be used to reconstruct this authority.
///
/// Semantic Abstraction Gate:
/// - Unit: one remote-provider operation and its ordered settlement stream.
/// - Selected input: one sealed static-provider execution result.
/// - Receipt/Fault: a sealed item, done, or error settlement product.
/// - Private phases: ready-output custody and provider-host resource releases.
/// - Too low: a raw handle, fingerprint, JSON payload, or cursor bridge.
/// - Too high: a generic session/workflow settlement abstraction.
/// - First source edit: consume the execution result into this finite source at
///   the provider-host/mesh-host owner boundary.
///
/// Edit Gate / Repair Contract:
/// - Bucket: mesh remote-provider settlement-stream authority.
/// - Owner boundary: static provider host -> mesh capability host.
/// - Selected input: `RustSdkStaticProviderExecutionResultForProviderHostOwner`.
/// - Consuming owner: `MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1`.
/// - Output product/state: sealed Item then Done; Error remains a closed typed
///   settlement arm for provider-owned failure settlement.
/// - Final observation owner: mesh capability host.
/// - One-shot proof: the selected result is moved into `next_result` and taken
///   exactly once by `next_for_mesh_capability_host_owner_v1`.
/// - Forbidden old shape removed: consuming the result solely into a diagnostic
///   observation while discarding settlement custody.
/// - First stale caller now: mesh shell provider execution.
/// - Tripwire terms: raw mesh operation/stream selectors and JSON authority.
#[must_use = "a mesh provider settlement source must be retained and advanced by its operation owner"]
pub struct MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1 {
    next_result: Option<RustSdkStaticProviderExecutionResultForProviderHostOwner>,
}

/// Closed settlement algebra returned by one finite source advance. The tuple
/// products have private fields and cannot be caller-minted.
#[must_use = "a mesh provider settlement step must be consumed by the operation owner"]
pub enum MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1 {
    Item(MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1),
    Done(MeshProviderSettlementStreamDoneForMeshCapabilityHostOwnerV1),
    Error(MeshProviderSettlementStreamErrorForMeshCapabilityHostOwnerV1),
}

/// Sealed item settlement retaining the exact provider-host execution result.
/// Mesh may observe its diagnostic projection but cannot extract or recreate
/// provider output authority.
#[must_use = "a mesh provider settlement item must be consumed by the operation owner"]
pub struct MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1 {
    _execution_result: RustSdkStaticProviderExecutionResultForProviderHostOwner,
    observation: RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1,
}

/// Sealed terminal product for an exhausted settlement source.
#[must_use = "a mesh provider settlement done product must be consumed"]
pub struct MeshProviderSettlementStreamDoneForMeshCapabilityHostOwnerV1 {
    _private: (),
}

/// Sealed provider-owned error settlement. The constructor stays private so a
/// dependent crate cannot synthesize failure authority from diagnostic data.
#[must_use = "a mesh provider settlement error product must be consumed"]
pub struct MeshProviderSettlementStreamErrorForMeshCapabilityHostOwnerV1 {
    fault: MeshProviderSettlementStreamFaultForMeshCapabilityHostOwnerV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MeshProviderSettlementStreamFaultForMeshCapabilityHostOwnerV1 {
    ProviderSettlementFailed,
}

/// Opaque, move-only authority for releasing one provider-owned host resource.
/// The private closed algebra keeps host-specific routing and resource identity
/// inside the provider host; no handle/provider selector crosses this boundary.
// compiler-custody: symbol=SelectedProviderHostResourceReleaseV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="resource release lineage terminates in compiler runtime; exact first root-scope consumer edit: OneShotHostResourceFinalizationObligation::commit_exact_provider_release_for_session_execution_kernel_owner_v1 must consume its ticket"
#[must_use = "a selected provider host-resource release must be carried into the matching session lifecycle"]
pub struct SelectedProviderHostResourceReleaseV1 {
    inner: SelectedProviderHostResourceReleaseInnerV1,
}

// compiler-custody: symbol=SelectedProviderHostResourceReleaseInnerV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="resource release lineage terminates in compiler runtime; exact first root-scope consumer edit: OneShotHostResourceFinalizationObligation::commit_exact_provider_release_for_session_execution_kernel_owner_v1 must consume its ticket"
enum SelectedProviderHostResourceReleaseInnerV1 {
    StaticTest(SelectedStaticTestScopedResourceReleaseV1),
    Consumed,
}

/// Opaque exact proof minted only after the concrete provider owner changes
/// the selected resource from active to released.
// compiler-custody: symbol=ProviderHostResourceReleaseReceiptV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="resource release lineage terminates in compiler runtime; exact first root-scope consumer edit: OneShotHostResourceFinalizationObligation::commit_exact_provider_release_for_session_execution_kernel_owner_v1 must consume its ticket"
#[must_use = "a provider host-resource release receipt must be consumed by the matching session finalization"]
pub struct ProviderHostResourceReleaseReceiptV1 {
    inner: ProviderHostResourceReleaseReceiptInnerV1,
}

enum ProviderHostResourceReleaseReceiptInnerV1 {
    StaticTest(StaticTestScopedResourceReleaseReceiptV1),
}

/// Closed provider-owned release faults. No variant exposes resource routing
/// identity or accepts caller-authored settlement cargo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderHostResourceReleaseFaultV1 {
    OwnerUnavailable,
    ResourceNotActive,
    TransferAuthorityMissing,
    TransferAuthorityUnexpected,
}

impl fmt::Display for ProviderHostResourceReleaseFaultV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::OwnerUnavailable => "provider host-resource owner is unavailable",
            Self::ResourceNotActive => "provider host resource is not active",
            Self::TransferAuthorityMissing => {
                "provider output host-resource carrier has no exact release authority"
            }
            Self::TransferAuthorityUnexpected => {
                "provider ready result contains an unconsumed host-resource release authority"
            }
        })
    }
}

impl std::error::Error for ProviderHostResourceReleaseFaultV1 {}

/// A failed consuming release retains the exact selected authority so the VM
/// pending obligation can be restored rather than falsely cleared.
// compiler-custody: symbol=ProviderHostResourceReleaseRefusalV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="retaining release refusal returns to compiler runtime; exact first root-scope consumer edit: OneShotHostResourceFinalizationObligation::commit_exact_provider_release_for_session_execution_kernel_owner_v1 must retain its ticket"
#[must_use = "a refused provider host-resource release must restore its selected authority"]
pub struct ProviderHostResourceReleaseRefusalV1 {
    selected: SelectedProviderHostResourceReleaseV1,
    fault: ProviderHostResourceReleaseFaultV1,
}

impl ProviderHostResourceReleaseRefusalV1 {
    pub fn into_selected_and_fault_for_session_execution_kernel_owner_v1(
        self,
    ) -> (
        SelectedProviderHostResourceReleaseV1,
        ProviderHostResourceReleaseFaultV1,
    ) {
        (self.selected, self.fault)
    }
}

/// Opaque one-shot sidecar transferred with a selected ready output. Runtime
/// admission may consume an authority only by presenting the exact carrier
/// minted alongside it; raw selectors are compared only inside this owner.
#[must_use = "all provider host-resource release authorities must be consumed by ready-output admission"]
pub struct ProviderHostResourceReleaseTransferSetV1 {
    selected: Vec<SelectedProviderHostResourceReleaseV1>,
}

impl ProviderHostResourceReleaseTransferSetV1 {
    fn empty_for_static_provider_host_owner_v1() -> Self {
        Self {
            selected: Vec::new(),
        }
    }

    /// Empty release sidecar for a ready result produced by a closed
    /// kernel-internal provider route.  Kernel-internal operations cannot mint
    /// provider-host resource release authority.
    pub fn empty_for_kernel_internal_provider_route_owner_v1() -> Self {
        Self::empty_for_static_provider_host_owner_v1()
    }

    fn one_for_static_provider_host_owner_v1(
        selected: SelectedProviderHostResourceReleaseV1,
    ) -> Self {
        Self {
            selected: vec![selected],
        }
    }

    pub fn consume_exact_for_provider_output_carrier_for_session_execution_kernel_owner_v1(
        &mut self,
        carrier: &swarm_capability_linker_core::HostResourceHandleCarrier,
    ) -> Result<SelectedProviderHostResourceReleaseV1, ProviderHostResourceReleaseFaultV1> {
        let Some(index) = self.selected.iter().position(|selected| {
            selected.matches_provider_output_carrier_for_static_provider_host_owner_v1(carrier)
        }) else {
            return Err(ProviderHostResourceReleaseFaultV1::TransferAuthorityMissing);
        };
        Ok(self.selected.remove(index))
    }

    pub fn finish_for_session_execution_kernel_owner_v1(
        self,
    ) -> Result<(), ProviderHostResourceReleaseFaultV1> {
        if self.selected.is_empty() {
            Ok(())
        } else {
            Err(ProviderHostResourceReleaseFaultV1::TransferAuthorityUnexpected)
        }
    }
}

impl fmt::Debug for SelectedProviderHostResourceReleaseV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SelectedProviderHostResourceReleaseV1")
            .field("authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for ProviderHostResourceReleaseReceiptV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = &self.inner;
        formatter
            .debug_struct("ProviderHostResourceReleaseReceiptV1")
            .field("proof", &"sealed")
            .finish()
    }
}

impl SelectedProviderHostResourceReleaseV1 {
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn authority_state_for_test_support_v1(&self) -> &'static str {
        match &self.inner {
            SelectedProviderHostResourceReleaseInnerV1::StaticTest(_) => "static_test",
            SelectedProviderHostResourceReleaseInnerV1::Consumed => "consumed",
        }
    }

    fn matches_provider_output_carrier_for_static_provider_host_owner_v1(
        &self,
        carrier: &swarm_capability_linker_core::HostResourceHandleCarrier,
    ) -> bool {
        match &self.inner {
            SelectedProviderHostResourceReleaseInnerV1::StaticTest(selected) => {
                selected.matches_provider_output_carrier_for_static_provider_host_owner_v1(carrier)
            }
            SelectedProviderHostResourceReleaseInnerV1::Consumed => false,
        }
    }

    pub(crate) fn commit_release_borrowed_for_static_provider_host_set_owner_v1(
        &mut self,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseFaultV1> {
        let receipt = match &self.inner {
            SelectedProviderHostResourceReleaseInnerV1::StaticTest(selected) => selected
                .commit_release_borrowed_for_static_test_scoped_resource_owner_v1()
                .map(|receipt| ProviderHostResourceReleaseReceiptV1 {
                    inner: ProviderHostResourceReleaseReceiptInnerV1::StaticTest(receipt),
                }),
            SelectedProviderHostResourceReleaseInnerV1::Consumed => {
                Err(ProviderHostResourceReleaseFaultV1::ResourceNotActive)
            }
        }?;
        self.inner = SelectedProviderHostResourceReleaseInnerV1::Consumed;
        Ok(receipt)
    }

    pub(crate) fn commit_release_for_static_provider_host_set_owner_v1(
        self,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseRefusalV1> {
        match self.inner {
            SelectedProviderHostResourceReleaseInnerV1::StaticTest(selected) => {
                match selected.commit_release_for_static_test_scoped_resource_owner_v1() {
                    Ok(receipt) => Ok(ProviderHostResourceReleaseReceiptV1 {
                        inner: ProviderHostResourceReleaseReceiptInnerV1::StaticTest(receipt),
                    }),
                    Err((selected, fault)) => Err(ProviderHostResourceReleaseRefusalV1 {
                        selected: SelectedProviderHostResourceReleaseV1 {
                            inner: SelectedProviderHostResourceReleaseInnerV1::StaticTest(selected),
                        },
                        fault,
                    }),
                }
            }
            SelectedProviderHostResourceReleaseInnerV1::Consumed => {
                Err(ProviderHostResourceReleaseRefusalV1 {
                    selected: SelectedProviderHostResourceReleaseV1 {
                        inner: SelectedProviderHostResourceReleaseInnerV1::Consumed,
                    },
                    fault: ProviderHostResourceReleaseFaultV1::ResourceNotActive,
                })
            }
        }
    }
}

// compiler-custody: symbol=SelectedProviderBoundaryExecutionResultForProviderHostOwner disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="provider boundary lineage starts in compiler runtime; exact first root-scope edit: WorkRuntimeStores::commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1 must pass its ticket through host settlement"
#[must_use = "a selected provider-boundary execution result must be consumed into one corresponded ready output"]
pub struct SelectedProviderBoundaryExecutionResultForProviderHostOwner {
    result: RustSdkStaticProviderExecutionResultForProviderHostOwner,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
}

/// Rust-adapter output after every fallible output-shape and authored-result
/// admission step has completed. The provider-host-set owner may now consume
/// its retained selected-output authority through the infallible commit below.
#[must_use = "preflighted static-provider output must be committed by the provider-host-set owner"]
pub struct RustSdkStaticProviderPreflightedOutputForProviderHostSetOwnerV1 {
    settlement: RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1,
    output_effect_drain_receipts:
        Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
    host_resource_releases: ProviderHostResourceReleaseTransferSetV1,
}

enum RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1 {
    Authored(ProviderValue),
    Accepted(ProviderValue),
    Rejected(ProviderValue),
}

enum RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner {
    Ready {
        output: RustSdkStaticProviderReadyOutputForProviderHostOwner,
        host_resource_releases: ProviderHostResourceReleaseTransferSetV1,
    },
}

impl fmt::Debug for HostAdmittedTypedProviderRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostAdmittedTypedProviderRequest")
            .field("host_id", &self.host.host_id())
            .field(
                "provider_execution_domain",
                &self.host.provider_execution_domain(),
            )
            .field("provider_id", &self.binding.provider_id())
            .field(
                "contract_fingerprint",
                &self
                    .binding
                    .contract_projection_observation_for_static_provider_host_owner_v1()
                    .fingerprint(),
            )
            .field("provider_input", &"redacted")
            .field("hidden_provider_request_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderReadyOutputForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderReadyOutputForProviderHostOwner")
            .field("provider_id", &"redacted")
            .field("contract_fingerprint", &"redacted")
            .field("output_fingerprint", &"redacted")
            .field("provider_output", &"redacted")
            .field("hidden_provider_output_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderContractOutputForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderContractOutputForProviderHostOwner")
            .field("provider_id", &self.provider_id)
            .field("contract_fingerprint", &self.contract.fingerprint())
            .field("output_fingerprint", &"redacted")
            .field("provider_output", &"redacted")
            .field("hidden_provider_contract_output_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner")
            .field("provider_id", &self.provider_id)
            .field("contract_fingerprint", &self.contract.fingerprint())
            .field("output_fingerprint", &"redacted")
            .field("provider_output", &"redacted")
            .field("hidden_provider_closed_sum_output_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner")
            .field("provider_id", &self.provider_id)
            .field("contract_fingerprint", &self.contract.fingerprint())
            .field("output_fingerprint", &"redacted")
            .field("provider_output", &"redacted")
            .field(
                "hidden_provider_closed_sum_output_body_authority",
                &"redacted",
            )
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderClosedSumVariantForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderClosedSumVariantForProviderHostOwner")
            .field("variant", &"redacted")
            .field("payload", &"redacted")
            .field("hidden_provider_closed_sum_variant_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner")
            .field("std_result_payload", &"redacted")
            .field("hidden_provider_closed_sum_result_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for RustSdkStaticProviderExecutionResultForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug =
            formatter.debug_struct("RustSdkStaticProviderExecutionResultForProviderHostOwner");
        match &self.payload {
            RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner::Ready {
                output: _,
                host_resource_releases: _,
            } => {
                debug
                    .field("kind", &"ready")
                    .field("provider_id", &"redacted")
                    .field("output_fingerprint", &"redacted");
            }
        }
        debug
            .field(
                "hidden_static_provider_execution_result_authority",
                &"redacted",
            )
            .finish()
    }
}

impl fmt::Debug for MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1")
            .field("settlement_pending", &self.next_result.is_some())
            .field("provider_execution_authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1")
            .field("kind", &self.kind_tag_for_mesh_capability_host_owner_v1())
            .field("settlement_authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1")
            .field("provider_execution_authority", &"sealed")
            .field("observation", &"owner-observed")
            .finish()
    }
}

impl fmt::Debug for MeshProviderSettlementStreamDoneForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MeshProviderSettlementStreamDoneForMeshCapabilityHostOwnerV1")
            .field("terminal_authority", &"sealed")
            .finish()
    }
}

impl fmt::Debug for MeshProviderSettlementStreamErrorForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MeshProviderSettlementStreamErrorForMeshCapabilityHostOwnerV1")
            .field("fault", &self.fault)
            .field("error_authority", &"sealed")
            .finish()
    }
}

impl HostAdmittedTypedProviderRequest {
    pub fn implementation_output_for_rust_sdk_static_provider_executor_owner_v1(
        output: ProviderValue,
    ) -> RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
        RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
            output,
            output_effect_drain_receipts: Vec::new(),
            host_resource_releases:
                ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
        }
    }

    fn implementation_output_with_effect_drain_receipts_for_static_provider_host_owner_v1(
        output: ProviderValue,
        output_effect_drain_receipts: Vec<
            RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1,
        >,
    ) -> RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
        RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
            output,
            output_effect_drain_receipts,
            host_resource_releases:
                ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
        }
    }

    fn implementation_output_with_host_resource_release_for_static_provider_host_owner_v1(
        output: ProviderValue,
        selected_release: SelectedProviderHostResourceReleaseV1,
    ) -> RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
        RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
            output,
            output_effect_drain_receipts: Vec::new(),
            host_resource_releases:
                ProviderHostResourceReleaseTransferSetV1::one_for_static_provider_host_owner_v1(
                    selected_release,
                ),
        }
    }

    pub fn into_selected_provider_boundary_request_for_provider_host_set_owner_v1(
        self,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> SelectedProviderBoundaryHostRequest {
        SelectedProviderBoundaryHostRequest {
            request: SelectedProviderBoundaryTypedRequest::RustSdk(self),
            selected_output_authority,
        }
    }

    pub fn host(&self) -> &ProviderHostRequestAdmission {
        &self.host
    }

    pub fn provider_id(&self) -> &str {
        self.request.provider_id()
    }

    pub fn contract(&self) -> &CapabilityContractIdentity {
        self.request.contract()
    }

    pub fn into_output_type_contract_for_provider_drive_result_owner_v1(
        self,
    ) -> CapabilityContractOutputTypeContractAuthorityProduct {
        self.request.output_type_contract
    }

    pub fn provider_input(&self) -> &ProviderValue {
        self.request.input()
    }

    /// R-B OPT-3 (#127) — executor-boundary request-input adapter for the five
    /// @swarm/mesh DIRECT-RUN executors.
    ///
    /// The direct-run family materializes an op call as POSITIONAL command
    /// arguments: `provider_input = Array([arg0])` — exactly one command
    /// argument, itself the request-input object. The mesh-control ops read a
    /// BARE request-input object (`request_input_object`, kept byte-identical
    /// and shared unchanged with the service-loop frame path). This consumes
    /// the request and returns it with its input normalized to that single
    /// command argument: the sealed route authority (host, binding, output-type
    /// contract, invocation) is MOVED through unchanged; only the RAW input
    /// payload shape is adapted. No authority is minted, reconstructed, routed,
    /// resumed, settled, or forged, and the linear `self` consumption keeps the
    /// normalization one-shot.
    ///
    /// A request whose input is not exactly one command argument (empty,
    /// multiple arguments, or a single non-object argument where an object is
    /// required) faults with the existing SDK-boundary typed error. There is no
    /// new fault vocabulary, no fallback, and no dual-convention pass-through.
    ///
    /// DATED BRIDGE: when R-C (#128) first exercises the service-loop frame path
    /// (giving that path an oracle), ONE request-input convention wins and this
    /// adapter DIES; it is not a permanent shim.
    pub fn into_single_command_argument_request_input_normalized_for_mesh_direct_run_executor_owner_v1(
        self,
    ) -> CapabilitySdkResult<Self> {
        let Self {
            host,
            binding,
            request,
        } = self;
        let TypedProviderRequest {
            provider_id,
            contract,
            operation,
            output_type_contract,
            input,
            invocation,
        } = request;
        let normalized_input = match input {
            ProviderValue::Array(arguments) if arguments.len() == 1 => {
                match arguments.into_iter().next() {
                    Some(object @ ProviderValue::Object(_)) => object,
                    _ => {
                        return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                            format!(
                                "@swarm/mesh direct-run provider {provider_id} single command \
                                 argument must be an object",
                            ),
                        ));
                    }
                }
            }
            _ => {
                return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    format!(
                        "@swarm/mesh direct-run provider {provider_id} request input must materialize \
                     exactly one command argument",
                    ),
                ));
            }
        };
        Ok(Self {
            host,
            binding,
            request: TypedProviderRequest {
                provider_id,
                contract,
                operation,
                output_type_contract,
                input: normalized_input,
                invocation,
            },
        })
    }

    pub fn into_ready_output_for_rust_sdk_static_provider_executor_owner_v1(
        self,
        output: ProviderValue,
    ) -> CapabilitySdkResult<RustSdkStaticProviderReadyOutputForProviderHostOwner> {
        self.into_ready_output_with_effect_drain_receipts_for_rust_sdk_static_provider_executor_owner_v1(
            output,
            Vec::new(),
        )
    }

    fn into_ready_output_with_effect_drain_receipts_for_rust_sdk_static_provider_executor_owner_v1(
        self,
        output: ProviderValue,
        output_effect_drain_receipts: Vec<
            RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1,
        >,
    ) -> CapabilitySdkResult<RustSdkStaticProviderReadyOutputForProviderHostOwner> {
        let Self {
            host: _,
            binding: _,
            request,
        } = self;
        TypedProviderOutputSettlementAuthority::from_typed_request_for_provider_host_owner_v1(
            request,
        )
        .into_ready_output_for_provider_host_owner_v1(output, output_effect_drain_receipts)
    }

    pub fn into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(
        self,
        output: ProviderValue,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let output =
            self.into_ready_output_for_rust_sdk_static_provider_executor_owner_v1(output)?;
        Ok(
            RustSdkStaticProviderExecutionResultForProviderHostOwner::ready_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
            ),
        )
    }

    fn into_execution_result_from_implementation_output_for_static_provider_host_owner_v1(
        self,
        implementation_output: RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
            output,
            output_effect_drain_receipts,
            host_resource_releases,
        } = implementation_output;
        let output = self
            .into_ready_output_with_effect_drain_receipts_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                output_effect_drain_receipts,
            )?;
        Ok(
            RustSdkStaticProviderExecutionResultForProviderHostOwner::ready_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                host_resource_releases,
            ),
        )
    }

    fn into_execution_result_with_host_resource_release_for_static_provider_host_owner_v1(
        self,
        output: ProviderValue,
        selected_release: SelectedProviderHostResourceReleaseV1,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let output =
            self.into_ready_output_for_rust_sdk_static_provider_executor_owner_v1(output)?;
        Ok(
            RustSdkStaticProviderExecutionResultForProviderHostOwner::ready_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                ProviderHostResourceReleaseTransferSetV1::one_for_static_provider_host_owner_v1(
                    selected_release,
                ),
            ),
        )
    }

    fn into_execution_result_with_effect_drain_receipts_for_rust_sdk_static_provider_executor_owner_v1(
        self,
        output: ProviderValue,
        output_effect_drain_receipts: Vec<
            RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1,
        >,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let output =
            self.into_ready_output_with_effect_drain_receipts_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                output_effect_drain_receipts,
            )?;
        Ok(
            RustSdkStaticProviderExecutionResultForProviderHostOwner::ready_for_rust_sdk_static_provider_executor_owner_v1(
                output,
                ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
            ),
        )
    }

    pub fn diagnostic_value(&self) -> CapabilitySdkResult<Value> {
        Ok(json!({
            "schema": "swarm.rust_sdk_static_provider_host.host_admitted_typed_provider_request.diagnostic.v1",
            "host": self.host.diagnostic_value(),
            "request": self.request.diagnostic_value()?,
        }))
    }

    pub fn typed_request_diagnostic_value(&self) -> CapabilitySdkResult<Value> {
        self.request.diagnostic_value()
    }
}

impl RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1 {
    pub fn authored_output_for_rust_sdk_provider_adapter_owner_v1(output: ProviderValue) -> Self {
        Self {
            output,
            output_effect_drain_receipts: Vec::new(),
            host_resource_releases:
                ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
        }
    }
}

impl fmt::Debug for SelectedProviderBoundaryHostRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let route = match &self.request {
            SelectedProviderBoundaryTypedRequest::RustSdk(_) => "rust_sdk",
            SelectedProviderBoundaryTypedRequest::ManifestResolvedExternal { .. } => {
                "manifest_resolved_external"
            }
        };
        formatter
            .debug_struct("SelectedProviderBoundaryHostRequest")
            .field("route", &route)
            .field("request", &"<sealed>")
            .field("selected_output_authority", &"<sealed>")
            .finish()
    }
}

impl SelectedProviderBoundaryHostRequest {
    pub fn from_manifest_resolved_external_call_admission_for_provider_host_set_owner_v1(
        admission: ManifestResolvedExternalProviderCallAdmission,
        input: ProviderValue,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> CapabilitySdkResult<Self> {
        // The prepared-runtime owner already joined the private route operation
        // to this Contract-TSON. Keep that proof whole across the public host-set
        // aperture; splitting the two arguments there would permit a same-contract
        // Contract-TSON for a different operation to be substituted here.
        let (call_authority, contract) =
            admission.into_call_authority_and_contract_for_provider_host_set_owner_v1();
        let contract_identity = contract.identity().duplicate_for_capability_model_owner();
        if call_authority.contract_for_provider_host_set_owner_v1() != &contract_identity {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                "manifest-resolved external call authority does not match selected Contract-TSON identity"
                    .to_owned(),
            ));
        }
        let provider_id = contract_identity.provider_id();
        let (operation, output_type_contract) = contract
            .into_operation_and_output_type_contract_authority_for_provider_host_owner_v1()
            .map_err(|source| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                    "manifest-resolved external provider Contract-TSON operation admission failed: {source}"
                ))
            })?;
        Ok(Self {
            request: SelectedProviderBoundaryTypedRequest::ManifestResolvedExternal {
                call_authority,
                request: TypedProviderRequest {
                    provider_id,
                    contract: contract_identity,
                    operation,
                    output_type_contract,
                    input,
                    invocation: None,
                },
            },
            selected_output_authority,
        })
    }

    pub fn provider_id(&self) -> &str {
        match &self.request {
            SelectedProviderBoundaryTypedRequest::RustSdk(request) => request.provider_id(),
            SelectedProviderBoundaryTypedRequest::ManifestResolvedExternal { request, .. } => {
                request.provider_id()
            }
        }
    }

    pub fn provider_input(&self) -> &ProviderValue {
        match &self.request {
            SelectedProviderBoundaryTypedRequest::RustSdk(request) => request.provider_input(),
            SelectedProviderBoundaryTypedRequest::ManifestResolvedExternal { request, .. } => {
                request.input()
            }
        }
    }

    pub fn into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(
        self,
        output: ProviderValue,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        let Self {
            request,
            selected_output_authority,
        } = self;
        let SelectedProviderBoundaryTypedRequest::RustSdk(request) = request else {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                "manifest-resolved external request cannot be settled by a Rust SDK executor"
                    .to_owned(),
            ));
        };
        let result =
            request.into_execution_result_for_rust_sdk_static_provider_executor_owner_v1(output)?;
        Ok(
            SelectedProviderBoundaryExecutionResultForProviderHostOwner {
                result,
                selected_output_authority,
            },
        )
    }

    fn into_request_and_selected_output_authority_for_static_provider_host_owner_v1(
        self,
    ) -> CapabilitySdkResult<(
        HostAdmittedTypedProviderRequest,
        SelectedProviderBoundaryOutputAuthority,
    )> {
        let SelectedProviderBoundaryTypedRequest::RustSdk(request) = self.request else {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                "manifest-resolved external request cannot enter the Rust SDK host set".to_owned(),
            ));
        };
        Ok((request, self.selected_output_authority))
    }

    pub fn into_route_for_provider_host_set_owner_v1(
        self,
    ) -> SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1 {
        let Self {
            request,
            selected_output_authority,
        } = self;
        match request {
            SelectedProviderBoundaryTypedRequest::RustSdk(request) => {
                SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1::RustSdk(Self {
                    request: SelectedProviderBoundaryTypedRequest::RustSdk(request),
                    selected_output_authority,
                })
            }
            SelectedProviderBoundaryTypedRequest::ManifestResolvedExternal {
                call_authority,
                request,
            } => {
                let TypedProviderRequest {
                    provider_id,
                    contract,
                    operation: _,
                    output_type_contract,
                    input,
                    invocation,
                } = request;
                SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1::ManifestResolvedExternal(
                    DurableExternalProviderInvocationAuthority {
                        call_authority,
                        provider_input: input,
                        output_settlement: DurableExternalProviderOutputSettlementAuthority {
                            request: TypedProviderOutputSettlementAuthority {
                                provider_id,
                                contract,
                                output_type_contract,
                                invocation,
                            },
                            selected_output_authority,
                        },
                    },
                )
            }
        }
    }
}

impl DurableExternalProviderInvocationAuthority {
    pub fn into_call_input_and_output_settlement_for_durable_external_provider_owner_v1(
        self,
    ) -> (
        ManifestResolvedExternalProviderCallAuthority,
        ProviderValue,
        DurableExternalProviderOutputSettlementAuthority,
    ) {
        (
            self.call_authority,
            self.provider_input,
            self.output_settlement,
        )
    }
}

impl DurableExternalProviderOutputSettlementAuthority {
    pub fn settle_ready_for_durable_external_provider_owner_v1(
        self,
        output: ProviderValue,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        let ready_output = self
            .request
            .into_ready_output_for_provider_host_owner_v1(output, Vec::new())?;
        let result = RustSdkStaticProviderExecutionResultForProviderHostOwner::ready_for_rust_sdk_static_provider_executor_owner_v1(
            ready_output,
            ProviderHostResourceReleaseTransferSetV1::empty_for_static_provider_host_owner_v1(),
        );
        Ok(
            SelectedProviderBoundaryExecutionResultForProviderHostOwner {
                result,
                selected_output_authority: self.selected_output_authority,
            },
        )
    }
}

impl ProviderHostContext {
    fn new(host_id: impl Into<String>) -> CapabilitySdkResult<Self> {
        let host_id = host_id.into();
        require_trimmed_nonblank_for_context("provider host id", &host_id)?;
        Ok(Self { host_id })
    }

    pub fn from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
        request: &HostAdmittedTypedProviderRequest,
    ) -> CapabilitySdkResult<Self> {
        Self::new(request.host().host_id().to_owned())
    }

    pub fn from_static_provider_host_set_owner_contract_v1(
        host_set: &RustSdkStaticProviderHostSet,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<Self> {
        host_set.require_exact_contract_v1(contract)?;
        Self::new(host_set.host_id().to_owned())
    }

    pub fn from_native_provider_host_set_owner_admission_v1(
        admission: &NativeProviderInstalledHostRequestAdmission,
    ) -> CapabilitySdkResult<Self> {
        Self::new(admission.host.host_id().to_owned())
    }

    pub fn duplicate_for_mesh_capability_host_owner_v1(&self) -> Self {
        Self {
            host_id: self.host_id.clone(),
        }
    }

    pub fn host_id(&self) -> &str {
        &self.host_id
    }
}

impl fmt::Debug for ProviderHostRequestAdmission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderHostRequestAdmission")
            .field("host_id", &self.host_id)
            .field("host_kind", &self.host_kind)
            .field("provider_execution_domain", &self.provider_execution_domain)
            .field("provider_count", &self.provider_count)
            .field("hidden_provider_host_request_authority", &"redacted")
            .finish()
    }
}

impl ProviderHostRequestAdmission {
    fn new(
        host_id: impl Into<String>,
        host_kind: impl Into<String>,
        provider_execution_domain: impl Into<String>,
        provider_count: usize,
    ) -> CapabilitySdkResult<Self> {
        let host_id = host_id.into();
        require_trimmed_nonblank_for_context("provider host request host_id", &host_id)?;
        let host_kind = host_kind.into();
        require_trimmed_nonblank_for_context("provider host request host_kind", &host_kind)?;
        let provider_execution_domain = provider_execution_domain.into();
        require_trimmed_nonblank_for_context(
            "provider host request provider_execution_domain",
            &provider_execution_domain,
        )?;
        Ok(Self {
            host_id,
            host_kind,
            provider_execution_domain,
            provider_count,
        })
    }

    fn rust_sdk_static_provider_host_for_static_provider_host_owner_v1(
        provider_count: usize,
    ) -> CapabilitySdkResult<Self> {
        Self::new(
            RUST_SDK_PROVIDER_HOST_ID,
            RUST_SDK_PROVIDER_HOST_KIND,
            RUST_SDK_PROVIDER_DOMAIN,
            provider_count,
        )
    }

    fn loaded_native_provider_host_for_static_provider_host_owner_v1<A>(
        admission: &A,
    ) -> CapabilitySdkResult<Self>
    where
        A: RustSdkStaticProviderInstalledNativeHostAdmission + ?Sized,
    {
        Self::new(
            admission.host_id_for_static_provider_host_owner_v1(),
            LOADED_NATIVE_PROVIDER_HOST_KIND,
            RUST_SDK_PROVIDER_DOMAIN,
            admission.provider_count_for_static_provider_host_owner_v1(),
        )
    }

    pub fn host_id(&self) -> &str {
        &self.host_id
    }

    pub fn host_kind(&self) -> &str {
        &self.host_kind
    }

    pub fn provider_execution_domain(&self) -> &str {
        &self.provider_execution_domain
    }

    pub fn provider_count(&self) -> usize {
        self.provider_count
    }

    fn duplicate_for_static_provider_host_owner_v1(&self) -> Self {
        Self {
            host_id: self.host_id.clone(),
            host_kind: self.host_kind.clone(),
            provider_execution_domain: self.provider_execution_domain.clone(),
            provider_count: self.provider_count,
        }
    }

    pub fn diagnostic_value(&self) -> Value {
        json!({
            "schema": "swarm.rust_sdk_static_provider_host.provider_host_request_admission.diagnostic.v1",
            "host_id": self.host_id,
            "host_kind": self.host_kind,
            "provider_execution_domain": self.provider_execution_domain,
            "provider_count": self.provider_count,
        })
    }
}

fn require_trimmed_nonblank_for_context(label: &str, value: &str) -> CapabilitySdkResult<()> {
    if value.trim().is_empty() || value.trim() != value {
        return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
            format!("{label} must be nonblank and trimmed"),
        ));
    }
    Ok(())
}

pub struct RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
    output_fingerprint: String,
    diagnostic_value: Value,
}

impl std::fmt::Debug for RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1")
            .field("output_fingerprint", &"redacted")
            .field("diagnostic_value", &"owner-observed")
            .finish()
    }
}

impl RustSdkStaticProviderReadyOutputForProviderHostOwner {
    fn provider_id(&self) -> &str {
        self.contract_output.provider_id()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        self.contract_output.contract()
    }

    fn output_fingerprint_for_provider_host_owner_v1(&self) -> &str {
        self.contract_output
            .output_fingerprint_for_provider_host_owner_v1()
    }

    fn diagnostic_value(&self) -> Value {
        self.contract_output.diagnostic_value()
    }

    pub fn observation_for_mesh_capability_host_owner_v1(
        &self,
    ) -> RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
        RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
            output_fingerprint: self
                .output_fingerprint_for_provider_host_owner_v1()
                .to_owned(),
            diagnostic_value: self.diagnostic_value(),
        }
    }

    pub fn require_provider_contract_fingerprint_for_mesh_capability_host_owner_v1(
        &self,
        expected_provider_id: &str,
        expected_contract_fingerprint: &str,
    ) -> CapabilitySdkResult<RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1>
    {
        if self.provider_id() != expected_provider_id {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                format!(
                    "mesh ready output provider mismatch: expected {expected_provider_id}, observed {}",
                    self.provider_id()
                ),
            ));
        }
        let observed_contract_fingerprint = self
            .contract()
            .fingerprint()
            .map(|fingerprint| fingerprint.as_str())
            .unwrap_or("<none>");
        if observed_contract_fingerprint != expected_contract_fingerprint {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                format!(
                    "mesh ready output contract fingerprint mismatch: expected {expected_contract_fingerprint}, observed {observed_contract_fingerprint}",
                ),
            ));
        }
        Ok(self.observation_for_mesh_capability_host_owner_v1())
    }

    pub fn into_contract_output_for_provider_drive_result_owner_v1(
        self,
    ) -> RustSdkStaticProviderReadyContractOutputForProviderHostOwner {
        self.contract_output
    }
}

impl RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
    pub fn output_fingerprint_for_mesh_capability_host_owner_v1(&self) -> &str {
        self.output_fingerprint.as_str()
    }

    pub fn diagnostic_value_for_mesh_capability_host_owner_v1(&self) -> Value {
        self.diagnostic_value.clone()
    }
}

impl RustSdkStaticProviderReadyContractOutputForProviderHostOwner {
    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        &self.contract
    }

    fn output_fingerprint_for_provider_host_owner_v1(&self) -> &str {
        self.output_fingerprint.as_str()
    }

    fn diagnostic_value(&self) -> Value {
        json!({
            "schema": "swarm.rust_sdk_static_provider_host.contract_output.diagnostic.v1",
            "provider_id": self.provider_id,
            "contract": self.contract.projection(),
            "output_fingerprint": "redacted",
            "provider_output": "redacted",
        })
    }

    pub fn into_static_provider_output_and_effect_drain_receipts_for_provider_drive_result_owner_v1(
        self,
    ) -> Result<
        (
            RustSdkStaticProviderOutputForSessionRuntimeOwnerV1,
            Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
        ),
        CapabilitySdkError,
    > {
        let Self {
            provider_id,
            contract,
            output_type_contract,
            invocation_result,
            output_effect_drain_receipts,
            output_fingerprint,
            output,
        } = self;
        if let Some(invocation_result) = invocation_result {
            return Ok((
                RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_invocation_result_for_provider_drive_result_owner_v1(
                    invocation_result,
                ),
                output_effect_drain_receipts,
            ));
        }
        match output_type_contract.into_closed_sum_output_type_for_provider_host_owner_v1() {
            Ok(closed_sum_output_type) => {
                Ok((
                    RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_closed_sum_for_provider_drive_result_owner_v1(
                        Self::closed_sum_contract_output_with_type_for_provider_host_owner_v1(
                            provider_id,
                            contract,
                            output_fingerprint,
                            output,
                            closed_sum_output_type,
                        ),
                    ),
                    output_effect_drain_receipts,
                ))
            }
            Err(CapabilityTypeContractError::OutputTypeNotClosedSum) => {
                Ok((
                    RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_plain_contract_output_for_provider_drive_result_owner_v1(
                    RustSdkStaticProviderContractOutputForProviderHostOwner {
                        provider_id,
                        contract,
                        output_fingerprint,
                        output,
                    },
                    ),
                    output_effect_drain_receipts,
                ))
            }
            Err(source) => Err(CapabilitySdkError::ProviderHostReadyOutputTypeAdmission { source }),
        }
    }

    fn closed_sum_contract_output_with_type_for_provider_host_owner_v1(
        provider_id: String,
        contract: CapabilityContractIdentity,
        output_fingerprint: String,
        output: ProviderValue,
        closed_sum_output_type: CapabilityContractClosedSumOutputTypeForProviderHostOwner,
    ) -> RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
        let output_body =
            RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner::from_provider_host_owner_contract_output_v1(
                provider_id.clone(),
                contract.duplicate_for_capability_model_owner(),
                output_fingerprint,
                output,
            );
        RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
            provider_id,
            contract,
            closed_sum_output_type,
            output_body,
        }
    }

    fn into_closed_sum_contract_output_for_provider_host_owner_v1(
        self,
    ) -> Result<
        RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner,
        CapabilityTypeContractError,
    > {
        let Self {
            provider_id,
            contract,
            output_type_contract,
            invocation_result: _,
            output_effect_drain_receipts: _,
            output_fingerprint,
            output,
        } = self;
        let closed_sum_output_type =
            output_type_contract.into_closed_sum_output_type_for_provider_host_owner_v1()?;
        Ok(
            Self::closed_sum_contract_output_with_type_for_provider_host_owner_v1(
                provider_id,
                contract,
                output_fingerprint,
                output,
                closed_sum_output_type,
            ),
        )
    }
}

impl RustSdkStaticProviderContractOutputForProviderHostOwner {
    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        &self.contract
    }

    fn output_fingerprint_for_provider_host_owner_v1(&self) -> &str {
        self.output_fingerprint.as_str()
    }

    fn diagnostic_value(&self) -> Value {
        json!({
            "schema": "swarm.rust_sdk_static_provider_host.contract_output.diagnostic.v1",
            "provider_id": self.provider_id,
            "contract": self.contract.projection(),
            "output_fingerprint": "redacted",
            "provider_output": "redacted",
        })
    }
}

impl RustSdkStaticProviderOutputForSessionRuntimeOwnerV1 {
    fn from_closed_sum_for_provider_drive_result_owner_v1(
        contract_output: RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner,
    ) -> Self {
        Self {
            inner: RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::ClosedSum(
                contract_output,
            ),
        }
    }

    fn from_invocation_result_for_provider_drive_result_owner_v1(
        result: AdmittedProviderOperationInvocationResult,
    ) -> Self {
        Self {
            inner: RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::InvocationResult(
                result,
            ),
        }
    }

    fn from_plain_contract_output_for_provider_drive_result_owner_v1(
        contract_output: RustSdkStaticProviderContractOutputForProviderHostOwner,
    ) -> Self {
        Self {
            inner: RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::PlainContractOutput(
                contract_output,
            ),
        }
    }

    fn into_provider_ready_boundary_output_for_selected_boundary_owner_v1(
        self,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> CapabilitySdkResult<ProviderReadyBoundaryOutput> {
        match self.inner {
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::ClosedSum(output) => {
                match output.into_settlement_for_session_work_runtime_owner_v1()? {
                    RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::Authored(
                        output,
                    ) => Ok(selected_output_authority
                        .admit_ready_output_for_provider_host_owner_v1(output.output)),
                    RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::StdResult(
                        result,
                    ) => Ok(result
                        .into_ready_boundary_output_for_session_work_runtime_owner_v1(
                            selected_output_authority,
                        )),
                }
            }
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::InvocationResult(result) => {
                Ok(selected_output_authority.admit_accepted_result_for_provider_host_owner_v1(
                    result.into_provider_value_for_swarmvm_session_runtime_owner_v1(),
                ))
            }
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::PlainContractOutput(
                output,
            ) => Ok(selected_output_authority
                .admit_ready_output_for_provider_host_owner_v1(output.output)),
        }
    }

    fn into_preflighted_settlement_for_provider_host_set_owner_v1(
        self,
    ) -> CapabilitySdkResult<
        RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1,
    > {
        match self.inner {
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::ClosedSum(output) => {
                match output.into_settlement_for_session_work_runtime_owner_v1()? {
                    RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::Authored(
                        output,
                    ) => Ok(
                        RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Authored(
                            output.output,
                        ),
                    ),
                    RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::StdResult(
                        result,
                    ) => match result.inner {
                        RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Ok(
                            output,
                        ) => Ok(
                            RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Accepted(
                                output,
                            ),
                        ),
                        RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Err(
                            output,
                        ) => Ok(
                            RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Rejected(
                                output,
                            ),
                        ),
                    },
                }
            }
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::InvocationResult(result) => {
                Ok(
                    RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Accepted(
                        result.into_provider_value_for_swarmvm_session_runtime_owner_v1(),
                    ),
                )
            }
            RustSdkStaticProviderOutputForSessionRuntimeOwnerInnerV1::PlainContractOutput(
                output,
            ) => Ok(
                RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Authored(
                    output.output,
                ),
            ),
        }
    }
}

impl RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        &self.contract
    }

    fn output_fingerprint_for_provider_host_owner_v1(&self) -> &str {
        self.output_body
            .output_fingerprint_for_provider_host_owner_v1()
    }

    fn diagnostic_value(&self) -> Value {
        json!({
            "schema": "swarm.rust_sdk_static_provider_host.closed_sum_contract_output.diagnostic.v1",
            "provider_id": self.provider_id,
            "contract": self.contract.projection(),
            "output_fingerprint": "redacted",
            "provider_output": "redacted",
        })
    }

    fn into_output_body_for_provider_host_owner_v1(
        self,
    ) -> RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner {
        self.output_body
    }

    fn into_std_result_closed_sum_for_session_work_runtime_owner_v1(
        self,
    ) -> CapabilitySdkResult<RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner>
    {
        let Self {
            provider_id,
            contract: _,
            closed_sum_output_type,
            output_body,
        } = self;
        let variant = output_body
            .into_std_result_variant_for_provider_host_owner_v1(closed_sum_output_type)?;
        RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner::from_provider_host_owner_variant_v1(provider_id, variant)
    }

    fn into_settlement_for_session_work_runtime_owner_v1(
        self,
    ) -> CapabilitySdkResult<RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner>
    {
        let Self {
            provider_id,
            contract,
            closed_sum_output_type,
            output_body,
        } = self;
        match closed_sum_output_type.into_exact_std_result_for_provider_host_owner_v1() {
            Ok(closed_sum_output_type) => Self {
                provider_id,
                contract,
                closed_sum_output_type,
                output_body,
            }
            .into_std_result_closed_sum_for_session_work_runtime_owner_v1()
            .map(RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::StdResult),
            Err(_) => Ok(
                RustSdkStaticProviderClosedSumSettlementForSessionWorkRuntimeOwner::Authored(
                    output_body,
                ),
            ),
        }
    }
}

impl RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner {
    fn from_provider_host_owner_contract_output_v1(
        provider_id: String,
        contract: CapabilityContractIdentity,
        output_fingerprint: String,
        output: ProviderValue,
    ) -> Self {
        Self {
            provider_id,
            contract,
            output_fingerprint,
            output,
        }
    }

    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn contract(&self) -> &CapabilityContractIdentity {
        &self.contract
    }

    fn output_fingerprint_for_provider_host_owner_v1(&self) -> &str {
        self.output_fingerprint.as_str()
    }

    fn diagnostic_value_for_provider_host_owner_projection_v1(&self) -> Value {
        json!({
            "schema": "swarm.rust_sdk_static_provider_host.closed_sum_output_body.diagnostic.v1",
            "provider_id": self.provider_id,
            "contract": self.contract.projection(),
            "output_fingerprint": "redacted",
            "provider_output": "redacted",
        })
    }

    fn into_closed_sum_variant_for_provider_host_owner_v1(
        self,
        closed_sum_output_type: CapabilityContractClosedSumOutputTypeForProviderHostOwner,
        owner_expected_symbol_path: &str,
    ) -> CapabilitySdkResult<RustSdkStaticProviderClosedSumVariantForProviderHostOwner> {
        let ProviderValue::Object(mut fields) = self.output else {
            return Err(
                CapabilitySdkError::ProviderHostClosedSumOutputNotCarrierObject {
                    provider_id: self.provider_id,
                },
            );
        };
        let expected_keys = [
            swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD,
            swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD,
            swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD,
            swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD,
        ];
        if fields.len() != expected_keys.len()
            || !expected_keys.iter().all(|key| fields.contains_key(*key))
        {
            return Err(
                CapabilitySdkError::ProviderHostClosedSumOutputCarrierMalformed {
                    provider_id: self.provider_id,
                },
            );
        }
        let symbol_path = match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD) {
            Some(ProviderValue::String(symbol_path)) => symbol_path,
            _ => {
                return Err(
                    CapabilitySdkError::ProviderHostClosedSumOutputSymbolMalformed {
                        provider_id: self.provider_id,
                    },
                );
            }
        };
        if symbol_path != owner_expected_symbol_path {
            return Err(
                CapabilitySdkError::ProviderHostClosedSumOutputSymbolMismatch {
                    provider_id: self.provider_id,
                    expected_symbol_path: owner_expected_symbol_path.to_owned(),
                    observed_symbol_path: symbol_path,
                },
            );
        }
        let variant = match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD) {
            Some(ProviderValue::String(variant)) if !variant.trim().is_empty() => variant,
            _ => {
                return Err(
                    CapabilitySdkError::ProviderHostClosedSumOutputVariantMalformed {
                        provider_id: self.provider_id,
                    },
                );
            }
        };
        let provider_id = self.provider_id.clone();
        let variant_admission = closed_sum_output_type
            .admit_variant_for_provider_host_owner_v1(variant)
            .map_err(|source| {
                CapabilitySdkError::ProviderHostClosedSumOutputVariantNotDeclared {
                    provider_id,
                    source,
                }
            })?;
        let variant = variant_admission.into_variant_label_for_provider_host_owner_v1();
        let has_payload =
            match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD) {
                Some(ProviderValue::Bool(has_payload)) => has_payload,
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputPayloadFlagMalformed {
                            provider_id: self.provider_id,
                        },
                    );
                }
            };
        let payload = fields
            .remove(swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD)
            .ok_or_else(
                || CapabilitySdkError::ProviderHostClosedSumOutputPayloadMissing {
                    provider_id: self.provider_id.clone(),
                },
            )?;
        let payload = if has_payload {
            Some(payload)
        } else {
            match payload {
                ProviderValue::Null => None,
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputUnitPayloadMalformed {
                            provider_id: self.provider_id,
                        },
                    );
                }
            }
        };
        Ok(RustSdkStaticProviderClosedSumVariantForProviderHostOwner { variant, payload })
    }

    fn into_std_result_variant_for_provider_host_owner_v1(
        self,
        closed_sum_output_type: CapabilityContractClosedSumOutputTypeForProviderHostOwner,
    ) -> CapabilitySdkResult<RustSdkStaticProviderClosedSumVariantForProviderHostOwner> {
        let Self {
            provider_id,
            contract: _,
            output_fingerprint: _,
            output,
        } = self;
        let ProviderValue::Object(mut fields) = output else {
            return Err(
                CapabilitySdkError::ProviderHostClosedSumOutputNotCarrierObject { provider_id },
            );
        };

        let (variant_label, payload) = if fields
            .contains_key(swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD)
        {
            let expected_keys = [
                swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD,
                swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD,
                swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD,
                swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD,
            ];
            if fields.len() != expected_keys.len()
                || !expected_keys.iter().all(|key| fields.contains_key(*key))
            {
                return Err(
                    CapabilitySdkError::ProviderHostClosedSumOutputCarrierMalformed { provider_id },
                );
            }
            match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD) {
                Some(ProviderValue::String(symbol_path))
                    if symbol_path == SESSION_WORK_RUNTIME_STD_RESULT_CLOSED_SUM_SYMBOL => {}
                Some(ProviderValue::String(observed_symbol_path)) => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputSymbolMismatch {
                            provider_id,
                            expected_symbol_path: SESSION_WORK_RUNTIME_STD_RESULT_CLOSED_SUM_SYMBOL
                                .to_owned(),
                            observed_symbol_path,
                        },
                    );
                }
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputSymbolMalformed {
                            provider_id,
                        },
                    );
                }
            }
            let variant_label =
                match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD) {
                    Some(ProviderValue::String(variant))
                        if variant == SESSION_WORK_RUNTIME_STD_RESULT_OK_VARIANT =>
                    {
                        EXTERNAL_PROVIDER_STD_RESULT_OK_VARIANT.to_owned()
                    }
                    Some(ProviderValue::String(variant))
                        if variant == SESSION_WORK_RUNTIME_STD_RESULT_ERR_VARIANT =>
                    {
                        EXTERNAL_PROVIDER_STD_RESULT_ERR_VARIANT.to_owned()
                    }
                    Some(ProviderValue::String(variant)) => {
                        return Err(
                            CapabilitySdkError::ProviderHostStdResultClosedSumVariantUnsupported {
                                provider_id,
                                variant,
                            },
                        );
                    }
                    _ => {
                        return Err(
                            CapabilitySdkError::ProviderHostClosedSumOutputVariantMalformed {
                                provider_id,
                            },
                        );
                    }
                };
            let provider_id_for_admission = provider_id.clone();
            let variant_label = closed_sum_output_type
                .admit_variant_for_provider_host_owner_v1(variant_label)
                .map_err(|source| {
                    CapabilitySdkError::ProviderHostClosedSumOutputVariantNotDeclared {
                        provider_id: provider_id_for_admission,
                        source,
                    }
                })?
                .into_variant_label_for_provider_host_owner_v1();
            match fields.remove(swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD) {
                Some(ProviderValue::Bool(true)) => {}
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputPayloadFlagMalformed {
                            provider_id,
                        },
                    );
                }
            }
            let payload = fields
                .remove(swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD)
                .ok_or_else(
                    || CapabilitySdkError::ProviderHostClosedSumOutputPayloadMissing {
                        provider_id: provider_id.clone(),
                    },
                )?;
            (variant_label, payload)
        } else {
            if fields.len() != 2 {
                return Err(
                    CapabilitySdkError::ProviderHostClosedSumOutputCarrierMalformed { provider_id },
                );
            }
            let variant_label = match fields.remove("kind") {
                Some(ProviderValue::String(variant)) if !variant.trim().is_empty() => variant,
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostClosedSumOutputVariantMalformed {
                            provider_id,
                        },
                    );
                }
            };
            let provider_id_for_admission = provider_id.clone();
            let variant_label = closed_sum_output_type
                .admit_variant_for_provider_host_owner_v1(variant_label)
                .map_err(|source| {
                    CapabilitySdkError::ProviderHostClosedSumOutputVariantNotDeclared {
                        provider_id: provider_id_for_admission,
                        source,
                    }
                })?
                .into_variant_label_for_provider_host_owner_v1();
            let payload_field = match variant_label.as_str() {
                EXTERNAL_PROVIDER_STD_RESULT_OK_VARIANT => "value",
                EXTERNAL_PROVIDER_STD_RESULT_ERR_VARIANT => "error",
                _ => {
                    return Err(
                        CapabilitySdkError::ProviderHostStdResultClosedSumVariantUnsupported {
                            provider_id,
                            variant: variant_label,
                        },
                    );
                }
            };
            let payload = fields.remove(payload_field).ok_or_else(|| {
                CapabilitySdkError::ProviderHostClosedSumOutputPayloadMissing {
                    provider_id: provider_id.clone(),
                }
            })?;
            if !fields.is_empty() {
                return Err(
                    CapabilitySdkError::ProviderHostClosedSumOutputCarrierMalformed { provider_id },
                );
            }
            (variant_label, payload)
        };
        Ok(RustSdkStaticProviderClosedSumVariantForProviderHostOwner {
            variant: variant_label,
            payload: Some(payload),
        })
    }
}

impl RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1 {
    fn swarm_io_stream_for_static_provider_host_owner_v1(
        provider_id: impl Into<String>,
        stream: RustSdkStaticProviderOutputEffectDrainStream,
        operation_export: impl Into<String>,
        rendered_value: impl Into<String>,
        emitted_text: impl Into<String>,
    ) -> Self {
        Self {
            inner: RustSdkStaticProviderOutputEffectDrainReceiptInner::SwarmIoStream {
                provider_id: provider_id.into(),
                stream,
                operation_export: operation_export.into(),
                rendered_value: rendered_value.into(),
                emitted_text: emitted_text.into(),
            },
        }
    }

    pub fn into_direct_run_output_drain_observation_for_provider_drive_result_owner_v1(
        self,
    ) -> serde_json::Value {
        self.into_direct_run_output_drain_observation_and_process_output_record_for_provider_drive_result_owner_v1()
            .0
    }

    pub fn into_direct_run_output_drain_observation_and_process_output_record_for_provider_drive_result_owner_v1(
        self,
    ) -> (
        serde_json::Value,
        RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1,
    ) {
        match self.inner {
            RustSdkStaticProviderOutputEffectDrainReceiptInner::SwarmIoStream {
                provider_id,
                stream,
                operation_export,
                rendered_value,
                emitted_text,
            } => {
                let stream_observation = match stream {
                    RustSdkStaticProviderOutputEffectDrainStream::Stdout => "stdout",
                    RustSdkStaticProviderOutputEffectDrainStream::Stderr => "stderr",
                };
                let observation = serde_json::json!({
                    "schema": "swarm.provider_drive_result.output_effect_drain_receipt.swarm_io_stream.v1",
                    "providerId": provider_id.as_str(),
                    "stream": stream_observation,
                    "operationExport": operation_export.as_str(),
                    "renderedValue": rendered_value.as_str(),
                });
                let record = RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1 {
                    _inner: RustSdkStaticProviderProcessOutputRecordInner::SwarmIoStream {
                        provider_id,
                        stream,
                        operation_export,
                        rendered_value,
                        emitted_text,
                    },
                };
                (observation, record)
            }
        }
    }
}

impl RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1 {
    pub fn into_body_local_process_output_observation_for_provider_drive_result_owner_v1(
        self,
    ) -> serde_json::Value {
        match self._inner {
            RustSdkStaticProviderProcessOutputRecordInner::SwarmIoStream {
                provider_id: _,
                stream,
                operation_export,
                rendered_value,
                emitted_text: _,
            } => {
                let stream_observation = match stream {
                    RustSdkStaticProviderOutputEffectDrainStream::Stdout => "stdout",
                    RustSdkStaticProviderOutputEffectDrainStream::Stderr => "stderr",
                };
                serde_json::json!({
                    "schema": "swarm.ss.test.body_local.process_output_record.v1",
                    "stream": stream_observation,
                    "operationExport": operation_export.as_str(),
                    "renderedValue": rendered_value.as_str(),
                })
            }
        }
    }
}

impl RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1 {
    pub fn from_exact_process_output_records_for_provider_drive_result_owner_v1<'a>(
        records: impl IntoIterator<
            Item = &'a RustSdkStaticProviderProcessOutputRecordForProviderHostOwnerV1,
        >,
    ) -> Self {
        let records = records
            .into_iter()
            .map(|record| match &record._inner {
                RustSdkStaticProviderProcessOutputRecordInner::SwarmIoStream {
                    stream,
                    emitted_text,
                    ..
                } => RustSdkStaticProviderBodyLocalProcessOutputObservationForProviderHostOwnerV1 {
                    stream: *stream,
                    text: emitted_text.clone(),
                },
            })
            .collect();
        Self { records }
    }

    pub(crate) fn into_records_for_static_test_provider_executor_owner_v1(
        self,
    ) -> Vec<RustSdkStaticProviderBodyLocalProcessOutputObservationForProviderHostOwnerV1> {
        self.records
    }
}

impl RustSdkStaticProviderStdResultClosedSumForSessionWorkRuntimeOwner {
    fn from_provider_host_owner_variant_v1(
        provider_id: String,
        variant: RustSdkStaticProviderClosedSumVariantForProviderHostOwner,
    ) -> CapabilitySdkResult<Self> {
        let RustSdkStaticProviderClosedSumVariantForProviderHostOwner { variant, payload } =
            variant;
        let payload = payload.ok_or_else(|| {
            CapabilitySdkError::ProviderHostStdResultClosedSumPayloadMissing {
                provider_id: provider_id.clone(),
            }
        })?;
        let inner = match variant.as_str() {
            EXTERNAL_PROVIDER_STD_RESULT_OK_VARIANT => {
                RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Ok(payload)
            }
            EXTERNAL_PROVIDER_STD_RESULT_ERR_VARIANT => {
                RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Err(payload)
            }
            _ => {
                return Err(
                    CapabilitySdkError::ProviderHostStdResultClosedSumVariantUnsupported {
                        provider_id,
                        variant,
                    },
                );
            }
        };
        Ok(Self { inner })
    }

    fn into_ready_boundary_output_for_session_work_runtime_owner_v1(
        self,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> ProviderReadyBoundaryOutput {
        match self.inner {
            RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Ok(payload) => {
                selected_output_authority.admit_accepted_result_for_provider_host_owner_v1(payload)
            }
            RustSdkStaticProviderStdResultClosedSumInnerForSessionWorkRuntimeOwner::Err(
                payload,
            ) => {
                selected_output_authority.admit_rejected_result_for_provider_host_owner_v1(payload)
            }
        }
    }
}

impl RustSdkStaticProviderExecutionResultForProviderHostOwner {
    fn ready_for_rust_sdk_static_provider_executor_owner_v1(
        output: RustSdkStaticProviderReadyOutputForProviderHostOwner,
        host_resource_releases: ProviderHostResourceReleaseTransferSetV1,
    ) -> Self {
        Self {
            payload: RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner::Ready {
                output,
                host_resource_releases,
            },
        }
    }

    fn into_ready_output_for_static_provider_host_owner_v1(
        self,
    ) -> (
        RustSdkStaticProviderReadyOutputForProviderHostOwner,
        ProviderHostResourceReleaseTransferSetV1,
    ) {
        match self.payload {
            RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner::Ready {
                output,
                host_resource_releases,
            } => (output, host_resource_releases),
        }
    }

    pub fn preflight_output_for_provider_host_set_owner_v1(
        self,
    ) -> CapabilitySdkResult<
        RustSdkStaticProviderPreflightedOutputForProviderHostSetOwnerV1,
    > {
        let (ready_output, host_resource_releases) =
            self.into_ready_output_for_static_provider_host_owner_v1();
        let (output, output_effect_drain_receipts) = ready_output
            .into_contract_output_for_provider_drive_result_owner_v1()
            .into_static_provider_output_and_effect_drain_receipts_for_provider_drive_result_owner_v1()?;
        let settlement = output
            .into_preflighted_settlement_for_provider_host_set_owner_v1()?;
        Ok(
            RustSdkStaticProviderPreflightedOutputForProviderHostSetOwnerV1 {
                settlement,
                output_effect_drain_receipts,
                host_resource_releases,
            },
        )
    }

    /// OBS accessor for the mesh-capability-host owner: the ready-output
    /// observation of this SEALED execution result, borrowed so the owner can
    /// derive service-loop diagnostics without consuming the authority. This is
    /// observation only — it can never feed authority minting/routing.
    pub fn ready_output_observation_for_mesh_capability_host_owner_v1(
        &self,
    ) -> RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
        match &self.payload {
            RustSdkStaticProviderExecutionResultPayloadForProviderHostOwner::Ready {
                output,
                host_resource_releases: _,
            } => output.observation_for_mesh_capability_host_owner_v1(),
        }
    }

    /// Consumes the selected provider result into its mesh response observation
    /// and the matching sealed settlement-stream source. The observation is
    /// inert; only the source retains settlement authority.
    pub fn into_ready_output_observation_and_mesh_provider_settlement_stream_source_for_mesh_capability_host_owner_v1(
        self,
    ) -> (
        RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1,
        MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1,
    ) {
        let observation = self.ready_output_observation_for_mesh_capability_host_owner_v1();
        let source = MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1 {
            next_result: Some(self),
        };
        (observation, source)
    }
}

impl MeshProviderSettlementStreamSourceForMeshCapabilityHostOwnerV1 {
    /// Advances this finite source exactly once to Item and thereafter to Done.
    /// The item retains the exact execution result; only its inert diagnostic
    /// observation is exposed to the mesh capability host.
    pub fn next_for_mesh_capability_host_owner_v1(
        &mut self,
    ) -> MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1 {
        let Some(execution_result) = self.next_result.take() else {
            return MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1::Done(
                MeshProviderSettlementStreamDoneForMeshCapabilityHostOwnerV1 { _private: () },
            );
        };
        let observation =
            execution_result.ready_output_observation_for_mesh_capability_host_owner_v1();
        MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1::Item(
            MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1 {
                _execution_result: execution_result,
                observation,
            },
        )
    }
}

impl MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1 {
    pub fn kind_tag_for_mesh_capability_host_owner_v1(&self) -> &'static str {
        match self {
            Self::Item(_) => "item",
            Self::Done(_) => "done",
            Self::Error(_) => "error",
        }
    }
}

impl MeshProviderSettlementStreamItemForMeshCapabilityHostOwnerV1 {
    pub fn output_observation_for_mesh_capability_host_owner_v1(
        &self,
    ) -> &RustSdkStaticProviderReadyOutputObservationForMeshCapabilityHostOwnerV1 {
        &self.observation
    }
}

impl SelectedProviderBoundaryExecutionResultForProviderHostOwner {
    fn from_static_provider_host_owner_v1(
        result: RustSdkStaticProviderExecutionResultForProviderHostOwner,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> Self {
        Self {
            result,
            selected_output_authority,
        }
    }

    pub fn into_provider_ready_boundary_output_and_effect_drain_receipts_for_provider_drive_result_owner_v1(
        self,
    ) -> CapabilitySdkResult<(
        ProviderReadyBoundaryOutput,
        Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
        ProviderHostResourceReleaseTransferSetV1,
    )> {
        let Self { result, selected_output_authority } = self;
        Ok(result
            .preflight_output_for_provider_host_set_owner_v1()?
            .commit_with_selected_output_authority_for_provider_host_set_owner_v1(
                selected_output_authority,
            ))
    }
}

impl RustSdkStaticProviderPreflightedOutputForProviderHostSetOwnerV1 {
    pub fn commit_with_selected_output_authority_for_provider_host_set_owner_v1(
        self,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> (
        ProviderReadyBoundaryOutput,
        Vec<RustSdkStaticProviderOutputEffectDrainReceiptForProviderHostOwnerV1>,
        ProviderHostResourceReleaseTransferSetV1,
    ) {
        let Self {
            settlement,
            output_effect_drain_receipts,
            host_resource_releases,
        } = self;
        let ready_output = match settlement {
            RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Authored(
                output,
            ) => selected_output_authority
                .admit_ready_output_for_provider_host_owner_v1(output),
            RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Accepted(
                output,
            ) => selected_output_authority
                .admit_accepted_result_for_provider_host_owner_v1(output),
            RustSdkStaticProviderPreflightedSettlementForProviderHostSetOwnerV1::Rejected(
                output,
            ) => selected_output_authority
                .admit_rejected_result_for_provider_host_owner_v1(output),
        };
        (
            ready_output,
            output_effect_drain_receipts,
            host_resource_releases,
        )
    }
}

fn provider_output_fingerprint_v1(
    provider_id: &str,
    contract: &CapabilityContractIdentity,
    output: &ProviderValue,
) -> CapabilitySdkResult<String> {
    let output_canonical_json = provider_value_to_canonical_output_observation_json_v1(output)
        .map_err(|source| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "typed provider output canonicalization failed for {provider_id}: {source}"
            ))
        })?;
    let contract_fingerprint = contract
        .fingerprint()
        .map(|fingerprint| fingerprint.as_str())
        .unwrap_or("<none>");
    let mut hasher = Sha256::new();
    hasher.update(b"swarm.rust_sdk_static_provider_host.ready_output.v1\0");
    hasher.update(provider_id.as_bytes());
    hasher.update(b"\0");
    hasher.update(contract_fingerprint.as_bytes());
    hasher.update(b"\0");
    hasher.update(output_canonical_json.as_bytes());
    Ok(format!(
        "sha256:rust-sdk-static-provider-ready-output:{:x}",
        hasher.finalize()
    ))
}

use std::{fmt, sync::Arc};

use swarm_capability_linker_core::{
    CapabilityContractOutputTypeContractAuthorityProduct, CapabilityTypeContractError,
    ProviderValue,
};
use swarm_capability_model::{
    MatchedProviderBoundaryOutputAuthority, PendingProviderBoundaryOutputCommitAuthority,
    ProviderBoundaryOutputAuthorityJoin, SelectedProviderBoundaryOutputAuthority,
};
use swarmvm_isa_types::authority_ids::RegisterIndex;

use crate::session::execution_kernel::executable_image::ExecutableCursorTarget;

/// Private correspondence identity for one live process-invoke execution.
/// Pointer identity is observed only by the finite join below; no string,
/// numeric key, source descriptor, or provider metadata exists to project.
struct ProcessInvokeExecutionSeal;

/// Private correspondence identity for one live process-run child.
struct ProcessRunChildSeal;

/// Exact correspondence for one outward invoke-await transition.  The
/// session half retains the result producer and continuation; the direct-run
/// half retains the selected execution frontier.  Neither half exposes an id
/// from which the other could be reconstructed.
// compiler-custody: symbol=ProcessInvokeAwaitTransitionSeal disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct ProcessInvokeAwaitTransitionSeal;

/// Exact correspondence for one outward process-run terminal transition.
// compiler-custody: symbol=ProcessRunDriveTransitionSeal disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct ProcessRunDriveTransitionSeal;

/// Private one-shot correspondence for an owner-native process-control
/// request and its durable-owner receipt.
// compiler-custody: symbol=ProcessControlTransitionSeal disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct ProcessControlTransitionSeal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessControlKindForDirectRunOwnerV1 {
    TerminateGracefully,
    TerminateForcefully,
    Interrupt,
}

impl ProcessControlKindForDirectRunOwnerV1 {
    pub fn stable_tag_for_direct_run_owner_v1(self) -> &'static str {
        match self {
            Self::TerminateGracefully => "terminate_gracefully",
            Self::TerminateForcefully => "terminate_forcefully",
            Self::Interrupt => "interrupt",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessControlChildPolicyForDirectRunOwnerV1 {
    FailIfChildren,
    Cascade,
    TransferToInit,
}

impl ProcessControlChildPolicyForDirectRunOwnerV1 {
    pub fn stable_tag_for_direct_run_owner_v1(self) -> &'static str {
        match self {
            Self::FailIfChildren => "fail_if_children",
            Self::Cascade => "cascade",
            Self::TransferToInit => "transfer_to_init",
        }
    }
}

/// Session ingress half minted together with the durable registration for one
/// process-invoke execution. It can only become the matching private
/// `RuntimeValue` carrier.
#[must_use = "a process-invoke provider output must enter its selected session boundary"]
pub struct ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1 {
    seal: Arc<ProcessInvokeExecutionSeal>,
}

/// Durable registration half for one process-invoke execution. Durable-direct-
/// run retains this value beside the child execution it owns; it has no raw
/// identity projection.
#[must_use = "a process-invoke execution registration must be retained by its durable owner"]
pub struct ProcessInvokeExecutionRegistrationForDirectRunOwnerV1 {
    seal: Arc<ProcessInvokeExecutionSeal>,
}

/// Selected process-invoke carrier moved out of a checker-proven runtime
/// receiver. Its only operation is the exact registration join below.
#[must_use = "a selected process-invoke execution must be joined to its durable registration"]
// compiler-custody: symbol=SelectedProcessInvokeExecutionForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(crate) struct SelectedProcessInvokeExecutionForDirectRunOwnerV1 {
    seal: Arc<ProcessInvokeExecutionSeal>,
}

/// Successful one-shot correspondence between a selected runtime carrier and
/// the durable registration born with it.
pub(crate) struct MatchedProcessInvokeExecutionForDirectRunOwnerV1 {
    _seal: Arc<ProcessInvokeExecutionSeal>,
}

/// Non-destructive finite join. An unmatched durable registry row returns both
/// inputs so the registry owner can continue searching without reconstructing
/// either side.
pub(crate) enum ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1 {
    Joined(MatchedProcessInvokeExecutionForDirectRunOwnerV1),
    Unmatched {
        selected: SelectedProcessInvokeExecutionForDirectRunOwnerV1,
        registration: ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    },
}

/// Session ingress half minted together with the durable registration for one
/// process-run child.
#[must_use = "a process-run provider output must enter its selected session boundary"]
pub struct ProcessRunChildProviderOutputForDirectRunOwnerV1 {
    seal: Arc<ProcessRunChildSeal>,
    process: Option<swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1>,
}

/// Durable registration half retained beside the exact child run.
#[must_use = "a process-run child registration must be retained by its durable owner"]
pub struct ProcessRunChildRegistrationForDirectRunOwnerV1 {
    seal: Arc<ProcessRunChildSeal>,
}

/// Complete nominal ingress for the result of the exact kernel-internal
/// `process.invoke` provider command. The process carrier, selected provider
/// boundary half, and admitted output-type contract remain fused until the
/// originating session consumes all three.
#[must_use = "a process-invoke nominal provider ingress must enter its exact pending provider boundary"]
pub struct ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1 {
    output: ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
}

/// Complete nominal ingress for the result of the exact kernel-internal
/// `process.run` provider command.
#[must_use = "a process-run nominal provider ingress must enter its exact pending provider boundary"]
pub struct ProcessRunChildProviderIngressForDirectRunOwnerV1 {
    output: ProcessRunChildProviderOutputForDirectRunOwnerV1,
    selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
}

pub(crate) struct MatchedProcessInvokeExecutionProviderIngressForSessionExecutionKernelOwnerV1 {
    _provider_boundary: MatchedProviderBoundaryOutputAuthority,
    output: ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
}

pub(crate) struct MatchedProcessRunChildProviderIngressForSessionExecutionKernelOwnerV1 {
    _provider_boundary: MatchedProviderBoundaryOutputAuthority,
    output: ProcessRunChildProviderOutputForDirectRunOwnerV1,
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
}

pub(crate) enum ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1 {
    Joined(MatchedProcessInvokeExecutionProviderIngressForSessionExecutionKernelOwnerV1),
    Unmatched {
        pending_output_authority: PendingProviderBoundaryOutputCommitAuthority,
        ingress: ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    },
}

pub(crate) enum ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1 {
    Joined(MatchedProcessRunChildProviderIngressForSessionExecutionKernelOwnerV1),
    Unmatched {
        pending_output_authority: PendingProviderBoundaryOutputCommitAuthority,
        ingress: ProcessRunChildProviderIngressForDirectRunOwnerV1,
    },
}

/// Selected process-run child carrier moved out of a checker-proven receiver.
#[must_use = "a selected process-run child must be joined to its durable registration"]
// compiler-custody: symbol=SelectedProcessRunChildForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(crate) struct SelectedProcessRunChildForDirectRunOwnerV1 {
    seal: Arc<ProcessRunChildSeal>,
}

pub(crate) struct MatchedProcessRunChildForDirectRunOwnerV1 {
    _seal: Arc<ProcessRunChildSeal>,
}

pub(crate) enum ProcessRunChildRegistrationJoinForDirectRunOwnerV1 {
    Joined(MatchedProcessRunChildForDirectRunOwnerV1),
    Unmatched {
        selected: SelectedProcessRunChildForDirectRunOwnerV1,
        registration: ProcessRunChildRegistrationForDirectRunOwnerV1,
    },
}

#[must_use = "a selected process-invoke frontier must be joined and resumed exactly once"]
// compiler-custody: symbol=SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
    selected_execution: SelectedProcessInvokeExecutionForDirectRunOwnerV1,
    transition_seal: Arc<ProcessInvokeAwaitTransitionSeal>,
}

#[must_use = "a selected process-run child must be joined and resumed exactly once"]
// compiler-custody: symbol=SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
    selected_child: SelectedProcessRunChildForDirectRunOwnerV1,
    transition_seal: Arc<ProcessRunDriveTransitionSeal>,
}

#[must_use = "a selected process-control request must be consumed and resumed exactly once"]
// compiler-custody: symbol=SelectedProcessControlBoundaryForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct SelectedProcessControlBoundaryForDirectRunOwnerV1 {
    process: swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
    control: ProcessControlKindForDirectRunOwnerV1,
    children: ProcessControlChildPolicyForDirectRunOwnerV1,
    reason: Option<String>,
    transition_seal: Arc<ProcessControlTransitionSeal>,
}

#[must_use = "a process-control completion must return its owner-issued observation"]
pub struct ProcessControlCompletionForDirectRunOwnerV1 {
    control: ProcessControlKindForDirectRunOwnerV1,
    children: ProcessControlChildPolicyForDirectRunOwnerV1,
    reason: Option<String>,
    transition_seal: Arc<ProcessControlTransitionSeal>,
}

/// Registry-owned successful join. The matched frontier and transition seal
/// stay fused, so independently successful joins cannot swap resume authority.
#[must_use = "a matched invoke-await boundary must admit exactly one result"]
pub struct MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
    _matched_execution: MatchedProcessInvokeExecutionForDirectRunOwnerV1,
    transition_seal: Arc<ProcessInvokeAwaitTransitionSeal>,
}

#[must_use = "a matched process-run boundary must admit exactly one terminal"]
pub struct MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
    _matched_child: MatchedProcessRunChildForDirectRunOwnerV1,
    transition_seal: Arc<ProcessRunDriveTransitionSeal>,
}

pub enum ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1 {
    Joined(MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1),
    Unmatched {
        boundary: SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
        registration: ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    },
}

pub enum ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1 {
    Joined(MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1),
    Unmatched {
        boundary: SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
        registration: ProcessRunChildRegistrationForDirectRunOwnerV1,
    },
}

/// Exact resume product returned by durable-direct-run after driving the
/// corresponded invoke execution to its awaited frontier.
#[must_use = "an invoke-await resume product must re-enter its originating session"]
// compiler-custody: symbol=ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1 {
    seal: Arc<ProcessInvokeAwaitTransitionSeal>,
    result: ProviderValue,
}

/// Exact resume product returned after driving the corresponded child run to
/// terminal settlement.
#[must_use = "a process-run terminal resume product must re-enter its originating session"]
// compiler-custody: symbol=ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1 {
    seal: Arc<ProcessRunDriveTransitionSeal>,
    terminal: ProviderValue,
}

#[must_use = "a process-control resume product must re-enter its originating session"]
// compiler-custody: symbol=ProcessControlResumeProductForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct ProcessControlResumeProductForDirectRunOwnerV1 {
    seal: Arc<ProcessControlTransitionSeal>,
    receipt: ProviderValue,
}

/// Public, retry-preserving failures for one invoke-await resume.  The three
/// pre-consume refusals return the complete resume product; after exact
/// correspondence succeeds, cargo admission and execution commit are
/// deliberately terminal because the one-shot transition has been consumed.
#[must_use = "an invoke-await resume failure must be handled, and retryable cargo must not be dropped"]
pub enum ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1 {
    NoPendingBoundary {
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    },
    DifferentPendingBoundary {
        pending_kind: &'static str,
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    },
    CorrespondenceMismatch {
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    },
    Commit {
        source: ProcessKernelBoundaryResumeCommitFaultV1,
    },
    Drive {
        source: crate::ProcessSessionRunError,
    },
}

/// Public, retry-preserving failures for one process-run terminal resume.
#[must_use = "a process-run terminal resume failure must be handled, and retryable cargo must not be dropped"]
pub enum ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1 {
    NoPendingBoundary {
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    },
    DifferentPendingBoundary {
        pending_kind: &'static str,
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    },
    CorrespondenceMismatch {
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    },
    Commit {
        source: ProcessKernelBoundaryResumeCommitFaultV1,
    },
    Drive {
        source: crate::ProcessSessionRunError,
    },
}

#[must_use = "a process-control resume failure must preserve retryable cargo"]
pub enum ProcessControlResumeDriveFailureForDirectRunOwnerV1 {
    NoPendingBoundary {
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    },
    DifferentPendingBoundary {
        pending_kind: &'static str,
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    },
    CorrespondenceMismatch {
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    },
    Commit {
        source: ProcessKernelBoundaryResumeCommitFaultV1,
    },
    Drive {
        source: crate::ProcessSessionRunError,
    },
}

/// Retry-preserving failure while committing the nominal output of the exact
/// kernel-internal `process.invoke` command into its selected provider frame.
#[must_use = "a process-invoke ingress failure must be handled, and retryable ingress must not be dropped"]
pub enum ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1 {
    NoPendingBoundary {
        ingress: ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    },
    DifferentPendingBoundary {
        pending_kind: &'static str,
        ingress: ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    },
    CorrespondenceMismatch {
        ingress: ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1,
    },
    Commit {
        source: ProcessNominalProviderIngressCommitFaultV1,
    },
    Drive {
        source: crate::ProcessSessionRunError,
    },
}

/// Retry-preserving sibling for the nominal `process.run` child carrier.
#[must_use = "a process-run ingress failure must be handled, and retryable ingress must not be dropped"]
pub enum ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1 {
    NoPendingBoundary {
        ingress: ProcessRunChildProviderIngressForDirectRunOwnerV1,
    },
    DifferentPendingBoundary {
        pending_kind: &'static str,
        ingress: ProcessRunChildProviderIngressForDirectRunOwnerV1,
    },
    CorrespondenceMismatch {
        ingress: ProcessRunChildProviderIngressForDirectRunOwnerV1,
    },
    Commit {
        source: ProcessNominalProviderIngressCommitFaultV1,
    },
    Drive {
        source: crate::ProcessSessionRunError,
    },
}

/// Terminal typed fault after the exact provider-boundary correspondence has
/// been consumed. Before this point every refusal returns the complete ingress.
pub struct ProcessNominalProviderIngressCommitFaultV1 {
    source: ProcessNominalProviderIngressCommitFaultSourceV1,
}

enum ProcessNominalProviderIngressCommitFaultSourceV1 {
    OutputContract(CapabilityTypeContractError),
    ClosedSumOutputContract,
    ExecutionCommit(crate::ProviderBoundaryExecutionCommitFault),
}

/// Opaque terminal fault after an exact process boundary has been consumed.
/// Register indices, cursor coordinates, and private correspondence identity
/// remain inside their owners and cannot be projected by direct-run.
pub struct ProcessKernelBoundaryResumeCommitFaultV1 {
    source: ProcessKernelBoundaryResumeCommitFaultSourceV1,
}

enum ProcessKernelBoundaryResumeCommitFaultSourceV1 {
    ResultAdmission(crate::VmRuntimeHeapAllocationError),
    ExecutionCommit(crate::session::execution_state::ExecutionStateFault),
    ProviderResultCommit(crate::session::execution_state::ProviderExecutionFrameFault),
}

/// Session-private half of an invoke-await transition.  The producer and
/// continuation are born in one mint with the selected receiver/frontier
/// boundary; neither can be replayed independently.
pub(crate) struct PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
    seal: Arc<ProcessInvokeAwaitTransitionSeal>,
    producer: ProcessInvokeAwaitExecutionResultProducer,
    continuation: ExecutableCursorTarget,
}

/// Session-private half of a process-run terminal transition.
pub(crate) struct PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
    seal: Arc<ProcessRunDriveTransitionSeal>,
    producer: ProcessRunDriveTerminalResultProducer,
    continuation: ExecutableCursorTarget,
}

pub(crate) struct PendingProcessControlResumeForSessionExecutionKernelOwnerV1 {
    seal: Arc<ProcessControlTransitionSeal>,
    result_mode: swarmvm_isa_types::HostActivityResultMode,
    result_commit: crate::session::execution_kernel::provider_effect_runtime::ProviderEffectExecutableResultCommitForSessionExecutionKernelOwnerV1,
    continuation: ExecutableCursorTarget,
}

/// Destination producer for the invoke result.  It is not a general register
/// selector: the constructor is private and only checker-prepared invoke work
/// can mint it after frame preflight.
pub(crate) struct ProcessInvokeAwaitExecutionResultProducer {
    destination: RegisterIndex,
}

/// Destination producer for the ProcessRun terminal.
pub(crate) struct ProcessRunDriveTerminalResultProducer {
    destination: RegisterIndex,
}

pub(crate) struct MatchedProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) producer: ProcessInvokeAwaitExecutionResultProducer,
    pub(crate) continuation: ExecutableCursorTarget,
    pub(crate) result: ProviderValue,
}

pub(crate) struct MatchedProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) producer: ProcessRunDriveTerminalResultProducer,
    pub(crate) continuation: ExecutableCursorTarget,
    pub(crate) terminal: ProviderValue,
}

pub(crate) struct MatchedProcessControlResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) result_mode: swarmvm_isa_types::HostActivityResultMode,
    pub(crate) result_commit: crate::session::execution_kernel::provider_effect_runtime::ProviderEffectExecutableResultCommitForSessionExecutionKernelOwnerV1,
    pub(crate) continuation: ExecutableCursorTarget,
    pub(crate) receipt: ProviderValue,
}

pub(crate) enum ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1 {
    Joined(MatchedProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1),
    Unmatched {
        pending: PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1,
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    },
}

pub(crate) enum ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1 {
    Joined(MatchedProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1),
    Unmatched {
        pending: PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1,
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    },
}

pub(crate) enum ProcessControlResumeJoinForSessionExecutionKernelOwnerV1 {
    Joined(MatchedProcessControlResumeForSessionExecutionKernelOwnerV1),
    Unmatched {
        pending: PendingProcessControlResumeForSessionExecutionKernelOwnerV1,
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    },
}

/// Kernel-private live value. Only the corresponded provider ingress can mint
/// it and only the process-invoke boundary can select it.
pub(crate) struct ProcessInvokeExecutionRuntimeCarrier {
    seal: Arc<ProcessInvokeExecutionSeal>,
}

/// Kernel-private live value for one process-run child.
pub(crate) struct ProcessRunChildRuntimeCarrier {
    seal: Arc<ProcessRunChildSeal>,
    process: Option<swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1>,
}

/// Exact nominal process output after the pending/selected provider-boundary
/// join and output-contract admission have both succeeded. The carrier remains
/// in its native process domain until the checked executable local takes it;
/// it never becomes provider-input-shaped cargo and exposes no constructor
/// from an ordinary runtime value.
#[must_use = "a matched nominal process output must enter its exact one-take executable local"]
pub(crate) struct MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1 {
    carrier: MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1,
}

enum MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1 {
    Invoke(ProcessInvokeExecutionRuntimeCarrier),
    Run(ProcessRunChildRuntimeCarrier),
}

pub fn mint_process_invoke_execution_carrier_for_durable_direct_run_owner_v1() -> (
    ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
    ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessInvokeExecutionSeal);
    (
        ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1 {
            seal: Arc::clone(&seal),
        },
        ProcessInvokeExecutionRegistrationForDirectRunOwnerV1 { seal },
    )
}

pub fn mint_process_run_child_carrier_for_durable_direct_run_owner_v1() -> (
    ProcessRunChildProviderOutputForDirectRunOwnerV1,
    ProcessRunChildRegistrationForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessRunChildSeal);
    (
        ProcessRunChildProviderOutputForDirectRunOwnerV1 {
            seal: Arc::clone(&seal),
            process: None,
        },
        ProcessRunChildRegistrationForDirectRunOwnerV1 { seal },
    )
}

pub fn mint_process_run_child_carrier_with_process_for_durable_direct_run_owner_v1(
    process: swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
) -> (
    ProcessRunChildProviderOutputForDirectRunOwnerV1,
    ProcessRunChildRegistrationForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessRunChildSeal);
    (
        ProcessRunChildProviderOutputForDirectRunOwnerV1 {
            seal: Arc::clone(&seal),
            process: Some(process),
        },
        ProcessRunChildRegistrationForDirectRunOwnerV1 { seal },
    )
}

impl ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1 {
    pub(crate) fn into_runtime_carrier_for_session_execution_kernel_owner_v1(
        self,
    ) -> ProcessInvokeExecutionRuntimeCarrier {
        ProcessInvokeExecutionRuntimeCarrier { seal: self.seal }
    }
}

impl ProcessRunChildProviderOutputForDirectRunOwnerV1 {
    pub(crate) fn into_runtime_carrier_for_session_execution_kernel_owner_v1(
        self,
    ) -> ProcessRunChildRuntimeCarrier {
        ProcessRunChildRuntimeCarrier {
            seal: self.seal,
            process: self.process,
        }
    }
}

impl ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1 {
    pub(crate) fn from_exact_kernel_internal_provider_output_for_session_work_runtime_owner_v1(
        output: ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
        output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    ) -> Self {
        Self {
            output,
            selected_output_authority,
            output_type_contract,
        }
    }

    pub(crate) fn try_join_pending_output_authority_for_session_execution_kernel_owner_v1(
        self,
        pending_output_authority: PendingProviderBoundaryOutputCommitAuthority,
    ) -> ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1 {
        let Self {
            output,
            selected_output_authority,
            output_type_contract,
        } = self;
        match pending_output_authority
            .try_join_selected_output_authority_for_kernel_internal_owner_v1(
                selected_output_authority,
            ) {
            ProviderBoundaryOutputAuthorityJoin::Joined(provider_boundary) => {
                ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1::Joined(
                    MatchedProcessInvokeExecutionProviderIngressForSessionExecutionKernelOwnerV1 {
                        _provider_boundary: provider_boundary,
                        output,
                        output_type_contract,
                    },
                )
            }
            ProviderBoundaryOutputAuthorityJoin::Unmatched { pending, selected } => {
                ProcessInvokeExecutionProviderIngressJoinForSessionExecutionKernelOwnerV1::Unmatched {
                    pending_output_authority: pending,
                    ingress: Self {
                        output,
                        selected_output_authority: selected,
                        output_type_contract,
                    },
                }
            }
        }
    }
}

impl ProcessRunChildProviderIngressForDirectRunOwnerV1 {
    pub(crate) fn from_exact_kernel_internal_provider_output_for_session_work_runtime_owner_v1(
        output: ProcessRunChildProviderOutputForDirectRunOwnerV1,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
        output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    ) -> Self {
        Self {
            output,
            selected_output_authority,
            output_type_contract,
        }
    }

    pub(crate) fn try_join_pending_output_authority_for_session_execution_kernel_owner_v1(
        self,
        pending_output_authority: PendingProviderBoundaryOutputCommitAuthority,
    ) -> ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1 {
        let Self {
            output,
            selected_output_authority,
            output_type_contract,
        } = self;
        match pending_output_authority
            .try_join_selected_output_authority_for_kernel_internal_owner_v1(
                selected_output_authority,
            ) {
            ProviderBoundaryOutputAuthorityJoin::Joined(provider_boundary) => {
                ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1::Joined(
                    MatchedProcessRunChildProviderIngressForSessionExecutionKernelOwnerV1 {
                        _provider_boundary: provider_boundary,
                        output,
                        output_type_contract,
                    },
                )
            }
            ProviderBoundaryOutputAuthorityJoin::Unmatched { pending, selected } => {
                ProcessRunChildProviderIngressJoinForSessionExecutionKernelOwnerV1::Unmatched {
                    pending_output_authority: pending,
                    ingress: Self {
                        output,
                        selected_output_authority: selected,
                        output_type_contract,
                    },
                }
            }
        }
    }
}

fn consume_nominal_process_output_contract_for_session_execution_kernel_owner_v1(
    output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
) -> Result<(), ProcessNominalProviderIngressCommitFaultV1> {
    match output_type_contract.into_closed_sum_output_type_for_provider_host_owner_v1() {
        Err(CapabilityTypeContractError::OutputTypeNotClosedSum) => Ok(()),
        Ok(_) => Err(
            ProcessNominalProviderIngressCommitFaultV1::closed_sum_output_contract_for_session_execution_kernel_owner_v1(),
        ),
        Err(source) => Err(
            ProcessNominalProviderIngressCommitFaultV1::output_contract_for_session_execution_kernel_owner_v1(source),
        ),
    }
}

impl MatchedProcessInvokeExecutionProviderIngressForSessionExecutionKernelOwnerV1 {
    pub(crate) fn into_nominal_output_for_session_execution_kernel_owner_v1(
        self,
    ) -> Result<
        MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1,
        ProcessNominalProviderIngressCommitFaultV1,
    > {
        consume_nominal_process_output_contract_for_session_execution_kernel_owner_v1(
            self.output_type_contract,
        )?;
        Ok(
            MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1 {
                carrier:
                    MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Invoke(
                        self.output
                            .into_runtime_carrier_for_session_execution_kernel_owner_v1(),
                    ),
            },
        )
    }
}

impl MatchedProcessRunChildProviderIngressForSessionExecutionKernelOwnerV1 {
    pub(crate) fn into_nominal_output_for_session_execution_kernel_owner_v1(
        self,
    ) -> Result<
        MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1,
        ProcessNominalProviderIngressCommitFaultV1,
    > {
        consume_nominal_process_output_contract_for_session_execution_kernel_owner_v1(
            self.output_type_contract,
        )?;
        Ok(
            MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1 {
                carrier: MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Run(
                    self.output
                        .into_runtime_carrier_for_session_execution_kernel_owner_v1(),
                ),
            },
        )
    }
}

impl MatchedProcessNominalProviderOutputForSessionExecutionKernelOwnerV1 {
    pub(in crate::session) fn duplicate_authored_member_for_session_execution_kernel_owner_v1(
        &self,
        member: &str,
    ) -> Option<ProviderValue> {
        match &self.carrier {
            MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Run(carrier) => {
                match member {
                    "process" => carrier.process.as_ref().map(|process| {
                        ProviderValue::CurrentProcess(
                            process.duplicate_for_session_runtime_owner_v1(),
                        )
                    }),
                    "status" => Some(ProviderValue::String("running".to_owned())),
                    _ => None,
                }
            }
            MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Invoke(_) => None,
        }
    }

    /// The sole flattening of exact nominal process authority into a runtime
    /// register value. It is reachable only after the checked local removes
    /// this whole carrier from one-take custody.
    pub(in crate::session) fn into_runtime_value_for_session_execution_kernel_owner_v1(
        self,
    ) -> crate::RuntimeValue {
        let inner = match self.carrier {
            MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Invoke(
                carrier,
            ) => super::RuntimeValueInner::ProcessInvokeExecution(carrier),
            MatchedProcessNominalRuntimeCarrierForSessionExecutionKernelOwnerV1::Run(carrier) => {
                super::RuntimeValueInner::ProcessRunChild(carrier)
            }
        };
        crate::RuntimeValue { _inner: inner }
    }
}

impl ProcessInvokeExecutionRuntimeCarrier {
    pub(crate) fn into_selected_for_durable_direct_run_owner_v1(
        self,
    ) -> SelectedProcessInvokeExecutionForDirectRunOwnerV1 {
        SelectedProcessInvokeExecutionForDirectRunOwnerV1 { seal: self.seal }
    }
}

impl ProcessRunChildRuntimeCarrier {
    pub(crate) fn into_selected_for_durable_direct_run_owner_v1(
        self,
    ) -> SelectedProcessRunChildForDirectRunOwnerV1 {
        SelectedProcessRunChildForDirectRunOwnerV1 { seal: self.seal }
    }
}

impl SelectedProcessInvokeExecutionForDirectRunOwnerV1 {
    pub(crate) fn try_join_registration_for_durable_direct_run_owner_v1(
        self,
        registration: ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    ) -> ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1 {
        if Arc::ptr_eq(&self.seal, &registration.seal) {
            ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1::Joined(
                MatchedProcessInvokeExecutionForDirectRunOwnerV1 { _seal: self.seal },
            )
        } else {
            ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1::Unmatched {
                selected: self,
                registration,
            }
        }
    }
}

impl SelectedProcessRunChildForDirectRunOwnerV1 {
    pub(crate) fn try_join_registration_for_durable_direct_run_owner_v1(
        self,
        registration: ProcessRunChildRegistrationForDirectRunOwnerV1,
    ) -> ProcessRunChildRegistrationJoinForDirectRunOwnerV1 {
        if Arc::ptr_eq(&self.seal, &registration.seal) {
            ProcessRunChildRegistrationJoinForDirectRunOwnerV1::Joined(
                MatchedProcessRunChildForDirectRunOwnerV1 { _seal: self.seal },
            )
        } else {
            ProcessRunChildRegistrationJoinForDirectRunOwnerV1::Unmatched {
                selected: self,
                registration,
            }
        }
    }
}

impl ProcessInvokeAwaitExecutionResultProducer {
    pub(crate) fn from_preflighted_destination_for_session_execution_kernel_owner_v1(
        destination: RegisterIndex,
    ) -> Self {
        Self { destination }
    }

    pub(crate) fn destination_for_session_execution_kernel_owner_v1(&self) -> RegisterIndex {
        self.destination
    }

    pub(crate) fn into_destination_for_session_execution_kernel_owner_v1(self) -> RegisterIndex {
        self.destination
    }

    pub(crate) fn commit_for_session_execution_kernel_owner_v1(
        self,
        execution_state: &mut crate::session::execution_state::KernelExecutionState,
        result: crate::RuntimeValue,
    ) -> Result<(), crate::session::execution_state::ExecutionStateFault> {
        execution_state.write_plain_register(self.destination, result)
    }
}

impl ProcessRunDriveTerminalResultProducer {
    pub(crate) fn from_preflighted_destination_for_session_execution_kernel_owner_v1(
        destination: RegisterIndex,
    ) -> Self {
        Self { destination }
    }

    pub(crate) fn destination_for_session_execution_kernel_owner_v1(&self) -> RegisterIndex {
        self.destination
    }

    pub(crate) fn into_destination_for_session_execution_kernel_owner_v1(self) -> RegisterIndex {
        self.destination
    }

    pub(crate) fn commit_for_session_execution_kernel_owner_v1(
        self,
        execution_state: &mut crate::session::execution_state::KernelExecutionState,
        terminal: crate::RuntimeValue,
    ) -> Result<(), crate::session::execution_state::ExecutionStateFault> {
        execution_state.write_plain_register(self.destination, terminal)
    }
}

impl ProcessKernelBoundaryResumeCommitFaultV1 {
    pub(crate) fn result_admission_for_session_execution_kernel_owner_v1(
        source: crate::VmRuntimeHeapAllocationError,
    ) -> Self {
        Self {
            source: ProcessKernelBoundaryResumeCommitFaultSourceV1::ResultAdmission(source),
        }
    }

    pub(crate) fn execution_commit_for_session_execution_kernel_owner_v1(
        source: crate::session::execution_state::ExecutionStateFault,
    ) -> Self {
        Self {
            source: ProcessKernelBoundaryResumeCommitFaultSourceV1::ExecutionCommit(source),
        }
    }

    pub(crate) fn provider_result_commit_for_session_execution_kernel_owner_v1(
        source: crate::session::execution_state::ProviderExecutionFrameFault,
    ) -> Self {
        Self {
            source: ProcessKernelBoundaryResumeCommitFaultSourceV1::ProviderResultCommit(source),
        }
    }

    pub fn diagnostic_kind_for_direct_run_owner_v1(&self) -> &'static str {
        match self.source {
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ResultAdmission(_) => {
                "process_kernel_boundary_result_admission_failed"
            }
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ExecutionCommit(_) => {
                "process_kernel_boundary_execution_commit_failed"
            }
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ProviderResultCommit(_) => {
                "process_control_provider_result_commit_failed"
            }
        }
    }
}

impl fmt::Debug for ProcessKernelBoundaryResumeCommitFaultV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessKernelBoundaryResumeCommitFaultV1")
            .field("kind", &self.diagnostic_kind_for_direct_run_owner_v1())
            .field("source", &self.to_string())
            .finish()
    }
}

impl fmt::Display for ProcessKernelBoundaryResumeCommitFaultV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ResultAdmission(source) => {
                write!(
                    formatter,
                    "process kernel boundary result admission failed: {source}"
                )
            }
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ExecutionCommit(source) => {
                write!(
                    formatter,
                    "process kernel boundary execution-state commit failed: {source}"
                )
            }
            ProcessKernelBoundaryResumeCommitFaultSourceV1::ProviderResultCommit(source) => {
                write!(
                    formatter,
                    "process-control provider result commit failed: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ProcessKernelBoundaryResumeCommitFaultV1 {}

impl ProcessNominalProviderIngressCommitFaultV1 {
    pub(crate) fn output_contract_for_session_execution_kernel_owner_v1(
        source: CapabilityTypeContractError,
    ) -> Self {
        Self {
            source: ProcessNominalProviderIngressCommitFaultSourceV1::OutputContract(source),
        }
    }

    pub(crate) fn closed_sum_output_contract_for_session_execution_kernel_owner_v1() -> Self {
        Self {
            source: ProcessNominalProviderIngressCommitFaultSourceV1::ClosedSumOutputContract,
        }
    }

    pub(crate) fn execution_commit_for_session_execution_kernel_owner_v1(
        source: crate::ProviderBoundaryExecutionCommitFault,
    ) -> Self {
        Self {
            source: ProcessNominalProviderIngressCommitFaultSourceV1::ExecutionCommit(source),
        }
    }

    pub fn diagnostic_kind_for_direct_run_owner_v1(&self) -> &'static str {
        match self.source {
            ProcessNominalProviderIngressCommitFaultSourceV1::OutputContract(_) => {
                "process_nominal_provider_output_contract_admission_failed"
            }
            ProcessNominalProviderIngressCommitFaultSourceV1::ClosedSumOutputContract => {
                "process_nominal_provider_output_contract_closed_sum_forbidden"
            }
            ProcessNominalProviderIngressCommitFaultSourceV1::ExecutionCommit(_) => {
                "process_nominal_provider_output_execution_commit_failed"
            }
        }
    }
}

impl fmt::Debug for ProcessNominalProviderIngressCommitFaultV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessNominalProviderIngressCommitFaultV1")
            .field("kind", &self.diagnostic_kind_for_direct_run_owner_v1())
            .field("source", &self.to_string())
            .finish()
    }
}

impl fmt::Display for ProcessNominalProviderIngressCommitFaultV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.source {
            ProcessNominalProviderIngressCommitFaultSourceV1::OutputContract(source) => write!(
                formatter,
                "process nominal provider output contract admission failed: {source}"
            ),
            ProcessNominalProviderIngressCommitFaultSourceV1::ClosedSumOutputContract => formatter
                .write_str(
                    "process nominal provider output cannot consume a closed-sum output contract",
                ),
            ProcessNominalProviderIngressCommitFaultSourceV1::ExecutionCommit(source) => write!(
                formatter,
                "process nominal provider output executable-state commit failed: {source}"
            ),
        }
    }
}

impl std::error::Error for ProcessNominalProviderIngressCommitFaultV1 {}

macro_rules! process_resume_drive_failure_impls {
    ($failure:ty, $name:literal) => {
        impl fmt::Debug for $failure {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct($name)
                    .field("kind", &self.diagnostic_kind_for_direct_run_owner_v1())
                    .finish()
            }
        }

        impl fmt::Display for $failure {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::NoPendingBoundary { .. } => formatter.write_str(
                        "process boundary resume refused because the session has no pending kernel boundary",
                    ),
                    Self::DifferentPendingBoundary { pending_kind, .. } => write!(
                        formatter,
                        "process boundary resume refused because the session has a different pending kernel boundary: {pending_kind}",
                    ),
                    Self::CorrespondenceMismatch { .. } => formatter.write_str(
                        "process boundary resume does not correspond to this session's pending transition",
                    ),
                    Self::Commit { source } => source.fmt(formatter),
                    Self::Drive { source } => source.fmt(formatter),
                }
            }
        }

        impl std::error::Error for $failure {}

        impl $failure {
            pub fn diagnostic_kind_for_direct_run_owner_v1(&self) -> &'static str {
                match self {
                    Self::NoPendingBoundary { .. } => {
                        "process_boundary_resume_without_pending_boundary"
                    }
                    Self::DifferentPendingBoundary { .. } => {
                        "process_boundary_resume_different_pending_boundary"
                    }
                    Self::CorrespondenceMismatch { .. } => {
                        "process_boundary_resume_correspondence_mismatch"
                    }
                    Self::Commit { source } => {
                        source.diagnostic_kind_for_direct_run_owner_v1()
                    }
                    Self::Drive { .. } => "process_boundary_resume_continuation_drive_failed",
                }
            }
        }
    };
}

process_resume_drive_failure_impls!(
    ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1,
    "ProcessInvokeAwaitExecutionResumeDriveFailureForDirectRunOwnerV1"
);

macro_rules! process_nominal_ingress_drive_failure_impls {
    ($failure:ty, $name:literal) => {
        impl fmt::Debug for $failure {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_struct($name)
                    .field("kind", &self.diagnostic_kind_for_direct_run_owner_v1())
                    .finish()
            }
        }

        impl fmt::Display for $failure {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::NoPendingBoundary { .. } => formatter.write_str(
                        "process nominal provider ingress refused because the session has no pending kernel boundary",
                    ),
                    Self::DifferentPendingBoundary { pending_kind, .. } => write!(
                        formatter,
                        "process nominal provider ingress refused because the session has a different pending kernel boundary: {pending_kind}",
                    ),
                    Self::CorrespondenceMismatch { .. } => formatter.write_str(
                        "process nominal provider ingress does not correspond to this session's pending provider boundary",
                    ),
                    Self::Commit { source } => source.fmt(formatter),
                    Self::Drive { source } => source.fmt(formatter),
                }
            }
        }

        impl std::error::Error for $failure {}

        impl $failure {
            pub fn diagnostic_kind_for_direct_run_owner_v1(&self) -> &'static str {
                match self {
                    Self::NoPendingBoundary { .. } => {
                        "process_nominal_provider_ingress_without_pending_boundary"
                    }
                    Self::DifferentPendingBoundary { .. } => {
                        "process_nominal_provider_ingress_different_pending_boundary"
                    }
                    Self::CorrespondenceMismatch { .. } => {
                        "process_nominal_provider_ingress_correspondence_mismatch"
                    }
                    Self::Commit { source } => {
                        source.diagnostic_kind_for_direct_run_owner_v1()
                    }
                    Self::Drive { .. } => {
                        "process_nominal_provider_ingress_continuation_drive_failed"
                    }
                }
            }
        }
    };
}

process_nominal_ingress_drive_failure_impls!(
    ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1,
    "ProcessInvokeExecutionProviderIngressDriveFailureForDirectRunOwnerV1"
);
process_nominal_ingress_drive_failure_impls!(
    ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1,
    "ProcessRunChildProviderIngressDriveFailureForDirectRunOwnerV1"
);
process_resume_drive_failure_impls!(
    ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1,
    "ProcessRunDriveTerminalResumeDriveFailureForDirectRunOwnerV1"
);
process_resume_drive_failure_impls!(
    ProcessControlResumeDriveFailureForDirectRunOwnerV1,
    "ProcessControlResumeDriveFailureForDirectRunOwnerV1"
);

pub(crate) fn mint_process_invoke_await_execution_boundary_for_session_execution_kernel_owner_v1(
    selected_execution: SelectedProcessInvokeExecutionForDirectRunOwnerV1,
    producer: ProcessInvokeAwaitExecutionResultProducer,
    continuation: ExecutableCursorTarget,
) -> (
    PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1,
    SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessInvokeAwaitTransitionSeal);
    (
        PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
            seal: Arc::clone(&seal),
            producer,
            continuation,
        },
        SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
            selected_execution,
            transition_seal: seal,
        },
    )
}

pub(crate) fn mint_process_run_drive_terminal_boundary_for_session_execution_kernel_owner_v1(
    selected_child: SelectedProcessRunChildForDirectRunOwnerV1,
    producer: ProcessRunDriveTerminalResultProducer,
    continuation: ExecutableCursorTarget,
) -> (
    PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1,
    SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessRunDriveTransitionSeal);
    (
        PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
            seal: Arc::clone(&seal),
            producer,
            continuation,
        },
        SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
            selected_child,
            transition_seal: seal,
        },
    )
}

pub(crate) fn mint_process_control_boundary_for_session_execution_kernel_owner_v1(
    process: swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
    control: ProcessControlKindForDirectRunOwnerV1,
    children: ProcessControlChildPolicyForDirectRunOwnerV1,
    reason: Option<String>,
    result_mode: swarmvm_isa_types::HostActivityResultMode,
    result_commit: crate::session::execution_kernel::provider_effect_runtime::ProviderEffectExecutableResultCommitForSessionExecutionKernelOwnerV1,
    continuation: ExecutableCursorTarget,
) -> (
    PendingProcessControlResumeForSessionExecutionKernelOwnerV1,
    SelectedProcessControlBoundaryForDirectRunOwnerV1,
) {
    let seal = Arc::new(ProcessControlTransitionSeal);
    (
        PendingProcessControlResumeForSessionExecutionKernelOwnerV1 {
            seal: Arc::clone(&seal),
            result_mode,
            result_commit,
            continuation,
        },
        SelectedProcessControlBoundaryForDirectRunOwnerV1 {
            process,
            control,
            children,
            reason,
            transition_seal: seal,
        },
    )
}

impl SelectedProcessControlBoundaryForDirectRunOwnerV1 {
    pub fn consume_for_durable_direct_run_owner_v1(
        self,
    ) -> (
        swarm_provider_value_model::CurrentProcessCarrierForSessionRuntimeOwnerV1,
        ProcessControlCompletionForDirectRunOwnerV1,
    ) {
        (
            self.process,
            ProcessControlCompletionForDirectRunOwnerV1 {
                control: self.control,
                children: self.children,
                reason: self.reason,
                transition_seal: self.transition_seal,
            },
        )
    }
}

impl ProcessControlCompletionForDirectRunOwnerV1 {
    pub fn control_for_direct_run_owner_v1(&self) -> ProcessControlKindForDirectRunOwnerV1 {
        self.control
    }

    pub fn children_for_direct_run_owner_v1(&self) -> ProcessControlChildPolicyForDirectRunOwnerV1 {
        self.children
    }

    pub fn reason_for_direct_run_owner_v1(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn admit_owner_control_receipt_for_direct_run_owner_v1(
        self,
        process_id: String,
        root_scope_id: String,
    ) -> ProcessControlResumeProductForDirectRunOwnerV1 {
        let receipt = ProviderValue::Object(
            std::collections::BTreeMap::from([
                (
                    "schema".to_owned(),
                    ProviderValue::String("swarm.process.control_receipt.v1".to_owned()),
                ),
                ("process_id".to_owned(), ProviderValue::String(process_id)),
                (
                    "root_scope_id".to_owned(),
                    ProviderValue::String(root_scope_id),
                ),
                (
                    "control".to_owned(),
                    ProviderValue::String(
                        self.control.stable_tag_for_direct_run_owner_v1().to_owned(),
                    ),
                ),
                (
                    "children".to_owned(),
                    ProviderValue::String(
                        self.children
                            .stable_tag_for_direct_run_owner_v1()
                            .to_owned(),
                    ),
                ),
            ])
            .into(),
        );
        ProcessControlResumeProductForDirectRunOwnerV1 {
            seal: self.transition_seal,
            receipt,
        }
    }
}

impl SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
    pub fn try_join_registration_for_durable_direct_run_owner_v1(
        self,
        registration: ProcessInvokeExecutionRegistrationForDirectRunOwnerV1,
    ) -> ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1 {
        let Self {
            selected_execution,
            transition_seal,
        } = self;
        match selected_execution.try_join_registration_for_durable_direct_run_owner_v1(registration)
        {
            ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1::Joined(
                matched_execution,
            ) => ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1::Joined(
                MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
                    _matched_execution: matched_execution,
                    transition_seal,
                },
            ),
            ProcessInvokeExecutionRegistrationJoinForDirectRunOwnerV1::Unmatched {
                selected,
                registration,
            } => ProcessInvokeAwaitExecutionBoundaryJoinForDirectRunOwnerV1::Unmatched {
                boundary: Self {
                    selected_execution: selected,
                    transition_seal,
                },
                registration,
            },
        }
    }
}

impl SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
    pub fn try_join_registration_for_durable_direct_run_owner_v1(
        self,
        registration: ProcessRunChildRegistrationForDirectRunOwnerV1,
    ) -> ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1 {
        let Self {
            selected_child,
            transition_seal,
        } = self;
        match selected_child.try_join_registration_for_durable_direct_run_owner_v1(registration) {
            ProcessRunChildRegistrationJoinForDirectRunOwnerV1::Joined(matched_child) => {
                ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1::Joined(
                    MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
                        _matched_child: matched_child,
                        transition_seal,
                    },
                )
            }
            ProcessRunChildRegistrationJoinForDirectRunOwnerV1::Unmatched {
                selected,
                registration,
            } => ProcessRunDriveTerminalBoundaryJoinForDirectRunOwnerV1::Unmatched {
                boundary: Self {
                    selected_child: selected,
                    transition_seal,
                },
                registration,
            },
        }
    }
}

impl MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 {
    pub fn admit_result_for_durable_direct_run_owner_v1(
        self,
        result: ProviderValue,
    ) -> ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1 {
        ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1 {
            seal: self.transition_seal,
            result,
        }
    }
}

impl MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 {
    pub fn admit_terminal_for_durable_direct_run_owner_v1(
        self,
        terminal: ProviderValue,
    ) -> ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1 {
        ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1 {
            seal: self.transition_seal,
            terminal,
        }
    }
}

impl PendingProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) fn try_join_resume_for_session_execution_kernel_owner_v1(
        self,
        resume: ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1,
    ) -> ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1 {
        if Arc::ptr_eq(&self.seal, &resume.seal) {
            ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1::Joined(
                MatchedProcessInvokeAwaitExecutionResumeForSessionExecutionKernelOwnerV1 {
                    producer: self.producer,
                    continuation: self.continuation,
                    result: resume.result,
                },
            )
        } else {
            ProcessInvokeAwaitExecutionResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending: self,
                resume,
            }
        }
    }
}

impl PendingProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) fn try_join_resume_for_session_execution_kernel_owner_v1(
        self,
        resume: ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1,
    ) -> ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1 {
        if Arc::ptr_eq(&self.seal, &resume.seal) {
            ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1::Joined(
                MatchedProcessRunDriveTerminalResumeForSessionExecutionKernelOwnerV1 {
                    producer: self.producer,
                    continuation: self.continuation,
                    terminal: resume.terminal,
                },
            )
        } else {
            ProcessRunDriveTerminalResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending: self,
                resume,
            }
        }
    }
}

impl PendingProcessControlResumeForSessionExecutionKernelOwnerV1 {
    pub(crate) fn try_join_resume_for_session_execution_kernel_owner_v1(
        self,
        resume: ProcessControlResumeProductForDirectRunOwnerV1,
    ) -> ProcessControlResumeJoinForSessionExecutionKernelOwnerV1 {
        if Arc::ptr_eq(&self.seal, &resume.seal) {
            ProcessControlResumeJoinForSessionExecutionKernelOwnerV1::Joined(
                MatchedProcessControlResumeForSessionExecutionKernelOwnerV1 {
                    result_mode: self.result_mode,
                    result_commit: self.result_commit,
                    continuation: self.continuation,
                    receipt: resume.receipt,
                },
            )
        } else {
            ProcessControlResumeJoinForSessionExecutionKernelOwnerV1::Unmatched {
                pending: self,
                resume,
            }
        }
    }
}

macro_rules! sealed_debug {
    ($($ty:ty => $name:literal),+ $(,)?) => {
        $(
            impl fmt::Debug for $ty {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str(concat!($name, "(<sealed>)"))
                }
            }
        )+
    };
}

sealed_debug!(
    ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1 => "ProcessInvokeExecutionProviderOutputForDirectRunOwnerV1",
    ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1 => "ProcessInvokeExecutionProviderIngressForDirectRunOwnerV1",
    ProcessInvokeExecutionRegistrationForDirectRunOwnerV1 => "ProcessInvokeExecutionRegistrationForDirectRunOwnerV1",
    SelectedProcessInvokeExecutionForDirectRunOwnerV1 => "SelectedProcessInvokeExecutionForDirectRunOwnerV1",
    MatchedProcessInvokeExecutionForDirectRunOwnerV1 => "MatchedProcessInvokeExecutionForDirectRunOwnerV1",
    ProcessRunChildProviderOutputForDirectRunOwnerV1 => "ProcessRunChildProviderOutputForDirectRunOwnerV1",
    ProcessRunChildProviderIngressForDirectRunOwnerV1 => "ProcessRunChildProviderIngressForDirectRunOwnerV1",
    ProcessRunChildRegistrationForDirectRunOwnerV1 => "ProcessRunChildRegistrationForDirectRunOwnerV1",
    SelectedProcessRunChildForDirectRunOwnerV1 => "SelectedProcessRunChildForDirectRunOwnerV1",
    MatchedProcessRunChildForDirectRunOwnerV1 => "MatchedProcessRunChildForDirectRunOwnerV1",
    SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 => "SelectedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1",
    SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 => "SelectedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1",
    MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1 => "MatchedProcessInvokeAwaitExecutionBoundaryForDirectRunOwnerV1",
    MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1 => "MatchedProcessRunDriveTerminalBoundaryForDirectRunOwnerV1",
    ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1 => "ProcessInvokeAwaitExecutionResumeProductForDirectRunOwnerV1",
    ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1 => "ProcessRunDriveTerminalResumeProductForDirectRunOwnerV1",
    SelectedProcessControlBoundaryForDirectRunOwnerV1 => "SelectedProcessControlBoundaryForDirectRunOwnerV1",
    ProcessControlCompletionForDirectRunOwnerV1 => "ProcessControlCompletionForDirectRunOwnerV1",
    ProcessControlResumeProductForDirectRunOwnerV1 => "ProcessControlResumeProductForDirectRunOwnerV1",
);

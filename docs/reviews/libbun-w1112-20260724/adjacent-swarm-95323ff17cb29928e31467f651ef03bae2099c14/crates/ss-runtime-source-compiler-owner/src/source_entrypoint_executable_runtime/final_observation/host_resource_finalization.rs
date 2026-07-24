#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
// compiler-custody: symbol=ProcessSessionActorResourceCleanupAuthorityV0 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(crate) struct ProcessSessionActorResourceCleanupAuthorityV0 {
    handle_id: String,
    resource_kind: String,
    cleanup_authority: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_binding_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_entry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_outcome: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProcessSessionActivityResultBodyForbiddenV0 {}

pub(crate) enum ProcessSessionActivityEffectResultPayloadRefV0 {
    Ready {
        payload_handle: PayloadHandle,
        payload_shape: String,
        actor_resource_cleanup_authorities: Vec<ProcessSessionActorResourceCleanupAuthorityV0>,
    },
    Error {
        code: String,
        message: String,
        details_payload_handle: Option<PayloadHandle>,
        details_payload_shape: Option<String>,
        actor_resource_cleanup_authorities: Vec<ProcessSessionActorResourceCleanupAuthorityV0>,
    },
}

pub(in crate::session) enum ProcessSessionRunOutcomeV0 {
    NeedsHostActivityEffect {
        pending_effect: PendingActivityEffectFrame,
    },
    NeedsHostResourceFinalization {
        obligation: Box<OneShotHostResourceFinalizationObligation>,
    },
    WaitingOnLiveness {
        entry_outcome: Box<ProcessSessionEntryOutcomeV0>,
        live_blockers: crate::ProcessLivenessLiveBlockersV1,
        readiness_certificate: ProcessBoundaryReadinessCertificateV1,
        diagnostics: ProcessSessionPublicDiagnosticProjectionValueForbiddenRequireDiagnosticProjectionAuthority,
    },
    Completed {
        terminal_authority: ProcessCompletedTerminalOutcomeAuthorityV1,
        public_output: ProcessSessionCompletedTerminalPublicOutputProductV1,
        readiness_certificate: ProcessBoundaryReadinessCertificateV1,
    },
    Failed {
        terminal_authority: ProcessFailedTerminalOutcomeAuthorityV1,
        code: String,
        message: String,
        classified_cause: ProcessSessionClassifiedFailureCauseV0,
        readiness_certificate: ProcessBoundaryReadinessCertificateV1,
        diagnostics:
            Option<ProcessSessionPublicDiagnosticProjectionValueForbiddenRequireDiagnosticProjectionAuthority>,
    },
}

impl std::fmt::Debug for ProcessSessionRunOutcomeV0 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProcessSessionRunOutcomeV0")
            .field("kind", &process_session_run_outcome_kind(self))
            .field("authority", &"session_runtime_owned")
            .finish()
    }
}

pub struct DirectRunProcessSessionRunResultProductV1 {
    inner: DirectRunProcessSessionRunResultInnerV1,
}

const HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1: u8 = 0;
const HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1: u8 = 1;
const HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1: u8 = 2;
const HOST_RESOURCE_FINALIZATION_SELECTION_CONSUMED_V1: u8 = 3;

/// Private generative correspondence shared only by the installed session
/// half and its one opaque selected half. The state transition, rather than a
/// scalar or pointer comparison, proves that the selected half came from this
/// exact pending custody.
struct HostResourceFinalizationSelectionSealV1 {
    state: std::sync::atomic::AtomicU8,
}

// compiler-custody: symbol=SelectedHostResourceFinalizationSelectionDropGuardV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct SelectedHostResourceFinalizationSelectionDropGuardV1 {
    seal: Option<Arc<HostResourceFinalizationSelectionSealV1>>,
}

struct PresentedHostResourceFinalizationSelectionV1 {
    seal: Arc<HostResourceFinalizationSelectionSealV1>,
    cancel_on_drop: bool,
}

/// Session-private custody of the exact obligation represented by one
/// externally selected finalization boundary. The obligation never crosses
/// the session-kernel crate boundary; only the matching opaque selection does.
// compiler-custody: symbol=PendingHostResourceFinalizationSelectionCustodyV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct PendingHostResourceFinalizationSelectionCustodyV1<Obligation> {
    seal: Arc<HostResourceFinalizationSelectionSealV1>,
    obligation: Obligation,
}

/// Exact correspondence proof that borrows installed session custody. Keeping
/// the obligation behind this borrow is what makes provider unwinds retryable:
/// the armed presented guard cancels while the session-owned value cannot move.
// compiler-custody: symbol=MatchedHostResourceFinalizationSelectionCustodyV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct MatchedHostResourceFinalizationSelectionCustodyV1<'pending, Obligation> {
    pending: &'pending mut PendingHostResourceFinalizationSelectionCustodyV1<Obligation>,
    presented: PresentedHostResourceFinalizationSelectionV1,
}

enum HostResourceFinalizationSelectionConsumptionRefusalV1 {
    Correspondence {
        selected: SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
        fault: HostResourceFinalizationBoundaryFaultV1,
    },
    Invariant {
        fault: HostResourceFinalizationBoundaryFaultV1,
    },
}

pub(in crate::session) type PendingSelectedHostResourceFinalizationBoundaryV1 =
    PendingHostResourceFinalizationSelectionCustodyV1<
        Box<OneShotHostResourceFinalizationObligation>,
    >;

pub(in crate::session) type MatchedSelectedHostResourceFinalizationBoundaryV1<'pending> =
    MatchedHostResourceFinalizationSelectionCustodyV1<
        'pending,
        Box<OneShotHostResourceFinalizationObligation>,
    >;

// compiler-custody: symbol=SelectedHostResourceFinalizationConsumptionRefusalV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub(in crate::session) type SelectedHostResourceFinalizationConsumptionRefusalV1 =
    HostResourceFinalizationSelectionConsumptionRefusalV1;

pub(in crate::session) enum HostResourceFinalizationSelectionReissueV1 {
    Reissued(SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1),
    Outstanding,
}

/// Linear authority to commit the exact host-resource finalization boundary
/// selected by a live process session. Private fields and the absence of
/// `Clone`, `Copy`, serde, or raw getters prevent reconstruction and replay.
#[must_use = "a selected host-resource finalization boundary must be consumed by its originating process session"]
// compiler-custody: symbol=SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
    guard: SelectedHostResourceFinalizationSelectionDropGuardV1,
}

impl std::fmt::Debug for SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1")
            .field("authority", &"sealed")
            .finish()
    }
}

impl<Obligation> PendingHostResourceFinalizationSelectionCustodyV1<Obligation> {
    pub(in crate::session) fn select_for_session_runtime_owner_v1(
        obligation: Obligation,
    ) -> (
        Self,
        SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) {
        let seal = Arc::new(HostResourceFinalizationSelectionSealV1 {
            state: std::sync::atomic::AtomicU8::new(HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1),
        });
        (
            Self {
                seal: Arc::clone(&seal),
                obligation,
            },
            SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
                guard: SelectedHostResourceFinalizationSelectionDropGuardV1 { seal: Some(seal) },
            },
        )
    }

    pub(in crate::session) fn try_reissue_cancelled_selection_for_session_runtime_owner_v1(
        &self,
    ) -> Result<HostResourceFinalizationSelectionReissueV1, HostResourceFinalizationBoundaryFaultV1>
    {
        match self.seal.state.load(std::sync::atomic::Ordering::Acquire) {
            HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1 => {
                Ok(HostResourceFinalizationSelectionReissueV1::Outstanding)
            }
            HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1 => {
                self.seal
                    .state
                    .compare_exchange(
                        HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1,
                        HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1,
                        std::sync::atomic::Ordering::AcqRel,
                        std::sync::atomic::Ordering::Acquire,
                    )
                    .map_err(|observed| {
                        HostResourceFinalizationBoundaryFaultV1::SelectionStateInvariant {
                            expected: "cancelled",
                            observed: host_resource_finalization_selection_state_name_v1(observed),
                        }
                    })?;
                Ok(HostResourceFinalizationSelectionReissueV1::Reissued(
                    SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
                        guard: SelectedHostResourceFinalizationSelectionDropGuardV1 {
                            seal: Some(Arc::clone(&self.seal)),
                        },
                    },
                ))
            }
            observed => Err(
                HostResourceFinalizationBoundaryFaultV1::SelectionStateInvariant {
                    expected: "cancelled",
                    observed: host_resource_finalization_selection_state_name_v1(observed),
                },
            ),
        }
    }

    pub(in crate::session) fn consume_exact_selection_for_session_runtime_owner_v1(
        &mut self,
        selected: SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
    ) -> Result<
        MatchedHostResourceFinalizationSelectionCustodyV1<'_, Obligation>,
        HostResourceFinalizationSelectionConsumptionRefusalV1,
    > {
        let presented = match selected.try_present_for_session_runtime_owner_v1() {
            Ok(presented) => presented,
            Err((selected, fault)) => {
                return Err(
                    HostResourceFinalizationSelectionConsumptionRefusalV1::Correspondence {
                        selected,
                        fault,
                    },
                );
            }
        };
        let observed = self.seal.state.load(std::sync::atomic::Ordering::Acquire);
        if observed != HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1 {
            let selected = match presented.restore_selected_for_session_runtime_owner_v1() {
                Ok(selected) => selected,
                Err(fault) => {
                    return Err(
                        HostResourceFinalizationSelectionConsumptionRefusalV1::Invariant {
                            fault:
                                HostResourceFinalizationBoundaryFaultV1::SelectionStateInvariant {
                                    expected: "presented",
                                    observed: fault.observed,
                                },
                        },
                    );
                }
            };
            return Err(
                HostResourceFinalizationSelectionConsumptionRefusalV1::Correspondence {
                    selected,
                    fault: HostResourceFinalizationBoundaryFaultV1::CommitIdentityMismatch,
                },
            );
        }
        Ok(MatchedHostResourceFinalizationSelectionCustodyV1 {
            pending: self,
            presented,
        })
    }
}

impl PendingSelectedHostResourceFinalizationBoundaryV1 {
    pub(in crate::session) fn record_live_heap_roots_for_checkpoint_owner_v1<'a>(
        &'a self,
        roots: &mut Vec<&'a RuntimeValue>,
    ) {
        self.obligation
            .record_live_heap_roots_for_checkpoint_owner_v1(roots);
    }

    pub(in crate::session) fn record_runtime_handle_restore_requirements_for_checkpoint_owner_v1(
        &self,
        requirements: &mut Vec<
            crate::VmRuntimeHandleRestoreRequirementsForSwarmvmSessionRuntimeOwnerV1,
        >,
    ) {
        let mut roots = Vec::new();
        self.record_live_heap_roots_for_checkpoint_owner_v1(&mut roots);
        requirements.extend(roots.into_iter().map(|value| {
            value.runtime_handle_restore_requirements_for_swarmvm_session_runtime_owner_v1()
        }));
    }
}

struct HostResourceFinalizationSelectionRestoreFaultV1 {
    observed: &'static str,
}

fn host_resource_finalization_selection_state_name_v1(state: u8) -> &'static str {
    match state {
        HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1 => "issued",
        HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1 => "presented",
        HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1 => "cancelled",
        HOST_RESOURCE_FINALIZATION_SELECTION_CONSUMED_V1 => "consumed",
        _ => "invalid",
    }
}

impl SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
    fn try_present_for_session_runtime_owner_v1(
        self,
    ) -> Result<
        PresentedHostResourceFinalizationSelectionV1,
        (Self, HostResourceFinalizationBoundaryFaultV1),
    > {
        let Self { mut guard } = self;
        let Some(seal) = guard.seal.take() else {
            return Err((
                Self { guard },
                HostResourceFinalizationBoundaryFaultV1::SelectionStateInvariant {
                    expected: "issued",
                    observed: "missing_selected_custody",
                },
            ));
        };
        if let Err(observed) = seal.state.compare_exchange(
            HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1,
            HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
        ) {
            guard.seal = Some(seal);
            return Err((
                Self { guard },
                HostResourceFinalizationBoundaryFaultV1::SelectionStateInvariant {
                    expected: "issued",
                    observed: host_resource_finalization_selection_state_name_v1(observed),
                },
            ));
        }
        Ok(PresentedHostResourceFinalizationSelectionV1 {
            seal,
            cancel_on_drop: true,
        })
    }
}

impl Drop for SelectedHostResourceFinalizationSelectionDropGuardV1 {
    fn drop(&mut self) {
        if let Some(seal) = self.seal.as_ref() {
            let _ = seal.state.compare_exchange(
                HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1,
                HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            );
        }
    }
}

impl PresentedHostResourceFinalizationSelectionV1 {
    fn restore_selected_for_session_runtime_owner_v1(
        mut self,
    ) -> Result<
        SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1,
        HostResourceFinalizationSelectionRestoreFaultV1,
    > {
        if let Err(observed) = self.seal.state.compare_exchange(
            HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1,
            HOST_RESOURCE_FINALIZATION_SELECTION_ISSUED_V1,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
        ) {
            return Err(HostResourceFinalizationSelectionRestoreFaultV1 {
                observed: host_resource_finalization_selection_state_name_v1(observed),
            });
        }
        self.cancel_on_drop = false;
        Ok(
            SelectedHostResourceFinalizationBoundaryForDirectRunOwnerV1 {
                guard: SelectedHostResourceFinalizationSelectionDropGuardV1 {
                    seal: Some(Arc::clone(&self.seal)),
                },
            },
        )
    }

    fn commit_consumed_for_session_runtime_owner_v1(mut self) {
        self.seal.state.store(
            HOST_RESOURCE_FINALIZATION_SELECTION_CONSUMED_V1,
            std::sync::atomic::Ordering::Release,
        );
        self.cancel_on_drop = false;
    }
}

impl Drop for PresentedHostResourceFinalizationSelectionV1 {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            let _ = self.seal.state.compare_exchange(
                HOST_RESOURCE_FINALIZATION_SELECTION_PRESENTED_V1,
                HOST_RESOURCE_FINALIZATION_SELECTION_CANCELLED_V1,
                std::sync::atomic::Ordering::AcqRel,
                std::sync::atomic::Ordering::Acquire,
            );
        }
    }
}

#[cfg(test)]
impl<Obligation> MatchedHostResourceFinalizationSelectionCustodyV1<'_, Obligation> {
    fn commit_exact_with_for_session_execution_kernel_owner_v1<Receipt, Fault>(
        self,
        commit: impl FnOnce(&mut Obligation) -> Result<Receipt, Fault>,
    ) -> Result<Receipt, Fault> {
        let Self { pending, presented } = self;
        match commit(&mut pending.obligation) {
            Ok(receipt) => {
                presented.commit_consumed_for_session_runtime_owner_v1();
                Ok(receipt)
            }
            Err(fault) => Err(fault),
        }
    }
}

impl MatchedSelectedHostResourceFinalizationBoundaryV1<'_> {
    pub(in crate::session) fn commit_exact_provider_release_for_session_execution_kernel_owner_v1(
        self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Result<
        swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseReceiptV1,
        swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseFaultV1,
    > {
        let Self { pending, presented } = self;
        match pending
            .obligation
            .commit_exact_provider_release_for_session_execution_kernel_owner_v1(
                provider_execution_session,
            ) {
            Ok(receipt) => {
                presented.commit_consumed_for_session_runtime_owner_v1();
                Ok(receipt)
            }
            Err(fault) => Err(fault),
        }
    }
}

#[cfg(test)]
mod host_resource_finalization_selection_seal_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn observed_state(seal: &Arc<HostResourceFinalizationSelectionSealV1>) -> &'static str {
        host_resource_finalization_selection_state_name_v1(seal.state.load(Ordering::Acquire))
    }

    #[test]
    fn same_selection_commits_consumed_before_exactly_one_publication() {
        let (mut pending, selected) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                "same-selection-obligation",
            );
        let seal = Arc::clone(&pending.seal);
        let matched = pending
            .consume_exact_selection_for_session_runtime_owner_v1(selected)
            .unwrap_or_else(|_| panic!("the exact selected half must match its session half"));

        assert_eq!(observed_state(&seal), "presented");
        let publication = matched
            .commit_exact_with_for_session_execution_kernel_owner_v1(|obligation| {
                assert_eq!(*obligation, "same-selection-obligation");
                Ok::<_, ()>(())
            })
            .unwrap_or_else(|_| panic!("the exact obligation commit must succeed"));
        assert_eq!(
            observed_state(&seal),
            "consumed",
            "Consumed must be stored before drive/publication receives its permit",
        );

        let publication_count = AtomicUsize::new(0);
        let () = publication;
        publication_count.fetch_add(1, Ordering::AcqRel);
        assert_eq!(publication_count.load(Ordering::Acquire), 1);
    }

    #[test]
    fn foreign_cross_splice_returns_both_custodians_without_consuming_either() {
        let (mut pending_a, selected_a) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                "obligation-a",
            );
        let (mut pending_b, selected_b) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                "obligation-b",
            );
        let seal_a = Arc::clone(&pending_a.seal);
        let seal_b = Arc::clone(&pending_b.seal);

        let selected_b =
            match pending_a.consume_exact_selection_for_session_runtime_owner_v1(selected_b) {
                Err(HostResourceFinalizationSelectionConsumptionRefusalV1::Correspondence {
                    selected,
                    fault: HostResourceFinalizationBoundaryFaultV1::CommitIdentityMismatch,
                }) => selected,
                _ => panic!("a foreign selected half must refuse with both custodians"),
            };
        assert_eq!(pending_a.obligation, "obligation-a");
        assert_eq!(pending_b.obligation, "obligation-b");
        assert_eq!(observed_state(&seal_a), "issued");
        assert_eq!(observed_state(&seal_b), "issued");

        let matched_a = pending_a
            .consume_exact_selection_for_session_runtime_owner_v1(selected_a)
            .unwrap_or_else(|_| panic!("session A must retain its exact selection"));
        let matched_b = pending_b
            .consume_exact_selection_for_session_runtime_owner_v1(selected_b)
            .unwrap_or_else(|_| panic!("session B must retain its returned selection"));
        assert_eq!(matched_a.pending.obligation, "obligation-a");
        assert_eq!(matched_b.pending.obligation, "obligation-b");
        matched_a
            .commit_exact_with_for_session_execution_kernel_owner_v1(|obligation| {
                Ok::<_, ()>(*obligation)
            })
            .unwrap_or_else(|_| panic!("session A must consume only its own obligation"));
        matched_b
            .commit_exact_with_for_session_execution_kernel_owner_v1(|obligation| {
                Ok::<_, ()>(*obligation)
            })
            .unwrap_or_else(|_| panic!("session B must consume only its own obligation"));
        assert_eq!(observed_state(&seal_a), "consumed");
        assert_eq!(observed_state(&seal_b), "consumed");
    }

    #[test]
    fn selected_drop_reissues_twenty_thousand_times_with_128_kib_custody() {
        let (mut pending, mut selected) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                vec![0x5a_u8; 128 * 1024],
            );
        for _ in 0..20_000 {
            drop(selected);
            selected = match pending
                .try_reissue_cancelled_selection_for_session_runtime_owner_v1()
                .unwrap_or_else(|_| panic!("Drop must leave the selection reissuable"))
            {
                HostResourceFinalizationSelectionReissueV1::Reissued(selected) => selected,
                HostResourceFinalizationSelectionReissueV1::Outstanding => {
                    panic!("Drop cannot leave an outstanding selected half")
                }
            };
        }
        let matched = pending
            .consume_exact_selection_for_session_runtime_owner_v1(selected)
            .unwrap_or_else(|_| panic!("the last reissued half must remain exact"));
        assert_eq!(matched.pending.obligation.len(), 128 * 1024);
    }

    #[test]
    fn presented_guard_cancels_during_unwind_and_reissues() {
        let (mut pending, selected) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                "unwind-obligation",
            );
        let seal = Arc::clone(&pending.seal);
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _presented = selected
                .try_present_for_session_runtime_owner_v1()
                .unwrap_or_else(|_| panic!("issued selection must present"));
            panic!("synthetic unwind after Presented");
        }));
        assert!(unwind.is_err());
        assert_eq!(observed_state(&seal), "cancelled");
        assert!(matches!(
            pending
                .try_reissue_cancelled_selection_for_session_runtime_owner_v1()
                .unwrap_or_else(|_| panic!("unwind cancellation must be reissuable")),
            HostResourceFinalizationSelectionReissueV1::Reissued(_),
        ));
    }

    struct ExactObligationProbe {
        identity: u64,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for ExactObligationProbe {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::AcqRel);
        }
    }

    #[test]
    fn provider_refusal_returns_exact_obligation_cancelled_for_rebind_retry() {
        let drops = Arc::new(AtomicUsize::new(0));
        let obligation = Box::new(ExactObligationProbe {
            identity: 73,
            drops: Arc::clone(&drops),
        });
        let obligation_address = (&*obligation) as *const ExactObligationProbe;
        let (mut pending, selected) =
            PendingHostResourceFinalizationSelectionCustodyV1::select_for_session_runtime_owner_v1(
                obligation,
            );
        let seal = Arc::clone(&pending.seal);
        let matched = pending
            .consume_exact_selection_for_session_runtime_owner_v1(selected)
            .unwrap_or_else(|_| panic!("exact selection must present for provider commit"));
        let fault = matched
            .commit_exact_with_for_session_execution_kernel_owner_v1(|obligation| {
                let _ = obligation;
                Err::<(), _>("provider-refused")
            })
            .expect_err("provider refusal must restore pending custody");

        assert_eq!(fault, "provider-refused");
        assert_eq!((&*pending.obligation) as *const _, obligation_address);
        assert_eq!(pending.obligation.identity, 73);
        assert_eq!(drops.load(Ordering::Acquire), 0);
        assert_eq!(observed_state(&seal), "cancelled");
        let selected = match pending
            .try_reissue_cancelled_selection_for_session_runtime_owner_v1()
            .unwrap_or_else(|_| panic!("cancelled provider custody must rebind to a selection"))
        {
            HostResourceFinalizationSelectionReissueV1::Reissued(selected) => selected,
            HostResourceFinalizationSelectionReissueV1::Outstanding => {
                panic!("cancelled provider custody cannot be outstanding")
            }
        };
        let matched = pending
            .consume_exact_selection_for_session_runtime_owner_v1(selected)
            .unwrap_or_else(|_| panic!("rebound selection must match exact pending custody"));
        matched
            .commit_exact_with_for_session_execution_kernel_owner_v1(|obligation| {
                Ok::<_, ()>(obligation.identity)
            })
            .unwrap_or_else(|_| panic!("retry must consume the restored exact obligation"));
        assert_eq!(observed_state(&seal), "consumed");
        drop(pending);
        assert_eq!(drops.load(Ordering::Acquire), 1);
    }

    fn completed_terminal_evidence_for_direct_finalization_proof() -> TerminalResultEvidence {
        let value = swarmvm_runtime_types::VmBoundaryValue::admit_provider_output_value_for_swarmvm_session_runtime_owner_v1(
            swarm_provider_value_model::ProviderValue::String(
                "direct-finalization-publication".to_owned(),
            ),
        )
        .expect("the direct-finalization publication value is boundary-admissible");
        TerminalResultEvidence {
            terminal_authority:
                ProcessCompletedTerminalOutcomeAuthorityV1::from_session_runtime_terminal_owner_v1(
                    "direct_finalization_publication",
                ),
            public_output: ProcessSessionCompletedTerminalPublicOutputProductV1 {
                value,
                return_boundary: None,
                output_effect_settlement:
                    ProcessSessionCompletedTerminalOutputEffectSettlementProductV1 {
                        inner: ProcessSessionCompletedTerminalOutputEffectSettlementInnerV1::NoObservableEffects {
                            obligation_count: 0,
                            reached_count: 0,
                            deferred_behind_liveness_count: 0,
                            discharged_unreachable_by_sealed_frontier_count: 0,
                            poisoned_count: 0,
                        },
                    },
            },
            readiness_certificate: ProcessBoundaryReadinessCertificateV1 {
                boundary_kind: "completed",
                terminal_status: "completed",
                source_kind: "direct_finalization_publication",
                admission_mode: "direct_finalization_publication_test",
                sealed_frontier_kind: None,
                live_blocker_count: None,
                live_blockers_count: None,
                obligation_count: 0,
                reached_obligation_count: 0,
                deferred_obligation_count: 0,
                discharged_unreachable_obligation_count: 0,
                poisoned_obligation_count: 0,
                return_boundary: None,
                observable_effect_obligations: serde_json::json!({}),
                drive_progress_receipt: serde_json::json!({}),
                failure_code: None,
                failure_cause_kind: None,
            },
        }
    }

    #[test]
    fn process_session_provider_commit_unwind_retains_exact_custody_through_retry_publication() {
        let (static_provider_hosts, selected_provider_release, owner_observation) =
            swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostSet::host_resource_release_commit_fixture_for_test_support_v1()
                .expect("static-provider commit fixture must mint real release authority");
        let provider_hosts = swarm_provider_host_set::ProviderHostSet::from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
            static_provider_hosts,
        )
        .expect("provider host set must admit the static host");
        let mut provider_execution_session = provider_hosts.begin_provider_execution_session_v1();
        let drops = Arc::new(AtomicUsize::new(0));
        let obligation = Box::new(
            OneShotHostResourceFinalizationObligation::from_selected_provider_release_for_crate_unit_tests_v1(
                selected_provider_release,
                Arc::clone(&drops),
            ),
        );
        let obligation_address = (&*obligation) as *const OneShotHostResourceFinalizationObligation;
        let provider_authority_address =
            obligation.selected_provider_release_address_for_crate_unit_tests_v1();
        let (pending, selected) =
            PendingSelectedHostResourceFinalizationBoundaryV1::select_for_session_runtime_owner_v1(
                obligation,
            );
        let mut session = ProcessSessionV0::direct_finalization_proof_fixture();
        session.pending_selected_host_resource_finalization = Some(pending);
        let seal = Arc::clone(
            &session
                .pending_selected_host_resource_finalization
                .as_ref()
                .expect("real ProcessSession custody must be installed")
                .seal,
        );

        swarm_rust_sdk_static_provider_host::arm_next_static_provider_host_resource_release_commit_unwind_for_test_support_v1();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = session
                .commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1(
                    &mut provider_execution_session,
                    selected,
                );
        }));

        assert!(unwind.is_err());
        assert_eq!(observed_state(&seal), "cancelled");
        assert!(
            session
                .pending_selected_host_resource_finalization
                .is_some()
        );
        assert_eq!(
            (&*session
                .pending_selected_host_resource_finalization
                .as_ref()
                .expect("unwind must retain real ProcessSession custody")
                .obligation) as *const OneShotHostResourceFinalizationObligation,
            obligation_address,
        );
        assert_eq!(
            session
                .pending_selected_host_resource_finalization
                .as_ref()
                .expect("unwind must retain real ProcessSession custody")
                .obligation
                .selected_provider_release_address_for_crate_unit_tests_v1(),
            provider_authority_address,
        );
        assert_eq!(
            session
                .pending_selected_host_resource_finalization
                .as_ref()
                .expect("unwind must retain real ProcessSession custody")
                .obligation
                .selected_provider_release_state_for_crate_unit_tests_v1(),
            "static_test",
        );
        assert_eq!(
            owner_observation.resource_state_for_test_support_v1(),
            "active"
        );
        assert_eq!(drops.load(Ordering::Acquire), 0);

        let selected = match session
            .try_reissue_cancelled_host_resource_finalization_selection_for_session_runtime_owner_v1()
            .unwrap_or_else(|_| panic!("unwind-retained custody must reissue"))
        {
            HostResourceFinalizationSelectionReissueV1::Reissued(selected) => selected,
            HostResourceFinalizationSelectionReissueV1::Outstanding => {
                panic!("cancelled unwind custody cannot remain outstanding")
            }
        };
        let publication_count = Arc::new(AtomicUsize::new(0));
        let publication_count_probe = Arc::clone(&publication_count);
        let seal_probe = Arc::clone(&seal);
        let drops_probe = Arc::clone(&drops);
        crate::session::execution_kernel::arm_next_direct_finalization_publication_for_crate_unit_tests_v1(
            move |session| {
                assert!(
                    session.pending_selected_host_resource_finalization.is_none(),
                    "consumed session custody must leave the pending slot before publication",
                );
                assert_eq!(
                    observed_state(&seal_probe),
                    "consumed",
                    "provider success must store Consumed before continuation/publication",
                );
                assert_eq!(
                    drops_probe.load(Ordering::Acquire),
                    0,
                    "the exact obligation must remain alive through publication entry",
                );
                assert_eq!(
                    publication_count_probe.fetch_add(1, Ordering::AcqRel),
                    0,
                    "the retry may publish exactly once",
                );
                completed_terminal_evidence_for_direct_finalization_proof()
            },
        );
        let publication = session
            .commit_selected_host_resource_finalization_and_drive_for_direct_run_owner_v1(
                &mut provider_execution_session,
                selected,
            )
            .unwrap_or_else(|fault| {
                panic!("the exact unwind-retained obligation must publish: {fault}")
            });

        assert_eq!(
            publication.outcome_kind_for_direct_run_owner_v1(),
            "completed"
        );
        assert_eq!(observed_state(&seal), "consumed");
        assert_eq!(
            owner_observation.resource_state_for_test_support_v1(),
            "released"
        );
        assert!(session
            .try_reissue_cancelled_host_resource_finalization_selection_for_session_runtime_owner_v1()
            .is_err());
        assert_eq!(publication_count.load(Ordering::Acquire), 1);
        assert_eq!(drops.load(Ordering::Acquire), 1);
    }
}

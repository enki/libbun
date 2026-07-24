use std::{fmt, sync::Arc};

use swarm_capability_linker_core::ProviderValue;
use thiserror::Error;

/// The session-owned half of one selected provider boundary.
///
/// It never exposes an id or selector.  The session stores it beside the exact
/// pending result application and must consume it with the ready half born
/// from the paired host request before provider cargo can enter runtime state.
// compiler-custody: symbol=PendingProviderBoundaryOutputCommitAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="runtime boundary authority crosses host and session owners; exact first root-scope edit: work_runtime/work_runtime_stores_impl.rs at mint_provider_boundary_output_correspondence_v1 must mint and retain its ticket"
#[must_use = "a pending provider-boundary output authority must be consumed by the exact ready-result commit"]
pub struct PendingProviderBoundaryOutputCommitAuthority {
    seal: Arc<ProviderBoundaryCorrespondenceSeal>,
}

/// The host-request half of one selected provider boundary.
///
/// This half moves through the admitted host request and is consumed when the
/// provider host admits one ready output.  It is deliberately non-Clone.
// compiler-custody: symbol=SelectedProviderBoundaryOutputAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="runtime boundary authority crosses host and session owners; exact first root-scope edit: work_runtime/work_runtime_stores_impl.rs at mint_provider_boundary_output_correspondence_v1 must mint and retain its ticket"
#[must_use = "a selected provider-boundary output authority must move through one host request into one ready output"]
pub struct SelectedProviderBoundaryOutputAuthority {
    seal: Arc<ProviderBoundaryCorrespondenceSeal>,
}

/// Successful correspondence between the session-owned pending half and the
/// selected kernel-internal half of one provider boundary.
///
/// This product deliberately carries no provider value. Kernel-internal
/// commands whose result is a nominal runtime carrier use this join instead of
/// laundering that carrier through `ProviderValue` merely to prove boundary
/// correspondence.
// compiler-custody: symbol=MatchedProviderBoundaryOutputAuthority disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="runtime boundary authority crosses host and session owners; exact first root-scope edit: work_runtime/work_runtime_stores_impl.rs at mint_provider_boundary_output_correspondence_v1 must mint and retain its ticket"
#[must_use = "a matched provider-boundary output authority must be consumed by the nominal output owner"]
pub struct MatchedProviderBoundaryOutputAuthority {
    _seal: Arc<ProviderBoundaryCorrespondenceSeal>,
}

/// Non-destructive join for a kernel-internal nominal output. A mismatch
/// returns both complete halves so the pending session boundary and selected
/// command output can be restored and retried without reconstructing a key.
pub enum ProviderBoundaryOutputAuthorityJoin {
    Joined(MatchedProviderBoundaryOutputAuthority),
    Unmatched {
        pending: PendingProviderBoundaryOutputCommitAuthority,
        selected: SelectedProviderBoundaryOutputAuthority,
    },
}

/// Authored provider cargo paired with the exact selected host request that
/// produced it.  Its cargo is unavailable until the session consumes the
/// matching pending half.
#[must_use = "a ready provider-boundary output must be consumed by its exact pending session application"]
pub struct ProviderReadyBoundaryOutput {
    seal: Arc<ProviderBoundaryCorrespondenceSeal>,
    output: ProviderReadyBoundaryOutputInner,
}

enum ProviderReadyBoundaryOutputInner {
    Authored(ProviderValue),
    Accepted(ProviderValue),
    Rejected(ProviderValue),
}

/// One corresponded provider output whose settlement meaning remains sealed
/// while its authored payload crosses into the session-runtime value domain.
///
/// The payload mapper cannot select or alter the settlement branch.  The
/// correspondence owner applies it exactly once to the payload already paired
/// with the selected provider boundary.
#[must_use = "a corresponded provider output must be consumed into authored cargo or a typed result settlement"]
pub struct CorrelatedProviderBoundaryOutput<T> {
    inner: CorrelatedProviderBoundaryOutputInner<T>,
}

enum CorrelatedProviderBoundaryOutputInner<T> {
    Authored(T),
    Accepted(T),
    Rejected(T),
}

/// Ordinary authored provider cargo.  Unlike a typed command settlement, its
/// value may still contain a language-level Result envelope that only the
/// session runtime can classify.
#[must_use = "corresponded authored provider cargo must enter the session runtime exactly once"]
pub struct CorrelatedAuthoredProviderOutput<T> {
    payload: T,
}

/// A provider-host-admitted command settlement.  Its accepted/rejected branch
/// is private and can only be consumed through the finite one-shot settlement
/// operation below.
#[must_use = "a typed provider settlement must be consumed into one session-runtime Result carrier"]
pub struct CorrelatedProviderResultSettlement<T> {
    inner: CorrelatedProviderResultSettlementInner<T>,
}

enum CorrelatedProviderResultSettlementInner<T> {
    Accepted(T),
    Rejected(T),
}

/// The rejected half returned by the finite accepted-settlement consume.  It
/// is non-constructible and retains its payload until the session Result owner
/// consumes it into an Err carrier.
#[must_use = "a rejected provider settlement must be consumed into the session-runtime Err carrier"]
pub struct CorrelatedRejectedProviderResultSettlement<T> {
    payload: T,
}

struct ProviderBoundaryCorrespondenceSeal;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ProviderBoundaryOutputCorrespondenceFault {
    #[error("ready provider output does not belong to the selected pending provider boundary")]
    ReadyOutputDoesNotMatchPendingBoundary,
}

/// Mint the two linear halves of exactly one selected provider boundary.
///
/// The halves alone grant no execution authority: the pending half can only
/// enter a session through the private selected-boundary commit, and the host
/// half can only become useful after moving through the admitted request into
/// a ready output.  The shared seal is never projected as a raw id.
pub fn mint_provider_boundary_output_correspondence_v1() -> (
    PendingProviderBoundaryOutputCommitAuthority,
    SelectedProviderBoundaryOutputAuthority,
) {
    let seal = Arc::new(ProviderBoundaryCorrespondenceSeal);
    (
        PendingProviderBoundaryOutputCommitAuthority {
            seal: Arc::clone(&seal),
        },
        SelectedProviderBoundaryOutputAuthority { seal },
    )
}

impl SelectedProviderBoundaryOutputAuthority {
    /// Consume the selected-host half into the single ready output admitted by
    /// that host request.  ProviderValue is authored cargo; it cannot be read
    /// back until the paired pending session half is also consumed.
    pub fn admit_ready_output_for_provider_host_owner_v1(
        self,
        output: ProviderValue,
    ) -> ProviderReadyBoundaryOutput {
        ProviderReadyBoundaryOutput {
            seal: self.seal,
            output: ProviderReadyBoundaryOutputInner::Authored(output),
        }
    }

    /// Consume the selected-host half into a typed successful command
    /// settlement.  The provider host may call this only after admitting the
    /// exact operation invocation result; no result tag or object shape is
    /// reconstructed by the session.
    pub fn admit_accepted_result_for_provider_host_owner_v1(
        self,
        output: ProviderValue,
    ) -> ProviderReadyBoundaryOutput {
        ProviderReadyBoundaryOutput {
            seal: self.seal,
            output: ProviderReadyBoundaryOutputInner::Accepted(output),
        }
    }

    /// Consume the selected-host half into a typed rejected command
    /// settlement.  Runtime/transport/admission faults do not use this path;
    /// it is only for an authored rejection admitted by the provider host.
    pub fn admit_rejected_result_for_provider_host_owner_v1(
        self,
        output: ProviderValue,
    ) -> ProviderReadyBoundaryOutput {
        ProviderReadyBoundaryOutput {
            seal: self.seal,
            output: ProviderReadyBoundaryOutputInner::Rejected(output),
        }
    }
}

impl PendingProviderBoundaryOutputCommitAuthority {
    /// Join the pending session half directly to the selected half retained by
    /// an exact kernel-internal command. This is the nominal-output sibling of
    /// `consume_corresponded_ready_output_for_provider_boundary_owner_v1`.
    pub fn try_join_selected_output_authority_for_kernel_internal_owner_v1(
        self,
        selected: SelectedProviderBoundaryOutputAuthority,
    ) -> ProviderBoundaryOutputAuthorityJoin {
        if Arc::ptr_eq(&self.seal, &selected.seal) {
            ProviderBoundaryOutputAuthorityJoin::Joined(MatchedProviderBoundaryOutputAuthority {
                _seal: self.seal,
            })
        } else {
            ProviderBoundaryOutputAuthorityJoin::Unmatched {
                pending: self,
                selected,
            }
        }
    }

    /// Consume both halves and release authored cargo only when they came from
    /// the same mint.  The kernel calls this inside its finite provider-ready
    /// commit; no route, register, activity id, or fingerprint is reconstructed.
    pub fn consume_corresponded_ready_output_for_provider_boundary_owner_v1(
        self,
        ready: ProviderReadyBoundaryOutput,
    ) -> Result<
        CorrelatedProviderBoundaryOutput<ProviderValue>,
        ProviderBoundaryOutputCorrespondenceFault,
    > {
        if !Arc::ptr_eq(&self.seal, &ready.seal) {
            return Err(
                ProviderBoundaryOutputCorrespondenceFault::ReadyOutputDoesNotMatchPendingBoundary,
            );
        }
        let inner = match ready.output {
            ProviderReadyBoundaryOutputInner::Authored(payload) => {
                CorrelatedProviderBoundaryOutputInner::Authored(payload)
            }
            ProviderReadyBoundaryOutputInner::Accepted(payload) => {
                CorrelatedProviderBoundaryOutputInner::Accepted(payload)
            }
            ProviderReadyBoundaryOutputInner::Rejected(payload) => {
                CorrelatedProviderBoundaryOutputInner::Rejected(payload)
            }
        };
        Ok(CorrelatedProviderBoundaryOutput { inner })
    }
}

impl<T> CorrelatedProviderBoundaryOutput<T> {
    /// Map only the authored payload while preserving the private settlement
    /// branch byte-for-byte.  The mapper is invoked once and cannot choose,
    /// forge, or substitute Accepted/Rejected authority.
    pub fn try_map_payload_for_session_runtime_owner_v1<U, E>(
        self,
        map: impl FnOnce(T) -> Result<U, E>,
    ) -> Result<CorrelatedProviderBoundaryOutput<U>, E> {
        let inner = match self.inner {
            CorrelatedProviderBoundaryOutputInner::Authored(payload) => {
                CorrelatedProviderBoundaryOutputInner::Authored(map(payload)?)
            }
            CorrelatedProviderBoundaryOutputInner::Accepted(payload) => {
                CorrelatedProviderBoundaryOutputInner::Accepted(map(payload)?)
            }
            CorrelatedProviderBoundaryOutputInner::Rejected(payload) => {
                CorrelatedProviderBoundaryOutputInner::Rejected(map(payload)?)
            }
        };
        Ok(CorrelatedProviderBoundaryOutput { inner })
    }

    /// Separate ordinary authored cargo from a provider-host-admitted command
    /// settlement without exposing a public discriminant or result tag.
    pub fn into_result_settlement_for_session_runtime_owner_v1(
        self,
    ) -> Result<CorrelatedProviderResultSettlement<T>, CorrelatedAuthoredProviderOutput<T>> {
        match self.inner {
            CorrelatedProviderBoundaryOutputInner::Authored(payload) => {
                Err(CorrelatedAuthoredProviderOutput { payload })
            }
            CorrelatedProviderBoundaryOutputInner::Accepted(payload) => {
                Ok(CorrelatedProviderResultSettlement {
                    inner: CorrelatedProviderResultSettlementInner::Accepted(payload),
                })
            }
            CorrelatedProviderBoundaryOutputInner::Rejected(payload) => {
                Ok(CorrelatedProviderResultSettlement {
                    inner: CorrelatedProviderResultSettlementInner::Rejected(payload),
                })
            }
        }
    }
}

impl<T> CorrelatedAuthoredProviderOutput<T> {
    pub fn into_payload_for_session_runtime_owner_v1(self) -> T {
        self.payload
    }
}

impl<T> CorrelatedProviderResultSettlement<T> {
    /// Consume one typed settlement.  Accepted releases its payload; rejected
    /// remains sealed in a distinct one-shot product for the Err owner.
    pub fn into_accepted_payload_for_session_runtime_owner_v1(
        self,
    ) -> Result<T, CorrelatedRejectedProviderResultSettlement<T>> {
        match self.inner {
            CorrelatedProviderResultSettlementInner::Accepted(payload) => Ok(payload),
            CorrelatedProviderResultSettlementInner::Rejected(payload) => {
                Err(CorrelatedRejectedProviderResultSettlement { payload })
            }
        }
    }
}

impl<T> CorrelatedRejectedProviderResultSettlement<T> {
    pub fn into_payload_for_session_runtime_owner_v1(self) -> T {
        self.payload
    }
}

impl fmt::Debug for PendingProviderBoundaryOutputCommitAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PendingProviderBoundaryOutputCommitAuthority(<sealed>)")
    }
}

impl fmt::Debug for SelectedProviderBoundaryOutputAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SelectedProviderBoundaryOutputAuthority(<sealed>)")
    }
}

impl fmt::Debug for MatchedProviderBoundaryOutputAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MatchedProviderBoundaryOutputAuthority(<sealed>)")
    }
}

impl fmt::Debug for ProviderBoundaryOutputAuthorityJoin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Joined(_) => formatter.write_str("ProviderBoundaryOutputAuthorityJoin::Joined"),
            Self::Unmatched { .. } => {
                formatter.write_str("ProviderBoundaryOutputAuthorityJoin::Unmatched(<sealed>)")
            }
        }
    }
}

impl fmt::Debug for ProviderReadyBoundaryOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = &self.output;
        formatter.write_str("ProviderReadyBoundaryOutput(<sealed>)")
    }
}

impl<T> fmt::Debug for CorrelatedProviderBoundaryOutput<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CorrelatedProviderBoundaryOutput(<sealed>)")
    }
}

impl<T> fmt::Debug for CorrelatedAuthoredProviderOutput<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CorrelatedAuthoredProviderOutput(<sealed>)")
    }
}

impl<T> fmt::Debug for CorrelatedProviderResultSettlement<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CorrelatedProviderResultSettlement(<sealed>)")
    }
}

impl<T> fmt::Debug for CorrelatedRejectedProviderResultSettlement<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CorrelatedRejectedProviderResultSettlement(<sealed>)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_pair_releases_authored_output_once() {
        let (pending, selected) = mint_provider_boundary_output_correspondence_v1();
        let ready = selected.admit_ready_output_for_provider_host_owner_v1(ProviderValue::String(
            "ready".to_owned(),
        ));

        let correlated = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("the paired ready output must commit");
        let authored = correlated
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect_err("ordinary authored cargo must not become a typed settlement");
        assert_eq!(
            authored.into_payload_for_session_runtime_owner_v1(),
            ProviderValue::String("ready".to_owned())
        );
    }

    #[test]
    fn exact_pair_releases_typed_accepted_settlement_once() {
        let (pending, selected) = mint_provider_boundary_output_correspondence_v1();
        let ready = selected.admit_accepted_result_for_provider_host_owner_v1(
            ProviderValue::String("accepted".to_owned()),
        );
        let correlated = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("the paired accepted result must correspond")
            .try_map_payload_for_session_runtime_owner_v1(|payload| {
                Ok::<_, ()>(match payload {
                    ProviderValue::String(text) => text.len(),
                    _ => panic!("test payload must remain the admitted string"),
                })
            })
            .expect("payload mapping must succeed");
        let settlement = correlated
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect("accepted result must remain a typed settlement");
        assert_eq!(
            settlement
                .into_accepted_payload_for_session_runtime_owner_v1()
                .expect("accepted result must release its payload"),
            "accepted".len()
        );
    }

    #[test]
    fn exact_pair_releases_typed_rejected_settlement_once() {
        let (pending, selected) = mint_provider_boundary_output_correspondence_v1();
        let ready = selected.admit_rejected_result_for_provider_host_owner_v1(
            ProviderValue::String("rejected".to_owned()),
        );
        let settlement = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("the paired rejected result must correspond")
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect("rejected result must remain a typed settlement");
        let rejected = settlement
            .into_accepted_payload_for_session_runtime_owner_v1()
            .expect_err("rejected result must not release an accepted payload");
        assert_eq!(
            rejected.into_payload_for_session_runtime_owner_v1(),
            ProviderValue::String("rejected".to_owned())
        );
    }

    #[test]
    fn cross_boundary_swap_is_a_typed_fault() {
        let (left_pending, _left_selected) = mint_provider_boundary_output_correspondence_v1();
        let (_right_pending, right_selected) = mint_provider_boundary_output_correspondence_v1();
        let swapped = right_selected.admit_ready_output_for_provider_host_owner_v1(
            ProviderValue::String("wrong boundary".to_owned()),
        );

        assert_eq!(
            left_pending
                .consume_corresponded_ready_output_for_provider_boundary_owner_v1(swapped)
                .expect_err("a ready output from another boundary must be refused"),
            ProviderBoundaryOutputCorrespondenceFault::ReadyOutputDoesNotMatchPendingBoundary
        );
    }

    #[test]
    fn cross_boundary_swap_preserves_typed_settlement_refusal() {
        let (left_pending, _left_selected) = mint_provider_boundary_output_correspondence_v1();
        let (_right_pending, right_selected) = mint_provider_boundary_output_correspondence_v1();
        let swapped = right_selected.admit_accepted_result_for_provider_host_owner_v1(
            ProviderValue::String("wrong accepted boundary".to_owned()),
        );

        assert_eq!(
            left_pending
                .consume_corresponded_ready_output_for_provider_boundary_owner_v1(swapped)
                .expect_err("a typed settlement from another boundary must be refused"),
            ProviderBoundaryOutputCorrespondenceFault::ReadyOutputDoesNotMatchPendingBoundary
        );
    }

    #[test]
    fn nominal_join_preserves_both_halves_on_mismatch() {
        let (left_pending, _left_selected) = mint_provider_boundary_output_correspondence_v1();
        let (_right_pending, right_selected) = mint_provider_boundary_output_correspondence_v1();

        let ProviderBoundaryOutputAuthorityJoin::Unmatched { pending, selected } = left_pending
            .try_join_selected_output_authority_for_kernel_internal_owner_v1(right_selected)
        else {
            panic!("independently minted provider boundaries must not join");
        };

        assert!(matches!(
            pending.try_join_selected_output_authority_for_kernel_internal_owner_v1(selected),
            ProviderBoundaryOutputAuthorityJoin::Unmatched { .. }
        ));
    }
}

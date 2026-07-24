use serde::{Deserialize, Serialize, ser::SerializeStruct as _};
use std::sync::{Arc, Mutex};
use swarm_provider_value_model::{AuthoredResourceValue, ProviderValue};
use swarm_rust_sdk_static_provider_host::{
    ProviderHostResourceReleaseFaultV1, SelectedProviderHostResourceReleaseV1,
};
use swarmvm_isa_types::ManagedRegionExitCompletion;
use swarmvm_isa_types::authority_ids::*;

use swarmvm_runtime_activity_input_authority::{
    CallableLivenessOwnerRefValue, ExecutionRefValue, OperationHandleKind, OperationHandleValue,
    SemanticTypeRefValue, Sha256Hash, StreamHandleKind, StreamHandleValue,
    VmBoundaryOutputCallableRefValidationError, VmBoundaryValue, VmHandleScalarMetadata,
};

use super::RuntimeValue;

fn record_finalization_release_value_root<'value>(
    release_value: &'value RuntimeValue,
    roots: &mut Vec<&'value RuntimeValue>,
) {
    roots.push(release_value);
}

fn require_trimmed_non_empty(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} must be non-empty"))
    } else {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum HostResourceHandleKind {
    #[serde(rename = "host_resource_handle")]
    HostResourceHandle,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum HostResourceLifetime {
    #[serde(rename = "turn")]
    Turn,
    #[serde(rename = "execution")]
    Execution,
    #[serde(rename = "process")]
    Process,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum HostResourceResumePolicy {
    #[serde(rename = "not_resumable")]
    NotResumable,
    #[serde(rename = "host_rebind_required")]
    HostRebindRequired,
    #[serde(rename = "checkpoint_resumable")]
    CheckpointResumable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
pub enum OneShotHostResourceFinalizationObligationSchema {
    #[serde(rename = "swarm.vm.one-shot-host-resource-finalization-obligation.v1")]
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OneShotHostResourceFinalizationReason {
    #[serde(rename = "scope_released")]
    ScopeReleased,
    #[serde(rename = "execution_completed")]
    ExecutionCompleted,
    #[serde(rename = "execution_failed")]
    ExecutionFailed,
    #[serde(rename = "execution_cancelled")]
    ExecutionCancelled,
    #[serde(rename = "execution_terminal_shutdown")]
    ExecutionTerminalShutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
pub enum OneShotTransactionFrameEventSchema {
    #[serde(rename = "swarm.vm.one-shot.transaction-frame-event.v1")]
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OneShotTransactionFrameEventKind {
    #[serde(rename = "begin")]
    Begin,
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "rollback")]
    Rollback,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) enum OneShotTransactionFrameAuthorityKind {
    #[serde(rename = "scoped_frame_lifecycle")]
    ScopedFrameLifecycle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum OneShotTransactionFrameOutcome {
    #[serde(rename = "opened")]
    Opened,
    #[serde(rename = "committed")]
    Committed,
    #[serde(rename = "rolled_back")]
    RolledBack,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OneShotTransactionFrameEvent {
    schema: OneShotTransactionFrameEventSchema,
    kind: OneShotTransactionFrameEventKind,
    authority_kind: OneShotTransactionFrameAuthorityKind,
    outcome: OneShotTransactionFrameOutcome,
    handle: TransactionHandleValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HostResourceReleasePolicy {
    Required,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HostResourceReleaseAuthorityKind {
    ProviderBinding,
    HostAuthority,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct HostResourceReleaseAuthorityValue {
    inner: HostResourceReleaseAuthorityValueInner,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum HostResourceReleaseAuthorityValueInner {
    ProviderBinding { authority_id: String },
    HostAuthority { authority_id: String },
}

impl HostResourceReleaseAuthorityValue {
    pub fn kind(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> HostResourceReleaseAuthorityKind {
        let _ = self;
        match input {}
    }

    pub fn authority_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        match input {}
    }

    #[cfg(test)]
    pub(crate) fn authority_id_for_runtime_types_diagnostic_owner_v1(&self) -> &str {
        match &self.inner {
            HostResourceReleaseAuthorityValueInner::ProviderBinding { authority_id }
            | HostResourceReleaseAuthorityValueInner::HostAuthority { authority_id } => {
                authority_id
            }
        }
    }

    pub(crate) fn provider_binding_authority_id(authority_id: &str) -> Self {
        Self::try_provider_binding_authority_id(authority_id)
            .expect("generated provider binding release authority id is valid")
    }

    pub(crate) fn try_provider_binding_authority_id(authority_id: &str) -> Result<Self, String> {
        require_trimmed_non_empty(
            "host_resource_release_authority.provider_binding.authority_id",
            authority_id,
        )?;
        Ok(Self {
            inner: HostResourceReleaseAuthorityValueInner::ProviderBinding {
                authority_id: authority_id.to_owned(),
            },
        })
    }

    pub(crate) fn duplicate_for_runtime_types_owner_v1(&self) -> Self {
        match &self.inner {
            HostResourceReleaseAuthorityValueInner::ProviderBinding { authority_id } => Self {
                inner: HostResourceReleaseAuthorityValueInner::ProviderBinding {
                    authority_id: authority_id.clone(),
                },
            },
            HostResourceReleaseAuthorityValueInner::HostAuthority { authority_id } => Self {
                inner: HostResourceReleaseAuthorityValueInner::HostAuthority {
                    authority_id: authority_id.clone(),
                },
            },
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostResourceReleaseObligationValue {
    obligation_id: String,
    owner_authority_scope: HostResourceAuthorityScope,
    release_authority: HostResourceReleaseAuthorityValue,
    release_policy: HostResourceReleasePolicy,
    idempotency_key: String,
}

impl HostResourceReleaseObligationValue {
    pub(crate) fn required_provider_binding_authority_id(
        resource_binding_id: &ResourceBindingId,
        owner_authority_scope: HostResourceAuthorityScope,
        provider_authority_id: &str,
    ) -> Self {
        Self::try_required_provider_binding_authority_id(
            resource_binding_id,
            owner_authority_scope,
            provider_authority_id,
        )
        .expect("generated provider binding release obligation is valid")
    }

    pub(crate) fn try_required_provider_binding_authority_id(
        resource_binding_id: &ResourceBindingId,
        owner_authority_scope: HostResourceAuthorityScope,
        provider_authority_id: &str,
    ) -> Result<Self, String> {
        Ok(Self {
            obligation_id: format!(
                "swarm.vm.host-resource-finalization.v1:{}",
                resource_binding_id.as_str()
            ),
            owner_authority_scope,
            release_authority:
                HostResourceReleaseAuthorityValue::try_provider_binding_authority_id(
                    provider_authority_id,
                )?,
            release_policy: HostResourceReleasePolicy::Required,
            idempotency_key: format!(
                "swarm.vm.host-resource-finalization.idempotency.v1:{}",
                resource_binding_id.as_str()
            ),
        })
    }

    pub fn obligation_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        match input {}
    }

    pub fn obligation_id_for_one_shot_lifecycle_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        let _ = self;
        match input {}
    }

    pub fn owner_authority_scope(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceAuthorityScope {
        let _ = self;
        match input {}
    }

    pub fn release_authority_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        match input {}
    }

    #[cfg(test)]
    pub(crate) fn release_authority_id_for_runtime_types_diagnostic_owner_v1(&self) -> &str {
        self.release_authority
            .authority_id_for_runtime_types_diagnostic_owner_v1()
    }

    pub(crate) fn release_authority_kind(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> HostResourceReleaseAuthorityKind {
        let _ = self;
        match input {}
    }

    pub(crate) fn idempotency_key(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        match input {}
    }

    pub(crate) fn idempotency_key_for_one_shot_lifecycle_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &str {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_one_shot_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_runtime_heap_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_swarmvm_session_runtime_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_runtime_types_owner_v1(&self) -> Self {
        Self {
            obligation_id: self.obligation_id.clone(),
            owner_authority_scope: self
                .owner_authority_scope
                .duplicate_for_runtime_types_owner_v1(),
            release_authority: self
                .release_authority
                .duplicate_for_runtime_types_owner_v1(),
            release_policy: self.release_policy.clone(),
            idempotency_key: self.idempotency_key.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HostResourceHandleValue {
    kind: HostResourceHandleKind,
    resource_binding_id: ResourceBindingId,
    handle_id: HostResourceHandleId,
    provider_id: HostResourceProviderId,
    resource_kind: HostResourceKind,
    operation_id: InstructionOpId,
    authority_scope: HostResourceAuthorityScope,
    liveness_owner: Option<CallableLivenessOwnerRefValue>,
    lifetime: HostResourceLifetime,
    resume_policy: HostResourceResumePolicy,
    execution: Option<ExecutionRefValue>,
    contract_hash: Option<Sha256Hash>,
}

impl Serialize for HostResourceHandleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut len = 9;
        len += usize::from(self.liveness_owner.is_some());
        len += usize::from(self.execution.is_some());
        len += usize::from(self.contract_hash.is_some());
        let mut value = serializer.serialize_struct("HostResourceHandleValue", len)?;
        value.serialize_field("kind", &self.kind)?;
        value.serialize_field("resource_binding_id", &self.resource_binding_id)?;
        value.serialize_field("handle_id", &self.handle_id)?;
        value.serialize_field("provider_binding_id", &self.provider_id)?;
        value.serialize_field("resource_kind", &self.resource_kind)?;
        value.serialize_field("operation_id", &self.operation_id)?;
        value.serialize_field("authority_scope", &self.authority_scope)?;
        if let Some(liveness_owner) = &self.liveness_owner {
            value.serialize_field("liveness_owner", liveness_owner)?;
        }
        value.serialize_field("lifetime", &self.lifetime)?;
        value.serialize_field("resume_policy", &self.resume_policy)?;
        if let Some(execution) = &self.execution {
            value.serialize_field("execution", execution)?;
        }
        if let Some(contract_hash) = &self.contract_hash {
            value.serialize_field("contract_hash", contract_hash)?;
        }
        value.end()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OneShotHostResourceRebindRequirementSchema {
    #[serde(rename = "swarm.vm.one-shot-host-resource-rebind-requirement.v1")]
    V1,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OneShotHostResourceRebindRequirement {
    schema: OneShotHostResourceRebindRequirementSchema,
    path: String,
    handle: HostResourceHandleValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OneShotHostResourceRebindEvidenceSchema {
    #[serde(rename = "swarm.vm.one-shot-host-resource-rebind-evidence.v1")]
    V1,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OneShotHostResourceRebindEvidence {
    schema: OneShotHostResourceRebindEvidenceSchema,
    handle: HostResourceHandleValue,
}

mod host_resource_rebind_requirement_match_private {
    #[derive(Debug, PartialEq, Eq)]
    pub(super) struct Seal;
}

/// Owner-private proof that the complete rebind requirement set corresponds
/// one-to-one with the supplied evidence.  Its contents cannot be unpacked or
/// replayed by a caller.
pub(crate) struct OneShotHostResourceRebindRequirementMatchProduct {
    _requirements: Vec<OneShotHostResourceRebindRequirement>,
    _matched_evidence_count: usize,
    _seal: host_resource_rebind_requirement_match_private::Seal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OneShotHostResourceRebindRequirementMatchError {
    DuplicateRequirement,
    MissingEvidence,
    DuplicateEvidence,
    UnexpectedEvidence,
}

impl OneShotHostResourceRebindRequirementMatchError {
    pub(crate) fn message_for_swarmvm_session_runtime_owner_v1(self) -> &'static str {
        match self {
            Self::DuplicateRequirement => {
                "one_shot_host_resource_rebind_requirement_duplicate_forbidden"
            }
            Self::MissingEvidence => "one_shot_host_resource_rebind_evidence_missing_forbidden",
            Self::DuplicateEvidence => "one_shot_host_resource_rebind_evidence_duplicate_forbidden",
            Self::UnexpectedEvidence => {
                "one_shot_host_resource_rebind_evidence_unexpected_forbidden"
            }
        }
    }
}

impl OneShotHostResourceRebindRequirementMatchProduct {
    pub(crate) fn match_requirements_and_evidence_for_swarmvm_session_runtime_owner_v1(
        requirements: Vec<OneShotHostResourceRebindRequirement>,
        evidence: &[OneShotHostResourceRebindEvidence],
    ) -> Result<Self, OneShotHostResourceRebindRequirementMatchError> {
        for (index, requirement) in requirements.iter().enumerate() {
            if requirements[..index]
                .iter()
                .any(|previous| previous.handle == requirement.handle)
            {
                return Err(OneShotHostResourceRebindRequirementMatchError::DuplicateRequirement);
            }
        }

        let mut matched_evidence = vec![false; evidence.len()];
        for requirement in &requirements {
            let mut matching_evidence_index = None;
            for (index, candidate) in evidence.iter().enumerate() {
                if candidate.handle != requirement.handle {
                    continue;
                }
                if matching_evidence_index.is_some() {
                    return Err(OneShotHostResourceRebindRequirementMatchError::DuplicateEvidence);
                }
                matching_evidence_index = Some(index);
            }
            let Some(index) = matching_evidence_index else {
                return Err(OneShotHostResourceRebindRequirementMatchError::MissingEvidence);
            };
            matched_evidence[index] = true;
        }

        if matched_evidence.iter().any(|matched| !*matched) {
            return Err(OneShotHostResourceRebindRequirementMatchError::UnexpectedEvidence);
        }

        Ok(Self {
            _matched_evidence_count: evidence.len(),
            _requirements: requirements,
            _seal: host_resource_rebind_requirement_match_private::Seal,
        })
    }
}

impl OneShotHostResourceRebindEvidence {
    pub(crate) fn schema_for_swarmvm_session_runtime_owner_v1(
        &self,
    ) -> OneShotHostResourceRebindEvidenceSchema {
        self.schema
    }
}

impl HostResourceHandleValue {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_swarmvm_session_runtime_owner_parts_v1(
        value: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> Self {
        match value {}
    }

    pub(crate) fn from_one_shot_owner_parts(
        value: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> Self {
        match value {}
    }

    pub(crate) fn matches_scope_resource_observation_for_direct_run_resource_finalization_owner_v1(
        &self,
        resource_id: &str,
        resource_entry_id: &str,
    ) -> bool {
        resource_id == self.resource_binding_id.as_str()
            || resource_entry_id == self.resource_binding_id.as_str()
    }

    pub(crate) fn resource_binding_id_for_runtime_types_diagnostic_owner_v1(
        &self,
    ) -> &ResourceBindingId {
        &self.resource_binding_id
    }

    pub(crate) fn handle_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceHandleId {
        let _ = self;
        match input {}
    }

    pub(crate) fn handle_id_for_runtime_types_diagnostic_owner_v1(&self) -> &HostResourceHandleId {
        &self.handle_id
    }

    pub(crate) fn provider_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceProviderId {
        let _ = self;
        match input {}
    }

    pub(crate) fn resource_kind(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceKind {
        let _ = self;
        match input {}
    }

    pub(crate) fn resource_kind_for_one_shot_lifecycle_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceKind {
        let _ = self;
        match input {}
    }

    pub(crate) fn operation_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &InstructionOpId {
        let _ = self;
        match input {}
    }

    pub(crate) fn authority_scope(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceAuthorityScope {
        let _ = self;
        match input {}
    }

    pub(crate) fn liveness_owner(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&CallableLivenessOwnerRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn liveness_owner_for_swarmvm_image_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&CallableLivenessOwnerRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn liveness_owner_for_host_abi_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&CallableLivenessOwnerRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn lifetime(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceLifetime {
        let _ = self;
        match input {}
    }

    pub(crate) fn resume_policy(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceResumePolicy {
        let _ = self;
        match input {}
    }

    pub(crate) fn execution(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&ExecutionRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn execution_for_swarmvm_image_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&ExecutionRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn execution_for_host_abi_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&ExecutionRefValue> {
        let _ = self;
        match input {}
    }

    pub(crate) fn contract_hash(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Option<&Sha256Hash> {
        match input {}
    }

    pub(crate) fn duplicate_for_one_shot_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_one_shot_lifecycle_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_live_primitive_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_checkpoint_body_authority_owner_v1(&self) -> Self {
        self.duplicate_for_runtime_types_owner_inner()
    }

    pub(crate) fn duplicate_for_runtime_heap_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_swarmvm_session_runtime_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_checkpoint_resumable_without_liveness_owner_for_swarmvm_session_runtime_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    fn duplicate_for_runtime_types_owner_inner(&self) -> Self {
        Self {
            kind: HostResourceHandleKind::HostResourceHandle,
            resource_binding_id: self
                .resource_binding_id
                .duplicate_for_runtime_types_owner_v1(),
            handle_id: self.handle_id.duplicate_for_runtime_types_owner_v1(),
            provider_id: self.provider_id.duplicate_for_runtime_types_owner_v1(),
            resource_kind: self.resource_kind.duplicate_for_runtime_types_owner_v1(),
            operation_id: self.operation_id.duplicate_for_runtime_types_owner_v1(),
            authority_scope: self.authority_scope.duplicate_for_runtime_types_owner_v1(),
            liveness_owner: self.liveness_owner.as_ref().map(
                CallableLivenessOwnerRefValue::duplicate_for_session_execution_kernel_owner_v1,
            ),
            lifetime: self.lifetime.clone(),
            resume_policy: self.resume_policy.clone(),
            execution: self
                .execution
                .as_ref()
                .map(ExecutionRefValue::duplicate_for_session_execution_kernel_owner_v1),
            contract_hash: self
                .contract_hash
                .as_ref()
                .map(Sha256Hash::duplicate_for_runtime_types_owner_v1),
        }
    }

    pub(crate) fn duplicate_for_runtime_heap_owner_internal_v1(
        &self,
        projection_poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match projection_poison {}
    }

    pub(crate) fn duplicate_for_runtime_types_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum HostResourceBindingKind {
    #[serde(rename = "host_resource_binding")]
    HostResourceBinding,
}

pub struct HostResourceBindingValue {
    kind: HostResourceBindingKind,
    handle: HostResourceHandleValue,
    release_obligation: HostResourceReleaseObligationValue,
    value: Box<RuntimeValue>,
    // Row #153 CARRIED-SYNCHRONOUS: the resource's AUTHORED payload, consumed
    // into the binding by the rung-3 merge from the runtime identity carrier.
    // `Some` for a scoped resource (`Resource<T>`); `None` for a host-resource
    // binding with no authored value. PRIVATE OBS data — projected ONLY by the
    // finite binding-materialization owner as the final `.value` observation; it
    // never mints, routes, resumes, settles, or executes authority.
    authored_resource_value: Option<AuthoredResourceValue>,
    selected_provider_release: SelectedProviderHostResourceReleaseV1,
}

#[must_use = "runtime-family host-resource lifecycle entries carry handle/release/value authority"]
pub struct RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1 {
    _handle: HostResourceHandleValue,
    _release_obligation: HostResourceReleaseObligationValue,
    _release_value: Box<RuntimeValue>,
    _selected_provider_release: SelectedProviderHostResourceReleaseV1,
}

impl RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1 {
    pub(crate) fn into_finalization_parts_for_one_shot_lifecycle_owner_v1(
        self,
    ) -> (
        HostResourceHandleValue,
        HostResourceReleaseObligationValue,
        String,
        HostResourceAuthorityScope,
        String,
        RuntimeValue,
        SelectedProviderHostResourceReleaseV1,
    ) {
        let Self {
            _handle,
            _release_obligation,
            _release_value,
            _selected_provider_release,
        } = self;
        let obligation_id = _release_obligation.obligation_id.clone();
        let owner_authority_scope = _release_obligation
            .owner_authority_scope
            .duplicate_for_one_shot_lifecycle_owner_v1();
        let idempotency_key = _release_obligation.idempotency_key.clone();
        (
            _handle,
            _release_obligation,
            obligation_id,
            owner_authority_scope,
            idempotency_key,
            *_release_value,
            _selected_provider_release,
        )
    }
}

pub struct OneShotHostResourceFinalizationObligation {
    schema: OneShotHostResourceFinalizationObligationSchema,
    release_obligation: HostResourceReleaseObligationValue,
    obligation_id: String,
    reason: OneShotHostResourceFinalizationReason,
    completion: ManagedRegionExitCompletion,
    owner_authority_scope: HostResourceAuthorityScope,
    idempotency_key: String,
    handle: HostResourceHandleValue,
    release_value: Box<RuntimeValue>,
    selected_provider_release: SelectedProviderHostResourceReleaseV1,
    #[cfg(test)]
    _drop_probe_for_crate_unit_tests_v1:
        Option<OneShotHostResourceFinalizationObligationDropProbeForCrateUnitTestsV1>,
}

#[cfg(test)]
struct OneShotHostResourceFinalizationObligationDropProbeForCrateUnitTestsV1 {
    drops: Arc<std::sync::atomic::AtomicUsize>,
}

#[cfg(test)]
impl Drop for OneShotHostResourceFinalizationObligationDropProbeForCrateUnitTestsV1 {
    fn drop(&mut self) {
        self.drops.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

#[derive(Debug, PartialEq)]
pub struct OneShotExecutionLifetimeHostResourceFinalizationEvidence {
    handle: HostResourceHandleValue,
    release_obligation: HostResourceReleaseObligationValue,
}

pub struct ScopedHostResourceFrameLifecycle {
    _runtime_family_entry: RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1,
}

#[must_use = "the scoped host-resource frame-entry ticket must be consumed to release the frame lifecycle, not dropped"]
// compiler-custody: symbol=ScopedHostResourceFrameEntryTicket disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
pub struct ScopedHostResourceFrameEntryTicket {
    frame: ScopedHostResourceFrameLifecycle,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedTransactionFrameLifecycle {
    handle: TransactionHandleValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScopedFrameLifecycleError {
    TransactionHandleMismatch,
}

impl OneShotHostResourceFinalizationObligation {
    #[cfg(test)]
    pub(in crate::session) fn from_selected_provider_release_for_crate_unit_tests_v1(
        selected_provider_release: SelectedProviderHostResourceReleaseV1,
        drops: Arc<std::sync::atomic::AtomicUsize>,
    ) -> Self {
        let owner_authority_scope =
            HostResourceAuthorityScope::for_with_region_lexical_identity_for_swarmvm_image_owner_v1(
                "test.module",
                "test.body",
                "resource",
            );
        let provider_id =
            HostResourceProviderId::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                "static-test-body-local-scope",
            )
            .expect("test provider identity is valid");
        let resource_binding_id =
            ResourceBindingId::compose_for_provider_output_binding_for_swarmvm_session_runtime_owner_v1(
                &provider_id,
                &owner_authority_scope,
            );
        let release_obligation =
            HostResourceReleaseObligationValue::required_provider_binding_authority_id(
                &resource_binding_id,
                owner_authority_scope.duplicate_for_runtime_types_owner_v1(),
                provider_id.as_str(),
            );
        let obligation_id = release_obligation.obligation_id.clone();
        let idempotency_key = release_obligation.idempotency_key.clone();
        Self {
            schema: OneShotHostResourceFinalizationObligationSchema::V1,
            release_obligation,
            obligation_id,
            reason: OneShotHostResourceFinalizationReason::ScopeReleased,
            completion: ManagedRegionExitCompletion::BODY_RETURN,
            owner_authority_scope: owner_authority_scope
                .duplicate_for_runtime_types_owner_v1(),
            idempotency_key,
            handle: HostResourceHandleValue {
                kind: HostResourceHandleKind::HostResourceHandle,
                resource_binding_id,
                handle_id:
                    HostResourceHandleId::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                        "scoped-resource-test-support",
                    )
                    .expect("test handle identity is valid"),
                provider_id,
                resource_kind:
                    HostResourceKind::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                        "ss-test-scoped-resource",
                    )
                    .expect("test resource kind is valid"),
                operation_id:
                    InstructionOpId::make_host_resource_binding_operation_id_for_swarmvm_image_owner_v1(),
                authority_scope: owner_authority_scope,
                liveness_owner: None,
                lifetime: HostResourceLifetime::Turn,
                resume_policy: HostResourceResumePolicy::HostRebindRequired,
                execution: None,
                contract_hash: None,
            },
            release_value: Box::new(RuntimeValue::string_for_runtime_activity_input_owner_v1(
                "test-support-release-value".to_owned(),
            )),
            selected_provider_release,
            _drop_probe_for_crate_unit_tests_v1: Some(
                OneShotHostResourceFinalizationObligationDropProbeForCrateUnitTestsV1 { drops },
            ),
        }
    }

    #[cfg(test)]
    pub(in crate::session) fn selected_provider_release_address_for_crate_unit_tests_v1(
        &self,
    ) -> *const SelectedProviderHostResourceReleaseV1 {
        &self.selected_provider_release
    }

    #[cfg(test)]
    pub(in crate::session) fn selected_provider_release_state_for_crate_unit_tests_v1(
        &self,
    ) -> &'static str {
        self.selected_provider_release
            .authority_state_for_test_support_v1()
    }

    /// Contribute the exact release payload retained solely by this live
    /// obligation to a scheduler-safe-point root closure.
    pub(in crate::session) fn record_live_heap_roots_for_checkpoint_owner_v1<'a>(
        &'a self,
        roots: &mut Vec<&'a RuntimeValue>,
    ) {
        record_finalization_release_value_root(&self.release_value, roots);
    }

    pub(in crate::session) fn commit_exact_provider_release_for_session_execution_kernel_owner_v1(
        &mut self,
        provider_execution_session: &mut swarm_provider_host_set::ProviderHostExecutionSession,
    ) -> Result<
        swarm_rust_sdk_static_provider_host::ProviderHostResourceReleaseReceiptV1,
        ProviderHostResourceReleaseFaultV1,
    > {
        provider_execution_session
            .commit_selected_host_resource_release_borrowed_for_session_execution_kernel_owner_v1(
                &mut self.selected_provider_release,
            )
    }

    pub fn from_execution_lifetime_binding_for_one_shot_lifecycle_owner_v1(
        _reason: OneShotHostResourceFinalizationReason,
        _completion: ManagedRegionExitCompletion,
        _evidence: OneShotExecutionLifetimeHostResourceFinalizationEvidence,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        match poison {}
    }

    pub fn resource_kind(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceKind {
        let _ = self;
        match poison {}
    }

    pub fn release_authority_scope(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceAuthorityScope {
        let _ = self;
        match poison {}
    }

    pub fn release_authority_kind_for_foreign_resource_release_observation_v1(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> HostResourceReleaseAuthorityKind {
        let _ = self;
        match poison {}
    }

    pub(crate) fn completion_for_swarmvm_session_runtime_owner_v1(
        &self,
    ) -> &ManagedRegionExitCompletion {
        &self.completion
    }

    pub fn release_value_for_swarmvm_session_runtime_owner_v1(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &RuntimeValue {
        let _ = self;
        match poison {}
    }

    pub fn duplicate_with_release_value_for_swarmvm_session_runtime_owner_v1(
        &self,
        _release_value: RuntimeValue,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match poison {}
    }
}

#[cfg(test)]
mod runnable_finalization_root_tests {
    use super::*;
    use crate::session::execution_kernel::executable_value::runtime_heap::SessionRuntimeHeapOwner;

    #[test]
    fn detached_release_value_keeps_its_heap_object_live_through_collection() {
        let mut heap = SessionRuntimeHeapOwner::empty_for_crate_unit_tests_v1();
        let release_value = heap
            .allocate_object_for_swarmvm_session_runtime_owner_v1(std::collections::BTreeMap::from(
                [(
                    "release".to_owned(),
                    RuntimeValue::string_for_runtime_activity_input_owner_v1("retained".to_owned()),
                )],
            ))
            .expect("release payload object allocates");
        let mut roots = Vec::new();
        record_finalization_release_value_root(&release_value, &mut roots);

        heap.mark_sweep_for_swarmvm_session_runtime_owner_v1(roots)
            .expect("proposal-held release payload root must remain reachable");
        heap.duplicate_member_for_local_value_instruction_runtime_activity_input_owner_v1(
            &release_value,
            &RuntimeValue::string_for_runtime_activity_input_owner_v1("release".to_owned()),
            false,
        )
        .expect("rooted release payload must survive cadence collection");
    }
}

impl ScopedHostResourceFrameLifecycle {
    /// Narrow observation for the scoped-frame lifecycle owner while it still
    /// holds the complete live frame. Replan preservation records this id only
    /// as a fact projection; the handle and release authority remain sealed in
    /// the frame and cannot be reconstructed from the returned string.
    pub(in crate::session) fn resource_binding_id_projection_for_scoped_frame_lifecycle_owner_v1(
        &self,
    ) -> &str {
        self._runtime_family_entry
            ._handle
            .resource_binding_id
            .as_str()
    }

    pub fn into_finalization_obligation_for_one_shot_lifecycle_owner_v1(
        self,
        reason: OneShotHostResourceFinalizationReason,
        completion: ManagedRegionExitCompletion,
    ) -> OneShotHostResourceFinalizationObligation {
        let (
            handle,
            release_obligation,
            obligation_id,
            owner_authority_scope,
            idempotency_key,
            release_value,
            selected_provider_release,
        ) = self
            ._runtime_family_entry
            .into_finalization_parts_for_one_shot_lifecycle_owner_v1();
        OneShotHostResourceFinalizationObligation {
            schema: OneShotHostResourceFinalizationObligationSchema::V1,
            release_obligation,
            obligation_id,
            reason,
            completion,
            owner_authority_scope,
            idempotency_key,
            handle,
            release_value: Box::new(release_value),
            selected_provider_release,
            #[cfg(test)]
            _drop_probe_for_crate_unit_tests_v1: None,
        }
    }
}

impl ScopedHostResourceFrameEntryTicket {
    pub(crate) fn from_runtime_family_binding_for_swarmvm_session_runtime_owner_v1(
        binding: HostResourceBindingValue,
    ) -> Self {
        Self {
            frame: ScopedHostResourceFrameLifecycle {
                _runtime_family_entry: binding
                    .into_runtime_family_lifecycle_entry_for_one_shot_owner_v1(),
            },
        }
    }

    fn into_frame_for_one_shot_lifecycle_owner_v1(self) -> ScopedHostResourceFrameLifecycle {
        self.frame
    }

    // R41104 (B) the ENTER mint, kept whole INSIDE this module so the factory
    // stays pub(crate) and into_frame stays private: the substantive consume in
    // the authority crate calls THIS one-shot mint (binding by value -> sealed
    // frame), never the private into_frame directly. No visibility widening.
    pub(crate) fn mint_frame_from_runtime_family_binding_for_swarmvm_session_runtime_owner_v1(
        binding: HostResourceBindingValue,
    ) -> ScopedHostResourceFrameLifecycle {
        Self::from_runtime_family_binding_for_swarmvm_session_runtime_owner_v1(binding)
            .into_frame_for_one_shot_lifecycle_owner_v1()
    }
}

// R41104 (B) SEALED ALGEBRA crossing back to swarmvm-session-runtime's frame
// drive: Enter carries the ALREADY-SEALED, session-runtime-nameable frame
// product (minted whole in-crate above); Exit carries the region-exit
// completion. The session drive COMMITS carried data only — it never holds mint
// capability. Payloads are sealed (frame has a private runtime-family entry;
// completion is an isa value), so a sibling crate cannot forge either arm.
pub enum HostResourceFrameInstructionForSessionRuntimeOwnerV1 {
    Enter(ScopedHostResourceFrameLifecycle),
    Exit(ManagedRegionExitCompletion),
}

impl ScopedTransactionFrameLifecycle {
    pub(crate) fn begin_transaction_frame_for_one_shot_lifecycle_owner_v1(
        handle: TransactionHandleValue,
    ) -> (Self, OneShotTransactionFrameEvent) {
        let event = OneShotTransactionFrameEvent {
            schema: OneShotTransactionFrameEventSchema::V1,
            kind: OneShotTransactionFrameEventKind::Begin,
            authority_kind: OneShotTransactionFrameAuthorityKind::ScopedFrameLifecycle,
            outcome: OneShotTransactionFrameOutcome::Opened,
            handle: handle.duplicate_for_one_shot_scoped_lifecycle_owner_v1(),
        };
        (Self { handle }, event)
    }

    pub(crate) fn complete_transaction_frame_for_one_shot_lifecycle_owner_v1(
        self,
        observed: &TransactionHandleValue,
        kind: OneShotTransactionFrameEventKind,
    ) -> Result<(TransactionHandleValue, OneShotTransactionFrameEvent), ScopedFrameLifecycleError>
    {
        if !self
            .handle
            .matches_for_one_shot_scoped_lifecycle_owner_v1(observed)
        {
            return Err(ScopedFrameLifecycleError::TransactionHandleMismatch);
        }
        let outcome = match kind {
            OneShotTransactionFrameEventKind::Begin => OneShotTransactionFrameOutcome::Opened,
            OneShotTransactionFrameEventKind::Commit => OneShotTransactionFrameOutcome::Committed,
            OneShotTransactionFrameEventKind::Rollback => {
                OneShotTransactionFrameOutcome::RolledBack
            }
        };
        let event = OneShotTransactionFrameEvent {
            schema: OneShotTransactionFrameEventSchema::V1,
            kind,
            authority_kind: OneShotTransactionFrameAuthorityKind::ScopedFrameLifecycle,
            outcome,
            handle: self
                .handle
                .duplicate_for_one_shot_scoped_lifecycle_owner_v1(),
        };
        Ok((self.handle, event))
    }

    pub fn into_rollback_event_for_scoped_frame_lifecycle_owner_v1(
        self,
    ) -> OneShotTransactionFrameEvent {
        OneShotTransactionFrameEvent {
            schema: OneShotTransactionFrameEventSchema::V1,
            kind: OneShotTransactionFrameEventKind::Rollback,
            authority_kind: OneShotTransactionFrameAuthorityKind::ScopedFrameLifecycle,
            outcome: OneShotTransactionFrameOutcome::RolledBack,
            handle: self.handle,
        }
    }
}

impl HostResourceBindingValue {
    // A2-RUNG-2 (HRB): POISONED — the OLD static-template -> runtime-binding
    // merge. Superseded by the rung-3 merge op (template ⊕ runtime-identity
    // carrier -> full handle -> binding). Per the MERGE-SHAPE ratification
    // (ADR-2002 §2, hard cutover, no compatibility path), the reshaped
    // static-authority template carries NO handle / resource_binding_id /
    // release_obligation, so it can never mint a runtime binding on its own.
    // Dead scaffolding for the unbuilt rung-3 executor; uninhabited until
    // rung-3 consumes template ⊕ carrier.
    pub(crate) fn from_runtime_family_template(
        template: HostResourceBindingTemplateValue,
        value: RuntimeValue,
        projection_poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = (template, value);
        match projection_poison {}
    }

    fn from_runtime_family_parts(
        kind: HostResourceBindingKind,
        handle: HostResourceHandleValue,
        release_obligation: HostResourceReleaseObligationValue,
        value: RuntimeValue,
        authored_resource_value: Option<AuthoredResourceValue>,
        selected_provider_release: SelectedProviderHostResourceReleaseV1,
    ) -> Self {
        Self {
            kind,
            handle,
            release_obligation,
            value: Box::new(value),
            authored_resource_value,
            selected_provider_release,
        }
    }

    // HRB RUNG-3 (MERGE OWNER): the SOLE mint of the sealed full-authority
    // host-resource binding. Assembles the `HostResourceHandleValue` from THREE
    // sources and commits it via `from_runtime_family_parts`:
    //   1. the static-authority TEMPLATE (kind -> binding.kind;
    //      operation_id / authority_scope / lifetime -> handle), read in-module
    //      by reference and duplicated as owned typed ids;
    //   2. the rung-1 runtime identity CARRIER (handle_id / provider_id /
    //      resource_kind / resume_policy), consumed BY MOVE — the carrier's typed
    //      ids move straight into the handle with NO re-typing (representation is
    //      the interface; the String -> typed lowering already happened once at
    //      the provider-output admit, rung-1-carrier-delta);
    //   3. the runtime-composed `resource_binding_id` (the "4-in-neither" id),
    //      derived by the finite owner op `ResourceBindingId::compose_for_
    //      provider_output_binding_...` from the carrier's provider binding ⊕ the
    //      template's authority scope — deterministic + resume-stable.
    // The release obligation is minted from the composed id + the same provider
    // binding authority. `liveness_owner` / `execution` / `contract_hash` are the
    // ADR law-default `None` (region-scoped subset). The bound `value` rides on
    // as the binding's resource value. Private in-module struct literals; no RAW
    // getter, no cross-crate field read, no public reconstruction path.
    pub(crate) fn merge_static_authority_template_with_runtime_carrier_for_swarmvm_session_runtime_owner_v1(
        template: &HostResourceBindingTemplateValue,
        carrier: HostResourceHandleIdentityCarrierValue,
        selected_provider_release: SelectedProviderHostResourceReleaseV1,
        bound_value: RuntimeValue,
    ) -> Self {
        let HostResourceHandleIdentityCarrierValue {
            handle_id,
            provider_binding_id,
            resource_kind,
            resume_policy,
            observation_source_ref: _,
            // Row #153 CARRIED-SYNCHRONOUS: the AUTHORED payload is consumed BY
            // MOVE out of the carrier here and committed as the binding's authored
            // resource value — the binding no longer carries identity-only.
            authored_resource_value,
            release_custody: _,
        } = carrier;
        let resource_binding_id =
            ResourceBindingId::compose_for_provider_output_binding_for_swarmvm_session_runtime_owner_v1(
                &provider_binding_id,
                &template.authority_scope,
            );
        let release_obligation =
            HostResourceReleaseObligationValue::required_provider_binding_authority_id(
                &resource_binding_id,
                template
                    .authority_scope
                    .duplicate_for_runtime_types_owner_v1(),
                provider_binding_id.as_str(),
            );
        let handle = HostResourceHandleValue {
            kind: HostResourceHandleKind::HostResourceHandle,
            resource_binding_id,
            handle_id,
            provider_id: provider_binding_id,
            resource_kind,
            operation_id: template.operation_id.duplicate_for_runtime_types_owner_v1(),
            authority_scope: template
                .authority_scope
                .duplicate_for_runtime_types_owner_v1(),
            liveness_owner: None,
            lifetime: template.lifetime.clone(),
            resume_policy,
            execution: None,
            contract_hash: None,
        };
        Self::from_runtime_family_parts(
            template.kind,
            handle,
            release_obligation,
            bound_value,
            authored_resource_value,
            selected_provider_release,
        )
    }

    pub fn kind(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceBindingKind {
        let _ = self;
        match input {}
    }

    pub fn handle(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceHandleValue {
        let _ = self;
        match input {}
    }

    pub fn handle_for_swarmvm_image_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceHandleValue {
        let _ = self;
        match input {}
    }

    pub fn release_obligation(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceReleaseObligationValue {
        let _ = self;
        match input {}
    }

    pub fn release_obligation_for_swarmvm_image_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &HostResourceReleaseObligationValue {
        let _ = self;
        match input {}
    }

    fn value(
        &self,
        projection_poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &RuntimeValue {
        let _ = self;
        match projection_poison {}
    }

    pub(crate) fn value_for_swarmvm_image_owner_v1(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &RuntimeValue {
        let _ = self;
        match poison {}
    }

    pub(crate) fn value_for_machine_core_owner_v1(
        &self,
        poison: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &RuntimeValue {
        let _ = self;
        match poison {}
    }

    pub(crate) fn graph_parts_for_runtime_heap_owner_v1(
        &self,
    ) -> (
        HostResourceBindingKind,
        HostResourceHandleValue,
        HostResourceReleaseObligationValue,
        &RuntimeValue,
    ) {
        (
            self.kind,
            self.handle
                .duplicate_for_checkpoint_body_authority_owner_v1(),
            self.release_obligation
                .duplicate_for_runtime_types_owner_v1(),
            self.value.as_ref(),
        )
    }

    pub(crate) fn provider_value_for_runtime_heap_owner_v1(&self) -> ProviderValue {
        // Row #153 CARRIED-SYNCHRONOUS: the binding-materialization owner projects
        // the sealed AUTHORED payload as the resource's value-shape — this is the
        // final `.value` observation a strict member read folds through the
        // existing Object arm. Only a scoped-resource binding carries a payload; a
        // payload-less host-resource binding keeps the identity-handle round-trip.
        if let Some(authored) = &self.authored_resource_value {
            return authored
                .clone()
                .into_provider_value_for_provider_value_model_owner_v1();
        }
        let resume_policy = match self.handle.resume_policy {
            HostResourceResumePolicy::NotResumable => {
                swarm_provider_value_model::HostResourceResumePolicy::NotResumable
            }
            HostResourceResumePolicy::HostRebindRequired => {
                swarm_provider_value_model::HostResourceResumePolicy::HostRebindRequired
            }
            HostResourceResumePolicy::CheckpointResumable => {
                swarm_provider_value_model::HostResourceResumePolicy::CheckpointResumable
            }
        };
        ProviderValue::host_resource_handle_from_runtime_activity_input_owner_v1(
            self.handle.handle_id.as_str().to_owned(),
            self.handle.provider_id.as_str().to_owned(),
            self.handle.resource_kind.as_str().to_owned(),
            resume_policy,
            self.handle.resource_binding_id.as_str().to_owned(),
        )
    }

    pub(crate) fn into_runtime_family_lifecycle_entry_for_one_shot_owner_v1(
        self,
    ) -> RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1 {
        RuntimeFamilyHostResourceLifecycleEntryForOneShotOwnerV1 {
            _handle: self.handle,
            _release_obligation: self.release_obligation,
            _release_value: self.value,
            _selected_provider_release: self.selected_provider_release,
        }
    }
}

// A2-RUNG-2 (HRB): the sealed STATIC-AUTHORITY-ONLY template. RESHAPED per the
// MERGE-SHAPE ratification (ADR-2002 §2 placement law): the image-embedded
// template carries ONLY compile-time static-authority facts — the binding kind,
// the fixed canonical operation id, the with-region lexical authority scope, and
// the ADR-1970 law-default lifetime. It CANNOT hold the downstream-minted
// runtime identity (`handle` / `resource_binding_id` / `release_obligation`) —
// those are minted only at the rung-3 merge from template ⊕ runtime-identity
// carrier, so a compile-time image site can never carry a runtime binding
// identity. Private fields; no getter yields authority.
#[derive(Debug, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostResourceBindingTemplateValue {
    kind: HostResourceBindingKind,
    operation_id: InstructionOpId,
    authority_scope: HostResourceAuthorityScope,
    lifetime: HostResourceLifetime,
}

impl HostResourceBindingTemplateValue {
    // A2-RUNG-2 (HRB) step 2: the SOLE mint of the sealed static-authority
    // template, from the four checked/owned static-authority facts. The kind is
    // the checked host-resource discrimination literal; operation_id is the
    // fixed canonical make-host-resource-binding OPREF (isa-types owner op);
    // authority_scope is the with-region lexical identity (isa-types owner op
    // over the module/body/binding coordinates, PIN #1); lifetime is the
    // ADR-1970 law default for the region-scoped subset (PIN #2). No runtime
    // identity is representable, so re-minting the template per resume can never
    // forge a binding (the identity is minted only at rung-3).
    pub(crate) fn mint_static_authority_for_swarmvm_image_owner_v1(
        kind: HostResourceBindingKind,
        operation_id: InstructionOpId,
        authority_scope: HostResourceAuthorityScope,
        lifetime: HostResourceLifetime,
    ) -> Self {
        Self {
            kind,
            operation_id,
            authority_scope,
            lifetime,
        }
    }

    pub(crate) fn duplicate_for_runtime_types_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_runtime_types_owner_internal_v1(&self) -> Self {
        Self {
            kind: self.kind,
            operation_id: self.operation_id.duplicate_for_runtime_types_owner_v1(),
            authority_scope: self.authority_scope.duplicate_for_runtime_types_owner_v1(),
            lifetime: self.lifetime.clone(),
        }
    }
}

pub struct HostResourceHandleIdentityCarrierValue {
    handle_id: HostResourceHandleId,
    provider_binding_id: HostResourceProviderId,
    resource_kind: HostResourceKind,
    resume_policy: HostResourceResumePolicy,
    observation_source_ref: String,
    // Row #153 CARRIED-SYNCHRONOUS: the resource's AUTHORED payload sealed at the
    // provider output, riding the identity carrier by data (infallibly `Clone`
    // via `AuthoredResourceValue`). `Some` only for a scoped-resource carrier;
    // consumed BY MOVE into the binding by the rung-3 merge owner and projected
    // to the final `.value` observation by the binding-materialization owner.
    authored_resource_value: Option<AuthoredResourceValue>,
    release_custody: Arc<ProviderHostResourceReleaseCustodyV1>,
}

// compiler-custody: symbol=ProviderHostResourceReleaseCustodyV1 disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=SessionExecutionKernel::drive; consumer=state::ProcessSessionV0; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
struct ProviderHostResourceReleaseCustodyV1 {
    selected: Mutex<Option<SelectedProviderHostResourceReleaseV1>>,
}

impl std::fmt::Debug for HostResourceHandleIdentityCarrierValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HostResourceHandleIdentityCarrierValue")
            .field("handle_id", &self.handle_id)
            .field("provider_binding_id", &self.provider_binding_id)
            .field("resource_kind", &self.resource_kind)
            .field("resume_policy", &self.resume_policy)
            .field("observation_source_ref", &self.observation_source_ref)
            .field("release_authority", &"sealed_shared_one_take")
            .finish()
    }
}

impl PartialEq for HostResourceHandleIdentityCarrierValue {
    fn eq(&self, other: &Self) -> bool {
        self.handle_id == other.handle_id
            && self.provider_binding_id == other.provider_binding_id
            && self.resource_kind == other.resource_kind
            && self.resume_policy == other.resume_policy
            && self.observation_source_ref == other.observation_source_ref
            && self.authored_resource_value == other.authored_resource_value
    }
}

impl Serialize for HostResourceHandleIdentityCarrierValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut value = serializer.serialize_struct("HostResourceHandleIdentityCarrierValue", 6)?;
        value.serialize_field("kind", "host_resource_handle")?;
        value.serialize_field("handle_id", &self.handle_id)?;
        value.serialize_field("provider_binding_id", &self.provider_binding_id)?;
        value.serialize_field("resource_kind", &self.resource_kind)?;
        value.serialize_field("resume_policy", &self.resume_policy)?;
        value.serialize_field("observation_source_ref", &self.observation_source_ref)?;
        value.end()
    }
}

impl HostResourceHandleIdentityCarrierValue {
    /// The one admission lane: a provider-output host-resource handle carrier
    /// admits as sealed runtime-identity data. Provenance: a
    /// `ProviderValue::HostResourceHandle` can only arrive from a
    /// provider-effect result (the capability mint's own product).
    pub fn admit_provider_output_carrier_for_swarmvm_session_runtime_owner_v1(
        carrier: &swarm_provider_value_model::HostResourceHandleCarrier,
    ) -> Result<Self, String> {
        let resume_policy = match carrier.resume_policy() {
            swarm_provider_value_model::HostResourceResumePolicy::NotResumable => {
                HostResourceResumePolicy::NotResumable
            }
            swarm_provider_value_model::HostResourceResumePolicy::HostRebindRequired => {
                HostResourceResumePolicy::HostRebindRequired
            }
            swarm_provider_value_model::HostResourceResumePolicy::CheckpointResumable => {
                HostResourceResumePolicy::CheckpointResumable
            }
        };
        // HRB RUNG-1b: the SOLE String -> typed-id lowering, at the authority
        // boundary. A malformed provider transport string surfaces a typed admit
        // fault (threaded into the caller's admission error), never a panic.
        Ok(Self {
            handle_id: HostResourceHandleId::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                carrier.handle_id(),
            )?,
            provider_binding_id:
                HostResourceProviderId::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                    carrier.provider_binding_id(),
                )?,
            resource_kind: HostResourceKind::try_new_for_provider_output_swarmvm_session_runtime_owner_v1(
                carrier.resource_kind(),
            )?,
            resume_policy,
            observation_source_ref: carrier.observation_source_ref().to_owned(),
            // Row #153: the payload is attached separately by the provider-output
            // boundary owner (which owns the value-model carrier by value and
            // moves the sealed payload out). Admission builds the identity half.
            authored_resource_value: None,
            release_custody: Arc::new(ProviderHostResourceReleaseCustodyV1 {
                selected: Mutex::new(None),
            }),
        })
    }

    pub(crate) fn with_provider_host_resource_release_for_session_execution_kernel_owner_v1(
        mut self,
        selected: Option<SelectedProviderHostResourceReleaseV1>,
    ) -> Self {
        self.release_custody = Arc::new(ProviderHostResourceReleaseCustodyV1 {
            selected: Mutex::new(selected),
        });
        self
    }

    pub(crate) fn try_take_provider_host_resource_release_for_session_execution_kernel_owner_v1(
        self,
    ) -> Result<
        (Self, SelectedProviderHostResourceReleaseV1),
        (Self, ProviderHostResourceReleaseFaultV1),
    > {
        let selected = match self.release_custody.selected.lock() {
            Ok(mut custody) => custody
                .take()
                .ok_or(ProviderHostResourceReleaseFaultV1::TransferAuthorityMissing),
            Err(_) => Err(ProviderHostResourceReleaseFaultV1::OwnerUnavailable),
        };
        match selected {
            Ok(selected) => Ok((self, selected)),
            Err(fault) => Err((self, fault)),
        }
    }

    // Row #153: the provider-output boundary owner attaches the AUTHORED payload
    // it moved out of the value-model carrier onto the sealed runtime identity
    // carrier. `pub(crate)` builder consumed once at the boundary; no getter.
    pub(crate) fn with_authored_resource_value_for_swarmvm_session_runtime_owner_v1(
        mut self,
        authored_resource_value: Option<AuthoredResourceValue>,
    ) -> Self {
        self.authored_resource_value = authored_resource_value;
        self
    }

    /// Data duplicate (the carrier is identity data, not authority; the rung-3
    /// merge is the only authority mint and it consumes the carrier there).
    pub fn duplicate_for_session_execution_kernel_owner_v1(&self) -> Self {
        Self {
            handle_id: self.handle_id.duplicate_for_runtime_types_owner_v1(),
            provider_binding_id: self
                .provider_binding_id
                .duplicate_for_runtime_types_owner_v1(),
            resource_kind: self.resource_kind.duplicate_for_runtime_types_owner_v1(),
            resume_policy: self.resume_policy.clone(),
            observation_source_ref: self.observation_source_ref.clone(),
            // Row #153: the authored payload is data — duplicates as data.
            authored_resource_value: self.authored_resource_value.clone(),
            release_custody: Arc::clone(&self.release_custody),
        }
    }

    /// Crate-unit observation of the otherwise sealed custody identity. This
    /// exposes no release authority or selector; it only lets regression tests
    /// distinguish an exact carrier duplicate from data-only re-admission,
    /// which would silently mint a fresh empty custody cell.
    #[cfg(test)]
    pub(crate) fn shares_provider_host_resource_release_custody_for_crate_unit_tests_v1(
        &self,
        other: &Self,
    ) -> bool {
        Arc::ptr_eq(&self.release_custody, &other.release_custody)
    }

    pub(crate) fn requires_host_rebind_for_runtime_activity_input_owner_v1(&self) -> bool {
        matches!(
            self.resume_policy,
            HostResourceResumePolicy::HostRebindRequired
        )
    }

    // Row #153 CARRIED-SYNCHRONOUS: a member-read BASE sees the resource's
    // value-shape — the sealed AUTHORED payload projected as its Object/scalar —
    // not the identity handle. This is the final `.value` observation a strict
    // member read folds through the existing Object arm. A payload-less
    // host-resource handle keeps the identity round-trip byte-identical, so every
    // non-scoped host-resource handle is unchanged.
    pub(crate) fn provider_value_member_base_for_runtime_heap_owner_v1(&self) -> ProviderValue {
        match &self.authored_resource_value {
            Some(authored) => authored
                .clone()
                .into_provider_value_for_provider_value_model_owner_v1(),
            None => self.provider_value_for_session_execution_kernel_owner_v1(),
        }
    }

    /// Boundary round-trip: the carrier re-emits exactly the provider handle
    /// it was admitted from (data fidelity, no authority derivation).
    pub fn provider_value_for_session_execution_kernel_owner_v1(&self) -> ProviderValue {
        let resume_policy = match self.resume_policy {
            HostResourceResumePolicy::NotResumable => {
                swarm_provider_value_model::HostResourceResumePolicy::NotResumable
            }
            HostResourceResumePolicy::HostRebindRequired => {
                swarm_provider_value_model::HostResourceResumePolicy::HostRebindRequired
            }
            HostResourceResumePolicy::CheckpointResumable => {
                swarm_provider_value_model::HostResourceResumePolicy::CheckpointResumable
            }
        };
        // Row #153: a payload-bearing carrier round-trips WITH its sealed
        // authored value (data fidelity); an identity-only carrier keeps the
        // five-field mint unchanged.
        match &self.authored_resource_value {
            Some(authored) => {
                ProviderValue::host_resource_handle_with_authored_resource_value_for_runtime_activity_input_owner_v1(
                    self.handle_id.as_str().to_owned(),
                    self.provider_binding_id.as_str().to_owned(),
                    self.resource_kind.as_str().to_owned(),
                    resume_policy,
                    self.observation_source_ref.clone(),
                    authored.clone(),
                )
            }
            None => ProviderValue::host_resource_handle_from_runtime_activity_input_owner_v1(
                self.handle_id.as_str().to_owned(),
                self.provider_binding_id.as_str().to_owned(),
                self.resource_kind.as_str().to_owned(),
                resume_policy,
                self.observation_source_ref.clone(),
            ),
        }
    }

    /// Observation-only logical size (memory accounting).
    pub(crate) fn logical_byte_estimate_for_runtime_observation_owner_v1(&self) -> u64 {
        (self.handle_id.as_str().len()
            + self.provider_binding_id.as_str().len()
            + self.resource_kind.as_str().len()
            + self.observation_source_ref.len()) as u64
            + 1
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TransactionHandleKind {
    #[serde(rename = "transaction_handle")]
    TransactionHandle,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TransactionHandleValue {
    kind: TransactionHandleKind,
    handle_id: TransactionHandleId,
    operation_id: InstructionOpId,
}

impl Serialize for TransactionHandleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut value = serializer.serialize_struct("TransactionHandleValue", 3)?;
        value.serialize_field("kind", &self.kind)?;
        value.serialize_field("handle_id", &self.handle_id)?;
        value.serialize_field("operation_id", &self.operation_id)?;
        value.end()
    }
}

impl TransactionHandleValue {
    pub(crate) fn from_swarmvm_session_runtime_owner_parts_v1(
        value: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> Self {
        match value {}
    }

    pub(crate) fn handle_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &TransactionHandleId {
        let _ = self;
        match input {}
    }

    pub(crate) fn operation_id(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> &InstructionOpId {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_one_shot_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_runtime_heap_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }

    pub(crate) fn duplicate_for_checkpoint_body_authority_owner_v1(&self) -> Self {
        Self {
            kind: self.kind,
            handle_id: self.handle_id.duplicate_for_runtime_types_owner_v1(),
            operation_id: self.operation_id.duplicate_for_runtime_types_owner_v1(),
        }
    }

    pub(crate) fn duplicate_for_one_shot_scoped_lifecycle_owner_v1(&self) -> Self {
        Self {
            kind: self.kind,
            handle_id: self.handle_id.duplicate_for_runtime_types_owner_v1(),
            operation_id: self.operation_id.duplicate_for_runtime_types_owner_v1(),
        }
    }

    pub(crate) fn matches_for_one_shot_scoped_lifecycle_owner_v1(&self, observed: &Self) -> bool {
        self == observed
    }

    pub(crate) fn duplicate_for_swarmvm_session_runtime_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }
}

impl TransactionHandleValue {
    pub(crate) fn duplicate_for_runtime_types_owner_v1(
        &self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Self {
        let _ = self;
        match input {}
    }
}

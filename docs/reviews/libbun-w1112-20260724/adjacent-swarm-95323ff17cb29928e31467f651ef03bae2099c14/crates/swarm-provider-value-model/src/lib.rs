#![forbid(unsafe_code)]

/// Canonical-string owner re-export (string-model rung S2; S5 re-plumb).
///
/// The ONE NFC owner lives in the leaf crate `swarm-canonical-string` so the
/// syntax-phase consumers (lexer cook, diagnostic twin) can reach it without a
/// dependency cycle through this crate. This re-export is the stable
/// value-space path: every existing `swarm_provider_value_model::
/// canonical_string` consumer keeps compiling unchanged.
pub mod canonical_string {
    pub use swarm_canonical_string::{CanonicalString, canonical_string};
}
mod canonical_json;
pub mod grapheme;
mod mesh_operation_authority;
mod process_lifecycle_authority;
mod process_replan_authority;
pub mod semantics_version;
mod value_algebra;
use canonical_json::{
    CanonicalJsonV1LexicalAdmission, ProviderValueJsonProjectionMode,
    WIDE_INTEGER_JSON_PROJECTION_KIND, WIDE_INTEGER_JSON_PROJECTION_KIND_FIELD,
    WIDE_INTEGER_JSON_PROJECTION_TEXT_FIELD, canonical_json_v1_lexical_admission,
    is_reserved_provider_carrier_kind, provider_value_from_json_value,
    provider_value_to_json_string_with_mode,
};
pub use canonical_string::{CanonicalString, canonical_string};
pub use grapheme::{byte_to_grapheme_index, grapheme_len, grapheme_nth, grapheme_slice};
pub use mesh_operation_authority::{
    LiveOperationHandleCarrier, LiveStreamHandleCarrier,
    MeshProviderOperationAuthorityForMeshControlOwnerV1,
    MeshProviderOperationCarrierJoinForMeshControlOwnerV1,
    MeshProviderOperationCarrierRolesForMeshControlOwnerV1,
    MeshProviderOperationCarrierUseForMeshControlOwnerV1,
    MeshProviderOperationStreamAuthorityForMeshControlOwnerV1,
    MeshProviderOperationStreamCarrierJoinForMeshControlOwnerV1,
    MeshProviderOperationStreamCarrierUseForMeshControlOwnerV1,
};
pub use process_lifecycle_authority::{
    MatchedProcessCheckpointForDirectRunOwnerV1, ProcessCheckpointAuthorityForDirectRunOwnerV1,
    ProcessCheckpointCarrierForSessionRuntimeOwnerV1,
    ProcessCheckpointCarrierJoinForDirectRunOwnerV1, ProcessCheckpointRolesForDirectRunOwnerV1,
};
pub use process_replan_authority::{
    CurrentProcessAuthorityForSessionRuntimeOwnerV1, CurrentProcessCarrierForSessionRuntimeOwnerV1,
    CurrentProcessCarrierJoinForSessionRuntimeOwnerV1,
    CurrentProcessLineageForProcessReplanAuthorityOwnerV1,
    CurrentProcessSnapshotJoinForProcessReplanAuthorityOwnerV1,
    CurrentProcessSubjectForProcessReplanAuthorityOwnerV1,
    CurrentProcessSubjectLineageJoinForProcessReplanAuthorityOwnerV1,
    JoinedCurrentProcessSnapshotForProcessReplanAuthorityOwnerV1,
    JoinedProcessPlanSnapshotCarriersForCompilerGraphReconcileOwnerV1,
    ProcessPlanSnapshotAuthorityForProcessReplanAuthorityOwnerV1,
    ProcessPlanSnapshotCarrierExactJoinForCompilerGraphReconcileOwnerV1,
    ProcessPlanSnapshotCarrierForSessionRuntimeOwnerV1,
    ProcessPlanSnapshotRolesForProcessReplanAuthorityOwnerV1,
};
pub use semantics_version::{SEMANTICS_UNICODE_VERSION, SemanticsUnicodeVersion};
use std::collections::BTreeMap;
#[cfg(test)]
use std::fmt;

/// The exact, arbitrary-precision integer backing `ProviderValue::Integer`
/// (NUMBER-MODEL charter rung N2: exact carriage replaces the bounded `i64`).
/// Re-exported so every `ProviderValue` consumer mints/extracts through the
/// sealed number-model owner.
pub use swarmscript_number_model::SwarmInteger;
// Rung N4: `SwarmNumber` is part of the public mint surface
// (`number_from_swarm_number_canonical_v1`), so callers can name the
// parameter type without a separate number-model dependency.
pub use swarmscript_number_model::SwarmNumber;
pub const PROVIDER_VALUE_HOST_RESOURCE_HANDLE_KIND: &str = "swarm.provider.host_resource_handle.v1";
pub const PROVIDER_VALUE_LIVE_OPERATION_HANDLE_KIND: &str =
    "swarm.provider.live_operation_handle.v1";
pub const PROVIDER_VALUE_LIVE_STREAM_HANDLE_KIND: &str = "swarm.provider.live_stream_handle.v1";

#[derive(Debug, PartialEq, Eq)]
pub struct HostResourceHandleCarrier {
    handle_id: String,
    provider_binding_id: String,
    resource_kind: String,
    resume_policy: HostResourceResumePolicy,
    observation_source_ref: String,
    // Row #153 CARRIED-SYNCHRONOUS: the resource's AUTHORED payload (`Resource<T>`
    // for any data `T`), sealed at mint alongside the five identity fields. A
    // scoped-resource mint carries `Some`; every other host-resource handle mint
    // carries `None`. PRIVATE — never a public raw getter; the runtime boundary
    // owner consumes it BY MOVE (`into_authored_resource_value_*`) into the
    // sealed runtime identity carrier, and it is projected back to the final
    // `.value` observation only by the finite binding-materialization owner.
    authored_resource_value: Option<AuthoredResourceValue>,
}

/// Row #153 CARRIED-SYNCHRONOUS: the sealed AUTHORED RESOURCE VALUE — the
/// data-only payload a host-resource binding carries as its `Resource<T>.value`
/// surface. A structural mirror of the DATA variants of [`ProviderValue`] (no
/// host-resource-handle / live-operation / live-stream variant), so it is
/// infallibly `Clone`. Minted at the
/// provider output from the authored payload (refusing any non-data variant),
/// carried through the host-resource handle carrier, consumed into the binding by
/// the runtime merge owner, and projected back to a `ProviderValue` for the final
/// `.value` observation.
pub struct AuthoredResourceValue {
    inner: value_algebra::AuthoredResourceValueInner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostResourceResumePolicy {
    NotResumable,
    HostRebindRequired,
    CheckpointResumable,
}

pub enum ProviderValue {
    Null,
    Bool(bool),
    Integer(SwarmInteger),
    Number(FiniteProviderNumber),
    String(String),
    /// Exact binary payload (string-model rung S4 / value-model ruling: "Bytes =
    /// exact binary payload; byte-exact round-trip lives in Bytes"). Derived
    /// `Eq` is byte equality — one payload, one representation. Minting a
    /// `string` from bytes normalizes (`Bytes.decodeUtf8`); there is no
    /// string-as-byte-bag.
    Bytes(Vec<u8>),
    Array(ProviderValueArray),
    Object(ProviderValueObject),
    HostResourceHandle(HostResourceHandleCarrier),
    LiveOperationHandle(LiveOperationHandleCarrier),
    LiveStreamHandle(LiveStreamHandleCarrier),
    /// Nominal current-process authority paired with a data-only authored
    /// projection. It has no JSON or structural-identity representation.
    CurrentProcess(CurrentProcessCarrierForSessionRuntimeOwnerV1),
    /// Owner-issued durable checkpoint. Its public fields are inert
    /// observations; only the sealed carrier can select restore authority.
    ProcessCheckpoint(ProcessCheckpointCarrierForSessionRuntimeOwnerV1),
    /// Owner-issued process-plan snapshot. The authored object is projection;
    /// only this opaque carrier can select the owner-held snapshot authority.
    ProcessPlanSnapshot(ProcessPlanSnapshotCarrierForSessionRuntimeOwnerV1),
}

/// Language-level structural equality for values that expose observable value
/// semantics.
///

/// Shallow array edge for [`ProviderValue`]. Public construction remains an
/// explicit `Vec::into()` conversion while all ordinary teardown is iterative.
pub struct ProviderValueArray(Vec<ProviderValue>);

/// Shallow object edge for [`ProviderValue`]. `BTreeMap` order and lookup
/// behavior are preserved through the map view implementations below.
pub struct ProviderValueObject(BTreeMap<String, ProviderValue>);

#[derive(Clone, Copy, Debug)]
pub struct FiniteProviderNumber {
    value: f64,
}

pub fn provider_value_to_canonical_json_v1(
    value: &ProviderValue,
) -> Result<String, ProviderValueJsonAdmissionError> {
    validate_provider_value(value)
        .map_err(|source| ProviderValueJsonAdmissionError::ValueAdmissionFailed { source })?;
    provider_value_to_json_string_with_mode(value, ProviderValueJsonProjectionMode::DataBoundary)
        .map_err(|source| ProviderValueJsonAdmissionError::ValueAdmissionFailed { source })
}

/// Canonical data-only observation used to fingerprint a provider output that
/// may carry owner-minted mesh operation authority. The authority itself is
/// never serialized: live operation and stream carriers contribute only their
/// authored projections. Generic JSON encoding remains fail-closed, so this
/// observation cannot become a wire or re-mint path for either carrier.
pub fn provider_value_to_canonical_output_observation_json_v1(
    value: &ProviderValue,
) -> Result<String, ProviderValueJsonAdmissionError> {
    validate_provider_value(value)
        .map_err(|source| ProviderValueJsonAdmissionError::ValueAdmissionFailed { source })?;
    provider_value_to_json_string_with_mode(
        value,
        ProviderValueJsonProjectionMode::ProviderOutputObservation,
    )
    .map_err(|source| ProviderValueJsonAdmissionError::ValueAdmissionFailed { source })
}

/// THE canonical inverse of [`provider_value_to_canonical_json_v1`] — admit a
/// provider *user-payload* value from its canonical JSON text. This data
/// admission does not select an invocation or admit provider settlement cargo.
/// Decode canonicalization mirrors the datastore twin (rung N5): bare negative
/// integers through `i64::MIN` and nonnegative integers through `u64::MAX`
/// re-mint exactly, wider bare integers refuse before serde and must use the
/// reserved wide-integer projection, integral finite floats re-mint their exact
/// binary64 integer value, non-integral finite numbers become `Number`, and
/// non-finite values refuse.
/// Reserved provider-value carrier object kinds (host/handle carriers) are
/// refused fail-closed so a provider response can never forge a handle from
/// RAW JSON.
pub fn provider_value_from_canonical_json_v1(
    json: &str,
) -> Result<ProviderValue, ProviderValueJsonAdmissionError> {
    match canonical_json_v1_lexical_admission(json) {
        CanonicalJsonV1LexicalAdmission::Admitted => {}
        CanonicalJsonV1LexicalAdmission::Invalid { offset, expected } => {
            return Err(ProviderValueJsonAdmissionError::InvalidJson(format!(
                "canonical JSON lexical admission failed at byte {offset}: {expected}"
            )));
        }
        CanonicalJsonV1LexicalAdmission::BareIntegerRequiresWideProjection {
            offset,
            length,
            observation,
        } => {
            return Err(
                ProviderValueJsonAdmissionError::BareIntegerRequiresWideProjection {
                    offset,
                    length,
                    observation,
                },
            );
        }
        CanonicalJsonV1LexicalAdmission::DuplicateObjectKey { offset, path } => {
            return Err(ProviderValueJsonAdmissionError::DuplicateObjectKey { offset, path });
        }
    }
    let json_value: serde_json::Value = serde_json::from_str(json)
        .map_err(|source| ProviderValueJsonAdmissionError::InvalidJson(source.to_string()))?;
    let value = provider_value_from_json_value(json_value)?;
    validate_provider_value(&value)
        .map_err(|source| ProviderValueJsonAdmissionError::ValueAdmissionFailed { source })?;
    Ok(value)
}

/// TRUE iff a provider-value object `kind` string is reserved carrier
/// vocabulary (raw host carriers + the wide-integer projection). Exported so
/// non-JSON encode funnels (e.g. the datastore DAG-CBOR twin) can refuse the
/// same reserved vocabulary fail-closed before minting their own envelopes.
pub fn is_reserved_provider_value_object_kind_v1(kind: &str) -> bool {
    is_reserved_provider_carrier_kind(kind)
}

/// THE canonical JSON projection for an exact integer (rung N5 — encode is
/// TOTAL): `i64`/`u64`-fitting values stay plain JSON integers (byte-compatible
/// with the pre-N5 encoding); wider magnitudes project as the reserved
/// wide-integer object carrying canonical decimal digits. Never a rounded
/// float, never a refusal.
pub fn integer_json_projection_value_v1(value: &SwarmInteger) -> serde_json::Value {
    if let Some(value) = value.to_i64() {
        return serde_json::Value::Number(serde_json::Number::from(value));
    }
    if let Some(value) = value.to_u64() {
        return serde_json::Value::Number(serde_json::Number::from(value));
    }
    let mut fields = serde_json::Map::with_capacity(2);
    fields.insert(
        WIDE_INTEGER_JSON_PROJECTION_KIND_FIELD.to_owned(),
        serde_json::Value::String(WIDE_INTEGER_JSON_PROJECTION_KIND.to_owned()),
    );
    fields.insert(
        WIDE_INTEGER_JSON_PROJECTION_TEXT_FIELD.to_owned(),
        serde_json::Value::String(value.to_string()),
    );
    serde_json::Value::Object(fields)
}

/// Decode verdict for a JSON object that may be the wide-integer projection.
/// A closed algebra, not an `Option` pair: `NotAWideIntegerProjection` (the
/// object does not carry the reserved kind — plain user cargo, pass through),
/// `Malformed` (the reserved kind with a non-canonical body — corruption, the
/// consumer must refuse typed; it can never be user cargo because encode
/// reserves the kind), or the exact re-minted integer.
#[derive(Debug, PartialEq)]
pub enum WideIntegerJsonProjectionDecodeV1 {
    NotAWideIntegerProjection,
    Malformed,
    Integer(SwarmInteger),
}

/// THE decode entry for the wide-integer JSON projection: exact re-mint via
/// [`SwarmInteger::from_canonical_decimal_str`] (fail-closed — one value, one
/// wire representation). A projection whose digits FIT `i64`/`u64` is also
/// `Malformed`: in-range integers have exactly one representation (the plain
/// JSON integer), so a wrapped in-range value is non-canonical by law.
pub fn wide_integer_json_projection_decode_v1(
    fields: &serde_json::Map<String, serde_json::Value>,
) -> WideIntegerJsonProjectionDecodeV1 {
    match fields.get(WIDE_INTEGER_JSON_PROJECTION_KIND_FIELD) {
        Some(serde_json::Value::String(kind)) if kind == WIDE_INTEGER_JSON_PROJECTION_KIND => {}
        _ => return WideIntegerJsonProjectionDecodeV1::NotAWideIntegerProjection,
    }
    if fields.len() != 2 {
        return WideIntegerJsonProjectionDecodeV1::Malformed;
    }
    let Some(serde_json::Value::String(text)) = fields.get(WIDE_INTEGER_JSON_PROJECTION_TEXT_FIELD)
    else {
        return WideIntegerJsonProjectionDecodeV1::Malformed;
    };
    let Some(value) = SwarmInteger::from_canonical_decimal_str(text) else {
        return WideIntegerJsonProjectionDecodeV1::Malformed;
    };
    if value.to_i64().is_some() || value.to_u64().is_some() {
        return WideIntegerJsonProjectionDecodeV1::Malformed;
    }
    WideIntegerJsonProjectionDecodeV1::Integer(value)
}

pub fn validate_provider_value(value: &ProviderValue) -> Result<(), ProviderValueAdmissionError> {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            ProviderValue::Array(values) => pending.extend(values.iter().rev()),
            ProviderValue::Object(properties) => {
                if let Some(ProviderValue::String(kind)) = properties.get("kind") {
                    if is_reserved_provider_carrier_kind(kind) {
                        return Err(ProviderValueAdmissionError::ReservedRawCarrierObject {
                            carrier_kind: kind.clone(),
                        });
                    }
                }
                pending.extend(properties.values().rev());
            }
            ProviderValue::Null
            | ProviderValue::Bool(_)
            | ProviderValue::Integer(_)
            | ProviderValue::Number(_)
            | ProviderValue::String(_)
            | ProviderValue::Bytes(_)
            | ProviderValue::HostResourceHandle(_)
            | ProviderValue::LiveOperationHandle(_)
            | ProviderValue::LiveStreamHandle(_)
            | ProviderValue::CurrentProcess(_)
            | ProviderValue::ProcessCheckpoint(_)
            | ProviderValue::ProcessPlanSnapshot(_) => {}
        }
    }
    Ok(())
}

pub fn validate_provider_user_payload_value(
    value: &ProviderValue,
    provider_binding_id: &str,
) -> Result<(), ProviderValueAdmissionError> {
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            ProviderValue::Array(values) => pending.extend(values.iter().rev()),
            ProviderValue::Object(properties) => {
                if let Some(ProviderValue::String(kind)) = properties.get("kind") {
                    if is_reserved_provider_carrier_kind(kind) {
                        return Err(ProviderValueAdmissionError::ReservedRawCarrierObject {
                            carrier_kind: kind.clone(),
                        });
                    }
                }
                pending.extend(properties.values().rev());
            }
            ProviderValue::HostResourceHandle(carrier)
                if carrier.provider_binding_id() != provider_binding_id =>
            {
                return Err(
                    ProviderValueAdmissionError::HostResourceProviderBindingDrift {
                        expected: provider_binding_id.to_owned(),
                        observed: carrier.provider_binding_id().to_owned(),
                    },
                );
            }
            ProviderValue::CurrentProcess(_)
            | ProviderValue::ProcessCheckpoint(_)
            | ProviderValue::ProcessPlanSnapshot(_) => {
                return Err(ProviderValueAdmissionError::ProcessAuthorityHasNoJsonRepresentation);
            }
            ProviderValue::Null
            | ProviderValue::Bool(_)
            | ProviderValue::Integer(_)
            | ProviderValue::Number(_)
            | ProviderValue::String(_)
            | ProviderValue::Bytes(_)
            | ProviderValue::HostResourceHandle(_)
            | ProviderValue::LiveOperationHandle(_)
            | ProviderValue::LiveStreamHandle(_) => {}
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderValueAdmissionError {
    ReservedRawCarrierObject {
        carrier_kind: String,
    },
    HostResourceProviderBindingDrift {
        expected: String,
        observed: String,
    },
    /// A `Bytes` payload has no JSON representation at all. Encoding refuses
    /// instead of inventing a lossy carrier; byte carriage across a JSON
    /// boundary is an explicit boundary-contract concern, never a default.
    BytesHaveNoJsonRepresentation {
        len: usize,
    },
    /// Mesh operation authority is session-local and opaque. Its authored
    /// projection is observable through the runtime heap, but the carrier
    /// itself never has a generic JSON representation.
    LiveOperationHandleHasNoJsonRepresentation,
    /// Mesh operation-stream authority follows the same sealed-carrier law as
    /// its operation. JSON cannot carry or recreate stream selection rights.
    LiveStreamHandleHasNoJsonRepresentation,
    /// Current-process and replan carriers are session-local nominal
    /// authority. Their paired authored projection may be observed, but the
    /// carrier itself can never enter JSON or another generic data boundary.
    ProcessAuthorityHasNoJsonRepresentation,
}

pub type ProviderResultAdmissionError = ProviderValueAdmissionError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderValueJsonAdmissionError {
    InvalidJson(String),
    /// A decoded object-key identity occurred twice in one object. Offset
    /// points at the duplicate key's opening quote in the caller-retained
    /// input; path is a bounded JSON-pointer observation only.
    DuplicateObjectKey {
        offset: usize,
        path: String,
    },
    /// A plain JSON integer outside the exact V1 `i64`-negative / `u64`-
    /// nonnegative range must use the reserved `swarm.integer.v1` projection.
    /// Offset and length identify the complete lexeme in the caller-retained
    /// input; observation is a bounded exact prefix/suffix view for diagnostics.
    BareIntegerRequiresWideProjection {
        offset: usize,
        length: usize,
        observation: String,
    },
    ValueAdmissionFailed {
        source: ProviderValueAdmissionError,
    },
}

#[cfg(test)]
#[path = "tests/authored_resource_value_stack_safety_tests.rs"]
mod authored_resource_value_stack_safety_tests;

#[cfg(test)]
#[path = "tests/provider_bytes_tests.rs"]
mod provider_bytes_tests;

#[cfg(test)]
#[path = "tests/finite_provider_number_negative_zero_tests.rs"]
mod finite_provider_number_negative_zero_tests;

#[cfg(test)]
#[path = "tests/canonical_number_mint_tests.rs"]
mod canonical_number_mint_tests;

#[cfg(test)]
#[path = "tests/wide_integer_json_projection_tests.rs"]
mod wide_integer_json_projection_tests;

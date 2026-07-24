#![forbid(unsafe_code)]

// compiler-custody-scope: status=complete reviewer=package-root-compiler-kernel-hardcut-20260722 justification="all Rust source in this assigned owner was reviewed; every lexical custody candidate is classified adjacent to its item"

use serde::{Deserialize, Serialize};
use std::fmt;
use swarm_capability_linker_core::{
    CapabilityContractIdentityPartsForCapabilityModelOwner, CapabilityTypeContractError,
    LinkerIdentifierError,
};
use swarm_runtime_authority::RuntimeAuthorityError;
use thiserror::Error;

mod provider_boundary_correspondence;
pub use provider_boundary_correspondence::{
    CorrelatedAuthoredProviderOutput, CorrelatedProviderBoundaryOutput,
    CorrelatedProviderResultSettlement, CorrelatedRejectedProviderResultSettlement,
    MatchedProviderBoundaryOutputAuthority, PendingProviderBoundaryOutputCommitAuthority,
    ProviderBoundaryOutputAuthorityJoin, ProviderBoundaryOutputCorrespondenceFault,
    ProviderReadyBoundaryOutput, SelectedProviderBoundaryOutputAuthority,
    mint_provider_boundary_output_correspondence_v1,
};

pub const RUST_SDK_PROVIDER_REF_KIND: &str = "swarm.capability_grant_provider_ref.rust_sdk.v1";
pub const RUST_SDK_PROVIDER_DOMAIN: &str = "rust_sdk";
pub const PROVIDER_REQUEST_SCHEMA: &str = "swarm.capability_sdk.provider_request.v1";
pub const PROVIDER_RESULT_SCHEMA: &str = "swarm.capability_sdk.provider_result.v1";
pub const PROVIDER_ERROR_SCHEMA: &str = "swarm.capability_sdk.provider_error.v1";
pub const PROVIDER_STREAM_EVENT_SCHEMA: &str = "swarm.capability_sdk.provider_stream_event.v1";
pub const PROVIDER_PRIMITIVE_OPERATION_START_SCHEMA: &str =
    "swarm.capability_sdk.provider_primitive_operation_start.v1";
pub const PROVIDER_CONTINUATION_REF_SCHEMA: &str =
    "swarm.capability_sdk.provider_continuation_ref.v1";
pub const BOUNDARY_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.boundary_receipt.v1";
pub const DEADLINE_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.deadline_receipt.v1";
pub const CANCELLATION_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.cancellation_receipt.v1";
pub const LIVENESS_WAIT_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.liveness_wait_receipt.v1";
pub const PROVIDER_PARK_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.provider_park_receipt.v1";
pub const PROVIDER_OPERATION_START_REQUEST_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_start_request.v1";
pub const PROVIDER_OPERATION_SETTLE_REQUEST_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_settle_request.v1";
pub const PROVIDER_STREAM_NEXT_REQUEST_SCHEMA: &str =
    "swarm.capability_sdk.provider_stream_next_request.v1";
pub const PROVIDER_OPERATION_JOIN_REQUEST_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_join_request.v1";
pub const PROVIDER_OPERATION_CANCEL_REQUEST_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_cancel_request.v1";
pub const PROVIDER_OPERATION_REF_SCHEMA: &str = "swarm.capability_sdk.provider_operation_ref.v1";
pub const PROVIDER_OPERATION_START_RECEIPT_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_start_receipt.v1";
pub const PROVIDER_OPERATION_SETTLE_RECEIPT_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_settle_receipt.v1";
pub const PROVIDER_STREAM_NEXT_RECEIPT_SCHEMA: &str =
    "swarm.capability_sdk.provider_stream_next_receipt.v1";
pub const PROVIDER_JOIN_RECEIPT_SCHEMA: &str = "swarm.capability_sdk.provider_join_receipt.v1";
pub const PROVIDER_OPERATION_CANCEL_RECEIPT_SCHEMA: &str =
    "swarm.capability_sdk.provider_operation_cancel_receipt.v1";
pub const COMPILED_SWARM_BINARY_MANIFEST_SCHEMA: &str = "swarm.compiled_swarm_binary_manifest.v1";
pub const NATIVE_PROVIDER_MANIFEST_SCHEMA: &str = "swarm.native_provider_manifest.v1";
pub const NATIVE_PROVIDER_ABI_V1: &str = "swarm.native_provider_abi.v1";
pub const RUST_SDK_PROVIDER_HOST_ID: &str = "rust_sdk";
pub const RUST_SDK_PROVIDER_HOST_KIND: &str = "rust_sdk";
pub const LOADED_NATIVE_PROVIDER_HOST_KIND: &str = "native_provider_artifact";
pub const EXTERNAL_TRANSPORT_CAPABILITY_PROVIDER_HOST_KIND: &str =
    "external_transport_capability_provider";
pub const EXTERNAL_TRANSPORT_CAPABILITY_PROVIDER_REF_KIND: &str =
    "swarm.capability_grant_provider_ref.external_transport_capability_provider.v1";
pub const EXTERNAL_TRANSPORT_PROVIDER_DOMAIN: &str = "external_transport_capability_provider";

#[derive(Debug, Error)]
pub enum CapabilitySdkError {
    #[error(transparent)]
    Identifier(#[from] LinkerIdentifierError),
    #[error("capability projection package specifier must be nonblank and trimmed: {0:?}")]
    InvalidProjectionPackageSpecifier(String),
    #[error("capability projection export name must be nonblank and trimmed: {0:?}")]
    InvalidProjectionExportName(String),
    #[error("capability contract fingerprint must be a lowercase sha256 ref: {0}")]
    InvalidFingerprint(String),
    #[error("capability contract TSON source rejected: {0}")]
    InvalidContractTson(String),
    #[error(
        "capability contract fingerprint mismatch for {package_specifier}:{export_name}: expected {expected}, observed {observed}"
    )]
    ContractFingerprintMismatch {
        package_specifier: String,
        export_name: String,
        expected: String,
        observed: String,
    },
    #[error("provider continuation id must be nonblank and trimmed: {0:?}")]
    InvalidContinuationId(String),
    #[error("provider error code must be nonblank and trimmed: {0:?}")]
    InvalidProviderErrorCode(String),
    #[error("provider error details must be an object descriptor")]
    InvalidProviderErrorDetails,
    #[error("provider stream id must be nonblank and trimmed: {0:?}")]
    InvalidStreamId(String),
    #[error("provider operation id must be nonblank and trimmed: {0:?}")]
    InvalidProviderOperationId(String),
    #[error("provider operation start request must be an object: {0}")]
    InvalidProviderOperationStartRequest(String),
    #[error(
        "provider operation route {route} cannot mint {carrier}: expected {expected_provider_id} with {expected_contract_fingerprint}, observed {observed_provider_id} with {observed_contract_fingerprint}"
    )]
    InvalidProviderOperationRouteBinding {
        route: String,
        carrier: String,
        expected_provider_id: String,
        observed_provider_id: String,
        expected_contract_fingerprint: String,
        observed_contract_fingerprint: String,
    },
    #[error("provider stream cursor must be nonblank and trimmed when present: {0:?}")]
    InvalidProviderStreamCursor(String),
    #[error("provider boundary name must be nonblank and trimmed: {0:?}")]
    InvalidBoundaryName(String),
    #[error("provider deadline id must be nonblank and trimmed: {0:?}")]
    InvalidDeadlineId(String),
    #[error("provider cancellation id must be nonblank and trimmed: {0:?}")]
    InvalidCancellationId(String),
    #[error("provider liveness blocker kind must be nonblank and trimmed: {0:?}")]
    InvalidLivenessBlockerKind(String),
    #[error("provider host id must be nonblank and trimmed: {0:?}")]
    InvalidProviderHostId(String),
    #[error("provider host set has duplicate provider binding for {0}")]
    DuplicateProviderHostProvider(String),
    #[error("provider host set direct-run requirement rejected: {0}")]
    InvalidDirectRunProviderRequirement(String),
    #[error(
        "provider-host closed-sum output for {provider_id} is not the compiler-owned carrier object"
    )]
    ProviderHostClosedSumOutputNotCarrierObject { provider_id: String },
    #[error("provider-host closed-sum output carrier for {provider_id} is malformed")]
    ProviderHostClosedSumOutputCarrierMalformed { provider_id: String },
    #[error("provider-host closed-sum output carrier symbol for {provider_id} is malformed")]
    ProviderHostClosedSumOutputSymbolMalformed { provider_id: String },
    #[error(
        "provider-host closed-sum output symbol for {provider_id} must be {expected_symbol_path}, observed {observed_symbol_path}"
    )]
    ProviderHostClosedSumOutputSymbolMismatch {
        provider_id: String,
        expected_symbol_path: String,
        observed_symbol_path: String,
    },
    #[error("provider-host closed-sum output carrier variant for {provider_id} is malformed")]
    ProviderHostClosedSumOutputVariantMalformed { provider_id: String },
    #[error(
        "provider-host closed-sum output variant for {provider_id} is not declared by the admitted output type: {source}"
    )]
    ProviderHostClosedSumOutputVariantNotDeclared {
        provider_id: String,
        source: CapabilityTypeContractError,
    },
    #[error("provider-host closed-sum output payload flag for {provider_id} is malformed")]
    ProviderHostClosedSumOutputPayloadFlagMalformed { provider_id: String },
    #[error("provider-host closed-sum output payload for {provider_id} is missing")]
    ProviderHostClosedSumOutputPayloadMissing { provider_id: String },
    #[error("provider-host closed-sum unit payload for {provider_id} is malformed")]
    ProviderHostClosedSumOutputUnitPayloadMalformed { provider_id: String },
    #[error(
        "provider-host std.Result closed-sum output for {provider_id} must carry a payload for Ok and Err variants"
    )]
    ProviderHostStdResultClosedSumPayloadMissing { provider_id: String },
    #[error(
        "provider-host std.Result closed-sum output for {provider_id} has unsupported variant {variant}"
    )]
    ProviderHostStdResultClosedSumVariantUnsupported {
        provider_id: String,
        variant: String,
    },
    #[error(
        "live primitive source advance requires closed-sum provider-host output authority, got finite invocation result"
    )]
    ProviderHostLivePrimitiveSourceAdvanceGotInvocationResult,
    #[error(
        "live primitive source advance requires closed-sum provider-host output authority, got plain provider contract output"
    )]
    ProviderHostLivePrimitiveSourceAdvanceGotPlainContractOutput,
    #[error("provider-drive ready output requires sealed provider-host output authority: {source}")]
    ProviderHostReadyOutputTypeAdmission { source: CapabilityTypeContractError },
    #[error("provider host set has no Rust SDK capability provider for {provider_id}")]
    NoRustSdkProvider { provider_id: String },
    #[error("provider contract registry has duplicate record for {provider_id}")]
    DuplicateProviderContractRecord { provider_id: String },
    #[error(
        "provider {provider_id} requires external capability provider host {provider_execution_domain}, but no such provider host is installed"
    )]
    NoExternalCapabilityProviderHost {
        provider_id: String,
        provider_execution_domain: String,
    },
    #[error("provider contract {provider_id} has been removed and cannot be executed: {reason}")]
    RemovedProviderContract { provider_id: String, reason: String },
    #[error("compiled Swarm binary manifest rejected: {0}")]
    InvalidCompiledSwarmBinaryManifest(String),
    #[error("compiled Swarm binary manifest names provider host {host_id}, but it is not admitted")]
    NativeBinaryManifestProviderHostNotAdmitted { host_id: String },
    #[error(
        "compiled Swarm binary manifest contract fingerprint mismatch for {package_specifier}:{export_name}: expected {expected}, observed {observed}"
    )]
    NativeBinaryManifestContractFingerprintMismatch {
        package_specifier: String,
        export_name: String,
        expected: String,
        observed: String,
    },
    #[error("native provider manifest rejected: {0}")]
    InvalidNativeProviderManifest(String),
    #[error("native provider package declaration rejected: {0}")]
    InvalidNativeProviderPackageManifest(String),
    #[error("native provider manifest has no artifact for platform {platform}")]
    MissingNativeProviderArtifact { platform: String },
    #[error(
        "native provider descriptor mismatch for {provider_id}: expected {expected}, observed {observed}"
    )]
    NativeProviderDescriptorMismatch {
        provider_id: String,
        expected: String,
        observed: String,
    },
    #[error("native provider manifest set has duplicate provider manifest for {provider_id}")]
    DuplicateNativeProviderManifest { provider_id: String },
    #[error("no native provider manifest satisfies required contract {provider_id}")]
    NoNativeProviderManifestForContract { provider_id: String },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ProviderExecutionErrorKind(
    swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
);

impl fmt::Debug for ProviderExecutionErrorKind {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl Serialize for ProviderExecutionErrorKind {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {}
    }
}

impl ProviderExecutionErrorKind {
    pub fn as_str(self) -> &'static str {
        match self.0 {}
    }
}

pub type CapabilitySdkResult<T> = Result<T, CapabilitySdkError>;

impl From<RuntimeAuthorityError> for CapabilitySdkError {
    fn from(error: RuntimeAuthorityError) -> Self {
        Self::InvalidDirectRunProviderRequirement(error.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityContractFingerprint(String);

impl CapabilityContractFingerprint {
    /// Mints an admitted contract fingerprint for the contract-TSON owner from
    /// that owner's canonical fingerprint output.
    ///
    /// The input is a contract-TSON owner product value, not a projection DTO,
    /// manifest, transport value, or arbitrary JSON body.
    pub fn admit_for_contract_tson_owner_v1(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Self> {
        match value {}
    }

    pub fn admit_for_libswarm_runtime_owner_v1(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Self> {
        match value {}
    }

    /// Mints an admitted contract fingerprint for the static provider catalogue
    /// owner from its checked built-in provider inventory.
    pub fn admit_for_static_provider_catalogue_owner_v1(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Self> {
        match value {}
    }

    /// Admits a contract fingerprint from a native provider manifest's artifact
    /// entry (#131 rung-1). The manifest is genuinely EXTERNAL, untrusted input
    /// read from disk at load time (the provider's build generates it; the
    /// runtime never minted it), so format-validating the fingerprint field at
    /// this trust boundary IS admission — NOT a projection of an internal sealed
    /// product.
    ///
    /// INERT UNTIL VERIFIED. `require_sha256_fingerprint` checks SHAPE, not
    /// authenticity — this is a FORMAT-VALIDATED CLAIM, NOT authority. It grants
    /// nothing until the loader verifies the CLAIM against ground truth: the same
    /// sha256 shape `require_admitted_artifact_bytes_v1` recomputes over the
    /// actual `.so` bytes at load, and the contract fingerprints the loader's C4
    /// correspondence binds to the loaded binary's data-driven descriptor. Never
    /// treat this fingerprint as authority pre-verification.
    pub fn admit_for_native_provider_authority_owner_v1(
        artifact_fingerprint: &str,
    ) -> CapabilitySdkResult<Self> {
        require_sha256_fingerprint(artifact_fingerprint)?;
        Ok(Self(artifact_fingerprint.to_owned()))
    }

    /// Mints an admitted contract fingerprint for provider-host requirement
    /// owners from already checked provider requirement material.
    pub fn admit_for_provider_host_set_requirement_owner_v1(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Self> {
        match value {}
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Duplicates an already admitted contract fingerprint for capability-model owners.
    ///
    /// This is a finite owner operation on a sealed carrier. It does not admit
    /// raw strings, JSON, manifests, or projection DTOs as authority.
    pub fn duplicate_for_capability_model_owner(&self) -> Self {
        Self(self.0.clone())
    }

    /// Duplicates an admitted fingerprint inside native-provider authority products.
    pub fn duplicate_for_native_provider_authority_owner_v1(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Display for CapabilityContractFingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapabilityContractIdentity {
    package_specifier: String,
    export_name: String,
    fingerprint: Option<CapabilityContractFingerprint>,
}

pub trait RustSdkStaticProviderInstalledNativeHostAdmission {
    fn host_id_for_static_provider_host_owner_v1(&self) -> &str;

    fn provider_count_for_static_provider_host_owner_v1(&self) -> usize;

    fn require_exact_contract_for_static_provider_host_owner_v1(
        &self,
        required_contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<()>;

    fn reject_package_export_conflict_for_static_provider_host_owner_v1(
        &self,
        package_specifier: &str,
        export_name: &str,
    ) -> CapabilitySdkResult<()>;

    fn reject_package_specifier_conflict_for_static_provider_host_owner_v1(
        &self,
        package_specifier: &str,
    ) -> CapabilitySdkResult<()>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct NativeProviderManifestAuthorityForCapabilityModelOwner {
    schema: String,
    provider_id: String,
    abi: String,
    contracts: Vec<CapabilityContractIdentity>,
    artifacts: Vec<NativeProviderArtifactAuthorityForCapabilityModelOwner>,
}

#[derive(PartialEq, Eq)]
pub struct NativeProviderArtifactAuthorityForCapabilityModelOwner {
    platform: String,
    path: String,
    artifact_fingerprint: CapabilityContractFingerprint,
}

impl fmt::Debug for NativeProviderArtifactAuthorityForCapabilityModelOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeProviderArtifactAuthorityForCapabilityModelOwner")
            .field("platform", &self.platform)
            .field("path", &"<redacted>")
            .field("artifact_fingerprint", &"<redacted>")
            .finish()
    }
}

impl NativeProviderManifestAuthorityForCapabilityModelOwner {
    pub fn schema_for_native_provider_authority_owner_v1(&self) -> &str {
        &self.schema
    }

    pub fn provider_id_for_native_provider_authority_owner_v1(&self) -> &str {
        &self.provider_id
    }

    pub fn abi_for_native_provider_authority_owner_v1(&self) -> &str {
        &self.abi
    }

    pub fn contracts_for_native_provider_authority_owner_v1(
        &self,
    ) -> &[CapabilityContractIdentity] {
        self.contracts.as_slice()
    }

    pub fn artifacts_for_native_provider_authority_owner_v1(
        &self,
    ) -> &[NativeProviderArtifactAuthorityForCapabilityModelOwner] {
        self.artifacts.as_slice()
    }
}

impl NativeProviderArtifactAuthorityForCapabilityModelOwner {
    pub fn platform_for_native_provider_authority_owner_v1(&self) -> &str {
        &self.platform
    }

    pub fn path_for_native_provider_authority_owner_v1(&self) -> &str {
        &self.path
    }

    pub fn artifact_fingerprint_for_native_provider_authority_owner_v1(
        &self,
    ) -> &CapabilityContractFingerprint {
        &self.artifact_fingerprint
    }
}

impl CapabilityContractIdentity {
    /// Mints an admitted contract identity for the capability-model owner from
    /// already admitted contract-TSON owner parts.
    ///
    /// The fingerprint is a sealed owner product, while the package and export
    /// labels are immediately validated into typed linker identifiers. This is
    /// not a projection, JSON, manifest, or DTO rehydration path.
    pub fn admit_contract_tson_identity_for_capability_model_owner_v1(
        identity_parts: CapabilityContractIdentityPartsForCapabilityModelOwner,
    ) -> CapabilitySdkResult<Self> {
        let package_specifier = identity_parts
            .package_specifier_for_capability_model_owner_v1()
            .to_owned();
        let export_name = identity_parts
            .export_name_for_capability_model_owner_v1()
            .to_owned();
        let contract_fingerprint = identity_parts
            .contract_fingerprint_for_capability_model_owner_v1()
            .to_owned();
        require_sha256_fingerprint(contract_fingerprint.as_str())?;
        Ok(Self {
            package_specifier,
            export_name,
            fingerprint: Some(CapabilityContractFingerprint(contract_fingerprint)),
        })
    }

    /// Admits a contract identity from a native provider manifest's contract
    /// entry (#131 rung-1). Unlike
    /// [`Self::admit_contract_tson_identity_for_capability_model_owner_v1`] (a
    /// sealed-parts path), this admits a genuinely-EXTERNAL manifest CLAIM: the
    /// manifest is untrusted input read from disk at load time — the provider's
    /// build generates it from its Contract-TSON and the swarm runtime never
    /// minted it — so format-validating the `package_specifier`/`export_name`/
    /// `fingerprint` fields at this trust boundary IS admission (the same
    /// `require_projection_*` + `require_sha256_fingerprint` validators the
    /// sealed-parts path applies, mirroring
    /// [`Self::admit_authored_static_provider_contract_identity_for_capability_model_owner_v1`]).
    ///
    /// INERT UNTIL C4. `require_sha256_fingerprint` checks SHAPE, not
    /// authenticity — the returned identity is a FORMAT-VALIDATED CLAIM, NOT
    /// authority. It grants nothing until the loader binds it to the actual
    /// loaded binary: the `.so`'s own data-driven `descriptor_json` is ground
    /// truth, and the loader's C4 correspondence
    /// (`require_matching_loaded_function_table_descriptor_v1`, provider_id + abi
    /// + sorted-contract-keys) enforces claim == truth during load. This identity
    /// must NEVER be treated as authority pre-correspondence, and this op must
    /// never itself perform or stand in for that C4 check.
    pub fn admit_native_provider_manifest_contract_identity_for_native_provider_authority_owner_v1(
        package_specifier: &str,
        export_name: &str,
        contract_fingerprint: &str,
    ) -> CapabilitySdkResult<Self> {
        require_projection_package_specifier(package_specifier)?;
        require_projection_export_name(export_name)?;
        require_sha256_fingerprint(contract_fingerprint)?;
        Ok(Self {
            package_specifier: package_specifier.to_owned(),
            export_name: export_name.to_owned(),
            fingerprint: Some(CapabilityContractFingerprint(
                contract_fingerprint.to_owned(),
            )),
        })
    }

    /// Mints an admitted Rust SDK binding identity for the capability-model owner.
    ///
    /// The string labels are validated at this boundary and the optional
    /// fingerprint is already sealed; callers cannot derive authority from
    /// projection cargo through this operation.
    pub fn admit_rust_sdk_binding_identity_for_capability_model_owner_v1(
        package_specifier: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _export_name: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _fingerprint: Option<CapabilityContractFingerprint>,
    ) -> CapabilitySdkResult<Self> {
        match package_specifier {}
    }

    /// Mints a provider-host-set requirement identity for the capability-model owner.
    ///
    /// The input labels are admitted into typed identifiers here and the
    /// optional fingerprint is sealed, so this does not accept projection cargo
    /// as authority.
    pub fn admit_provider_host_set_requirement_identity_for_capability_model_owner_v1(
        package_specifier: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _export_name: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _fingerprint: Option<CapabilityContractFingerprint>,
    ) -> CapabilitySdkResult<Self> {
        match package_specifier {}
    }

    /// Admits a static-provider contract identity from an AUTHORED
    /// `(package_specifier, export_name, derived_contract_fingerprint)` triple
    /// (#131 rung-4). This is the DATA-DRIVEN owner of static-provider identity:
    /// the caller supplies the authored package specifier + export name
    /// (validated here as linker ids) and the fingerprint DERIVED from the
    /// admitted Contract-TSON of the same authored source. It REPLACED the
    /// closed static-provider catalogue vocabulary enum (#131 rung-4 L2 deleted
    /// that enum outright): provider identity is no longer gated by a finite
    /// hardcoded `(package, export)` list — any authored contract source whose
    /// linker ids validate and whose fingerprint derives from its Contract-TSON
    /// mints an identity, and executor-existence (the admission-time dangling
    /// refusal) is the honest replacement for the deleted vocabulary gate.
    ///
    /// This op format-validates the labels + fingerprint SHAPE; it does not
    /// itself admit the TSON. Correspondence is enforced by the mint that calls
    /// it: the caller derives `derived_contract_fingerprint` from the same
    /// admitted source it also feeds to the derivation-input owner, so a
    /// same-source fingerprint mismatch is unrepresentable (R40973). A static
    /// in-process provider's contract IS its authored source, so this
    /// self-derived-fingerprint correspondence is the sealing guarantee (there
    /// is no native-lane C4 `.so` descriptor ground truth to bind against).
    pub fn admit_authored_static_provider_contract_identity_for_capability_model_owner_v1(
        package_specifier: &str,
        export_name: &str,
        derived_contract_fingerprint: &str,
    ) -> CapabilitySdkResult<Self> {
        require_projection_package_specifier(package_specifier)?;
        require_projection_export_name(export_name)?;
        require_sha256_fingerprint(derived_contract_fingerprint)?;
        Ok(Self {
            package_specifier: package_specifier.to_owned(),
            export_name: export_name.to_owned(),
            fingerprint: Some(CapabilityContractFingerprint(
                derived_contract_fingerprint.to_owned(),
            )),
        })
    }

    /// Duplicates an already sealed contract identity for capability-model owners.
    ///
    /// This operation does not admit raw strings, manifests, JSON, or
    /// projection DTOs. It exists so owner crates can move sealed authority
    /// across finite owner operations without deriving unconstrained `Clone`.
    pub fn duplicate_for_capability_model_owner(&self) -> Self {
        Self {
            package_specifier: self.package_specifier.clone(),
            export_name: self.export_name.clone(),
            fingerprint: self
                .fingerprint
                .as_ref()
                .map(CapabilityContractFingerprint::duplicate_for_capability_model_owner),
        }
    }

    /// Duplicates an already sealed contract identity for the Rust SDK capability owner.
    ///
    /// This operation accepts only `self`, an existing sealed identity product.
    /// It does not admit raw strings, manifests, JSON, projection DTOs, or
    /// debug data as authority.
    pub fn duplicate_for_rust_sdk_capability_owner_v1(&self) -> Self {
        self.duplicate_for_capability_model_owner()
    }

    pub fn package_specifier(&self) -> &str {
        &self.package_specifier
    }

    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn fingerprint(&self) -> Option<&CapabilityContractFingerprint> {
        self.fingerprint.as_ref()
    }

    pub fn provider_id(&self) -> String {
        format!("{}:{}", self.package_specifier, self.export_name)
    }

    pub fn projection(&self) -> CapabilityContractProjection {
        CapabilityContractProjection {
            package_specifier: self.package_specifier.clone(),
            export_name: self.export_name.clone(),
            fingerprint: self
                .fingerprint
                .as_ref()
                .map(|fingerprint| fingerprint.as_str().to_owned()),
        }
    }
}

pub fn admit_native_provider_manifest_json_for_capability_model_owner_v1(
    value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
) -> CapabilitySdkResult<NativeProviderManifestAuthorityForCapabilityModelOwner> {
    match value {}
}

pub fn validate_contract_projection(
    projection: &CapabilityContractProjection,
) -> CapabilitySdkResult<()> {
    require_projection_package_specifier(projection.package_specifier.as_str())?;
    require_projection_export_name(projection.export_name.as_str())?;
    Ok(())
}

fn require_projection_package_specifier(value: &str) -> CapabilitySdkResult<()> {
    if value.trim().is_empty() || value.trim() != value {
        return Err(CapabilitySdkError::InvalidProjectionPackageSpecifier(
            value.to_owned(),
        ));
    }
    Ok(())
}

fn require_projection_export_name(value: &str) -> CapabilitySdkResult<()> {
    if value.trim().is_empty() || value.trim() != value {
        return Err(CapabilitySdkError::InvalidProjectionExportName(
            value.to_owned(),
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct CapabilityContractProjection {
    package_specifier: String,
    export_name: String,
    fingerprint: Option<String>,
}

impl swarm_substrate_invariant::ProjectionCargo for CapabilityContractProjection {
    const PROJECTION_CARGO_KIND: &'static str = "capability_contract_projection";
}

impl CapabilityContractProjection {
    pub fn package_specifier(&self) -> &str {
        &self.package_specifier
    }

    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn fingerprint(&self) -> Option<&str> {
        self.fingerprint.as_deref()
    }

    /// Builds a diagnostic/transport projection for the prepared-runtime projection owner.
    ///
    /// This returns projection cargo only. It does not mint runtime, provider,
    /// package, or prepared-runtime authority from the supplied strings.
    pub fn from_prepared_runtime_projection_owner_v1(
        package_specifier: impl Into<String>,
        export_name: impl Into<String>,
        fingerprint: Option<String>,
    ) -> CapabilitySdkResult<Self> {
        let package_specifier = package_specifier.into();
        let export_name = export_name.into();
        require_projection_package_specifier(package_specifier.as_str())?;
        require_projection_export_name(export_name.as_str())?;
        if let Some(fingerprint) = fingerprint.as_deref() {
            require_sha256_fingerprint(fingerprint)?;
        }
        Ok(Self {
            package_specifier,
            export_name,
            fingerprint,
        })
    }

    /// Builds diagnostic/transport projection cargo for the Rust SDK static provider host.
    ///
    /// This validates projection shape only. It does not mint provider,
    /// executable, or contract authority from raw strings.
    pub fn from_swarm_rust_sdk_static_provider_host_owner_v1(
        package_specifier: impl Into<String>,
        export_name: impl Into<String>,
        fingerprint: Option<String>,
    ) -> CapabilitySdkResult<Self> {
        let package_specifier = package_specifier.into();
        let export_name = export_name.into();
        require_projection_package_specifier(package_specifier.as_str())?;
        require_projection_export_name(export_name.as_str())?;
        if let Some(fingerprint) = fingerprint.as_deref() {
            require_sha256_fingerprint(fingerprint)?;
        }
        Ok(Self {
            package_specifier,
            export_name,
            fingerprint,
        })
    }

    /// Duplicates projection cargo for prepared-runtime observation/transport.
    ///
    /// This is not an authority duplication operation.
    pub fn duplicate_for_prepared_runtime_projection_owner_v1(&self) -> Self {
        Self {
            package_specifier: self.package_specifier.clone(),
            export_name: self.export_name.clone(),
            fingerprint: self.fingerprint.clone(),
        }
    }
}

impl From<&CapabilityContractIdentity> for CapabilityContractProjection {
    fn from(identity: &CapabilityContractIdentity) -> Self {
        identity.projection()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderExecutionDomain {
    RustSdk,
    ExternalTransportCapabilityProvider,
}

impl ProviderExecutionDomain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RustSdk => RUST_SDK_PROVIDER_DOMAIN,
            Self::ExternalTransportCapabilityProvider => EXTERNAL_TRANSPORT_PROVIDER_DOMAIN,
        }
    }
}

fn require_sha256_fingerprint(value: &str) -> CapabilitySdkResult<()> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(CapabilitySdkError::InvalidFingerprint(value.to_owned()));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(CapabilitySdkError::InvalidFingerprint(value.to_owned()));
    }
    if hex.chars().any(|ch| ch.is_ascii_uppercase()) {
        return Err(CapabilitySdkError::InvalidFingerprint(value.to_owned()));
    }
    Ok(())
}

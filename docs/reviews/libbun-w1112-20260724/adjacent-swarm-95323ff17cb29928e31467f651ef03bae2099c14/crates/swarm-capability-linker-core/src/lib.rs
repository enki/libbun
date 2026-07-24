use std::collections::BTreeMap;
#[cfg(test)]
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use libswarm_package_graph_contract_source_admission::ContractTsonPackageIdentityForCapabilityLinkerOwner;
use swarm_contract_tson::{
    ContractTsonClosedSumOutputVariantForCapabilityLinkerOwner,
    ContractTsonCommandAcceptedOutputTypeContractForCapabilityLinkerOwner,
    ContractTsonInteractionOpenOutputTypeContractForCapabilityLinkerOwner,
    ContractTsonInteractionOpenProtocolBinderForCapabilityLinkerOwnerV1,
    ContractTsonKernelInternalPlainOutputPreflightFaultForCapabilityLinkerOwnerV1,
    ContractTsonNominalProviderResumeStartupOutputTypeForCapabilityLinkerOwnerV1,
};
mod contract_tson_witness;
#[cfg(test)]
mod provider_value;

pub use contract_tson_witness::{
    CONTRACT_TSON_WITNESS_SCHEMA, CapabilityContractTsonWitness,
    CapabilityContractTsonWitnessError, CapabilityContractTsonWitnessJson,
};

#[cfg(test)]
use provider_value::{
    AdmittedProviderInvocation, ProviderInvocationAdmissionError, ProviderInvocationRequest,
};
pub use swarm_provider_value_model::{
    AuthoredResourceValue, FiniteProviderNumber, HostResourceHandleCarrier,
    HostResourceResumePolicy, LiveOperationHandleCarrier, LiveStreamHandleCarrier,
    MeshProviderOperationAuthorityForMeshControlOwnerV1,
    MeshProviderOperationCarrierJoinForMeshControlOwnerV1,
    MeshProviderOperationCarrierRolesForMeshControlOwnerV1,
    MeshProviderOperationStreamAuthorityForMeshControlOwnerV1,
    MeshProviderOperationStreamCarrierJoinForMeshControlOwnerV1, ProviderResultAdmissionError,
    ProviderValue, ProviderValueAdmissionError, ProviderValueJsonAdmissionError, SwarmInteger,
    WideIntegerJsonProjectionDecodeV1, integer_json_projection_value_v1,
    is_reserved_provider_value_object_kind_v1, provider_value_from_canonical_json_v1,
    provider_value_to_canonical_json_v1, provider_value_to_canonical_output_observation_json_v1,
    validate_provider_user_payload_value, wide_integer_json_projection_decode_v1,
};

pub const PACKAGE_EXPORT_CONTRACT_WITNESS_KIND: &str =
    "swarm.package_export_capability_contract_witness.v1";
pub const CAPABILITY_REQUEST_MANIFEST_KIND: &str = "swarm.capability_request_manifest.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkerIdentifierKind {
    PackageSpecifier,
    ExportName,
    MemberSegment,
    ContractKey,
    ContractProvenance,
    OperationId,
    RequirementDescriptorId,
    ProviderBindingId,
    ProviderId,
    RustInternalProviderId,
    RustSdkProviderId,
    ExternalProviderBindingId,
    ProviderContextShapeId,
    HostResourceHandleId,
    HostResourceKindId,
    LiveOperationHandleId,
    LiveStreamHandleId,
}

impl fmt::Display for LinkerIdentifierKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PackageSpecifier => write!(f, "package specifier"),
            Self::ExportName => write!(f, "export name"),
            Self::MemberSegment => write!(f, "member segment"),
            Self::ContractKey => write!(f, "contract key"),
            Self::ContractProvenance => write!(f, "contract provenance"),
            Self::OperationId => write!(f, "operation id"),
            Self::RequirementDescriptorId => write!(f, "requirement descriptor id"),
            Self::ProviderBindingId => write!(f, "provider binding id"),
            Self::ProviderId => write!(f, "provider id"),
            Self::RustInternalProviderId => write!(f, "rust internal provider id"),
            Self::RustSdkProviderId => write!(f, "rust SDK provider id"),
            Self::ExternalProviderBindingId => write!(f, "external provider binding id"),
            Self::ProviderContextShapeId => write!(f, "provider context shape id"),
            Self::HostResourceHandleId => write!(f, "host resource handle id"),
            Self::HostResourceKindId => write!(f, "host resource kind id"),
            Self::LiveOperationHandleId => write!(f, "live operation handle id"),
            Self::LiveStreamHandleId => write!(f, "live stream handle id"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkerIdentifierError {
    Blank(LinkerIdentifierKind),
    SurroundingWhitespace {
        kind: LinkerIdentifierKind,
        value: String,
    },
}

impl fmt::Display for LinkerIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank(kind) => write!(f, "{kind} must not be blank"),
            Self::SurroundingWhitespace { kind, value } => {
                write!(f, "{kind} must not carry surrounding whitespace: {value:?}")
            }
        }
    }
}

impl Error for LinkerIdentifierError {}

fn admit_nonblank_owned(
    kind: LinkerIdentifierKind,
    value: impl Into<String>,
) -> Result<String, LinkerIdentifierError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(LinkerIdentifierError::Blank(kind));
    }
    if value.trim() != value {
        return Err(LinkerIdentifierError::SurroundingWhitespace { kind, value });
    }
    Ok(value)
}

macro_rules! linker_identifier {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[allow(dead_code)]
            pub(crate) fn new(value: impl Into<String>) -> Result<Self, LinkerIdentifierError> {
                admit_nonblank_owned($kind, value).map(Self)
            }

            #[allow(dead_code)]
            pub(crate) fn try_new_for_capability_linker_owner_v1(
                value: impl Into<String>,
            ) -> Result<Self, LinkerIdentifierError> {
                Self::new(value)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

linker_identifier!(PackageSpecifier, LinkerIdentifierKind::PackageSpecifier);
linker_identifier!(ExportName, LinkerIdentifierKind::ExportName);
linker_identifier!(MemberSegment, LinkerIdentifierKind::MemberSegment);
linker_identifier!(ContractKey, LinkerIdentifierKind::ContractKey);
linker_identifier!(ContractProvenance, LinkerIdentifierKind::ContractProvenance);
linker_identifier!(OperationId, LinkerIdentifierKind::OperationId);
linker_identifier!(
    RequirementDescriptorId,
    LinkerIdentifierKind::RequirementDescriptorId
);
linker_identifier!(ProviderBindingId, LinkerIdentifierKind::ProviderBindingId);
linker_identifier!(ProviderId, LinkerIdentifierKind::ProviderId);
linker_identifier!(
    RustInternalProviderId,
    LinkerIdentifierKind::RustInternalProviderId
);
linker_identifier!(RustSdkProviderId, LinkerIdentifierKind::RustSdkProviderId);
linker_identifier!(
    ExternalProviderBindingId,
    LinkerIdentifierKind::ExternalProviderBindingId
);
linker_identifier!(
    ProviderContextShapeId,
    LinkerIdentifierKind::ProviderContextShapeId
);
linker_identifier!(
    HostResourceHandleId,
    LinkerIdentifierKind::HostResourceHandleId
);
linker_identifier!(HostResourceKindId, LinkerIdentifierKind::HostResourceKindId);
linker_identifier!(
    LiveOperationHandleId,
    LinkerIdentifierKind::LiveOperationHandleId
);
linker_identifier!(LiveStreamHandleId, LinkerIdentifierKind::LiveStreamHandleId);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemberPath(Vec<MemberSegment>);

impl MemberPath {
    pub fn new(
        segments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, LinkerIdentifierError> {
        let segments = segments
            .into_iter()
            .map(MemberSegment::try_new_for_capability_linker_owner_v1)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(segments))
    }

    pub fn segments(&self) -> &[MemberSegment] {
        self.0.as_slice()
    }

    pub fn dotted(&self) -> String {
        self.0
            .iter()
            .map(MemberSegment::as_str)
            .collect::<Vec<_>>()
            .join(".")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityTarget {
    package_specifier: PackageSpecifier,
    export_name: ExportName,
    member_path: Option<MemberPath>,
}

impl CapabilityTarget {
    pub fn new(
        package_specifier: PackageSpecifier,
        export_name: ExportName,
        member_path: Option<MemberPath>,
    ) -> Self {
        Self {
            package_specifier,
            export_name,
            member_path,
        }
    }

    pub fn package_specifier(&self) -> &PackageSpecifier {
        &self.package_specifier
    }

    pub fn export_name(&self) -> &ExportName {
        &self.export_name
    }

    pub fn member_path(&self) -> Option<&MemberPath> {
        self.member_path.as_ref()
    }
}

#[derive(PartialEq, Eq)]
pub struct CapabilityTypeContractJson {
    canonical_json: String,
}

impl fmt::Debug for CapabilityTypeContractJson {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityTypeContractJson")
            .field("root", &"redacted")
            .finish()
    }
}

impl CapabilityTypeContractJson {
    pub fn new(
        root: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, CapabilityTypeContractError> {
        match root {}
    }
}

pub fn admit_contract_tson_command_accepted_output_type_contract_for_capability_linker_owner_v1(
    contract_tson_output_type: ContractTsonCommandAcceptedOutputTypeContractForCapabilityLinkerOwner,
) -> Result<CapabilityContractOutputTypeContractAuthorityProduct, CapabilityTypeContractError> {
    Ok(
        CapabilityContractOutputTypeContractAuthorityProduct::from_capability_linker_owner_output_type_contract_v1(
            contract_tson_output_type,
        ),
    )
}

pub fn admit_contract_tson_interaction_open_output_type_contract_for_capability_linker_owner_v1(
    contract_tson_output_type: ContractTsonInteractionOpenOutputTypeContractForCapabilityLinkerOwner,
) -> Result<CapabilityContractOutputTypeContractAuthorityProduct, CapabilityTypeContractError> {
    Ok(
        CapabilityContractOutputTypeContractAuthorityProduct::from_capability_linker_owner_interaction_open_output_type_contract_v1(
            contract_tson_output_type,
        ),
    )
}

#[derive(PartialEq, Eq)]
pub struct CapabilityContractIdentityPartsForCapabilityModelOwner {
    package_specifier: PackageSpecifier,
    export_name: ExportName,
    contract_fingerprint: String,
}

impl fmt::Debug for CapabilityContractIdentityPartsForCapabilityModelOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityContractIdentityPartsForCapabilityModelOwner")
            .field("package_specifier", &self.package_specifier)
            .field("export_name", &self.export_name)
            .field("contract_fingerprint", &"redacted")
            .finish()
    }
}

impl CapabilityContractIdentityPartsForCapabilityModelOwner {
    pub fn package_specifier_for_capability_model_owner_v1(&self) -> &str {
        self.package_specifier.as_str()
    }

    pub fn export_name_for_capability_model_owner_v1(&self) -> &str {
        self.export_name.as_str()
    }

    pub fn contract_fingerprint_for_capability_model_owner_v1(&self) -> &str {
        self.contract_fingerprint.as_str()
    }
}

pub fn admit_contract_tson_package_identity_for_capability_linker_owner_v1(
    package_identity: ContractTsonPackageIdentityForCapabilityLinkerOwner,
) -> Result<CapabilityContractIdentityPartsForCapabilityModelOwner, LinkerIdentifierError> {
    let (package_specifier, export_name, contract_fingerprint) =
        package_identity.into_parts_for_capability_linker_owner_v1();
    Ok(CapabilityContractIdentityPartsForCapabilityModelOwner {
        package_specifier: PackageSpecifier::try_new_for_capability_linker_owner_v1(
            package_specifier,
        )?,
        export_name: ExportName::try_new_for_capability_linker_owner_v1(export_name)?,
        contract_fingerprint,
    })
}

#[cfg(test)]
fn canonical_capability_type_contract_json_value_for_linker_owner(
    value: serde_json::Value,
) -> serde_json::Value {
    let canonical = match value {
        serde_json::Value::Array(items) => serde_json::Value::Array(
            items
                .into_iter()
                .map(canonical_capability_type_contract_json_value_for_linker_owner)
                .collect(),
        ),
        serde_json::Value::Object(fields) => {
            let mut sorted = serde_json::Map::new();
            for (key, value) in fields.into_iter().collect::<BTreeMap<_, _>>() {
                sorted.insert(
                    key,
                    canonical_capability_type_contract_json_value_for_linker_owner(value),
                );
            }
            serde_json::Value::Object(sorted)
        }
        other => other,
    };
    fold_node_like_carrier_for_linker_owner(canonical)
}

/// Folds every node-like carrier shape into the canonical
/// `{"item": X, "kind": "node"}` form so contract identity holds across
/// carrier spellings. Carrier equivalence is kernel law: `Node`,
/// `PromiseLike`, and `Promise` resolve to the same node type, the
/// `__swarm_node__` brand intersection is the branded node encoding, and
/// `X | PromiseLike<X>` is the maybe-promise encoding of the same
/// carrier. Children are already canonical when this runs (bottom-up).
#[cfg(test)]
fn fold_node_like_carrier_for_linker_owner(value: serde_json::Value) -> serde_json::Value {
    let serde_json::Value::Object(fields) = &value else {
        return value;
    };
    let kind = fields.get("kind").and_then(serde_json::Value::as_str);
    match kind {
        Some("ref") => {
            let symbol_path = fields
                .get("symbol_path")
                .or_else(|| fields.get("symbolPath"))
                .and_then(serde_json::Value::as_str);
            let node_like = matches!(symbol_path, Some("Node" | "PromiseLike" | "Promise"));
            let single_arg = fields
                .get("typeArgs")
                .and_then(serde_json::Value::as_array)
                .filter(|args| args.len() == 1)
                .map(|args| args[0].clone());
            match (node_like, single_arg) {
                (true, Some(item)) => canonical_node_carrier_for_linker_owner(item),
                _ => value,
            }
        }
        Some("intersection") => {
            let Some(items) = fields.get("items").and_then(serde_json::Value::as_array) else {
                return value;
            };
            let mut node_carrier: Option<&serde_json::Value> = None;
            for item in items {
                if item.get("kind").and_then(serde_json::Value::as_str) == Some("node") {
                    if node_carrier.is_some() {
                        return value;
                    }
                    node_carrier = Some(item);
                }
            }
            let Some(node_carrier) = node_carrier else {
                return value;
            };
            let node_item = node_carrier.get("item");
            let all_others_are_brands = items.iter().all(|item| {
                std::ptr::eq(item, node_carrier)
                    || node_brand_object_matches_item_for_linker_owner(item, node_item)
            });
            if all_others_are_brands {
                node_carrier.clone()
            } else {
                value
            }
        }
        Some("union") => {
            let Some(items) = fields.get("items").and_then(serde_json::Value::as_array) else {
                return value;
            };
            let [first, second] = items.as_slice() else {
                return value;
            };
            let node_side_folds = |node: &serde_json::Value, plain: &serde_json::Value| {
                node.get("kind").and_then(serde_json::Value::as_str) == Some("node")
                    && node.get("item") == Some(plain)
            };
            if node_side_folds(first, second) {
                first.clone()
            } else if node_side_folds(second, first) {
                second.clone()
            } else {
                value
            }
        }
        _ => value,
    }
}

#[cfg(test)]
fn canonical_node_carrier_for_linker_owner(item: serde_json::Value) -> serde_json::Value {
    let mut node = serde_json::Map::new();
    node.insert("item".to_owned(), item);
    node.insert(
        "kind".to_owned(),
        serde_json::Value::String("node".to_owned()),
    );
    serde_json::Value::Object(node)
}

#[cfg(test)]
fn node_brand_object_matches_item_for_linker_owner(
    candidate: &serde_json::Value,
    node_item: Option<&serde_json::Value>,
) -> bool {
    let Some(fields) = candidate
        .get("fields")
        .and_then(serde_json::Value::as_object)
    else {
        return false;
    };
    if candidate.get("kind").and_then(serde_json::Value::as_str) != Some("object")
        || fields.len() != 1
    {
        return false;
    }
    let Some(brand) = fields.get("__swarm_node__") else {
        return false;
    };
    brand.get("type").is_none() || brand.get("type") == node_item
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityTypeContractError {
    RootNotObject,
    JsonSerialization(String),
    OutputTypeContractAdmission(String),
    OutputTypeNotClosedSum,
    OutputTypeClosedSumHasNoVariants,
    OutputTypeNotInteractionOpen,
    InteractionInitialStateAdmission(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1 {
    ClosedSumOutputContract,
    ContractAdmission(swarm_contract_tson::ContractAdmissionError),
    InteractionOpenOutputContract,
}

impl fmt::Display for CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClosedSumOutputContract => formatter.write_str(
                "kernel-internal plain-output settlement cannot discard a closed-sum output contract",
            ),
            Self::ContractAdmission(source) => write!(
                formatter,
                "kernel-internal plain-output contract preflight failed: {source}"
            ),
            Self::InteractionOpenOutputContract => formatter.write_str(
                "kernel-internal command settlement received an interaction-open output contract",
            ),
        }
    }
}

impl Error for CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1 {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ContractAdmission(source) => Some(source),
            Self::ClosedSumOutputContract | Self::InteractionOpenOutputContract => None,
        }
    }
}

impl fmt::Display for CapabilityTypeContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootNotObject => write!(f, "capability type contract root must be an object"),
            Self::JsonSerialization(source) => {
                write!(
                    f,
                    "capability type contract canonicalization failed: {source}"
                )
            }
            Self::OutputTypeContractAdmission(source) => {
                write!(f, "capability type contract admission failed: {source}")
            }
            Self::OutputTypeNotClosedSum => {
                write!(
                    f,
                    "capability output type contract is not a closed-sum union"
                )
            }
            Self::OutputTypeClosedSumHasNoVariants => {
                write!(f, "capability output closed-sum type has no variants")
            }
            Self::OutputTypeNotInteractionOpen => {
                write!(
                    f,
                    "capability output contract is not an interaction-open protocol"
                )
            }
            Self::InteractionInitialStateAdmission(source) => {
                write!(
                    f,
                    "interaction protocol initial-state admission failed: {source}"
                )
            }
        }
    }
}

impl Error for CapabilityTypeContractError {}

#[derive(Debug, Clone)]
#[cfg(test)]
pub(crate) struct CapabilityTypeContract {
    canonical_json: String,
}

#[cfg(test)]
impl CapabilityTypeContract {
    #[allow(dead_code)]
    pub fn new(value: CapabilityTypeContractJson) -> Result<Self, CapabilityTypeContractError> {
        Ok(Self {
            canonical_json: value.canonical_json,
        })
    }

    pub(crate) fn from_capability_linker_fixture_owner_root_v1(
        root: serde_json::Value,
    ) -> Result<Self, CapabilityTypeContractError> {
        let canonical_json = serde_json::to_string(
            &canonical_capability_type_contract_json_value_for_linker_owner(root),
        )
        .map_err(|source| CapabilityTypeContractError::JsonSerialization(source.to_string()))?;
        Ok(Self { canonical_json })
    }
}

#[cfg(test)]
impl PartialEq for CapabilityTypeContract {
    fn eq(&self, other: &Self) -> bool {
        self.canonical_json == other.canonical_json
    }
}

#[cfg(test)]
impl Eq for CapabilityTypeContract {}

#[cfg(test)]
impl PartialOrd for CapabilityTypeContract {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
impl Ord for CapabilityTypeContract {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonical_json.cmp(&other.canonical_json)
    }
}

#[cfg(test)]
impl std::hash::Hash for CapabilityTypeContract {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.canonical_json.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) struct CapabilityContractTypesId {
    input_type_contract: CapabilityTypeContract,
    output_type_contract: CapabilityTypeContract,
}

#[cfg(test)]
impl CapabilityContractTypesId {
    pub fn new(
        input_type_contract: CapabilityTypeContract,
        output_type_contract: CapabilityTypeContract,
    ) -> Self {
        Self {
            input_type_contract,
            output_type_contract,
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct CapabilityContractOutputTypeContractAuthorityProduct {
    output_type_contract: CapabilityContractOutputTypeContractAuthorityKind,
}

/// A nominal command output contract admitted before ProviderResume child
/// startup.
///
/// This token consumes and retains the exact Contract-TSON authority.  It is
/// intentionally non-cloneable and non-serializable.
#[must_use = "an admitted nominal ProviderResume startup output contract must be consumed by its startup owner"]
pub struct AdmittedNominalProviderResumeStartupOutputContractForCapabilityLinkerOwnerV1 {
    contract_tson_output_type:
        ContractTsonNominalProviderResumeStartupOutputTypeForCapabilityLinkerOwnerV1,
}

/// Typed, lossless refusal of nominal ProviderResume startup admission.
///
/// The exact original linker authority remains recoverable for retry or a
/// different finite owner transition.  The refusal kind stays private so it
/// cannot become a public authority-routing discriminant.
#[must_use = "a refused nominal ProviderResume startup output contract retains its original authority"]
pub struct CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1
{
    original_output_type_contract: CapabilityContractOutputTypeContractAuthorityProduct,
    fault:
        CapabilityContractNominalProviderResumeStartupOutputContractAdmissionFaultForCapabilityLinkerOwnerV1,
}

enum CapabilityContractNominalProviderResumeStartupOutputContractAdmissionFaultForCapabilityLinkerOwnerV1
{
    CommandOutputContractRefused,
    InteractionOpenOutputContract,
}

/// Complete semantic protocol descriptor admitted from one exact
/// Contract-TSON interaction-open contract.
///
/// This product is intentionally non-cloneable and non-serializable.  The
/// canonical Contract-TSON initial state is admitted into `ProviderValue`
/// while the descriptor is still whole, so session runtime never reconstructs
/// protocol authority from JSON, package names, or operation strings.
#[must_use = "an admitted interaction protocol must be consumed by the session-runtime interaction owner"]
pub struct CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1 {
    protocol_name: String,
    open_type_name: String,
    state_type_name: String,
    initial_state: ProviderValue,
    event_type_name: String,
    output_type_name: String,
    command_input_type_names: BTreeMap<String, String>,
}

pub trait CapabilityInteractionOpenProtocolBinderForContractTsonOwnerV1 {
    type BoundArtifact;

    #[allow(clippy::too_many_arguments)]
    fn consume_with_interaction_open_protocol_for_contract_tson_owner_v1(
        self,
        protocol_name: String,
        open_type_name: String,
        state_type_name: String,
        initial_state: ProviderValue,
        event_type_name: String,
        output_type_name: String,
        command_input_type_names: BTreeMap<String, String>,
    ) -> Self::BoundArtifact;
}

struct PendingCapabilityInteractionOpenProtocolForCapabilityLinkerOwnerV1;

#[derive(PartialEq, Eq)]
enum CapabilityContractOutputTypeContractAuthorityKind {
    ContractTson(ContractTsonCommandAcceptedOutputTypeContractForCapabilityLinkerOwner),
    ContractTsonInteractionOpen(
        ContractTsonInteractionOpenOutputTypeContractForCapabilityLinkerOwner,
    ),
}

#[derive(PartialEq, Eq)]
pub struct CapabilityContractClosedSumOutputTypeForProviderHostOwner {
    inner: CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind,
}

#[derive(PartialEq, Eq)]
enum CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind {
    ContractTsonStdResult(Vec<CapabilityContractClosedSumOutputVariantForProviderHostOwner>),
    ContractTsonAuthored(Vec<CapabilityContractClosedSumOutputVariantForProviderHostOwner>),
}

#[derive(PartialEq, Eq)]
pub struct CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner {
    inner: CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwnerKind,
}

#[derive(PartialEq, Eq)]
enum CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwnerKind {
    ContractTson(String),
}

#[derive(PartialEq, Eq)]
pub struct CapabilityContractClosedSumOutputVariantForProviderHostOwner {
    variant: CapabilityContractClosedSumOutputVariantAuthorityKind,
}

#[derive(PartialEq, Eq)]
enum CapabilityContractClosedSumOutputVariantAuthorityKind {
    ContractTson(ContractTsonClosedSumOutputVariantForCapabilityLinkerOwner),
}

impl fmt::Debug for CapabilityContractOutputTypeContractAuthorityProduct {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityContractOutputTypeContractAuthorityProduct")
            .field("output_type_contract", &"redacted")
            .finish()
    }
}

impl fmt::Debug for AdmittedNominalProviderResumeStartupOutputContractForCapabilityLinkerOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _retained_authority = &self.contract_tson_output_type;
        formatter
            .debug_struct(
                "AdmittedNominalProviderResumeStartupOutputContractForCapabilityLinkerOwnerV1",
            )
            .field("hidden_nominal_output_type_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug
    for CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _typed_fault = &self.fault;
        formatter
            .debug_struct(
                "CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1",
            )
            .field("hidden_original_output_type_authority", &"redacted")
            .field("hidden_typed_fault", &"redacted")
            .finish()
    }
}

impl fmt::Debug for CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1")
            .field("protocol_name", &self.protocol_name)
            .field("hidden_interaction_protocol_authority", &"redacted")
            .finish()
    }
}

impl CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1 {
    pub fn consume_with_contract_tson_binder_for_session_runtime_owner_v1<Binder>(
        self,
        binder: Binder,
    ) -> Binder::BoundArtifact
    where
        Binder: CapabilityInteractionOpenProtocolBinderForContractTsonOwnerV1,
    {
        let Self {
            protocol_name,
            open_type_name,
            state_type_name,
            initial_state,
            event_type_name,
            output_type_name,
            command_input_type_names,
        } = self;
        binder.consume_with_interaction_open_protocol_for_contract_tson_owner_v1(
            protocol_name,
            open_type_name,
            state_type_name,
            initial_state,
            event_type_name,
            output_type_name,
            command_input_type_names,
        )
    }
}

impl ContractTsonInteractionOpenProtocolBinderForCapabilityLinkerOwnerV1
    for PendingCapabilityInteractionOpenProtocolForCapabilityLinkerOwnerV1
{
    type BoundArtifact = Result<
        CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1,
        CapabilityTypeContractError,
    >;

    fn consume_with_interaction_open_protocol_for_capability_linker_owner_v1(
        self,
        protocol_name: String,
        open_type_name: String,
        state_type_name: String,
        initial_state_canonical_json: String,
        event_type_name: String,
        output_type_name: String,
        command_input_type_names: BTreeMap<String, String>,
    ) -> Self::BoundArtifact {
        let initial_state = provider_value_from_canonical_json_v1(
            initial_state_canonical_json.as_str(),
        )
        .map_err(|source| {
            CapabilityTypeContractError::InteractionInitialStateAdmission(source.to_string())
        })?;
        Ok(CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1 {
            protocol_name,
            open_type_name,
            state_type_name,
            initial_state,
            event_type_name,
            output_type_name,
            command_input_type_names,
        })
    }
}

impl fmt::Debug for CapabilityContractClosedSumOutputTypeForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityContractClosedSumOutputTypeForProviderHostOwner")
            .field("hidden_output_type_contract_authority", &"redacted")
            .finish()
    }
}

impl fmt::Debug for CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner")
            .field("hidden_closed_sum_variant_admission", &"redacted")
            .finish()
    }
}

impl fmt::Debug for CapabilityContractClosedSumOutputVariantForProviderHostOwner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CapabilityContractClosedSumOutputVariantForProviderHostOwner")
            .field("hidden_variant_contract_authority", &"redacted")
            .finish()
    }
}

impl CapabilityContractOutputTypeContractAuthorityProduct {
    fn from_capability_linker_owner_output_type_contract_v1(
        output_type_contract: ContractTsonCommandAcceptedOutputTypeContractForCapabilityLinkerOwner,
    ) -> Self {
        Self {
            output_type_contract: CapabilityContractOutputTypeContractAuthorityKind::ContractTson(
                output_type_contract,
            ),
        }
    }

    fn from_capability_linker_owner_interaction_open_output_type_contract_v1(
        output_type_contract: ContractTsonInteractionOpenOutputTypeContractForCapabilityLinkerOwner,
    ) -> Self {
        Self {
            output_type_contract:
                CapabilityContractOutputTypeContractAuthorityKind::ContractTsonInteractionOpen(
                    output_type_contract,
                ),
        }
    }

    pub fn admit_nominal_provider_resume_startup_output_contract_for_capability_linker_owner_v1(
        self,
    ) -> Result<
        AdmittedNominalProviderResumeStartupOutputContractForCapabilityLinkerOwnerV1,
        CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1,
    >{
        match self.output_type_contract {
            CapabilityContractOutputTypeContractAuthorityKind::ContractTson(contract) => {
                match contract
                    .try_into_nominal_provider_resume_startup_output_type_for_capability_linker_owner_v1()
                {
                    Ok(contract_tson_output_type) => Ok(
                        AdmittedNominalProviderResumeStartupOutputContractForCapabilityLinkerOwnerV1 {
                            contract_tson_output_type,
                        },
                    ),
                    Err(refusal) => {
                        let original = refusal
                            .into_original_output_type_contract_for_capability_linker_owner_v1();
                        Err(
                            CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1 {
                                original_output_type_contract: Self::from_capability_linker_owner_output_type_contract_v1(original),
                                fault: CapabilityContractNominalProviderResumeStartupOutputContractAdmissionFaultForCapabilityLinkerOwnerV1::CommandOutputContractRefused,
                            },
                        )
                    }
                }
            }
            CapabilityContractOutputTypeContractAuthorityKind::ContractTsonInteractionOpen(
                contract,
            ) => Err(
                CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1 {
                    original_output_type_contract:
                        Self::from_capability_linker_owner_interaction_open_output_type_contract_v1(
                            contract,
                        ),
                    fault: CapabilityContractNominalProviderResumeStartupOutputContractAdmissionFaultForCapabilityLinkerOwnerV1::InteractionOpenOutputContract,
                },
            ),
        }
    }

    /// Classify an exact kernel-internal command's output contract without
    /// consuming it. Only a finite typed verdict crosses the Contract-TSON
    /// boundary; the private contract root remains unavailable to callers.
    pub fn preflight_kernel_internal_plain_output_for_direct_run_owner_v1(
        &self,
    ) -> Result<(), CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1> {
        match &self.output_type_contract {
            CapabilityContractOutputTypeContractAuthorityKind::ContractTson(contract) => contract
                .preflight_kernel_internal_plain_output_for_capability_linker_owner_v1()
                .map_err(|fault| match fault {
                    ContractTsonKernelInternalPlainOutputPreflightFaultForCapabilityLinkerOwnerV1::ClosedSumOutputContract => {
                        CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1::ClosedSumOutputContract
                    }
                    ContractTsonKernelInternalPlainOutputPreflightFaultForCapabilityLinkerOwnerV1::ContractAdmission(source) => {
                        CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1::ContractAdmission(source)
                    }
                }),
            CapabilityContractOutputTypeContractAuthorityKind::ContractTsonInteractionOpen(_) => {
                Err(
                    CapabilityKernelInternalPlainOutputPreflightFaultForDirectRunOwnerV1::InteractionOpenOutputContract,
                )
            }
        }
    }

    pub fn into_interaction_open_protocol_for_session_runtime_owner_v1(
        self,
    ) -> Result<
        CapabilityInteractionOpenProtocolForSessionRuntimeOwnerV1,
        CapabilityTypeContractError,
    > {
        match self.output_type_contract {
            CapabilityContractOutputTypeContractAuthorityKind::ContractTsonInteractionOpen(
                contract,
            ) => contract.consume_with_protocol_binder_for_capability_linker_owner_v1(
                PendingCapabilityInteractionOpenProtocolForCapabilityLinkerOwnerV1,
            ),
            CapabilityContractOutputTypeContractAuthorityKind::ContractTson(_) => {
                Err(CapabilityTypeContractError::OutputTypeNotInteractionOpen)
            }
        }
    }

    pub fn into_closed_sum_output_type_for_provider_host_owner_v1(
        self,
    ) -> Result<
        CapabilityContractClosedSumOutputTypeForProviderHostOwner,
        CapabilityTypeContractError,
    > {
        let inner = match self.output_type_contract {
            CapabilityContractOutputTypeContractAuthorityKind::ContractTson(contract) => {
                let closed_sum = contract
                    .into_closed_sum_output_type_for_capability_linker_owner_v1()
                    .map_err(|source| {
                        if source.is_output_type_not_closed_sum_for_capability_linker_owner_v1() {
                            CapabilityTypeContractError::OutputTypeNotClosedSum
                        } else {
                            CapabilityTypeContractError::OutputTypeContractAdmission(
                                source.to_string(),
                            )
                        }
                    })?;
                let is_std_result = closed_sum.is_exact_std_result_for_capability_linker_owner_v1();
                let variants = closed_sum
                    .into_variants_for_capability_linker_owner_v1()
                    .into_iter()
                    .map(CapabilityContractClosedSumOutputVariantForProviderHostOwner::from_capability_linker_owner_variant_v1)
                    .collect::<Vec<_>>();
                if is_std_result {
                    CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonStdResult(variants)
                } else {
                    CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonAuthored(variants)
                }
            }
            CapabilityContractOutputTypeContractAuthorityKind::ContractTsonInteractionOpen(
                contract,
            ) => {
                let variants = contract
                    .into_closed_sum_output_type_for_capability_linker_owner_v1()
                    .map_err(|source| {
                        if source.is_output_type_not_closed_sum_for_capability_linker_owner_v1() {
                            CapabilityTypeContractError::OutputTypeNotClosedSum
                        } else {
                            CapabilityTypeContractError::OutputTypeContractAdmission(
                                source.to_string(),
                            )
                        }
                    })?
                    .into_variants_for_capability_linker_owner_v1()
                    .into_iter()
                    .map(
                        CapabilityContractClosedSumOutputVariantForProviderHostOwner::from_capability_linker_owner_variant_v1,
                    )
                    .collect::<Vec<_>>();
                CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonAuthored(
                    variants,
                )
            }
        };
        Ok(CapabilityContractClosedSumOutputTypeForProviderHostOwner { inner })
    }
}

impl CapabilityContractNominalProviderResumeStartupOutputContractAdmissionRefusalForCapabilityLinkerOwnerV1 {
    pub fn into_original_output_type_contract_for_capability_linker_owner_v1(
        self,
    ) -> CapabilityContractOutputTypeContractAuthorityProduct {
        let Self {
            original_output_type_contract,
            fault: _,
        } = self;
        original_output_type_contract
    }
}

impl CapabilityContractClosedSumOutputTypeForProviderHostOwner {
    pub fn into_exact_std_result_for_provider_host_owner_v1(self) -> Result<Self, Self> {
        match self.inner {
            CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonStdResult(
                _,
            ) => Ok(self),
            CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonAuthored(
                _,
            ) => Err(self),
        }
    }

    pub fn admit_variant_for_provider_host_owner_v1(
        self,
        variant_label: String,
    ) -> Result<
        CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner,
        CapabilityTypeContractError,
    > {
        let admitted = match self.inner {
            CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonStdResult(
                variants,
            )
            | CapabilityContractClosedSumOutputTypeForProviderHostOwnerKind::ContractTsonAuthored(
                variants,
            ) => {
                let admitted = variants.into_iter().any(|variant| {
                    variant
                        .variant_label_for_provider_host_owner_v1()
                        .is_some_and(|declared| declared == variant_label.as_str())
                });
                if !admitted {
                    return Err(CapabilityTypeContractError::OutputTypeContractAdmission(
                        "closed-sum output variant is not declared by the admitted output type"
                            .to_owned(),
                    ));
                }
                CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwnerKind::ContractTson(
                    variant_label,
                )
            }
        };
        Ok(
            CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner {
                inner: admitted,
            },
        )
    }
}

impl CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwner {
    pub fn into_variant_label_for_provider_host_owner_v1(self) -> String {
        match self.inner {
            CapabilityContractClosedSumOutputVariantAdmissionForProviderHostOwnerKind::ContractTson(
                variant_label,
            ) => variant_label,
        }
    }
}

impl CapabilityContractClosedSumOutputVariantForProviderHostOwner {
    fn from_capability_linker_owner_variant_v1(
        contract_tson_variant: ContractTsonClosedSumOutputVariantForCapabilityLinkerOwner,
    ) -> Self {
        Self {
            variant: CapabilityContractClosedSumOutputVariantAuthorityKind::ContractTson(
                contract_tson_variant,
            ),
        }
    }

    fn variant_label_for_provider_host_owner_v1(&self) -> Option<&str> {
        match &self.variant {
            CapabilityContractClosedSumOutputVariantAuthorityKind::ContractTson(variant) => {
                variant.variant_label_for_capability_linker_owner_v1()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) struct CapabilityContractId {
    target: CapabilityTarget,
    contract_key: ContractKey,
    input_contract_key: ContractKey,
    output_contract_key: ContractKey,
    provenance: ContractProvenance,
    contract_types: CapabilityContractTypesId,
    contract_tson_witness: Option<CapabilityContractTsonWitness>,
}

#[cfg(test)]
impl CapabilityContractId {
    pub fn new(
        target: CapabilityTarget,
        contract_key: ContractKey,
        input_contract_key: ContractKey,
        output_contract_key: ContractKey,
        provenance: ContractProvenance,
        contract_types: CapabilityContractTypesId,
    ) -> Self {
        Self::new_with_contract_tson_witness(
            target,
            contract_key,
            input_contract_key,
            output_contract_key,
            provenance,
            contract_types,
            None,
        )
    }

    pub fn new_with_contract_tson_witness(
        target: CapabilityTarget,
        contract_key: ContractKey,
        input_contract_key: ContractKey,
        output_contract_key: ContractKey,
        provenance: ContractProvenance,
        contract_types: CapabilityContractTypesId,
        contract_tson_witness: Option<CapabilityContractTsonWitness>,
    ) -> Self {
        Self {
            target,
            contract_key,
            input_contract_key,
            output_contract_key,
            provenance,
            contract_types,
            contract_tson_witness,
        }
    }

    pub fn target(&self) -> &CapabilityTarget {
        &self.target
    }

    pub fn contract_key(&self) -> &ContractKey {
        &self.contract_key
    }

    pub fn input_contract_key(&self) -> &ContractKey {
        &self.input_contract_key
    }

    pub fn output_contract_key(&self) -> &ContractKey {
        &self.output_contract_key
    }

    pub fn provenance(&self) -> &ContractProvenance {
        &self.provenance
    }

    pub fn contract_types(&self) -> &CapabilityContractTypesId {
        &self.contract_types
    }

    pub fn contract_tson_witness(&self) -> Option<&CapabilityContractTsonWitness> {
        self.contract_tson_witness.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) struct CapabilityRequirementId {
    target: CapabilityTarget,
    operation_id: Option<OperationId>,
    contract_id: CapabilityContractId,
    requirement_descriptor_id: RequirementDescriptorId,
}

#[cfg(test)]
impl CapabilityRequirementId {
    pub fn new(
        target: CapabilityTarget,
        operation_id: Option<OperationId>,
        contract_id: CapabilityContractId,
        requirement_descriptor_id: RequirementDescriptorId,
    ) -> Self {
        Self {
            target,
            operation_id,
            contract_id,
            requirement_descriptor_id,
        }
    }

    pub fn target(&self) -> &CapabilityTarget {
        &self.target
    }

    pub fn operation_id(&self) -> Option<&OperationId> {
        self.operation_id.as_ref()
    }

    pub fn contract_id(&self) -> &CapabilityContractId {
        &self.contract_id
    }

    pub fn requirement_descriptor_id(&self) -> &RequirementDescriptorId {
        &self.requirement_descriptor_id
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityAuthoritySet {
    targets: BTreeSet<CapabilityTarget>,
}

#[cfg(test)]
impl CapabilityAuthoritySet {
    pub(crate) fn new(targets: impl IntoIterator<Item = CapabilityTarget>) -> Self {
        Self {
            targets: targets.into_iter().collect(),
        }
    }

    pub(crate) fn singleton(target: CapabilityTarget) -> Self {
        Self::new([target])
    }

    pub fn targets(&self) -> &BTreeSet<CapabilityTarget> {
        &self.targets
    }

    pub fn contains(&self, target: &CapabilityTarget) -> bool {
        self.targets.contains(target)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.targets.is_subset(&other.targets)
    }

    fn difference_from(&self, other: &Self) -> Vec<CapabilityTarget> {
        self.targets.difference(&other.targets).cloned().collect()
    }

    fn duplicate_for_capability_link_plan_owner_v1(&self) -> Self {
        Self {
            targets: self.targets.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityRequestManifestEntry {
    requirement_id: CapabilityRequirementId,
    optional: bool,
    execution_authority: CapabilityAuthoritySet,
    effective_authority: CapabilityAuthoritySet,
}

#[cfg(test)]
impl CapabilityRequestManifestEntry {
    pub(crate) fn new(requirement_id: CapabilityRequirementId, optional: bool) -> Self {
        let execution_authority =
            CapabilityAuthoritySet::singleton(requirement_id.target().clone());
        let effective_authority =
            CapabilityAuthoritySet::singleton(requirement_id.target().clone());
        Self {
            requirement_id,
            optional,
            execution_authority,
            effective_authority,
        }
    }

    pub(crate) fn new_with_authority(
        requirement_id: CapabilityRequirementId,
        optional: bool,
        execution_authority: CapabilityAuthoritySet,
        effective_authority: CapabilityAuthoritySet,
    ) -> Result<Self, CapabilityRequestManifestAdmissionError> {
        let entry = Self {
            requirement_id,
            optional,
            execution_authority,
            effective_authority,
        };
        validate_request_entry_authority(&entry)?;
        Ok(entry)
    }

    pub fn requirement_id(&self) -> &CapabilityRequirementId {
        &self.requirement_id
    }

    pub(crate) fn execution_authority(&self) -> &CapabilityAuthoritySet {
        &self.execution_authority
    }

    pub(crate) fn effective_authority(&self) -> &CapabilityAuthoritySet {
        &self.effective_authority
    }

    #[cfg(test)]
    fn duplicate_for_capability_linker_fixture_owner_v1(&self) -> Self {
        Self {
            requirement_id: self.requirement_id.clone(),
            optional: self.optional,
            execution_authority: self
                .execution_authority
                .duplicate_for_capability_link_plan_owner_v1(),
            effective_authority: self
                .effective_authority
                .duplicate_for_capability_link_plan_owner_v1(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityRequestManifest {
    entries: Vec<CapabilityRequestManifestEntry>,
}

#[cfg(test)]
impl CapabilityRequestManifest {
    pub fn new(
        entries: Vec<CapabilityRequestManifestEntry>,
    ) -> Result<Self, CapabilityRequestManifestAdmissionError> {
        let mut seen_descriptors = BTreeSet::new();
        for entry in &entries {
            if entry.requirement_id().target() != entry.requirement_id().contract_id().target() {
                return Err(
                    CapabilityRequestManifestAdmissionError::RequestTargetDrift {
                        requirement_id: entry.requirement_id().clone(),
                    },
                );
            }
            validate_request_entry_authority(entry)?;
            let descriptor = entry.requirement_id().requirement_descriptor_id().clone();
            if !seen_descriptors.insert(descriptor.clone()) {
                return Err(
                    CapabilityRequestManifestAdmissionError::DuplicateRequirementDescriptor {
                        requirement_descriptor_id: descriptor,
                    },
                );
            }
        }
        Ok(Self { entries })
    }

    pub fn entries(&self) -> &[CapabilityRequestManifestEntry] {
        self.entries.as_slice()
    }

    #[cfg(test)]
    fn duplicate_for_capability_linker_fixture_owner_v1(&self) -> Self {
        Self {
            entries: self
                .entries
                .iter()
                .map(CapabilityRequestManifestEntry::duplicate_for_capability_linker_fixture_owner_v1)
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct AdmittedCapabilityRequest {
    requirement_id: CapabilityRequirementId,
    grant: ReconciledCapabilityGrant,
    optional: bool,
    execution_authority: CapabilityAuthoritySet,
    effective_authority: CapabilityAuthoritySet,
}

#[cfg(test)]
impl AdmittedCapabilityRequest {
    pub fn requirement_id(&self) -> &CapabilityRequirementId {
        &self.requirement_id
    }

    pub fn optional(&self) -> bool {
        self.optional
    }

    pub(crate) fn execution_authority(&self) -> &CapabilityAuthoritySet {
        &self.execution_authority
    }

    pub(crate) fn effective_authority(&self) -> &CapabilityAuthoritySet {
        &self.effective_authority
    }

    fn duplicate_for_capability_link_plan_owner_v1(&self) -> Self {
        Self {
            requirement_id: self.requirement_id.clone(),
            grant: self.grant.clone(),
            optional: self.optional,
            execution_authority: self
                .execution_authority
                .duplicate_for_capability_link_plan_owner_v1(),
            effective_authority: self
                .effective_authority
                .duplicate_for_capability_link_plan_owner_v1(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityRequestAdmissionPlan {
    requests: Vec<AdmittedCapabilityRequest>,
}

#[cfg(test)]
impl CapabilityRequestAdmissionPlan {
    pub fn requests(&self) -> &[AdmittedCapabilityRequest] {
        self.requests.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum CapabilityRequestManifestAdmissionError {
    MalformedIdentifier(LinkerIdentifierError),
    MalformedTypeContract(CapabilityTypeContractError),
    MalformedContractTsonWitness(CapabilityContractTsonWitnessError),
    RequestTargetDrift {
        requirement_id: CapabilityRequirementId,
    },
    DuplicateRequirementDescriptor {
        requirement_descriptor_id: RequirementDescriptorId,
    },
    MissingGrantForRequiredRequest {
        requirement_id: CapabilityRequirementId,
    },
    MissingGrantForOptionalRequest {
        requirement_id: CapabilityRequirementId,
    },
    RequestContractDrift {
        requirement_id: CapabilityRequirementId,
        grant_contract_id: CapabilityContractId,
    },
    RequestExecutionAuthorityMissingTarget {
        requirement_id: CapabilityRequirementId,
    },
    RequestEffectiveAuthorityMissingTarget {
        requirement_id: CapabilityRequirementId,
    },
    RequestEffectiveAuthorityExceedsExecution {
        requirement_id: CapabilityRequirementId,
        excess_targets: Vec<CapabilityTarget>,
    },
}

#[cfg(test)]
impl fmt::Display for CapabilityRequestManifestAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedIdentifier(source) => write!(f, "{source}"),
            Self::MalformedTypeContract(source) => write!(f, "{source}"),
            Self::MalformedContractTsonWitness(source) => write!(f, "{source}"),
            Self::RequestTargetDrift { requirement_id } => write!(
                f,
                "request manifest target {}:{} does not match request contract target",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name()
            ),
            Self::DuplicateRequirementDescriptor {
                requirement_descriptor_id,
            } => write!(
                f,
                "request manifest carries duplicate requirement descriptor {requirement_descriptor_id}"
            ),
            Self::MissingGrantForRequiredRequest { requirement_id } => write!(
                f,
                "required request {}:{} has no admitted grant",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name()
            ),
            Self::MissingGrantForOptionalRequest { requirement_id } => write!(
                f,
                "optional request {}:{} was not admitted as bound before provider invocation",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name()
            ),
            Self::RequestContractDrift {
                requirement_id,
                grant_contract_id,
            } => write!(
                f,
                "request {}:{} requires contract {}, grant carries {}",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name(),
                requirement_id.contract_id().contract_key(),
                grant_contract_id.contract_key()
            ),
            Self::RequestExecutionAuthorityMissingTarget { requirement_id } => write!(
                f,
                "request {}:{} execution authority does not include its request target",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name()
            ),
            Self::RequestEffectiveAuthorityMissingTarget { requirement_id } => write!(
                f,
                "request {}:{} effective authority does not include its request target",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name()
            ),
            Self::RequestEffectiveAuthorityExceedsExecution {
                requirement_id,
                excess_targets,
            } => write!(
                f,
                "request {}:{} effective authority exceeds execution authority by {} target(s)",
                requirement_id.target().package_specifier(),
                requirement_id.target().export_name(),
                excess_targets.len()
            ),
        }
    }
}

#[cfg(test)]
impl Error for CapabilityRequestManifestAdmissionError {}

#[cfg(test)]
impl From<LinkerIdentifierError> for CapabilityRequestManifestAdmissionError {
    fn from(value: LinkerIdentifierError) -> Self {
        Self::MalformedIdentifier(value)
    }
}

#[cfg(test)]
impl From<CapabilityTypeContractError> for CapabilityRequestManifestAdmissionError {
    fn from(value: CapabilityTypeContractError) -> Self {
        Self::MalformedTypeContract(value)
    }
}

#[cfg(test)]
impl From<CapabilityContractTsonWitnessError> for CapabilityRequestManifestAdmissionError {
    fn from(value: CapabilityContractTsonWitnessError) -> Self {
        Self::MalformedContractTsonWitness(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) struct ProviderBindingIdentity {
    target: CapabilityTarget,
    contract_id: CapabilityContractId,
    provider_binding_id: ProviderBindingId,
}

#[cfg(test)]
impl ProviderBindingIdentity {
    pub fn new(
        target: CapabilityTarget,
        contract_id: CapabilityContractId,
        provider_binding_id: ProviderBindingId,
    ) -> Self {
        Self {
            target,
            contract_id,
            provider_binding_id,
        }
    }

    pub fn target(&self) -> &CapabilityTarget {
        &self.target
    }

    pub fn contract_id(&self) -> &CapabilityContractId {
        &self.contract_id
    }

    pub fn provider_binding_id(&self) -> &ProviderBindingId {
        &self.provider_binding_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) enum ProviderPrivilege {
    BuiltIn,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) enum ProviderExecutionDomain {
    RustInternal,
    RustSdk,
    ExternalHost,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) enum ProviderRef {
    RustInternal(RustInternalProviderId),
    RustSdk(RustSdkProviderId),
    ExternalHost(ExternalProviderBindingId),
}

#[cfg(test)]
impl ProviderRef {
    pub fn execution_domain(&self) -> ProviderExecutionDomain {
        match self {
            Self::RustInternal(_) => ProviderExecutionDomain::RustInternal,
            Self::RustSdk(_) => ProviderExecutionDomain::RustSdk,
            Self::ExternalHost(_) => ProviderExecutionDomain::ExternalHost,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::RustInternal(id) => id.as_str(),
            Self::RustSdk(id) => id.as_str(),
            Self::ExternalHost(id) => id.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
pub(crate) struct ProviderBindingSpec {
    identity: ProviderBindingIdentity,
    provider_id: ProviderId,
    privilege: ProviderPrivilege,
    provider_ref: ProviderRef,
    context_shape_id: ProviderContextShapeId,
}

#[cfg(test)]
impl ProviderBindingSpec {
    pub fn new(
        identity: ProviderBindingIdentity,
        provider_id: ProviderId,
        privilege: ProviderPrivilege,
        provider_ref: ProviderRef,
        context_shape_id: ProviderContextShapeId,
    ) -> Self {
        Self {
            identity,
            provider_id,
            privilege,
            provider_ref,
            context_shape_id,
        }
    }

    pub fn identity(&self) -> &ProviderBindingIdentity {
        &self.identity
    }

    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    pub fn privilege(&self) -> ProviderPrivilege {
        self.privilege
    }

    pub fn execution_domain(&self) -> ProviderExecutionDomain {
        self.provider_ref.execution_domain()
    }

    pub fn provider_ref(&self) -> &ProviderRef {
        &self.provider_ref
    }

    pub fn context_shape_id(&self) -> &ProviderContextShapeId {
        &self.context_shape_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityRequestManifestWireBuildError {
    MalformedIdentifier(LinkerIdentifierError),
    MalformedTypeContract(CapabilityTypeContractError),
    MalformedContractTsonWitness(CapabilityContractTsonWitnessError),
    ContractWitnessTargetDrift {
        module_id: String,
        export_name: String,
    },
    DuplicateRequirementDescriptor {
        requirement_descriptor_id: String,
    },
    JsonSerialization {
        source: String,
    },
}

impl fmt::Display for CapabilityRequestManifestWireBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedIdentifier(error) => write!(
                f,
                "capability request manifest wire identifier is malformed: {error}"
            ),
            Self::MalformedTypeContract(error) => write!(
                f,
                "capability request manifest wire type contract is malformed: {error}"
            ),
            Self::MalformedContractTsonWitness(error) => write!(
                f,
                "capability request manifest wire contract TSON witness is malformed: {error}"
            ),
            Self::ContractWitnessTargetDrift {
                module_id,
                export_name,
            } => write!(
                f,
                "capability request manifest witness target drifts from {module_id}:{export_name}"
            ),
            Self::DuplicateRequirementDescriptor {
                requirement_descriptor_id,
            } => write!(
                f,
                "capability request manifest carries duplicate requirement descriptor {requirement_descriptor_id}"
            ),
            Self::JsonSerialization { source } => write!(
                f,
                "capability request manifest JSON serialization failed: {source}"
            ),
        }
    }
}

impl Error for CapabilityRequestManifestWireBuildError {}

impl From<LinkerIdentifierError> for CapabilityRequestManifestWireBuildError {
    fn from(value: LinkerIdentifierError) -> Self {
        Self::MalformedIdentifier(value)
    }
}

impl From<CapabilityTypeContractError> for CapabilityRequestManifestWireBuildError {
    fn from(value: CapabilityTypeContractError) -> Self {
        Self::MalformedTypeContract(value)
    }
}

impl From<CapabilityContractTsonWitnessError> for CapabilityRequestManifestWireBuildError {
    fn from(value: CapabilityContractTsonWitnessError) -> Self {
        Self::MalformedContractTsonWitness(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapabilityGrantContractWitnessJson {
    kind: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    module_id: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    export_name: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    member_path: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    input_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    output_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    provenance: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    input_type_contract: CapabilityTypeContractJson,
    output_type_contract: CapabilityTypeContractJson,
    contract_tson_witness: Option<CapabilityContractTsonWitnessJson>,
}

impl CapabilityGrantContractWitnessJson {
    pub fn new(
        module_id: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _export_name: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _member_path: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _input_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _output_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _provenance: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _input_type_contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _output_type_contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, CapabilityRequestManifestWireBuildError> {
        match module_id {}
    }

    pub fn new_with_contract_tson_witness(
        module_id: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _export_name: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _member_path: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _input_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _output_contract_key: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _provenance: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _input_type_contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _output_type_contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        _contract_tson_witness: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, CapabilityRequestManifestWireBuildError> {
        match module_id {}
    }

    pub fn into_output_type_contract_authority_for_capability_linker_owner_v1(
        self,
        input: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilityContractOutputTypeContractAuthorityProduct {
        let _ = self;
        match input {}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapabilityRequestManifestEntryJson {
    module_id: String,
    export_name: String,
    member_path: Option<Vec<String>>,
    operation_id: Option<String>,
    requirement_descriptor_id: String,
    optional: bool,
    contract_witness: CapabilityGrantContractWitnessJson,
}

impl CapabilityRequestManifestEntryJson {
    /// Builds a typed capability-request manifest transport DTO from compiler/linker
    /// products. This serializes request evidence only; it does not admit or mint
    /// executable/provider authority from the resulting manifest cargo.
    pub fn new(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, CapabilityRequestManifestWireBuildError> {
        match value {}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CapabilityRequestManifestJson {
    kind: String,
    requests: Vec<CapabilityRequestManifestEntryJson>,
}

impl CapabilityRequestManifestJson {
    /// Builds a typed capability-request manifest transport DTO from already
    /// admitted request entries. This boundary is for canonical wire emission,
    /// not manifest-to-authority rehydration.
    pub fn new(
        value: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> Result<Self, CapabilityRequestManifestWireBuildError> {
        match value {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum ContractBundleAdmissionError {
    EmptyBundle {
        package_specifier: PackageSpecifier,
    },
    PackageSpecifierDrift {
        expected: PackageSpecifier,
        received: PackageSpecifier,
    },
    DuplicateExport {
        export_name: ExportName,
    },
    DuplicateMemberPath {
        export_name: ExportName,
        member_path: String,
    },
    ContractKeyDrift {
        export_name: ExportName,
        member_path: Option<String>,
    },
}

#[cfg(test)]
impl fmt::Display for ContractBundleAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBundle { package_specifier } => {
                write!(
                    f,
                    "contract bundle for {package_specifier} must not be empty"
                )
            }
            Self::PackageSpecifierDrift { expected, received } => write!(
                f,
                "contract bundle for {expected} cannot install contract from {received}"
            ),
            Self::DuplicateExport { export_name } => {
                write!(f, "contract bundle carries duplicate export {export_name}")
            }
            Self::DuplicateMemberPath {
                export_name,
                member_path,
            } => write!(
                f,
                "contract bundle carries duplicate member path {export_name}.{member_path}"
            ),
            Self::ContractKeyDrift {
                export_name,
                member_path,
            } => match member_path {
                Some(member_path) => write!(
                    f,
                    "contract bundle carries contract-key drift for {export_name}.{member_path}"
                ),
                None => write!(
                    f,
                    "contract bundle carries contract-key drift for {export_name}"
                ),
            },
        }
    }
}

#[cfg(test)]
impl Error for ContractBundleAdmissionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum ProviderBindingAdmissionError {
    TargetDrift {
        identity_target: CapabilityTarget,
        contract_target: CapabilityTarget,
    },
    MissingInstalledContract {
        contract_id: CapabilityContractId,
    },
    ContractKeyDrift {
        requested: CapabilityContractId,
        installed: CapabilityContractId,
    },
    DuplicateProviderBinding {
        contract_id: CapabilityContractId,
    },
    ProviderIdDrift {
        contract_id: CapabilityContractId,
        existing: ProviderId,
        received: ProviderId,
    },
    ProviderPrivilegeDrift {
        contract_id: CapabilityContractId,
        existing: ProviderPrivilege,
        received: ProviderPrivilege,
    },
    ProviderReferenceDrift {
        contract_id: CapabilityContractId,
        existing: ProviderRef,
        received: ProviderRef,
    },
    BuiltInProviderRequiresRustInternalRef {
        contract_id: CapabilityContractId,
        provider_ref: ProviderRef,
    },
    ExternalProviderCannotUseRustInternalRef {
        contract_id: CapabilityContractId,
        provider_ref: ProviderRef,
    },
    ProviderContextShapeDrift {
        contract_id: CapabilityContractId,
        existing: ProviderContextShapeId,
        received: ProviderContextShapeId,
    },
}

#[cfg(test)]
impl fmt::Display for ProviderBindingAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TargetDrift {
                identity_target,
                contract_target,
            } => write!(
                f,
                "provider binding target {}:{} does not match contract target {}:{}",
                identity_target.package_specifier(),
                identity_target.export_name(),
                contract_target.package_specifier(),
                contract_target.export_name()
            ),
            Self::MissingInstalledContract { contract_id } => write!(
                f,
                "provider binding requires installed contract {}:{}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name()
            ),
            Self::ContractKeyDrift {
                requested,
                installed,
            } => write!(
                f,
                "provider binding contract-key drift for {}:{}: requested {}, installed {}",
                requested.target().package_specifier(),
                requested.target().export_name(),
                requested.contract_key(),
                installed.contract_key()
            ),
            Self::DuplicateProviderBinding { contract_id } => write!(
                f,
                "provider binding already exists for {}:{}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name()
            ),
            Self::ProviderIdDrift {
                contract_id,
                existing,
                received,
            } => write!(
                f,
                "provider id drift for {}:{}: existing {}, received {}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                existing,
                received
            ),
            Self::ProviderPrivilegeDrift {
                contract_id,
                existing,
                received,
            } => write!(
                f,
                "provider privilege drift for {}:{}: existing {:?}, received {:?}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                existing,
                received
            ),
            Self::ProviderReferenceDrift {
                contract_id,
                existing,
                received,
            } => write!(
                f,
                "provider reference drift for {}:{}: existing {}, received {}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                existing.as_str(),
                received.as_str()
            ),
            Self::BuiltInProviderRequiresRustInternalRef {
                contract_id,
                provider_ref,
            } => write!(
                f,
                "built-in provider binding for {}:{} requires a Rust-internal provider reference, received {}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                provider_ref.as_str()
            ),
            Self::ExternalProviderCannotUseRustInternalRef {
                contract_id,
                provider_ref,
            } => write!(
                f,
                "external provider binding for {}:{} requires a non-internal provider reference, received {}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                provider_ref.as_str()
            ),
            Self::ProviderContextShapeDrift {
                contract_id,
                existing,
                received,
            } => write!(
                f,
                "provider context shape drift for {}:{}: existing {}, received {}",
                contract_id.target().package_specifier(),
                contract_id.target().export_name(),
                existing,
                received
            ),
        }
    }
}

#[cfg(test)]
impl Error for ProviderBindingAdmissionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum ProviderAttachmentAdmissionError {
    MissingProviderBinding { target: CapabilityTarget },
    UnauthorizedProviderBinding { target: CapabilityTarget },
    DuplicateProviderAttachment { target: CapabilityTarget },
    UninstalledProviderBinding { target: CapabilityTarget },
    ProviderBindingDrift { target: CapabilityTarget },
}

#[cfg(test)]
impl fmt::Display for ProviderAttachmentAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProviderBinding { target } => write!(
                f,
                "provider attachment is missing binding for {}:{}",
                target.package_specifier(),
                target.export_name()
            ),
            Self::UnauthorizedProviderBinding { target } => write!(
                f,
                "provider attachment supplied unauthorized binding for {}:{}",
                target.package_specifier(),
                target.export_name()
            ),
            Self::DuplicateProviderAttachment { target } => write!(
                f,
                "provider attachment supplied duplicate binding for {}:{}",
                target.package_specifier(),
                target.export_name()
            ),
            Self::UninstalledProviderBinding { target } => write!(
                f,
                "provider attachment supplied uninstalled binding for {}:{}",
                target.package_specifier(),
                target.export_name()
            ),
            Self::ProviderBindingDrift { target } => write!(
                f,
                "provider attachment binding drift for {}:{}",
                target.package_specifier(),
                target.export_name()
            ),
        }
    }
}

#[cfg(test)]
impl Error for ProviderAttachmentAdmissionError {}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct ProviderAttachmentRequest {
    bindings: Vec<ProviderBindingSpec>,
}

#[cfg(test)]
impl ProviderAttachmentRequest {
    pub(crate) fn new(bindings: Vec<ProviderBindingSpec>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &[ProviderBindingSpec] {
        self.bindings.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct ProviderAttachmentPlan {
    bindings: BTreeMap<CapabilityTarget, ProviderBindingSpec>,
}

#[cfg(test)]
impl ProviderAttachmentPlan {
    pub fn bindings(&self) -> &BTreeMap<CapabilityTarget, ProviderBindingSpec> {
        &self.bindings
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct HostCapabilityLinkPlan {
    grants: CapabilityGrantContractReconciliationPlan,
    requests: CapabilityRequestAdmissionPlan,
    providers: ProviderAttachmentPlan,
    target_aliases: CapabilityTargetAliasPlan,
}

#[cfg(test)]
impl HostCapabilityLinkPlan {
    pub(crate) fn requests(&self) -> &CapabilityRequestAdmissionPlan {
        &self.requests
    }

    pub fn providers(&self) -> &ProviderAttachmentPlan {
        &self.providers
    }

    pub fn target_aliases(&self) -> &CapabilityTargetAliasPlan {
        &self.target_aliases
    }

    pub(crate) fn admit_provider_invocation_for_link_plan_owner_v1(
        &self,
        request: ProviderInvocationRequest,
    ) -> Result<AdmittedProviderInvocation, ProviderInvocationAdmissionError> {
        let (requirement_descriptor_id, operation_id, input) = request.into_parts();
        let Some(admitted_request) = self.requests.requests().iter().find(|request| {
            request.requirement_id().requirement_descriptor_id() == &requirement_descriptor_id
        }) else {
            return Err(
                ProviderInvocationAdmissionError::UnknownRequirementDescriptor {
                    requirement_descriptor_id,
                },
            );
        };
        if admitted_request.requirement_id().operation_id() != operation_id.as_ref() {
            return Err(ProviderInvocationAdmissionError::OperationIdDrift {
                requirement_descriptor_id,
                expected: admitted_request.requirement_id().operation_id().cloned(),
                received: operation_id,
            });
        }
        let provider_target = self
            .target_aliases
            .provider_target_for(admitted_request.requirement_id().target());
        let Some(provider) = self.providers.bindings().get(&provider_target) else {
            return Err(ProviderInvocationAdmissionError::MissingProviderBinding {
                target: provider_target,
            });
        };
        validate_provider_user_payload_value(
            &input,
            provider.identity().provider_binding_id().as_str(),
        )
        .map_err(|source| ProviderInvocationAdmissionError::InputAdmissionFailed { source })?;
        Ok(AdmittedProviderInvocation::new(
            admitted_request.duplicate_for_capability_link_plan_owner_v1(),
            provider.clone(),
            input,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityTargetAlias {
    source_target: CapabilityTarget,
    provider_target: CapabilityTarget,
}

#[cfg(test)]
impl CapabilityTargetAlias {
    pub fn new(source_target: CapabilityTarget, provider_target: CapabilityTarget) -> Self {
        Self {
            source_target,
            provider_target,
        }
    }

    pub fn source_target(&self) -> &CapabilityTarget {
        &self.source_target
    }

    pub fn provider_target(&self) -> &CapabilityTarget {
        &self.provider_target
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct CapabilityTargetAliasPlan {
    aliases: BTreeMap<CapabilityTarget, CapabilityTarget>,
}

#[cfg(test)]
impl CapabilityTargetAliasPlan {
    pub fn new(
        aliases: Vec<CapabilityTargetAlias>,
    ) -> Result<Self, CapabilityTargetAliasAdmissionError> {
        let mut by_source_target = BTreeMap::new();
        for alias in aliases {
            if alias.source_target() == alias.provider_target() {
                return Err(CapabilityTargetAliasAdmissionError::IdentityAlias {
                    target: alias.source_target,
                });
            }
            if by_source_target
                .insert(alias.source_target.clone(), alias.provider_target.clone())
                .is_some()
            {
                return Err(CapabilityTargetAliasAdmissionError::DuplicateSourceTarget {
                    source_target: alias.source_target,
                });
            }
        }

        Ok(Self {
            aliases: by_source_target,
        })
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn provider_target_for(&self, source_target: &CapabilityTarget) -> CapabilityTarget {
        self.aliases
            .get(source_target)
            .cloned()
            .unwrap_or_else(|| source_target.clone())
    }
}

#[cfg(test)]
fn validate_request_entry_authority(
    entry: &CapabilityRequestManifestEntry,
) -> Result<(), CapabilityRequestManifestAdmissionError> {
    let request_target = entry.requirement_id().target();
    if !entry.execution_authority().contains(request_target) {
        return Err(
            CapabilityRequestManifestAdmissionError::RequestExecutionAuthorityMissingTarget {
                requirement_id: entry.requirement_id().clone(),
            },
        );
    }
    if !entry.effective_authority().contains(request_target) {
        return Err(
            CapabilityRequestManifestAdmissionError::RequestEffectiveAuthorityMissingTarget {
                requirement_id: entry.requirement_id().clone(),
            },
        );
    }
    if !entry
        .effective_authority()
        .is_subset(entry.execution_authority())
    {
        return Err(
            CapabilityRequestManifestAdmissionError::RequestEffectiveAuthorityExceedsExecution {
                requirement_id: entry.requirement_id().clone(),
                excess_targets: entry
                    .effective_authority()
                    .difference_from(entry.execution_authority()),
            },
        );
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
struct ReconciledCapabilityGrant {
    target: CapabilityTarget,
    contract_id: CapabilityContractId,
}

#[cfg(test)]
impl ReconciledCapabilityGrant {
    fn contract_id(&self) -> &CapabilityContractId {
        &self.contract_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
struct CapabilityGrantContractReconciliationPlan {
    grants: BTreeMap<CapabilityTarget, ReconciledCapabilityGrant>,
}

#[cfg(test)]
impl CapabilityGrantContractReconciliationPlan {
    fn grants(&self) -> &BTreeMap<CapabilityTarget, ReconciledCapabilityGrant> {
        &self.grants
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum HostCapabilityLinkPlanAdmissionError {
    RequestManifestAdmissionFailed(CapabilityRequestManifestAdmissionError),
    ProviderAttachmentAdmissionFailed(ProviderAttachmentAdmissionError),
}

#[cfg(test)]
impl fmt::Display for HostCapabilityLinkPlanAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestManifestAdmissionFailed(source) => {
                write!(f, "host capability link request admission failed: {source}")
            }
            Self::ProviderAttachmentAdmissionFailed(source) => {
                write!(
                    f,
                    "host capability link provider admission failed: {source}"
                )
            }
        }
    }
}

#[cfg(test)]
impl Error for HostCapabilityLinkPlanAdmissionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum CapabilityTargetAliasAdmissionError {
    IdentityAlias { target: CapabilityTarget },
    DuplicateSourceTarget { source_target: CapabilityTarget },
}

#[cfg(test)]
impl fmt::Display for CapabilityTargetAliasAdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityAlias { target } => write!(
                f,
                "capability target alias for {}:{} maps to itself",
                target.package_specifier(),
                target.export_name()
            ),
            Self::DuplicateSourceTarget { source_target } => write!(
                f,
                "capability target alias has duplicate source target {}:{}",
                source_target.package_specifier(),
                source_target.export_name()
            ),
        }
    }
}

#[cfg(test)]
impl Error for CapabilityTargetAliasAdmissionError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(test)]
struct ContractTargetKey {
    export_name: ExportName,
    member_path: Option<MemberPath>,
}

#[cfg(test)]
impl ContractTargetKey {
    fn from_contract_id(contract_id: &CapabilityContractId) -> Self {
        Self {
            export_name: contract_id.target().export_name().clone(),
            member_path: contract_id.target().member_path().cloned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct PackageContractBundle {
    package_specifier: PackageSpecifier,
    contracts: Vec<CapabilityContractId>,
}

#[cfg(test)]
impl PackageContractBundle {
    pub fn new(
        package_specifier: PackageSpecifier,
        contracts: Vec<CapabilityContractId>,
    ) -> Result<Self, ContractBundleAdmissionError> {
        if contracts.is_empty() {
            return Err(ContractBundleAdmissionError::EmptyBundle { package_specifier });
        }

        let mut by_target = BTreeMap::<ContractTargetKey, CapabilityContractId>::new();
        for contract in &contracts {
            if contract.target().package_specifier() != &package_specifier {
                return Err(ContractBundleAdmissionError::PackageSpecifierDrift {
                    expected: package_specifier,
                    received: contract.target().package_specifier().clone(),
                });
            }

            let target_key = ContractTargetKey::from_contract_id(contract);
            if let Some(existing) = by_target.get(&target_key) {
                if existing != contract {
                    return Err(ContractBundleAdmissionError::ContractKeyDrift {
                        export_name: target_key.export_name,
                        member_path: target_key.member_path.as_ref().map(MemberPath::dotted),
                    });
                }

                return match target_key.member_path.as_ref() {
                    Some(member_path) => Err(ContractBundleAdmissionError::DuplicateMemberPath {
                        export_name: target_key.export_name,
                        member_path: member_path.dotted(),
                    }),
                    None => Err(ContractBundleAdmissionError::DuplicateExport {
                        export_name: target_key.export_name,
                    }),
                };
            }

            by_target.insert(target_key, contract.clone());
        }

        Ok(Self {
            package_specifier,
            contracts,
        })
    }

    pub fn contracts(&self) -> &[CapabilityContractId] {
        self.contracts.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct InstalledContractBundle {
    package_specifier: PackageSpecifier,
    contracts: Vec<CapabilityContractId>,
}

#[cfg(test)]
impl InstalledContractBundle {
    pub fn package_specifier(&self) -> &PackageSpecifier {
        &self.package_specifier
    }

    pub fn contracts(&self) -> &[CapabilityContractId] {
        self.contracts.as_slice()
    }
}

#[cfg(test)]
fn request_contract_matches_grant_contract(
    requirement_id: &CapabilityRequirementId,
    grant_contract_id: &CapabilityContractId,
) -> bool {
    let request_contract_id = requirement_id.contract_id();
    request_contract_id == grant_contract_id
        || (request_contract_id.contract_key() == grant_contract_id.contract_key()
            && request_contract_id.input_contract_key() == grant_contract_id.input_contract_key()
            && request_contract_id.output_contract_key() == grant_contract_id.output_contract_key()
            && request_contract_id.provenance() == grant_contract_id.provenance()
            && request_contract_id.contract_types() == grant_contract_id.contract_types())
}

#[derive(Debug, Default)]
#[cfg(test)]
pub(crate) struct HostCapabilityLinker {
    admitted_contracts: BTreeSet<CapabilityContractId>,
    provider_bindings: BTreeMap<CapabilityContractId, ProviderBindingSpec>,
}

#[cfg(test)]
impl HostCapabilityLinker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn install_contract_bundle(
        &mut self,
        bundle: PackageContractBundle,
    ) -> InstalledContractBundle {
        for contract in bundle.contracts() {
            self.admitted_contracts.insert(contract.clone());
        }

        InstalledContractBundle {
            package_specifier: bundle.package_specifier,
            contracts: bundle.contracts,
        }
    }

    pub fn admitted_contract_count(&self) -> usize {
        self.admitted_contracts.len()
    }

    pub fn provider_binding_count(&self) -> usize {
        self.provider_bindings.len()
    }

    fn admit_capability_request_manifest_against_reconciled_grants(
        grant_plan: &CapabilityGrantContractReconciliationPlan,
        manifest: CapabilityRequestManifest,
        aliases: &CapabilityTargetAliasPlan,
    ) -> Result<CapabilityRequestAdmissionPlan, CapabilityRequestManifestAdmissionError> {
        let mut requests = Vec::new();
        for entry in manifest.entries {
            let requirement_id = entry.requirement_id;
            let provider_target = aliases.provider_target_for(requirement_id.target());
            let Some(grant) = grant_plan.grants().get(&provider_target) else {
                return if entry.optional {
                    Err(
                        CapabilityRequestManifestAdmissionError::MissingGrantForOptionalRequest {
                            requirement_id,
                        },
                    )
                } else {
                    Err(
                        CapabilityRequestManifestAdmissionError::MissingGrantForRequiredRequest {
                            requirement_id,
                        },
                    )
                };
            };

            if !request_contract_matches_grant_contract(&requirement_id, grant.contract_id()) {
                return Err(
                    CapabilityRequestManifestAdmissionError::RequestContractDrift {
                        requirement_id,
                        grant_contract_id: grant.contract_id().clone(),
                    },
                );
            }

            requests.push(AdmittedCapabilityRequest {
                requirement_id,
                grant: grant.clone(),
                optional: entry.optional,
                execution_authority: entry.execution_authority,
                effective_authority: entry.effective_authority,
            });
        }

        Ok(CapabilityRequestAdmissionPlan { requests })
    }

    pub fn bind_provider(
        &mut self,
        binding: ProviderBindingSpec,
    ) -> Result<ProviderBindingSpec, ProviderBindingAdmissionError> {
        if binding.identity().target() != binding.identity().contract_id().target() {
            return Err(ProviderBindingAdmissionError::TargetDrift {
                identity_target: binding.identity().target().clone(),
                contract_target: binding.identity().contract_id().target().clone(),
            });
        }

        let contract_id = binding.identity().contract_id();
        match (binding.privilege(), binding.execution_domain()) {
            (ProviderPrivilege::BuiltIn, ProviderExecutionDomain::RustInternal)
            | (ProviderPrivilege::External, ProviderExecutionDomain::RustSdk)
            | (ProviderPrivilege::External, ProviderExecutionDomain::ExternalHost) => {}
            (ProviderPrivilege::BuiltIn, ProviderExecutionDomain::RustSdk)
            | (ProviderPrivilege::BuiltIn, ProviderExecutionDomain::ExternalHost) => {
                return Err(
                    ProviderBindingAdmissionError::BuiltInProviderRequiresRustInternalRef {
                        contract_id: contract_id.clone(),
                        provider_ref: binding.provider_ref().clone(),
                    },
                );
            }
            (ProviderPrivilege::External, ProviderExecutionDomain::RustInternal) => {
                return Err(
                    ProviderBindingAdmissionError::ExternalProviderCannotUseRustInternalRef {
                        contract_id: contract_id.clone(),
                        provider_ref: binding.provider_ref().clone(),
                    },
                );
            }
        }

        if !self.admitted_contracts.contains(contract_id) {
            if let Some(installed) = self
                .admitted_contracts
                .iter()
                .find(|installed| installed.target() == contract_id.target())
            {
                return Err(ProviderBindingAdmissionError::ContractKeyDrift {
                    requested: contract_id.clone(),
                    installed: installed.clone(),
                });
            }

            return Err(ProviderBindingAdmissionError::MissingInstalledContract {
                contract_id: contract_id.clone(),
            });
        }

        if let Some(existing) = self.provider_bindings.get(contract_id) {
            if existing.provider_id() != binding.provider_id() {
                return Err(ProviderBindingAdmissionError::ProviderIdDrift {
                    contract_id: contract_id.clone(),
                    existing: existing.provider_id().clone(),
                    received: binding.provider_id().clone(),
                });
            }

            if existing.privilege() != binding.privilege() {
                return Err(ProviderBindingAdmissionError::ProviderPrivilegeDrift {
                    contract_id: contract_id.clone(),
                    existing: existing.privilege(),
                    received: binding.privilege(),
                });
            }

            if existing.provider_ref() != binding.provider_ref() {
                return Err(ProviderBindingAdmissionError::ProviderReferenceDrift {
                    contract_id: contract_id.clone(),
                    existing: existing.provider_ref().clone(),
                    received: binding.provider_ref().clone(),
                });
            }

            if existing.context_shape_id() != binding.context_shape_id() {
                return Err(ProviderBindingAdmissionError::ProviderContextShapeDrift {
                    contract_id: contract_id.clone(),
                    existing: existing.context_shape_id().clone(),
                    received: binding.context_shape_id().clone(),
                });
            }

            return Err(ProviderBindingAdmissionError::DuplicateProviderBinding {
                contract_id: contract_id.clone(),
            });
        }

        self.provider_bindings
            .insert(contract_id.clone(), binding.clone());
        Ok(binding)
    }

    pub(crate) fn admit_provider_bound_capability_link_plan(
        &self,
        manifest: CapabilityRequestManifest,
        providers: ProviderAttachmentRequest,
    ) -> Result<HostCapabilityLinkPlan, HostCapabilityLinkPlanAdmissionError> {
        self.admit_provider_bound_capability_link_plan_with_target_aliases(
            manifest,
            providers,
            CapabilityTargetAliasPlan::empty(),
        )
    }

    pub(crate) fn admit_provider_bound_capability_link_plan_with_target_aliases(
        &self,
        manifest: CapabilityRequestManifest,
        providers: ProviderAttachmentRequest,
        target_aliases: CapabilityTargetAliasPlan,
    ) -> Result<HostCapabilityLinkPlan, HostCapabilityLinkPlanAdmissionError> {
        let mut providers_by_target = BTreeMap::new();
        for binding in providers.bindings() {
            let target = binding.identity().target().clone();
            if providers_by_target
                .insert(target.clone(), binding)
                .is_some()
            {
                return Err(
                    HostCapabilityLinkPlanAdmissionError::ProviderAttachmentAdmissionFailed(
                        ProviderAttachmentAdmissionError::DuplicateProviderAttachment { target },
                    ),
                );
            }
        }

        let mut grants = BTreeMap::new();
        for entry in manifest.entries() {
            let provider_target =
                target_aliases.provider_target_for(entry.requirement_id().target());
            if let Some(binding) = providers_by_target.get(&provider_target) {
                grants.entry(provider_target.clone()).or_insert_with(|| {
                    ReconciledCapabilityGrant {
                        target: provider_target,
                        contract_id: binding.identity().contract_id().clone(),
                    }
                });
            }
        }

        for target in providers_by_target.keys() {
            if !grants.contains_key(target) {
                return Err(
                    HostCapabilityLinkPlanAdmissionError::ProviderAttachmentAdmissionFailed(
                        ProviderAttachmentAdmissionError::UnauthorizedProviderBinding {
                            target: target.clone(),
                        },
                    ),
                );
            }
        }
        let grants = CapabilityGrantContractReconciliationPlan { grants };
        let requests = Self::admit_capability_request_manifest_against_reconciled_grants(
            &grants,
            manifest,
            &target_aliases,
        )
        .map_err(HostCapabilityLinkPlanAdmissionError::RequestManifestAdmissionFailed)?;
        let providers = self
            .admit_provider_attachment_request_against_reconciled_grants(&grants, providers)
            .map_err(HostCapabilityLinkPlanAdmissionError::ProviderAttachmentAdmissionFailed)?;

        Ok(HostCapabilityLinkPlan {
            grants,
            requests,
            providers,
            target_aliases,
        })
    }

    fn admit_provider_attachment_request_against_reconciled_grants(
        &self,
        reconciled_grants: &CapabilityGrantContractReconciliationPlan,
        request: ProviderAttachmentRequest,
    ) -> Result<ProviderAttachmentPlan, ProviderAttachmentAdmissionError> {
        let mut bindings_by_target = BTreeMap::<CapabilityTarget, ProviderBindingSpec>::new();
        for binding in request.bindings {
            let target = binding.identity().target().clone();
            if !reconciled_grants.grants().contains_key(&target) {
                return Err(
                    ProviderAttachmentAdmissionError::UnauthorizedProviderBinding { target },
                );
            }

            let Some(installed_binding) =
                self.provider_bindings.get(binding.identity().contract_id())
            else {
                return Err(
                    ProviderAttachmentAdmissionError::UninstalledProviderBinding { target },
                );
            };

            if installed_binding != &binding {
                return Err(ProviderAttachmentAdmissionError::ProviderBindingDrift { target });
            }

            if bindings_by_target.insert(target.clone(), binding).is_some() {
                return Err(
                    ProviderAttachmentAdmissionError::DuplicateProviderAttachment { target },
                );
            }
        }

        let mut admitted_bindings = BTreeMap::new();
        for (target, grant) in reconciled_grants.grants() {
            let Some(binding) = bindings_by_target.get(target) else {
                return Err(ProviderAttachmentAdmissionError::MissingProviderBinding {
                    target: target.clone(),
                });
            };

            if binding.identity().contract_id() != grant.contract_id() {
                return Err(ProviderAttachmentAdmissionError::ProviderBindingDrift {
                    target: target.clone(),
                });
            }

            admitted_bindings.insert(target.clone(), binding.clone());
        }

        Ok(ProviderAttachmentPlan {
            bindings: admitted_bindings,
        })
    }
}

#[cfg(test)]
mod tests;

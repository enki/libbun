#![forbid(unsafe_code)]

use libswarm_package_graph_executable_program_receipt_owner::AdmittedPackageGraphNativeProviderArtifact;
use libswarm_package_graph_provider_requirements::{
    PackageGraphExactProviderImportSelectionBinderForPreparedRuntimeOwnerV1,
    PackageGraphExactProviderImportSelectionForPreparedRuntimeOwnerV1,
    PackageGraphExactProviderOperationForPreparedRuntimeOwnerV1,
    PackageGraphProviderImportOperationSelectionBinderForPreparedRuntimeOwnerV1,
    PackageGraphProviderImportOperationSelectionForPreparedRuntimeOwnerV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use swarm_capability_contract_tson::{
    AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
    AdmittedCapabilityContractOperationExactJoinForProviderRouteOwnerV1,
    AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1,
};
use swarm_capability_model::CapabilityContractProjection;

const DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_PROVIDER_IMPORT_IDENTITY_SIDECAR_SCHEMA: &str =
    "swarm.semantic_kernel.direct_run.prepared_runtime_image.provider_import_identity_sidecar.v3";
const PREPARED_RUNTIME_PROVIDER_IMPORT_IDENTITY_KEY_SCHEMA: &str =
    "swarm.prepared_runtime.provider_import_identity.v3";
const DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_DAG_CBOR_HASH_PREFIX: &str =
    "sha256:dag-cbor:direct-run-prepared-runtime-image-artifact.";
const DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_DOMAIN_PROVIDER_IMPORT_IDENTITY: &str =
    "provider-import-identity";

// compiler-custody: symbol=SelectedImplementationTarget disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: pass the compiler root scope into ImplementationSelectionOwner selection and carry the ticket into PreparedRuntimeImageProviderImportIdentity"
#[must_use = "the selected implementation target must be consumed by the prepared-runtime artifact owner, not dropped"]
pub struct SelectedImplementationTarget {
    contract: CapabilityContractProjection,
    kind: SelectedImplementationTargetKind,
}

// compiler-custody: symbol=SelectedImplementationTargetKind disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: mint the selected-target ticket before this owner-internal branch and move it unchanged through both implementation variants"
enum SelectedImplementationTargetKind {
    ExternalOperation {
        operation: AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
        provider_module_import_path: String,
        provider_module_export: String,
    },
    NativeContract {
        provider_module_import_path: String,
        provider_module_export: String,
        native_provider_artifact: AdmittedPackageGraphNativeProviderArtifact,
    },
}

enum ImplementationSelectionOwnerKind {
    SourceMaterializationPackageResolution,
    PreparedRuntimeArtifactProviderImport,
    PreparedStaticChildProviderImport,
}

impl ImplementationSelectionOwnerKind {
    fn label(&self) -> &'static str {
        match self {
            Self::SourceMaterializationPackageResolution => {
                "source_materialization_package_resolution"
            }
            Self::PreparedRuntimeArtifactProviderImport => {
                "prepared_runtime_artifact_provider_import"
            }
            Self::PreparedStaticChildProviderImport => "prepared_static_child_provider_import",
        }
    }
}

pub struct ImplementationSelectionOwner {
    kind: ImplementationSelectionOwnerKind,
}

struct PackageGraphProviderOperationSelectionBinderForPreparedRuntimeImageOwnerV1 {
    contract: CapabilityContractProjection,
    operation: AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
}

struct PackageGraphExactProviderImportSelectionBinderForPreparedRuntimeImageOwnerV1;

impl ImplementationSelectionOwner {
    pub fn purpose_label(&self) -> &'static str {
        self.kind.label()
    }

    pub fn source_materialization_package_resolution_v1() -> Self {
        Self {
            kind: ImplementationSelectionOwnerKind::SourceMaterializationPackageResolution,
        }
    }

    pub(crate) fn prepared_runtime_artifact_provider_import_v1() -> Self {
        Self {
            kind: ImplementationSelectionOwnerKind::PreparedRuntimeArtifactProviderImport,
        }
    }

    pub(crate) fn prepared_static_child_provider_import_v1() -> Self {
        Self {
            kind: ImplementationSelectionOwnerKind::PreparedStaticChildProviderImport,
        }
    }

    pub fn admit_exact_package_graph_provider_import_selection_for_source_materialization_owner_v1(
        &self,
        selection: PackageGraphExactProviderImportSelectionForPreparedRuntimeOwnerV1,
    ) -> Result<PreparedRuntimeImageProviderImportIdentity, String> {
        self.require_source_materialization_owner("provider identity operation import")?;
        selection.consume_with_binder_for_prepared_runtime_owner_v1(
            PackageGraphExactProviderImportSelectionBinderForPreparedRuntimeImageOwnerV1,
        )
    }

    fn admit_exact_package_graph_provider_operation_selection_for_source_materialization_owner_v1(
        contract_source: libswarm_package_graph_contract_source_admission::AdmittedPackageGraphContractSource,
        operation: PackageGraphExactProviderOperationForPreparedRuntimeOwnerV1,
        selected_operation: PackageGraphProviderImportOperationSelectionForPreparedRuntimeOwnerV1,
    ) -> Result<PreparedRuntimeImageProviderImportIdentity, String> {
        let route_identity = match operation {
            PackageGraphExactProviderOperationForPreparedRuntimeOwnerV1::Command(operation) => {
                swarm_capability_contract_tson::AdmittedCapabilityContractTson::admit_exact_command_route_identity_from_package_graph_contract_source_for_prepared_runtime_owner_v1(
                    contract_source,
                    operation,
                )
            }
            PackageGraphExactProviderOperationForPreparedRuntimeOwnerV1::InteractionOpen(
                operation,
            ) => swarm_capability_contract_tson::AdmittedCapabilityContractTson::admit_exact_interaction_open_route_identity_from_package_graph_contract_source_for_prepared_runtime_owner_v1(
                contract_source,
                operation,
            ),
        }
        .map_err(|source| source.to_string())?;
        let (contract, operation) =
            route_identity.into_contract_and_operation_for_prepared_runtime_owner_v1();
        selected_operation.consume_with_binder_for_prepared_runtime_owner_v1(
            PackageGraphProviderOperationSelectionBinderForPreparedRuntimeImageOwnerV1 {
                contract,
                operation,
            },
        )
    }

    pub(crate) fn select_package_graph_native_provider_artifact_v1(
        &self,
        native_provider_artifact: AdmittedPackageGraphNativeProviderArtifact,
        contract: CapabilityContractProjection,
    ) -> Result<SelectedImplementationTarget, String> {
        self.require_source_materialization_owner("native provider artifact")?;
        swarm_capability_model::validate_contract_projection(&contract)
            .map_err(|error| error.to_string())?;
        let provider_module_export = contract.export_name().to_owned();
        Ok(SelectedImplementationTarget {
            contract,
            kind: SelectedImplementationTargetKind::NativeContract {
                provider_module_import_path: native_provider_artifact
                    .native_provider_manifest_path_for_prepared_runtime_owner_v1()
                    .to_owned(),
                provider_module_export,
                native_provider_artifact,
            },
        })
    }

    fn require_source_materialization_owner(&self, operation: &'static str) -> Result<(), String> {
        if matches!(
            self.kind,
            ImplementationSelectionOwnerKind::SourceMaterializationPackageResolution
        ) {
            Ok(())
        } else {
            Err(format!(
                "{} owner cannot select package graph {operation}",
                self.kind.label()
            ))
        }
    }
}

impl PackageGraphExactProviderImportSelectionBinderForPreparedRuntimeOwnerV1
    for PackageGraphExactProviderImportSelectionBinderForPreparedRuntimeImageOwnerV1
{
    type BoundArtifact = Result<PreparedRuntimeImageProviderImportIdentity, String>;

    fn consume_with_package_validated_import_for_prepared_runtime_owner_v1(
        self,
        contract_source: libswarm_package_graph_contract_source_admission::AdmittedPackageGraphContractSource,
        operation: PackageGraphExactProviderOperationForPreparedRuntimeOwnerV1,
        selected_operation: PackageGraphProviderImportOperationSelectionForPreparedRuntimeOwnerV1,
    ) -> Self::BoundArtifact {
        ImplementationSelectionOwner::admit_exact_package_graph_provider_operation_selection_for_source_materialization_owner_v1(
            contract_source,
            operation,
            selected_operation,
        )
    }
}

impl PackageGraphProviderImportOperationSelectionBinderForPreparedRuntimeOwnerV1
    for PackageGraphProviderOperationSelectionBinderForPreparedRuntimeImageOwnerV1
{
    type BoundArtifact = Result<PreparedRuntimeImageProviderImportIdentity, String>;

    fn consume_with_package_graph_provider_operation_for_prepared_runtime_owner_v1(
        self,
        provider_module_import_path: String,
        provider_module_export: String,
    ) -> Self::BoundArtifact {
        PreparedRuntimeImageProviderImportIdentity::admit_selected_implementation_target_v1(
            SelectedImplementationTarget {
                contract: self.contract,
                kind: SelectedImplementationTargetKind::ExternalOperation {
                    operation: self.operation,
                    provider_module_import_path,
                    provider_module_export,
                },
            },
        )
    }
}

pub struct PreparedRuntimeImageProviderImportIdentity {
    identity_key: String,
    contract: CapabilityContractProjection,
    routes: PreparedRuntimeImageProviderImportRoutes,
}

/// The complete non-empty package-selected provider-import authority for one
/// prepared runtime image. The private registry key is only an owner index;
/// callers can neither supply it nor recover the map.
///
/// ```compile_fail
/// let _ = swarmvm_image::prepared_runtime_artifact_owner::PreparedRuntimeProviderImportAuthoritySet {
///     provider_imports: std::collections::BTreeMap::new(),
/// };
/// ```
// compiler-custody: symbol=PreparedRuntimeProviderImportAuthoritySet disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: pass the compiler root scope into admit_package_selected_batch_for_prepared_runtime_owner_v1 and carry the ticket to execution-start admission"
#[must_use = "the prepared-runtime provider-import authority set must remain in image custody until execution-start admission"]
pub struct PreparedRuntimeProviderImportAuthoritySet {
    provider_imports: BTreeMap<String, PreparedRuntimeImageProviderImportIdentity>,
}

#[derive(Debug, Clone, Copy)]
enum PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason {
    EmptySelection,
    RegistryKeyDrift,
    ExactContractConflict,
    ExternalOperationTargetConflict,
    NativeContractTargetConflict,
    ProviderExecutionDomainConflict,
}

/// Batch admission fails before commit and retains every selected identity.
pub struct PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
    selected_inputs: Vec<PreparedRuntimeImageProviderImportIdentity>,
    reason: PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason,
}

/// A by-value merge fault retains both complete sealed sets.
pub struct PreparedRuntimeProviderImportAuthoritySetMergeFault {
    prior: PreparedRuntimeProviderImportAuthoritySet,
    incoming: PreparedRuntimeProviderImportAuthoritySet,
    reason: PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason,
}

// compiler-custody: symbol=PreparedRuntimeProviderImportAuthoritySetCancellationCustody disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: carry the authority-set ticket into both admission and merge cancellation variants, then cancel it at the finite cancellation owner"
enum PreparedRuntimeProviderImportAuthoritySetCancellationCustody {
    Admission(Vec<PreparedRuntimeImageProviderImportIdentity>),
    Merge {
        prior: PreparedRuntimeProviderImportAuthoritySet,
        incoming: PreparedRuntimeProviderImportAuthoritySet,
    },
}

/// Explicit cancellation consumes, but does not project, selected authority.
// compiler-custody: symbol=PreparedRuntimeProviderImportAuthoritySetCancellation disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: replace deliberate cancellation drop with a root-scoped ticket cancel in the compiler owner that consumes this cancellation"
#[must_use = "provider-import cancellation owns selected work until the caller deliberately drops the cancellation"]
pub struct PreparedRuntimeProviderImportAuthoritySetCancellation {
    _custody: PreparedRuntimeProviderImportAuthoritySetCancellationCustody,
}

/// Final manifest observation. It is deliberately incapable of producing an
/// authority set or an execution-start admission.
pub struct PreparedRuntimeProviderImportManifestObservation {
    projection: serde_json::Value,
    identity_fingerprint: String,
    provider_import_count: usize,
    operation_route_count: usize,
}

enum PreparedRuntimeImageProviderImportRoutes {
    ExternalOperations(
        BTreeMap<
            AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
            PreparedRuntimeExternalProviderOperationTarget,
        >,
    ),
    NativeContract(PreparedRuntimeNativeProviderContractTarget),
}

#[derive(PartialEq, Eq)]
struct PreparedRuntimeExternalProviderOperationTarget {
    provider_module_import_path: String,
    provider_module_export: String,
}

#[derive(PartialEq, Eq)]
struct PreparedRuntimeNativeProviderContractTarget {
    provider_module_import_path: String,
    provider_module_export: String,
    native_provider_artifact: AdmittedPackageGraphNativeProviderArtifact,
}

// compiler-custody: symbol=PreparedRuntimeProviderImportExecutionStartAdmissionSet disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="first edit: move the authority-set ticket through into_execution_start_admission_for_direct_run_owner_v1 and settle it in provider_host_set admission"
#[must_use = "prepared-runtime provider imports must remain correlated through capability-link admission and external route installation"]
pub struct PreparedRuntimeProviderImportExecutionStartAdmissionSet {
    external_routes: Option<ManifestResolvedExternalProviderRouteSet>,
}

#[must_use = "a manifest-resolved external route must remain sealed until it selects one exact provider call"]
pub struct ManifestResolvedExternalProviderRoute {
    contract: CapabilityContractProjection,
    operation: AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
    provider_module_import_path: String,
    provider_module_export: String,
}

// compiler-custody: symbol=ManifestResolvedExternalProviderCallAuthority disposition=runtime-only reviewer=package-root-compiler-kernel-hardcut-20260722 justification="post-close-mint=Self::select_exact_call_for_provider_host_set_owner_v1; consumer=Self::contract_for_provider_host_set_owner_v1; publication=SourceEntrypointDirectRunPreparedRuntime; compiler-product-or-ticket-crossing=none"
#[must_use = "a manifest-resolved external call authority is move-only and must be consumed by the durable external provider"]
pub struct ManifestResolvedExternalProviderCallAuthority {
    contract: swarm_capability_model::CapabilityContractIdentity,
    _operation: AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
    provider_module_import_path: String,
    provider_module_export: String,
}

#[must_use = "an admitted manifest-resolved external call must be consumed together with its exact admitted Contract-TSON"]
pub struct ManifestResolvedExternalProviderCallAdmission {
    call_authority: ManifestResolvedExternalProviderCallAuthority,
    contract: swarm_capability_contract_tson::AdmittedCapabilityContractTson,
}

#[must_use = "manifest-resolved external route selection must consume the admitted branch or retain the complete unmatched Contract-TSON"]
pub enum ManifestResolvedExternalProviderCallAdmissionSelection {
    Admitted(ManifestResolvedExternalProviderCallAdmission),
    Unmatched(swarm_capability_contract_tson::AdmittedCapabilityContractTson),
}

#[derive(Debug)]
pub struct ManifestResolvedExternalProviderCallAdmissionSelectionFault {
    original_contract: swarm_capability_contract_tson::AdmittedCapabilityContractTson,
    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason,
}

#[derive(Debug)]
enum ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason {
    ContractFingerprintMismatch {
        expected_fingerprint: Option<String>,
        observed_fingerprint: Option<String>,
    },
    ExactContractOperationMismatch {
        admitted_operation_route_count: usize,
    },
}

struct ManifestResolvedExternalProviderRouteSet {
    routes: Vec<ManifestResolvedExternalProviderRoute>,
}

impl PreparedRuntimeImageProviderImportIdentity {
    pub(crate) fn admit_selected_implementation_target_v1(
        target: SelectedImplementationTarget,
    ) -> Result<Self, String> {
        swarm_capability_model::validate_contract_projection(&target.contract)
            .map_err(|error| error.to_string())?;
        let identity_key =
            provider_import_identity_key_for_prepared_runtime_artifact_owner_v1(&target.contract)?;
        let routes = match target.kind {
            SelectedImplementationTargetKind::ExternalOperation {
                operation,
                provider_module_import_path,
                provider_module_export,
            } => {
                require_non_empty(
                    &provider_module_import_path,
                    "selected external provider operation module path",
                )?;
                require_non_empty(
                    &provider_module_export,
                    "selected external provider operation module export",
                )?;
                if target.contract.fingerprint().is_none() {
                    return Err(
                        "external transport provider operation requires an exact contract fingerprint"
                            .to_owned(),
                    );
                }
                PreparedRuntimeImageProviderImportRoutes::ExternalOperations(BTreeMap::from([(
                    operation,
                    PreparedRuntimeExternalProviderOperationTarget {
                        provider_module_import_path,
                        provider_module_export,
                    },
                )]))
            }
            SelectedImplementationTargetKind::NativeContract {
                provider_module_import_path,
                provider_module_export,
                native_provider_artifact,
            } => {
                require_non_empty(
                    &provider_module_import_path,
                    "selected native provider module path",
                )?;
                require_non_empty(
                    &provider_module_export,
                    "selected native provider module export",
                )?;
                PreparedRuntimeImageProviderImportRoutes::NativeContract(
                    PreparedRuntimeNativeProviderContractTarget {
                        provider_module_import_path,
                        provider_module_export,
                        native_provider_artifact,
                    },
                )
            }
        };
        Ok(Self {
            identity_key,
            contract: target.contract,
            routes,
        })
    }

    pub(crate) fn contract(&self) -> &CapabilityContractProjection {
        &self.contract
    }

    fn provider_execution_domain(&self) -> &'static str {
        match &self.routes {
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(_) => {
                swarm_capability_model::EXTERNAL_TRANSPORT_PROVIDER_DOMAIN
            }
            PreparedRuntimeImageProviderImportRoutes::NativeContract(_) => {
                swarm_capability_model::LOADED_NATIVE_PROVIDER_HOST_KIND
            }
        }
    }

    pub fn observe_for_direct_run_prepared_runtime_manifest_metadata_owner_v1(
        &self,
    ) -> serde_json::Value {
        let (external_operation_routes, native_provider_artifact) = match &self.routes {
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(routes) => (
                routes
                    .iter()
                    .map(|(operation, target)| {
                        serde_json::json!({
                            "operationKind": operation_kind_label(operation.kind_for_prepared_runtime_artifact_owner_v1()),
                            "operationName": operation.operation_name_for_prepared_runtime_artifact_owner_v1(),
                            "providerModuleImportPath": target.provider_module_import_path,
                            "providerModuleExport": target.provider_module_export,
                        })
                    })
                    .collect::<Vec<_>>(),
                None,
            ),
            PreparedRuntimeImageProviderImportRoutes::NativeContract(target) => (
                Vec::new(),
                Some(serde_json::json!({
                    "providerModuleImportPath": target.provider_module_import_path,
                    "providerModuleExport": target.provider_module_export,
                    "packageLabel": target.native_provider_artifact.package_label(),
                    "packageRoot": target.native_provider_artifact.package_root_for_prepared_runtime_owner_v1(),
                    "nativeProviderManifestPath": target.native_provider_artifact.native_provider_manifest_path_for_prepared_runtime_owner_v1(),
                    "packageGraphManifestFingerprint": target.native_provider_artifact.package_graph_manifest_fingerprint_for_prepared_runtime_owner_v1(),
                })),
            ),
        };
        serde_json::json!({
            "contract": {
                "packageSpecifier": self.contract.package_specifier(),
                "exportName": self.contract.export_name(),
                "fingerprint": self.contract.fingerprint().map(ToString::to_string),
            },
            "providerExecutionDomain": self.provider_execution_domain(),
            "externalOperationRoutes": external_operation_routes,
            "nativeProviderArtifact": native_provider_artifact,
        })
    }

    pub(crate) fn same_prepared_runtime_provider_import_for_source_materialization_owner_v1(
        &self,
        other: &Self,
    ) -> bool {
        self.identity_key == other.identity_key
            && self.contract == other.contract
            && self.routes.same_routes(&other.routes)
    }

    fn merge_exact_contract_routes(&mut self, incoming: Self) -> Result<(), String> {
        if self.identity_key != incoming.identity_key || self.contract != incoming.contract {
            return Err(
                "prepared-runtime provider import identity key collides across distinct exact contracts"
                    .to_owned(),
            );
        }
        self.routes.merge_exact_contract_routes(incoming.routes)
    }
}

impl PreparedRuntimeImageProviderImportRoutes {
    fn same_routes(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ExternalOperations(left), Self::ExternalOperations(right)) => left == right,
            (Self::NativeContract(left), Self::NativeContract(right)) => left == right,
            _ => false,
        }
    }

    fn merge_exact_contract_routes(&mut self, incoming: Self) -> Result<(), String> {
        match (self, incoming) {
            (Self::ExternalOperations(existing), Self::ExternalOperations(incoming)) => {
                for (operation, target) in &incoming {
                    if let Some(existing_target) = existing.get(operation) {
                        if existing_target != target {
                            return Err(
                                "prepared-runtime external provider operation has conflicting implementation targets"
                                    .to_owned(),
                            );
                        }
                    }
                }
                existing.extend(incoming);
                Ok(())
            }
            (Self::NativeContract(existing), Self::NativeContract(incoming))
                if existing == &incoming =>
            {
                Ok(())
            }
            (Self::NativeContract(_), Self::NativeContract(_)) => Err(
                "prepared-runtime native provider contract has conflicting implementation targets"
                    .to_owned(),
            ),
            _ => Err(
                "prepared-runtime provider contract cannot mix external-operation and native-contract routes"
                    .to_owned(),
            ),
        }
    }
}

impl PreparedRuntimeProviderImportAuthoritySet {
    pub fn admit_package_selected_batch_for_prepared_runtime_owner_v1(
        selected_inputs: Vec<PreparedRuntimeImageProviderImportIdentity>,
    ) -> Result<Self, PreparedRuntimeProviderImportAuthoritySetAdmissionFault> {
        if selected_inputs.is_empty() {
            return Err(PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
                selected_inputs,
                reason:
                    PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::EmptySelection,
            });
        }
        if let Err(reason) =
            preflight_prepared_runtime_provider_import_identity_batch(selected_inputs.as_slice())
        {
            return Err(PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
                selected_inputs,
                reason,
            });
        }
        Ok(Self {
            provider_imports: commit_preflighted_prepared_runtime_provider_import_identity_batch(
                selected_inputs,
            ),
        })
    }

    pub fn merge_for_prepared_runtime_image_owner_v1(
        self,
        incoming: Self,
    ) -> Result<Self, PreparedRuntimeProviderImportAuthoritySetMergeFault> {
        if let Err(reason) = preflight_prepared_runtime_provider_import_identity_sets(
            &self.provider_imports,
            &incoming.provider_imports,
        ) {
            return Err(PreparedRuntimeProviderImportAuthoritySetMergeFault {
                prior: self,
                incoming,
                reason,
            });
        }
        let mut selected_inputs = self.provider_imports.into_values().collect::<Vec<_>>();
        selected_inputs.extend(incoming.provider_imports.into_values());
        Ok(Self {
            provider_imports: commit_preflighted_prepared_runtime_provider_import_identity_batch(
                selected_inputs,
            ),
        })
    }

    pub fn observe_for_prepared_runtime_manifest_owner_v1(
        &self,
    ) -> Result<PreparedRuntimeProviderImportManifestObservation, String> {
        let provider_imports = self
            .provider_imports
            .values()
            .map(PreparedRuntimeImageProviderImportIdentity::observe_for_direct_run_prepared_runtime_manifest_metadata_owner_v1)
            .collect::<Vec<_>>();
        let operation_route_count = self
            .provider_imports
            .values()
            .map(|provider_import| match &provider_import.routes {
                PreparedRuntimeImageProviderImportRoutes::ExternalOperations(routes) => {
                    routes.len()
                }
                PreparedRuntimeImageProviderImportRoutes::NativeContract(_) => 1,
            })
            .sum();
        let identity_fingerprint =
            provider_import_identity_fingerprint_for_prepared_runtime_artifact_owner_v1(
                &self.provider_imports,
            )?;
        let provider_import_count = self.provider_imports.len();
        Ok(PreparedRuntimeProviderImportManifestObservation {
            projection: serde_json::json!({
                "schema": "swarm.direct_run.prepared_runtime_manifest_metadata.provider_import_observation.v1",
                "identityFingerprint": identity_fingerprint,
                "providerImportCount": provider_import_count,
                "operationRouteCount": operation_route_count,
                "providerImports": provider_imports,
            }),
            identity_fingerprint,
            provider_import_count,
            operation_route_count,
        })
    }

    pub fn into_execution_start_admission_for_direct_run_owner_v1(
        self,
    ) -> PreparedRuntimeProviderImportExecutionStartAdmissionSet {
        let mut external_routes = Vec::new();
        for provider_import in self.provider_imports.into_values() {
            let PreparedRuntimeImageProviderImportIdentity {
                identity_key: _,
                contract,
                routes,
            } = provider_import;
            if let PreparedRuntimeImageProviderImportRoutes::ExternalOperations(routes) = routes {
                external_routes.extend(routes.into_iter().map(|(operation, target)| {
                    ManifestResolvedExternalProviderRoute {
                        contract: contract.duplicate_for_prepared_runtime_projection_owner_v1(),
                        operation,
                        provider_module_import_path: target.provider_module_import_path,
                        provider_module_export: target.provider_module_export,
                    }
                }));
            }
        }
        PreparedRuntimeProviderImportExecutionStartAdmissionSet {
            external_routes: (!external_routes.is_empty()).then_some(
                ManifestResolvedExternalProviderRouteSet {
                    routes: external_routes,
                },
            ),
        }
    }
}

fn preflight_prepared_runtime_provider_import_identity_batch(
    selected_inputs: &[PreparedRuntimeImageProviderImportIdentity],
) -> Result<(), PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason> {
    for provider_import in selected_inputs {
        let expected_key = provider_import_identity_key_for_prepared_runtime_artifact_owner_v1(
            &provider_import.contract,
        )
        .map_err(|_| {
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::RegistryKeyDrift
        })?;
        if expected_key != provider_import.identity_key {
            return Err(
                PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::RegistryKeyDrift,
            );
        }
    }
    for (index, prior) in selected_inputs.iter().enumerate() {
        for incoming in selected_inputs.iter().skip(index + 1) {
            preflight_prepared_runtime_provider_import_identity_pair(prior, incoming)?;
        }
    }
    Ok(())
}

fn preflight_prepared_runtime_provider_import_identity_sets(
    prior: &BTreeMap<String, PreparedRuntimeImageProviderImportIdentity>,
    incoming: &BTreeMap<String, PreparedRuntimeImageProviderImportIdentity>,
) -> Result<(), PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason> {
    for (prior_key, prior_identity) in prior {
        if prior_key != &prior_identity.identity_key {
            return Err(
                PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::RegistryKeyDrift,
            );
        }
        for incoming_identity in incoming.values() {
            preflight_prepared_runtime_provider_import_identity_pair(
                prior_identity,
                incoming_identity,
            )?;
        }
    }
    for (incoming_key, incoming_identity) in incoming {
        if incoming_key != &incoming_identity.identity_key {
            return Err(
                PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::RegistryKeyDrift,
            );
        }
    }
    Ok(())
}

fn preflight_prepared_runtime_provider_import_identity_pair(
    prior: &PreparedRuntimeImageProviderImportIdentity,
    incoming: &PreparedRuntimeImageProviderImportIdentity,
) -> Result<(), PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason> {
    if prior.identity_key != incoming.identity_key {
        return Ok(());
    }
    if prior.contract != incoming.contract {
        return Err(
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::ExactContractConflict,
        );
    }
    match (&prior.routes, &incoming.routes) {
        (
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(prior_routes),
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(incoming_routes),
        ) => {
            for (operation, incoming_target) in incoming_routes {
                if prior_routes
                    .get(operation)
                    .is_some_and(|prior_target| prior_target != incoming_target)
                {
                    return Err(PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::ExternalOperationTargetConflict);
                }
            }
            Ok(())
        }
        (
            PreparedRuntimeImageProviderImportRoutes::NativeContract(prior_target),
            PreparedRuntimeImageProviderImportRoutes::NativeContract(incoming_target),
        ) if prior_target == incoming_target => Ok(()),
        (
            PreparedRuntimeImageProviderImportRoutes::NativeContract(_),
            PreparedRuntimeImageProviderImportRoutes::NativeContract(_),
        ) => Err(
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::NativeContractTargetConflict,
        ),
        _ => Err(
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::ProviderExecutionDomainConflict,
        ),
    }
}

fn commit_preflighted_prepared_runtime_provider_import_identity_batch(
    selected_inputs: Vec<PreparedRuntimeImageProviderImportIdentity>,
) -> BTreeMap<String, PreparedRuntimeImageProviderImportIdentity> {
    use std::collections::btree_map::Entry;

    let mut provider_imports = BTreeMap::new();
    for provider_import in selected_inputs {
        match provider_imports.entry(provider_import.identity_key.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(provider_import);
            }
            Entry::Occupied(mut entry) => {
                merge_preflighted_prepared_runtime_provider_import_identity(
                    entry.get_mut(),
                    provider_import,
                );
            }
        }
    }
    provider_imports
}

fn merge_preflighted_prepared_runtime_provider_import_identity(
    prior: &mut PreparedRuntimeImageProviderImportIdentity,
    incoming: PreparedRuntimeImageProviderImportIdentity,
) {
    match (&mut prior.routes, incoming.routes) {
        (
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(prior_routes),
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(incoming_routes),
        ) => prior_routes.extend(incoming_routes),
        (
            PreparedRuntimeImageProviderImportRoutes::NativeContract(prior_target),
            PreparedRuntimeImageProviderImportRoutes::NativeContract(incoming_target),
        ) => {
            assert!(
                *prior_target == incoming_target,
                "prepared-runtime provider-import commit received native targets that were not preflighted"
            );
        }
        (
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(_),
            PreparedRuntimeImageProviderImportRoutes::NativeContract(_),
        )
        | (
            PreparedRuntimeImageProviderImportRoutes::NativeContract(_),
            PreparedRuntimeImageProviderImportRoutes::ExternalOperations(_),
        ) => {
            unreachable!(
                "prepared-runtime provider-import commit received execution domains that were not preflighted"
            );
        }
    }
}

impl PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
    pub fn retry(self) -> Result<PreparedRuntimeProviderImportAuthoritySet, Self> {
        PreparedRuntimeProviderImportAuthoritySet::admit_package_selected_batch_for_prepared_runtime_owner_v1(
            self.selected_inputs,
        )
    }

    pub fn cancel(self) -> PreparedRuntimeProviderImportAuthoritySetCancellation {
        PreparedRuntimeProviderImportAuthoritySetCancellation {
            _custody: PreparedRuntimeProviderImportAuthoritySetCancellationCustody::Admission(
                self.selected_inputs,
            ),
        }
    }
}

impl PreparedRuntimeProviderImportAuthoritySetMergeFault {
    pub fn retry(self) -> Result<PreparedRuntimeProviderImportAuthoritySet, Self> {
        self.prior
            .merge_for_prepared_runtime_image_owner_v1(self.incoming)
    }

    pub fn cancel(self) -> PreparedRuntimeProviderImportAuthoritySetCancellation {
        PreparedRuntimeProviderImportAuthoritySetCancellation {
            _custody: PreparedRuntimeProviderImportAuthoritySetCancellationCustody::Merge {
                prior: self.prior,
                incoming: self.incoming,
            },
        }
    }
}

impl PreparedRuntimeProviderImportManifestObservation {
    pub fn projection_for_direct_run_manifest_metadata_owner_v1(&self) -> &serde_json::Value {
        &self.projection
    }

    pub fn identity_fingerprint_for_direct_run_manifest_metadata_owner_v1(&self) -> &str {
        &self.identity_fingerprint
    }

    pub fn provider_import_count_for_direct_run_manifest_metadata_owner_v1(&self) -> usize {
        self.provider_import_count
    }

    pub fn operation_route_count_for_direct_run_manifest_metadata_owner_v1(&self) -> usize {
        self.operation_route_count
    }
}

impl std::fmt::Display for PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "prepared-runtime provider-import batch admission refused: {:?}",
            self.reason
        )
    }
}

impl std::fmt::Debug for PreparedRuntimeProviderImportAuthoritySetAdmissionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRuntimeProviderImportAuthoritySetAdmissionFault")
            .field("reason", &self.reason)
            .field("retained_selected_input_count", &self.selected_inputs.len())
            .finish()
    }
}

impl std::error::Error for PreparedRuntimeProviderImportAuthoritySetAdmissionFault {}

impl std::fmt::Display for PreparedRuntimeProviderImportAuthoritySetMergeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "prepared-runtime provider-import set merge refused: {:?}",
            self.reason
        )
    }
}

impl std::fmt::Debug for PreparedRuntimeProviderImportAuthoritySetMergeFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRuntimeProviderImportAuthoritySetMergeFault")
            .field("reason", &self.reason)
            .finish_non_exhaustive()
    }
}

impl std::error::Error for PreparedRuntimeProviderImportAuthoritySetMergeFault {}

impl PreparedRuntimeProviderImportExecutionStartAdmissionSet {
    pub fn select_exact_external_call_for_provider_host_set_owner_v1(
        &self,
        contract: swarm_capability_contract_tson::AdmittedCapabilityContractTson,
    ) -> Result<
        ManifestResolvedExternalProviderCallAdmissionSelection,
        ManifestResolvedExternalProviderCallAdmissionSelectionFault,
    > {
        match &self.external_routes {
            Some(routes) => routes.select_exact_call_for_provider_host_set_owner_v1(contract),
            None => Ok(ManifestResolvedExternalProviderCallAdmissionSelection::Unmatched(contract)),
        }
    }
}

impl ManifestResolvedExternalProviderRouteSet {
    fn select_exact_call_for_provider_host_set_owner_v1(
        &self,
        contract: swarm_capability_contract_tson::AdmittedCapabilityContractTson,
    ) -> Result<
        ManifestResolvedExternalProviderCallAdmissionSelection,
        ManifestResolvedExternalProviderCallAdmissionSelectionFault,
    > {
        let contract_identity = contract.identity().duplicate_for_capability_model_owner();
        let exact_routes = self
            .routes
            .iter()
            .filter(|route| {
                route.contract.package_specifier() == contract_identity.package_specifier()
                    && route.contract.export_name() == contract_identity.export_name()
                    && route.contract.fingerprint()
                        == contract_identity
                            .fingerprint()
                            .map(|fingerprint| fingerprint.as_str())
            })
            .collect::<Vec<_>>();
        if !exact_routes.is_empty() {
            let mut unmatched = contract;
            for route in &exact_routes {
                unmatched = match unmatched
                    .try_join_exact_operation_for_provider_route_owner_v1(&route.operation)
                {
                    AdmittedCapabilityContractOperationExactJoinForProviderRouteOwnerV1::Joined(
                        contract,
                    ) => {
                        return Ok(
                            ManifestResolvedExternalProviderCallAdmissionSelection::Admitted(
                                ManifestResolvedExternalProviderCallAdmission {
                                    call_authority: ManifestResolvedExternalProviderCallAuthority {
                                        contract: contract_identity,
                                        _operation: route
                                            .operation
                                            .duplicate_for_prepared_runtime_provider_route_owner_v1(),
                                        provider_module_import_path: route
                                            .provider_module_import_path
                                            .clone(),
                                        provider_module_export: route
                                            .provider_module_export
                                            .clone(),
                                    },
                                    contract,
                                },
                            ),
                        );
                    }
                    AdmittedCapabilityContractOperationExactJoinForProviderRouteOwnerV1::Unmatched(
                        contract,
                    ) => contract,
                };
            }
            return Err(
                ManifestResolvedExternalProviderCallAdmissionSelectionFault {
                    original_contract: unmatched,
                    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ExactContractOperationMismatch {
                        admitted_operation_route_count: exact_routes.len(),
                    },
                },
            );
        }
        if let Some(route) = self.routes.iter().find(|route| {
            route.contract.package_specifier() == contract_identity.package_specifier()
                && route.contract.export_name() == contract_identity.export_name()
        }) {
            return Err(
                ManifestResolvedExternalProviderCallAdmissionSelectionFault {
                    original_contract: contract,
                    reason: ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ContractFingerprintMismatch {
                        expected_fingerprint: route.contract.fingerprint().map(str::to_owned),
                        observed_fingerprint: contract_identity
                            .fingerprint()
                            .map(|fingerprint| fingerprint.as_str().to_owned()),
                    },
                },
            );
        }
        Ok(ManifestResolvedExternalProviderCallAdmissionSelection::Unmatched(contract))
    }
}

impl ManifestResolvedExternalProviderCallAdmission {
    pub fn into_call_authority_and_contract_for_provider_host_set_owner_v1(
        self,
    ) -> (
        ManifestResolvedExternalProviderCallAuthority,
        swarm_capability_contract_tson::AdmittedCapabilityContractTson,
    ) {
        (self.call_authority, self.contract)
    }
}

impl ManifestResolvedExternalProviderCallAdmissionSelectionFault {
    pub fn into_original_contract_for_provider_host_set_owner_v1(
        self,
    ) -> swarm_capability_contract_tson::AdmittedCapabilityContractTson {
        self.original_contract
    }
}

impl std::fmt::Display for ManifestResolvedExternalProviderCallAdmissionSelectionFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ContractFingerprintMismatch {
                expected_fingerprint,
                observed_fingerprint,
            } => write!(
                formatter,
                "manifest-resolved external provider contract fingerprint mismatch for {}:{}: expected {}, observed {}",
                self.original_contract.identity().package_specifier(),
                self.original_contract.identity().export_name(),
                expected_fingerprint.as_deref().unwrap_or("<absent>"),
                observed_fingerprint.as_deref().unwrap_or("<absent>"),
            ),
            ManifestResolvedExternalProviderCallAdmissionSelectionFaultReason::ExactContractOperationMismatch {
                admitted_operation_route_count,
            } => write!(
                formatter,
                "manifest-resolved external provider exact contract {}:{} has no route for the admitted Contract-TSON operation across {admitted_operation_route_count} operation routes",
                self.original_contract.identity().package_specifier(),
                self.original_contract.identity().export_name(),
            ),
        }
    }
}

impl std::error::Error for ManifestResolvedExternalProviderCallAdmissionSelectionFault {}

impl ManifestResolvedExternalProviderCallAuthority {
    pub fn contract_for_provider_host_set_owner_v1(
        &self,
    ) -> &swarm_capability_model::CapabilityContractIdentity {
        &self.contract
    }

    pub fn into_contract_and_module_for_durable_external_provider_owner_v1(
        self,
    ) -> (
        swarm_capability_model::CapabilityContractIdentity,
        String,
        String,
    ) {
        (
            self.contract,
            self.provider_module_import_path,
            self.provider_module_export,
        )
    }
}

impl std::fmt::Debug for ManifestResolvedExternalProviderRoute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ManifestResolvedExternalProviderRoute")
            .field("contract", &self.contract)
            .field("operation", &self.operation)
            .field("hidden_external_route_authority", &"redacted")
            .finish()
    }
}

impl std::fmt::Debug for ManifestResolvedExternalProviderCallAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ManifestResolvedExternalProviderCallAuthority")
            .field("contract", &self.contract)
            .field("hidden_external_call_authority", &"redacted")
            .finish()
    }
}

impl std::fmt::Debug for ManifestResolvedExternalProviderRouteSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ManifestResolvedExternalProviderRouteSet")
            .field("route_count", &self.routes.len())
            .field("hidden_external_route_authority", &"redacted")
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedRuntimeProviderImportIdentityArtifactSidecarWire {
    schema: String,
    provider_imports: BTreeMap<String, PreparedRuntimeProviderImportIdentityArtifactWire>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedRuntimeProviderImportIdentityArtifactWire {
    contract: CapabilityContractProjection,
    routes: PreparedRuntimeProviderImportRoutesArtifactWire,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "routeKind", rename_all = "snake_case", deny_unknown_fields)]
enum PreparedRuntimeProviderImportRoutesArtifactWire {
    ExternalOperations {
        operations: Vec<PreparedRuntimeExternalProviderOperationRouteArtifactWire>,
    },
    NativeContract {
        provider_module_import_path: String,
        provider_module_export: String,
        native_provider_artifact: PreparedRuntimeNativeProviderArtifactWire,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedRuntimeExternalProviderOperationRouteArtifactWire {
    operation_kind: PreparedRuntimeProviderOperationKindArtifactWire,
    operation_name: String,
    provider_module_import_path: String,
    provider_module_export: String,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PreparedRuntimeProviderOperationKindArtifactWire {
    Command,
    InteractionOpen,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PreparedRuntimeNativeProviderArtifactWire {
    package_label: String,
    package_root: String,
    native_provider_manifest_path: String,
    package_graph_manifest_fingerprint: String,
}

/// Sealed write-half sidecar segment for the direct-run prepared-runtime
/// artifact publish chain. Derives entirely from admitted provider-import
/// identities the image owner already holds — no caller-supplied bytes. The
/// fingerprint is minted by the same projection the artifact reader
/// recomputes at admission, so a published segment verifies by construction.
pub(crate) struct PreparedRuntimeProviderImportIdentityArtifactSidecarSegmentForDirectRunPreparedRuntimeOwnerV1
{
    dag_cbor_bytes: Vec<u8>,
    fingerprint: String,
    provider_import_count: usize,
}

impl PreparedRuntimeProviderImportIdentityArtifactSidecarSegmentForDirectRunPreparedRuntimeOwnerV1 {
    pub(crate) fn fingerprint_for_direct_run_prepared_runtime_owner_v1(&self) -> &str {
        &self.fingerprint
    }

    pub(crate) fn provider_import_count_for_direct_run_prepared_runtime_owner_v1(&self) -> usize {
        self.provider_import_count
    }

    pub(crate) fn into_dag_cbor_payload_bytes_for_direct_run_prepared_runtime_owner_v1(
        self,
    ) -> Vec<u8> {
        self.dag_cbor_bytes
    }
}

pub(crate) fn provider_import_identity_artifact_sidecar_segment_for_direct_run_prepared_runtime_owner_v1(
    provider_imports: &PreparedRuntimeProviderImportAuthoritySet,
) -> Result<
    PreparedRuntimeProviderImportIdentityArtifactSidecarSegmentForDirectRunPreparedRuntimeOwnerV1,
    String,
> {
    let dag_cbor_bytes =
        provider_import_identity_artifact_sidecar_dag_cbor_payload_bytes_for_prepared_runtime_artifact_owner_v1(
            &provider_imports.provider_imports,
        )?;
    let fingerprint = provider_import_identity_fingerprint_for_prepared_runtime_artifact_owner_v1(
        &provider_imports.provider_imports,
    )?;
    Ok(
        PreparedRuntimeProviderImportIdentityArtifactSidecarSegmentForDirectRunPreparedRuntimeOwnerV1 {
            dag_cbor_bytes,
            fingerprint,
            provider_import_count: provider_imports.provider_imports.len(),
        },
    )
}

fn provider_import_identity_artifact_sidecar_dag_cbor_payload_bytes_for_prepared_runtime_artifact_owner_v1(
    provider_imports: &BTreeMap<String, PreparedRuntimeImageProviderImportIdentity>,
) -> Result<Vec<u8>, String> {
    let wire = PreparedRuntimeProviderImportIdentityArtifactSidecarWire {
        schema: DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_PROVIDER_IMPORT_IDENTITY_SIDECAR_SCHEMA
            .to_owned(),
        provider_imports: provider_imports
            .iter()
            .map(|(identity_key, provider_import)| {
                Ok((
                    identity_key.clone(),
                    provider_import_identity_artifact_wire_from_admitted_identity(
                        identity_key,
                        provider_import,
                    )?,
                ))
            })
            .collect::<Result<_, String>>()?,
    };
    serde_ipld_dagcbor::to_vec(&wire).map_err(|error| {
        format!(
            "prepared-runtime artifact provider-import identity sidecar DAG-CBOR encode failed: {error}"
        )
    })
}

fn provider_import_identity_artifact_wire_from_admitted_identity(
    identity_key: &str,
    provider_import: &PreparedRuntimeImageProviderImportIdentity,
) -> Result<PreparedRuntimeProviderImportIdentityArtifactWire, String> {
    require_provider_import_identity_key_matches_contract_for_prepared_runtime_artifact_owner_v1(
        identity_key,
        provider_import.contract(),
    )?;
    let routes = match &provider_import.routes {
        PreparedRuntimeImageProviderImportRoutes::ExternalOperations(routes) => {
            PreparedRuntimeProviderImportRoutesArtifactWire::ExternalOperations {
                operations: routes
                    .iter()
                    .map(|(operation, target)| {
                        PreparedRuntimeExternalProviderOperationRouteArtifactWire {
                            operation_kind: operation_kind_artifact_wire(
                                operation.kind_for_prepared_runtime_artifact_owner_v1(),
                            ),
                            operation_name: operation
                                .operation_name_for_prepared_runtime_artifact_owner_v1()
                                .to_owned(),
                            provider_module_import_path: target.provider_module_import_path.clone(),
                            provider_module_export: target.provider_module_export.clone(),
                        }
                    })
                    .collect(),
            }
        }
        PreparedRuntimeImageProviderImportRoutes::NativeContract(target) => {
            PreparedRuntimeProviderImportRoutesArtifactWire::NativeContract {
                provider_module_import_path: target.provider_module_import_path.clone(),
                provider_module_export: target.provider_module_export.clone(),
                native_provider_artifact: PreparedRuntimeNativeProviderArtifactWire {
                    package_label: target.native_provider_artifact.package_label().to_owned(),
                    package_root: target
                        .native_provider_artifact
                        .package_root_for_prepared_runtime_owner_v1()
                        .to_owned(),
                    native_provider_manifest_path: target
                        .native_provider_artifact
                        .native_provider_manifest_path_for_prepared_runtime_owner_v1()
                        .to_owned(),
                    package_graph_manifest_fingerprint: target
                        .native_provider_artifact
                        .package_graph_manifest_fingerprint_for_prepared_runtime_owner_v1()
                        .to_owned(),
                },
            }
        }
    };
    Ok(PreparedRuntimeProviderImportIdentityArtifactWire {
        contract: provider_import
            .contract()
            .duplicate_for_prepared_runtime_projection_owner_v1(),
        routes,
    })
}

fn admit_provider_import_identity_sidecar_for_prepared_runtime_artifact_owner_v1(
    payload_bytes: &[u8],
    expected_fingerprint: &str,
    expected_count: usize,
) -> Result<PreparedRuntimeProviderImportAuthoritySet, String> {
    if expected_count == 0 {
        return Err(
            "prepared-runtime provider-import identity sidecar cannot mint empty authority"
                .to_owned(),
        );
    }
    let sidecar: PreparedRuntimeProviderImportIdentityArtifactSidecarWire =
        serde_ipld_dagcbor::from_slice(payload_bytes).map_err(|error| {
            format!(
                "prepared-runtime artifact provider-import identity sidecar DAG-CBOR decode failed: {error}"
            )
        })?;
    require_exact_string(
        &sidecar.schema,
        DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_PROVIDER_IMPORT_IDENTITY_SIDECAR_SCHEMA,
        "prepared_runtime_image_artifact.provider_import_identity_sidecar.schema",
    )?;
    if sidecar.provider_imports.len() != expected_count {
        return Err(
            "prepared_runtime_image_artifact.provider_import_identity_sidecar count mismatch; header counts are not returned as authority cargo"
                .to_owned(),
        );
    }
    let provider_imports = sidecar
        .provider_imports
        .into_iter()
        .map(|(identity_key, wire)| {
            let provider_import = admit_provider_import_identity_artifact_wire_for_prepared_runtime_artifact_owner_v1(
                identity_key.as_str(),
                wire,
            )?;
            Ok((identity_key, provider_import))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    let observed_fingerprint =
        provider_import_identity_fingerprint_for_prepared_runtime_artifact_owner_v1(
            &provider_imports,
        )?;
    if observed_fingerprint != expected_fingerprint {
        return Err(
            "prepared_runtime_image_artifact.provider_import_identity_sidecar fingerprint mismatch; provider import authority must be carried by the admitted sidecar, not reconstructed from header projection"
                .to_owned(),
        );
    }
    Ok(PreparedRuntimeProviderImportAuthoritySet { provider_imports })
}

fn admit_provider_import_identity_artifact_wire_for_prepared_runtime_artifact_owner_v1(
    identity_key: &str,
    wire: PreparedRuntimeProviderImportIdentityArtifactWire,
) -> Result<PreparedRuntimeImageProviderImportIdentity, String> {
    require_provider_import_identity_key_matches_contract_for_prepared_runtime_artifact_owner_v1(
        identity_key,
        &wire.contract,
    )?;
    match wire.routes {
        PreparedRuntimeProviderImportRoutesArtifactWire::ExternalOperations { operations } => {
            if operations.is_empty() {
                return Err(
                    "prepared-runtime external provider identity sidecar requires at least one operation route"
                        .to_owned(),
                );
            }
            let mut admitted = None;
            for route in operations {
                let operation = AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1::admit_persisted_for_prepared_runtime_artifact_owner_v1(
                    admitted_operation_kind_from_artifact_wire(route.operation_kind),
                    route.operation_name,
                )?;
                let identity = PreparedRuntimeImageProviderImportIdentity::admit_selected_implementation_target_v1(
                    SelectedImplementationTarget {
                        contract: wire.contract.duplicate_for_prepared_runtime_projection_owner_v1(),
                        kind: SelectedImplementationTargetKind::ExternalOperation {
                            operation,
                            provider_module_import_path: route.provider_module_import_path,
                            provider_module_export: route.provider_module_export,
                        },
                    },
                )?;
                match &mut admitted {
                    None => admitted = Some(identity),
                    Some(existing) => existing.merge_exact_contract_routes(identity)?,
                }
            }
            admitted.ok_or_else(|| {
                "prepared-runtime external provider identity sidecar admitted no operation routes"
                    .to_owned()
            })
        }
        PreparedRuntimeProviderImportRoutesArtifactWire::NativeContract {
            provider_module_import_path,
            provider_module_export,
            native_provider_artifact,
        } => {
            let native_provider_artifact = AdmittedPackageGraphNativeProviderArtifact::admit_persisted_artifact_for_prepared_runtime_image_manifest_owner_v1(
                native_provider_artifact.package_label,
                native_provider_artifact.package_root,
                native_provider_artifact.native_provider_manifest_path,
                native_provider_artifact.package_graph_manifest_fingerprint,
            )
            .map_err(|error| error.to_string())?;
            PreparedRuntimeImageProviderImportIdentity::admit_selected_implementation_target_v1(
                SelectedImplementationTarget {
                    contract: wire.contract,
                    kind: SelectedImplementationTargetKind::NativeContract {
                        provider_module_import_path,
                        provider_module_export,
                        native_provider_artifact,
                    },
                },
            )
        }
    }
}

fn require_provider_import_identity_key_matches_contract_for_prepared_runtime_artifact_owner_v1(
    identity_key: &str,
    contract: &CapabilityContractProjection,
) -> Result<(), String> {
    let expected_identity_key =
        provider_import_identity_key_for_prepared_runtime_artifact_owner_v1(contract)?;
    if identity_key != expected_identity_key {
        return Err(
            "prepared_runtime_image_artifact.provider_import_identity_sidecar identity key must match admitted provider contract"
                .to_owned(),
        );
    }
    Ok(())
}

fn provider_import_identity_key_for_prepared_runtime_artifact_owner_v1(
    contract: &CapabilityContractProjection,
) -> Result<String, String> {
    let mut encoded = Vec::new();
    push_tagged_provider_import_identity_component_for_prepared_runtime_artifact_owner_v1(
        &mut encoded,
        1,
        Some(contract.package_specifier()),
    )?;
    push_tagged_provider_import_identity_component_for_prepared_runtime_artifact_owner_v1(
        &mut encoded,
        2,
        Some(contract.export_name()),
    )?;
    push_tagged_provider_import_identity_component_for_prepared_runtime_artifact_owner_v1(
        &mut encoded,
        3,
        contract.fingerprint(),
    )?;
    Ok(format!(
        "{PREPARED_RUNTIME_PROVIDER_IMPORT_IDENTITY_KEY_SCHEMA}:{}",
        lower_hex(&encoded)
    ))
}

fn push_tagged_provider_import_identity_component_for_prepared_runtime_artifact_owner_v1(
    encoded: &mut Vec<u8>,
    field_tag: u8,
    value: Option<&str>,
) -> Result<(), String> {
    encoded.push(field_tag);
    match value {
        Some(value) => {
            encoded.push(1);
            let byte_len = u64::try_from(value.len()).map_err(|_| {
                "prepared-runtime provider import identity component exceeds u64 length".to_owned()
            })?;
            encoded.extend_from_slice(&byte_len.to_be_bytes());
            encoded.extend_from_slice(value.as_bytes());
        }
        None => encoded.push(0),
    }
    Ok(())
}

fn provider_import_identity_fingerprint_for_prepared_runtime_artifact_owner_v1(
    provider_imports: &BTreeMap<String, PreparedRuntimeImageProviderImportIdentity>,
) -> Result<String, String> {
    let projections: BTreeMap<&str, PreparedRuntimeProviderImportIdentityArtifactWire> =
        provider_imports
            .iter()
            .map(|(identity_key, provider_import)| {
                Ok((
                    identity_key.as_str(),
                    provider_import_identity_artifact_wire_from_admitted_identity(
                        identity_key,
                        provider_import,
                    )?,
                ))
            })
            .collect::<Result<_, String>>()?;
    prepared_runtime_image_artifact_dag_cbor_sha256_with_domain(
        DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_DOMAIN_PROVIDER_IMPORT_IDENTITY,
        &projections,
    )
}

fn operation_kind_label(
    kind: AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1,
) -> &'static str {
    match kind {
        AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::Command => {
            "command"
        }
        AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::InteractionOpen => {
            "interaction_open"
        }
    }
}

fn operation_kind_artifact_wire(
    kind: AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1,
) -> PreparedRuntimeProviderOperationKindArtifactWire {
    match kind {
        AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::Command => {
            PreparedRuntimeProviderOperationKindArtifactWire::Command
        }
        AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::InteractionOpen => {
            PreparedRuntimeProviderOperationKindArtifactWire::InteractionOpen
        }
    }
}

fn admitted_operation_kind_from_artifact_wire(
    kind: PreparedRuntimeProviderOperationKindArtifactWire,
) -> AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1 {
    match kind {
        PreparedRuntimeProviderOperationKindArtifactWire::Command => {
            AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::Command
        }
        PreparedRuntimeProviderOperationKindArtifactWire::InteractionOpen => {
            AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::InteractionOpen
        }
    }
}

fn prepared_runtime_image_artifact_dag_cbor_sha256_with_domain<T: Serialize>(
    domain: &'static str,
    value: &T,
) -> Result<String, String> {
    let bytes = serde_ipld_dagcbor::to_vec(value).map_err(|error| {
        format!("prepared runtime image artifact metadata identity DAG-CBOR {domain} encode failed: {error}")
    })?;
    Ok(format!(
        "{DIRECT_RUN_PREPARED_RUNTIME_IMAGE_ARTIFACT_DAG_CBOR_HASH_PREFIX}{domain}:{}",
        lower_hex(Sha256::digest(&bytes).as_slice())
    ))
}

fn require_non_empty(value: &str, label: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{label} must be non-empty"))
    } else {
        Ok(())
    }
}

fn require_exact_string(value: &str, expected: &str, field: &'static str) -> Result<(), String> {
    if value == expected {
        Ok(())
    } else {
        Err(format!("{field} mismatch"))
    }
}

fn lower_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: fn() = || {
        trait AmbiguousIfClone<A> {
            fn probe() {}
        }
        impl<T: ?Sized> AmbiguousIfClone<()> for T {}
        impl<T: ?Sized + Clone> AmbiguousIfClone<u8> for T {}
        let _ = <PreparedRuntimeProviderImportAuthoritySet as AmbiguousIfClone<_>>::probe;
        let _ =
            <PreparedRuntimeProviderImportExecutionStartAdmissionSet as AmbiguousIfClone<_>>::probe;
    };

    fn exact_contract(fingerprint_digit: char) -> CapabilityContractProjection {
        CapabilityContractProjection::from_prepared_runtime_projection_owner_v1(
            "@fixture/graphstore",
            "GraphStore",
            Some(format!(
                "sha256:{}",
                fingerprint_digit.to_string().repeat(64)
            )),
        )
        .expect("test contract projection must be valid")
    }

    fn external_identity(
        contract: &CapabilityContractProjection,
        operation_name: &str,
    ) -> PreparedRuntimeImageProviderImportIdentity {
        external_identity_with_target(
            contract,
            operation_name,
            "./graphstore-provider.ts",
            "graphStoreProvider",
        )
    }

    fn external_identity_with_target(
        contract: &CapabilityContractProjection,
        operation_name: &str,
        provider_module_import_path: &str,
        provider_module_export: &str,
    ) -> PreparedRuntimeImageProviderImportIdentity {
        let operation =
            AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1::admit_persisted_for_prepared_runtime_artifact_owner_v1(
                AdmittedCapabilityContractOperationKindForPreparedRuntimeArtifactOwnerV1::Command,
                operation_name.to_owned(),
            )
            .expect("test operation must admit");
        PreparedRuntimeImageProviderImportIdentity::admit_selected_implementation_target_v1(
            SelectedImplementationTarget {
                contract: contract.duplicate_for_prepared_runtime_projection_owner_v1(),
                kind: SelectedImplementationTargetKind::ExternalOperation {
                    operation,
                    provider_module_import_path: provider_module_import_path.to_owned(),
                    provider_module_export: provider_module_export.to_owned(),
                },
            },
        )
        .expect("test provider import identity must admit")
    }

    fn native_identity(
        contract: &CapabilityContractProjection,
    ) -> PreparedRuntimeImageProviderImportIdentity {
        let native_provider_artifact = AdmittedPackageGraphNativeProviderArtifact::admit_persisted_artifact_for_prepared_runtime_image_manifest_owner_v1(
            "fixture-native-provider",
            "/fixture/native-provider",
            "swarm-native-provider.json",
            format!("sha256:{}", "a".repeat(64)),
        )
        .expect("test native provider artifact must admit");
        PreparedRuntimeImageProviderImportIdentity::admit_selected_implementation_target_v1(
            SelectedImplementationTarget {
                contract: contract.duplicate_for_prepared_runtime_projection_owner_v1(),
                kind: SelectedImplementationTargetKind::NativeContract {
                    provider_module_import_path: "./native-provider".to_owned(),
                    provider_module_export: "nativeProvider".to_owned(),
                    native_provider_artifact,
                },
            },
        )
        .expect("test native provider identity must admit")
    }

    fn admit_set(
        identities: Vec<PreparedRuntimeImageProviderImportIdentity>,
    ) -> PreparedRuntimeProviderImportAuthoritySet {
        PreparedRuntimeProviderImportAuthoritySet::admit_package_selected_batch_for_prepared_runtime_owner_v1(
            identities,
        )
        .expect("test provider-import set must admit")
    }

    #[test]
    fn exact_fingerprints_for_one_package_export_coexist_and_enter_external_execution_routes() {
        let first_contract = exact_contract('1');
        let second_contract = exact_contract('2');
        let provider_imports = admit_set(vec![
            external_identity(&first_contract, "get"),
            external_identity(&second_contract, "get"),
        ]);

        assert_eq!(provider_imports.provider_imports.len(), 2);
        let admission = provider_imports.into_execution_start_admission_for_direct_run_owner_v1();
        assert_eq!(
            admission
                .external_routes
                .as_ref()
                .expect("external route set must be present")
                .routes
                .len(),
            2
        );
    }

    #[test]
    fn identical_exact_fingerprint_is_idempotent_and_merges_operation_routes() {
        let contract = exact_contract('3');
        let provider_imports = admit_set(vec![
            external_identity(&contract, "get"),
            external_identity(&contract, "get"),
            external_identity(&contract, "put"),
        ]);

        assert_eq!(provider_imports.provider_imports.len(), 1);
        let provider_import = provider_imports
            .provider_imports
            .values()
            .next()
            .expect("merged identity must remain present");
        let PreparedRuntimeImageProviderImportRoutes::ExternalOperations(routes) =
            &provider_import.routes
        else {
            panic!("test identity must retain external routes")
        };
        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn sidecar_v3_roundtrip_preserves_distinct_exact_fingerprints() {
        let first_contract = exact_contract('4');
        let second_contract = exact_contract('5');
        let provider_imports = admit_set(vec![
            external_identity(&first_contract, "get"),
            external_identity(&second_contract, "put"),
        ]);

        let segment = provider_import_identity_artifact_sidecar_segment_for_direct_run_prepared_runtime_owner_v1(
            &provider_imports,
        )
        .expect("identity sidecar must encode");
        let expected_fingerprint = segment.fingerprint.clone();
        let payload = segment.dag_cbor_bytes;
        let admitted =
            admit_provider_import_identity_sidecar_for_prepared_runtime_artifact_owner_v1(
                &payload,
                &expected_fingerprint,
                2,
            )
            .expect("identity sidecar must roundtrip");

        assert_eq!(admitted.provider_imports.len(), 2);
        for (identity_key, provider_import) in &admitted.provider_imports {
            require_provider_import_identity_key_matches_contract_for_prepared_runtime_artifact_owner_v1(
                identity_key,
                provider_import.contract(),
            )
            .expect("roundtripped key must retain exact contract identity");
        }
    }

    #[test]
    fn conflicting_external_targets_retain_every_selected_input() {
        let contract = exact_contract('6');
        let fault = match PreparedRuntimeProviderImportAuthoritySet::admit_package_selected_batch_for_prepared_runtime_owner_v1(
            vec![
                external_identity_with_target(&contract, "get", "./first.ts", "first"),
                external_identity_with_target(&contract, "get", "./second.ts", "second"),
            ],
        ) {
            Ok(_) => panic!("conflicting exact operation targets must be refused"),
            Err(fault) => fault,
        };
        assert_eq!(fault.selected_inputs.len(), 2);
        assert!(matches!(
            fault.reason,
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::ExternalOperationTargetConflict
        ));
        let cancellation = fault.cancel();
        assert!(matches!(
            cancellation._custody,
            PreparedRuntimeProviderImportAuthoritySetCancellationCustody::Admission(ref inputs)
                if inputs.len() == 2
        ));
    }

    #[test]
    fn native_external_merge_conflict_retains_both_sealed_sets() {
        let contract = exact_contract('7');
        let external = admit_set(vec![external_identity(&contract, "get")]);
        let native = admit_set(vec![native_identity(&contract)]);
        let fault = match external.merge_for_prepared_runtime_image_owner_v1(native) {
            Ok(_) => panic!("native and external execution domains must conflict"),
            Err(fault) => fault,
        };
        assert_eq!(fault.prior.provider_imports.len(), 1);
        assert_eq!(fault.incoming.provider_imports.len(), 1);
        assert!(matches!(
            fault.reason,
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::ProviderExecutionDomainConflict
        ));
        let native_execution = fault
            .incoming
            .into_execution_start_admission_for_direct_run_owner_v1();
        assert!(native_execution.external_routes.is_none());
    }

    #[test]
    fn empty_batch_cannot_mint_authority() {
        let fault = match PreparedRuntimeProviderImportAuthoritySet::admit_package_selected_batch_for_prepared_runtime_owner_v1(Vec::new()) {
            Ok(_) => panic!("empty provider-import selection must not mint authority"),
            Err(fault) => fault,
        };
        assert!(fault.selected_inputs.is_empty());
        assert!(matches!(
            fault.reason,
            PreparedRuntimeProviderImportAuthoritySetAdmissionFaultReason::EmptySelection
        ));
    }

    #[test]
    fn observation_reports_exact_counts_without_consuming_authority() {
        let contract = exact_contract('8');
        let provider_imports = admit_set(vec![
            external_identity(&contract, "get"),
            external_identity(&contract, "put"),
        ]);
        let observation = provider_imports
            .observe_for_prepared_runtime_manifest_owner_v1()
            .expect("manifest observation must derive");
        assert_eq!(observation.provider_import_count, 1);
        assert_eq!(observation.operation_route_count, 2);
        assert!(observation.identity_fingerprint.starts_with("sha256:"));
        assert_eq!(
            observation.projection["providerImportCount"],
            serde_json::json!(1)
        );

        let execution = provider_imports.into_execution_start_admission_for_direct_run_owner_v1();
        assert_eq!(
            execution
                .external_routes
                .as_ref()
                .expect("the sealed execution-start set must retain external routes")
                .routes
                .len(),
            2
        );
    }
}

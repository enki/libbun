use swarm_capability_contract_tson::AdmittedCapabilityContractTson;
use swarm_capability_linker_core::ProviderValue;
use swarm_capability_model::{
    CapabilityContractIdentity, CapabilitySdkError, CapabilitySdkResult,
    EXTERNAL_TRANSPORT_PROVIDER_DOMAIN, LOADED_NATIVE_PROVIDER_HOST_KIND, RUST_SDK_PROVIDER_DOMAIN,
    RUST_SDK_PROVIDER_HOST_ID, RUST_SDK_PROVIDER_HOST_KIND,
    SelectedProviderBoundaryOutputAuthority,
};
use swarm_native_provider_authority::NativeProviderInstalledHostAdmission;
use swarm_rust_sdk_static_provider_host::{
    HostAdmittedTypedProviderRequest, ProviderHostContext, ProviderHostResourceReleaseFaultV1,
    ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseRefusalV1,
    RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1,
    RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
    RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1,
    RustSdkStaticProviderExecutionResultForProviderHostOwner, RustSdkStaticProviderHostSet,
    SelectedProviderBoundaryExecutionResultForProviderHostOwner,
    SelectedProviderBoundaryHostRequest,
    SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1,
    SelectedProviderHostResourceReleaseV1,
};
use swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary;

use crate::external_transport::ExternalTransportCapabilityProviderExecutionSession;
use crate::json_fields::require_trimmed_nonblank;
use crate::loaded_native::{LoadedNativeProviderHostRecord, LoadedNativeProviderLinkRecord};
use crate::removed_contracts::RemovedProviderContractRegistry;
use crate::{AdmittedCompiledSwarmBinaryManifest, ExternalTransportCapabilityProviderHost};
use prepared_runtime_image_manifest_model::{
    ManifestResolvedExternalProviderCallAdmissionSelection,
    PreparedRuntimeProviderImportExecutionStartAdmissionSet,
};

pub struct ProviderHostSet {
    rust_sdk: RustSdkStaticProviderHostSet,
    loaded_native_providers: Vec<LoadedNativeProviderHostRecord>,
    loaded_native_link_providers: Vec<LoadedNativeProviderLinkRecord>,
    external_transport_capability_provider: Option<ExternalTransportCapabilityProviderHost>,
    removed_provider_contracts: RemovedProviderContractRegistry,
}

pub struct ProviderHostExecutionSession {
    provider_hosts: ProviderHostSet,
    external_provider_session: Option<ExternalTransportCapabilityProviderExecutionSession>,
    provider_import_execution_start_admission:
        Option<PreparedRuntimeProviderImportExecutionStartAdmissionSet>,
}

// compiler-custody: symbol=ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 disposition=migrating reviewer=package-root-compiler-kernel-hardcut-20260722 justification="retaining refusal crosses into compiler process-start admission; exact first consumer edit: prepared_runtime.rs::DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1::admit_for_direct_run_public_aperture_owner_v1"
#[must_use = "a refused provider-import execution-start admission retains the execution session and the complete incoming admission set"]
pub struct ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 {
    session: ProviderHostExecutionSession,
    incoming: PreparedRuntimeProviderImportExecutionStartAdmissionSet,
}

pub struct NativeProviderLinkHostSetAdmission {
    loaded_native_link_providers: Vec<LoadedNativeProviderLinkRecord>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProviderHostSetObservation {
    host_id: String,
    host_kind: String,
    provider_execution_domain: String,
    provider_count: usize,
}

impl ProviderHostSetObservation {
    fn new(
        host_id: impl Into<String>,
        host_kind: impl Into<String>,
        provider_execution_domain: impl Into<String>,
        provider_count: usize,
    ) -> CapabilitySdkResult<Self> {
        let host_id = host_id.into();
        require_trimmed_nonblank(&host_id)
            .map_err(|_| CapabilitySdkError::InvalidProviderHostId(host_id.clone()))?;
        let host_kind = host_kind.into();
        require_trimmed_nonblank(&host_kind)
            .map_err(|_| CapabilitySdkError::InvalidProviderHostId(host_kind.clone()))?;
        let provider_execution_domain = provider_execution_domain.into();
        require_trimmed_nonblank(&provider_execution_domain).map_err(|_| {
            CapabilitySdkError::InvalidProviderHostId(provider_execution_domain.clone())
        })?;
        Ok(Self {
            host_id,
            host_kind,
            provider_execution_domain,
            provider_count,
        })
    }

    pub(crate) fn rust_sdk_for_provider_host_owner_v1(provider_count: usize) -> Self {
        Self::new(
            RUST_SDK_PROVIDER_HOST_ID,
            RUST_SDK_PROVIDER_HOST_KIND,
            RUST_SDK_PROVIDER_DOMAIN,
            provider_count,
        )
        .expect("static Rust SDK provider host observation is valid")
    }

    pub(crate) fn loaded_native_for_provider_host_owner_v1(
        host_id: impl Into<String>,
        provider_count: usize,
    ) -> Self {
        Self::new(
            host_id,
            LOADED_NATIVE_PROVIDER_HOST_KIND,
            RUST_SDK_PROVIDER_DOMAIN,
            provider_count,
        )
        .expect("loaded native provider host observation is valid")
    }

    pub(crate) fn external_transport_for_provider_host_owner_v1(
        host_id: impl Into<String>,
        host_kind: impl Into<String>,
    ) -> Self {
        Self::new(host_id, host_kind, EXTERNAL_TRANSPORT_PROVIDER_DOMAIN, 0)
            .expect("external transport provider host observation is valid")
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

    pub fn diagnostic_value(&self) -> serde_json::Value {
        serde_json::json!({
            "schema": "swarm.provider_host_set.observation.v1",
            "host_id": self.host_id,
            "host_kind": self.host_kind,
            "provider_execution_domain": self.provider_execution_domain,
            "provider_count": self.provider_count,
        })
    }
}

impl NativeProviderLinkHostSetAdmission {
    pub fn provider_count(&self) -> usize {
        self.loaded_native_link_providers.len()
    }
}

/// Sealed classification of a resolved provider-call contract against the
/// loaded-native host set. Minted ONLY by
/// [`ProviderHostSet::admit_loaded_native_park_typed_request_for_provider_host_set_owner_contract_tson_v1`]
/// (surfaced through [`ProviderHostExecutionSession`]); the classification fact
/// ("this contract targets a loaded native") becomes true here, where contract
/// identity meets the host set's loaded-native records.
///
/// The `LoadedNativePark` payload is a SEALED [`HostAdmittedTypedProviderRequest`]
/// (not RAW): the only lawful consumer, the kernel drive, consumes the closed
/// variant once via
/// [`LoadedNativeProviderKindAdmission::into_loaded_native_park_request_for_direct_run_kernel_owner_v1`]
/// and forwards the sealed request, by value, into the route-descriptor-free
/// loaded-native park settlement ctor. `NotLoadedNative` carries no authority:
/// the contract is served (if at all) by a non-loaded-native host and routes
/// through the existing provider lane, which this classification leaves
/// untouched. There is no raw selector/field getter; the sealed dlopen handle
/// never reaches this path (the loaded-native record is identity-only).
#[must_use = "a loaded-native provider-kind admission classifies park authority and must be consumed by the kernel drive"]
pub enum LoadedNativeProviderKindAdmission {
    LoadedNativePark(HostAdmittedTypedProviderRequest),
    NotLoadedNative,
}

impl LoadedNativeProviderKindAdmission {
    fn loaded_native_park_for_provider_host_set_owner_v1(
        request: HostAdmittedTypedProviderRequest,
    ) -> Self {
        Self::LoadedNativePark(request)
    }

    fn not_loaded_native_for_provider_host_set_owner_v1() -> Self {
        Self::NotLoadedNative
    }

    /// Consume the closed classification, yielding the sealed loaded-native
    /// request when (and only when) the contract is served by a loaded-native
    /// host. The kernel forwards the returned sealed request into the park
    /// settlement ctor; `None` routes the call through the existing lane. This is
    /// a one-shot consume (`self`): a classification admits one input into one
    /// park.
    pub fn into_loaded_native_park_request_for_direct_run_kernel_owner_v1(
        self,
    ) -> Option<HostAdmittedTypedProviderRequest> {
        match self {
            Self::LoadedNativePark(request) => Some(request),
            Self::NotLoadedNative => None,
        }
    }
}

impl ProviderHostSet {
    pub fn from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
        rust_sdk: RustSdkStaticProviderHostSet,
    ) -> CapabilitySdkResult<Self> {
        Ok(Self {
            rust_sdk,
            loaded_native_providers: Vec::new(),
            loaded_native_link_providers: Vec::new(),
            external_transport_capability_provider: None,
            removed_provider_contracts: RemovedProviderContractRegistry::builtin()?,
        })
    }

    pub fn observations(&self) -> Vec<ProviderHostSetObservation> {
        let mut observations = vec![
            ProviderHostSetObservation::rust_sdk_for_provider_host_owner_v1(
                self.rust_sdk.provider_count(),
            ),
        ];
        observations.extend(
            self.loaded_native_providers
                .iter()
                .map(LoadedNativeProviderHostRecord::observation),
        );
        observations.extend(
            self.loaded_native_link_providers
                .iter()
                .map(LoadedNativeProviderLinkRecord::observation),
        );
        if let Some(host) = &self.external_transport_capability_provider {
            observations.push(host.observation());
        }
        observations
    }

    fn duplicate_for_provider_host_owner_v1(&self) -> Self {
        Self {
            rust_sdk: self
                .rust_sdk
                .duplicate_for_swarm_provider_host_set_owner_v1(),
            loaded_native_providers: self
                .loaded_native_providers
                .iter()
                .map(LoadedNativeProviderHostRecord::duplicate_for_provider_host_owner_v1)
                .collect(),
            loaded_native_link_providers: self
                .loaded_native_link_providers
                .iter()
                .map(LoadedNativeProviderLinkRecord::duplicate_for_provider_host_owner_v1)
                .collect(),
            external_transport_capability_provider: self
                .external_transport_capability_provider
                .as_ref()
                .map(ExternalTransportCapabilityProviderHost::duplicate_for_provider_host_owner_v1),
            removed_provider_contracts: self
                .removed_provider_contracts
                .duplicate_for_provider_host_owner_v1(),
        }
    }

    pub fn begin_provider_execution_session_v1(&self) -> ProviderHostExecutionSession {
        ProviderHostExecutionSession {
            external_provider_session: self
                .external_transport_capability_provider
                .as_ref()
                .map(ExternalTransportCapabilityProviderHost::begin_execution_session_for_provider_host_owner_v1),
            provider_hosts: self.duplicate_for_provider_host_owner_v1(),
            provider_import_execution_start_admission: None,
        }
    }

    pub fn duplicate_for_ss_runtime_provider_host_set_owner_v1(&self) -> Self {
        self.duplicate_for_provider_host_owner_v1()
    }

    pub(crate) fn admit_loaded_native_provider_hosts_v1<I>(
        &self,
        admissions: I,
    ) -> CapabilitySdkResult<Self>
    where
        I: IntoIterator<Item = NativeProviderInstalledHostAdmission>,
    {
        self.duplicate_for_provider_host_owner_v1()
            .with_loaded_native_provider_admissions_v1(admissions)
    }

    pub fn admit_loaded_native_provider_hosts_for_durable_native_provider_loader_owner_v1<I>(
        &self,
        admissions: I,
    ) -> CapabilitySdkResult<Self>
    where
        I: IntoIterator<Item = NativeProviderInstalledHostAdmission>,
    {
        self.admit_loaded_native_provider_hosts_v1(admissions)
    }

    pub fn admit_native_provider_link_host_set_v1(
        &self,
        admission: NativeProviderLinkHostSetAdmission,
    ) -> CapabilitySdkResult<Self> {
        self.duplicate_for_provider_host_owner_v1()
            .with_native_provider_link_host_set_admission_v1(admission)
    }

    pub fn rust_sdk_provider_count(&self) -> usize {
        self.rust_sdk.provider_count()
    }

    pub fn rust_sdk_provider_listing_for_libswarm_runtime_owner_v1(
        &self,
    ) -> swarm_rust_sdk_static_provider_listing::RustSdkStaticProviderListing {
        self.rust_sdk
            .provider_listing_for_swarm_provider_host_set_owner_v1()
    }

    pub fn static_manifest_provider_bridge_for_ss_runtime_provider_host_set_owner_v1(
        &self,
    ) -> CapabilitySdkResult<Option<RustSdkStaticManifestProviderBridgeForPackageGraphOwner>> {
        let Some(inventory) = self
            .rust_sdk
            .installed_static_provider_inventory_for_static_provider_host_owner_v1()?
        else {
            return Ok(None);
        };
        RustSdkStaticManifestProviderBridgeForPackageGraphOwner::admit_from_installed_static_provider_inventory_for_package_graph_owner_v1(
            inventory,
        )
        .map(Some)
        .map_err(|error| CapabilitySdkError::InvalidDirectRunProviderRequirement(error.to_string()))
    }

    pub fn removed_provider_contracts(&self) -> &RemovedProviderContractRegistry {
        &self.removed_provider_contracts
    }

    pub fn with_external_transport_capability_provider_host(
        mut self,
        host: ExternalTransportCapabilityProviderHost,
    ) -> CapabilitySdkResult<Self> {
        if self.external_transport_capability_provider.is_some() {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                host.host_id().to_owned(),
            ));
        }
        if self.rust_sdk.host_id() == host.host_id() {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                host.host_id().to_owned(),
            ));
        }
        if self
            .loaded_native_providers
            .iter()
            .any(|provider| provider.host_id() == host.host_id())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                host.host_id().to_owned(),
            ));
        }
        self.external_transport_capability_provider = Some(host);
        Ok(self)
    }

    pub(crate) fn with_loaded_native_provider_admissions_v1<I>(
        mut self,
        admissions: I,
    ) -> CapabilitySdkResult<Self>
    where
        I: IntoIterator<Item = NativeProviderInstalledHostAdmission>,
    {
        for admission in admissions {
            let record =
                LoadedNativeProviderHostRecord::from_installed_host_admission_v1(admission)?;
            self = self.with_loaded_native_provider_record(record)?;
        }
        Ok(self)
    }

    fn with_native_provider_link_host_set_admission_v1(
        mut self,
        admission: NativeProviderLinkHostSetAdmission,
    ) -> CapabilitySdkResult<Self> {
        for record in admission.loaded_native_link_providers {
            self = self.with_loaded_native_provider_link_record(record)?;
        }
        Ok(self)
    }

    fn with_loaded_native_provider_record(
        mut self,
        loaded: LoadedNativeProviderHostRecord,
    ) -> CapabilitySdkResult<Self> {
        if self
            .loaded_native_providers
            .iter()
            .any(|provider| provider.provider_id() == loaded.provider_id())
            || self
                .loaded_native_link_providers
                .iter()
                .any(|provider| provider.provider_id() == loaded.provider_id())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                loaded.provider_id().to_owned(),
            ));
        }
        if self.rust_sdk.host_id() == loaded.host_id()
            || self
                .external_transport_capability_provider
                .as_ref()
                .is_some_and(|host| host.host_id() == loaded.host_id())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                loaded.host_id().to_owned(),
            ));
        }
        self.removed_provider_contracts
            .reject_loaded_native_provider_conflict(&loaded)?;
        self.rust_sdk
            .reject_installed_native_provider_overlap_for_provider_host_set_owner_v1(
                loaded.installed_admission(),
            )?;
        for existing in &self.loaded_native_providers {
            loaded.reject_installed_native_provider_overlap(existing)?;
        }
        for link in &self.loaded_native_link_providers {
            for contract in link.contracts() {
                if loaded
                    .installed_admission()
                    .require_exact_contract_for_provider_host_set_owner_v1(contract)
                    .is_ok()
                {
                    return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                        contract.provider_id(),
                    ));
                }
            }
        }
        self.loaded_native_providers.push(loaded);
        Ok(self)
    }

    fn with_loaded_native_provider_link_record(
        mut self,
        loaded: LoadedNativeProviderLinkRecord,
    ) -> CapabilitySdkResult<Self> {
        if self
            .loaded_native_providers
            .iter()
            .any(|provider| provider.provider_id() == loaded.provider_id())
            || self
                .loaded_native_link_providers
                .iter()
                .any(|provider| provider.provider_id() == loaded.provider_id())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                loaded.provider_id().to_owned(),
            ));
        }
        if self.rust_sdk.host_id() == loaded.host_id()
            || self
                .external_transport_capability_provider
                .as_ref()
                .is_some_and(|host| host.host_id() == loaded.host_id())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                loaded.host_id().to_owned(),
            ));
        }
        for contract in loaded.contracts() {
            self.removed_provider_contracts
                .reject_removed_contract_for(
                    contract.package_specifier(),
                    contract.export_name(),
                )?;
            if self.rust_sdk.require_exact_contract_v1(contract).is_ok() {
                return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                    contract.provider_id(),
                ));
            }
            if self.loaded_native_provider_for_contract(contract).is_some()
                || self
                    .loaded_native_link_provider_for_contract(contract)
                    .is_some()
            {
                return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                    contract.provider_id(),
                ));
            }
        }
        self.loaded_native_link_providers.push(loaded);
        Ok(self)
    }

    pub fn require_rust_sdk_exact_contract_v1(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<()> {
        self.rust_sdk.require_exact_contract_v1(contract)
    }

    pub(crate) fn admit_typed_request_for_rust_sdk_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let contract_identity = contract.identity().duplicate_for_capability_model_owner();
        self.admit_typed_request_for_rust_sdk_operation_contract_tson_v1(
            contract_identity,
            contract,
            input,
        )
    }

    pub(crate) fn admit_typed_request_for_rust_sdk_operation_contract_tson_v1(
        &self,
        provider_route_contract: CapabilityContractIdentity,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        match self
            .rust_sdk
            .require_exact_contract_v1(&provider_route_contract)
        {
            Ok(()) => self
                .rust_sdk
                .admit_typed_request_for_provider_host_set_operation_contract_tson_v1(
                    provider_route_contract,
                    contract,
                    input,
                ),
            Err(CapabilitySdkError::NoRustSdkProvider { .. }) => {
                if let Some(provider) =
                    self.loaded_native_provider_for_contract(&provider_route_contract)
                {
                    let request_admission = swarm_rust_sdk_static_provider_host::NativeProviderInstalledHostRequestAdmission::admit_for_provider_host_set_owner_v1(
                        provider.installed_admission(),
                    )?;
                    return request_admission
                        .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
                            provider.installed_admission(),
                            contract,
                            input,
                        );
                }
                if let Some(error) = self
                    .loaded_native_provider_contract_fingerprint_mismatch(&provider_route_contract)
                {
                    return Err(error);
                }
                Err(CapabilitySdkError::NoRustSdkProvider {
                    provider_id: provider_route_contract.provider_id(),
                })
            }
            Err(error) => Err(error),
        }
    }

    pub fn admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.admit_typed_request_for_rust_sdk_contract_tson_v1(contract, input)
    }

    pub fn admit_typed_request_for_provider_host_set_operation_contract_tson_v1(
        &self,
        provider_route_contract: CapabilityContractIdentity,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.admit_typed_request_for_rust_sdk_operation_contract_tson_v1(
            provider_route_contract,
            contract,
            input,
        )
    }

    /// Classify a resolved provider-call contract against the loaded-native host
    /// set and, for a loaded-native contract, admit the sealed typed request into
    /// a two-phase park. The membership decision
    /// ([`loaded_native_provider_for_contract`]) and the admission
    /// ([`request_admission`]) both stay owner-internal here; the sealed
    /// classification is the only thing that crosses to the kernel. A loaded-native
    /// contract cannot also be a rust-sdk contract (admission rejects the overlap),
    /// so membership in the loaded-native set is an unambiguous discriminator. The
    /// input is consumed by value: each admitted request is bound to one input. A
    /// fingerprint mismatch against a loaded-native record is the typed
    /// loaded-native fault; anything else is `NotLoadedNative` and routes through
    /// the existing lane.
    pub fn admit_loaded_native_park_typed_request_for_provider_host_set_owner_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<LoadedNativeProviderKindAdmission> {
        let provider_route_contract = contract.identity().duplicate_for_capability_model_owner();
        if let Some(provider) = self.loaded_native_provider_for_contract(&provider_route_contract) {
            let request_admission = swarm_rust_sdk_static_provider_host::NativeProviderInstalledHostRequestAdmission::admit_for_provider_host_set_owner_v1(
                provider.installed_admission(),
            )?;
            let request = request_admission
                .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
                    provider.installed_admission(),
                    contract,
                    input,
                )?;
            return Ok(
                LoadedNativeProviderKindAdmission::loaded_native_park_for_provider_host_set_owner_v1(
                    request,
                ),
            );
        }
        if let Some(error) =
            self.loaded_native_provider_contract_fingerprint_mismatch(&provider_route_contract)
        {
            return Err(error);
        }
        // Not served by a loaded-native host: the input is not admitted into a
        // loaded-native park; the contract routes through the existing provider
        // lane (rust-sdk / external transport), which this op leaves untouched.
        let _ = input;
        Ok(LoadedNativeProviderKindAdmission::not_loaded_native_for_provider_host_set_owner_v1())
    }

    pub fn admit_typed_request_for_provider_host_set_owner_contract_v1(
        &self,
        contract: ProjectionCargoForbiddenAtAuthorityBoundary,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let _ = (self, input);
        match contract {}
    }

    pub fn admit_provider_host_context_for_mesh_capability_host_owner_contract_v1(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<ProviderHostContext> {
        let context = match ProviderHostContext::from_static_provider_host_set_owner_contract_v1(
            &self.rust_sdk,
            contract,
        ) {
            Ok(context) => context,
            Err(CapabilitySdkError::NoRustSdkProvider { .. }) => {
                if let Some(provider) = self.loaded_native_provider_for_contract(contract) {
                    let request_admission = swarm_rust_sdk_static_provider_host::NativeProviderInstalledHostRequestAdmission::admit_for_provider_host_set_owner_v1(
                        provider.installed_admission(),
                    )?;
                    ProviderHostContext::from_native_provider_host_set_owner_admission_v1(
                        &request_admission,
                    )?
                } else if let Some(error) =
                    self.loaded_native_provider_contract_fingerprint_mismatch(contract)
                {
                    return Err(error);
                } else {
                    return Err(CapabilitySdkError::NoRustSdkProvider {
                        provider_id: contract.provider_id(),
                    });
                }
            }
            Err(error) => return Err(error),
        };
        Ok(context)
    }

    pub fn admit_provider_call_request_for_rust_sdk_contract_v1(
        &self,
        _contract: &CapabilityContractIdentity,
        call_input: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        match call_input {}
    }

    pub fn invoke_admitted_rust_sdk_request(
        &mut self,
        request: HostAdmittedTypedProviderRequest,
        context: ProviderHostContext,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        self.rust_sdk
            .invoke_admitted_request_for_swarm_provider_host_set_owner_v1(request, context)
    }

    pub fn invoke_admitted_rust_sdk_request_for_mesh_capability_host_owner_v1(
        &mut self,
        request: HostAdmittedTypedProviderRequest,
        context: ProviderHostContext,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        self.invoke_admitted_rust_sdk_request(request, context)
    }

    pub fn invoke_selected_provider_boundary_request_for_direct_run_owner_v1(
        &mut self,
        request: SelectedProviderBoundaryHostRequest,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        self.rust_sdk
            .invoke_selected_provider_boundary_request_for_swarm_provider_host_set_owner_v1(request)
    }

    pub fn linked_capabilities_for_direct_run_requirements(
        &self,
        required_capabilities: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> CapabilitySdkResult<swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary>
    {
        let _ = self;
        match required_capabilities {}
    }

    pub fn linked_capabilities_for_direct_run_capability_link(
        &self,
        required_capabilities: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
        optional_capabilities: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> CapabilitySdkResult<swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary>
    {
        let _ = (self, optional_capabilities);
        match required_capabilities {}
    }

    pub fn validate_compiled_swarm_binary_manifest_v1(
        &self,
        manifest: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<()> {
        let _ = self;
        match manifest {}
    }

    pub fn admit_compiled_swarm_binary_manifest_for_provider_host_set_owner_v1(
        &self,
        manifest: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<AdmittedCompiledSwarmBinaryManifest> {
        let _ = self;
        match manifest {}
    }

    pub fn admit_compiled_swarm_binary_manifest_from_owner_inputs_v1(
        &self,
        manifest_inputs: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<AdmittedCompiledSwarmBinaryManifest> {
        let _ = self;
        match manifest_inputs {}
    }

    pub fn admit_compiled_swarm_binary_manifest_for_durable_native_provider_loader_owner_v1(
        &self,
        manifest: ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<AdmittedCompiledSwarmBinaryManifest> {
        let _ = self;
        match manifest {}
    }

    fn loaded_native_provider_for_contract(
        &self,
        required_contract: &CapabilityContractIdentity,
    ) -> Option<&LoadedNativeProviderHostRecord> {
        self.loaded_native_providers
            .iter()
            .find(|provider| provider.satisfies_contract(required_contract))
    }

    fn loaded_native_link_provider_for_contract(
        &self,
        required_contract: &CapabilityContractIdentity,
    ) -> Option<&LoadedNativeProviderLinkRecord> {
        self.loaded_native_link_providers
            .iter()
            .find(|provider| provider.satisfies_direct_run_contract_target(required_contract))
    }

    fn loaded_native_provider_contract_fingerprint_mismatch(
        &self,
        required_contract: &CapabilityContractIdentity,
    ) -> Option<CapabilitySdkError> {
        self.loaded_native_providers
            .iter()
            .find_map(|provider| provider.contract_fingerprint_mismatch(required_contract))
    }
}

impl std::fmt::Debug for ProviderHostSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderHostSet")
            .field("rust_sdk_provider_count", &self.rust_sdk_provider_count())
            .field(
                "loaded_native_provider_count",
                &self.loaded_native_providers.len(),
            )
            .field(
                "loaded_native_link_provider_count",
                &self.loaded_native_link_providers.len(),
            )
            .field(
                "has_external_transport_capability_provider",
                &self.external_transport_capability_provider.is_some(),
            )
            .field("hidden_provider_host_authority", &"redacted")
            .finish()
    }
}

impl std::fmt::Debug for ProviderHostExecutionSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderHostExecutionSession")
            .field("hidden_provider_host_authority", &"redacted")
            .finish()
    }
}

impl std::fmt::Debug
    for ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1")
            .field("reason", &"execution_session_already_admitted")
            .field("hidden_provider_host_authority", &"redacted")
            .field("hidden_provider_import_authority", &"redacted")
            .finish()
    }
}

impl std::fmt::Display
    for ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "provider host execution session already owns one prepared-runtime provider-import execution-start admission",
        )
    }
}

impl std::error::Error
    for ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1
{
}

impl ProviderHostExecutionSession {
    pub fn admit_prepared_runtime_provider_import_execution_start_for_direct_run_owner_v1(
        self,
        incoming: PreparedRuntimeProviderImportExecutionStartAdmissionSet,
    ) -> Result<Self, ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1>
    {
        if self.provider_import_execution_start_admission.is_some() {
            return Err(
                ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 {
                    session: self,
                    incoming,
                },
            );
        }
        let mut admitted = self;
        admitted.provider_import_execution_start_admission = Some(incoming);
        Ok(admitted)
    }

    pub fn commit_selected_host_resource_release_for_session_execution_kernel_owner_v1(
        &mut self,
        selected: SelectedProviderHostResourceReleaseV1,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseRefusalV1> {
        self.provider_hosts
            .rust_sdk
            .commit_selected_host_resource_release_for_swarm_provider_host_set_owner_v1(selected)
    }

    pub fn commit_selected_host_resource_release_borrowed_for_session_execution_kernel_owner_v1(
        &mut self,
        selected: &mut SelectedProviderHostResourceReleaseV1,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseFaultV1> {
        self.provider_hosts
            .rust_sdk
            .commit_selected_host_resource_release_borrowed_for_swarm_provider_host_set_owner_v1(
                selected,
            )
    }

    pub fn admit_body_local_process_output_observations_for_direct_run_process_child_owner_v1(
        &mut self,
        observations: RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1,
    ) -> CapabilitySdkResult<RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1>
    {
        self.provider_hosts
            .rust_sdk
            .admit_body_local_process_output_observations_for_swarm_provider_host_set_owner_v1(
                observations,
            )
    }

    pub fn admit_typed_request_for_direct_run_provider_resume_owner_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.provider_hosts
            .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(contract, input)
    }

    pub fn admit_selected_boundary_typed_request_for_direct_run_provider_resume_owner_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
        selected_output_authority: SelectedProviderBoundaryOutputAuthority,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryHostRequest> {
        let provider_imports = self
            .provider_import_execution_start_admission
            .as_ref()
            .ok_or_else(|| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    "selected provider-boundary admission requires the prepared-runtime provider-import execution-start admission first"
                        .to_owned(),
                )
            })?;
        match provider_imports
            .select_exact_external_call_for_provider_host_set_owner_v1(contract)
            .map_err(|fault| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(fault.to_string())
            })? {
            ManifestResolvedExternalProviderCallAdmissionSelection::Admitted(admission) => {
                SelectedProviderBoundaryHostRequest::from_manifest_resolved_external_call_admission_for_provider_host_set_owner_v1(
                    admission,
                    input,
                    selected_output_authority,
                )
            }
            ManifestResolvedExternalProviderCallAdmissionSelection::Unmatched(contract) => Ok(self
                .provider_hosts
                .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(contract, input)?
                .into_selected_provider_boundary_request_for_provider_host_set_owner_v1(
                    selected_output_authority,
                )),
        }
    }

    /// Classify + admit a loaded-native provider call at the kernel drive. The
    /// session already wraps the full [`ProviderHostSet`] (the kernel holds it
    /// across the drive); this is the existing threaded consume — no new
    /// dependency edge. The kernel consumes the returned sealed classification to
    /// mint the route-descriptor-free park; the sealed dlopen handle stays in the
    /// loader (identity-admission at park, handle-resolution at source-advance).
    pub fn admit_loaded_native_park_typed_request_for_direct_run_kernel_owner_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<LoadedNativeProviderKindAdmission> {
        self.provider_hosts
            .admit_loaded_native_park_typed_request_for_provider_host_set_owner_contract_tson_v1(
                contract, input,
            )
    }

    pub fn drive_rust_sdk_static_provider_request_for_provider_host_set_owner_contract_tson_v1(
        &mut self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let request = self
            .provider_hosts
            .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(contract, input)?;
        let context =
            ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )?;
        self.provider_hosts
            .invoke_admitted_rust_sdk_request(request, context)
    }

    pub fn drive_rust_sdk_static_provider_request_for_provider_host_set_owner_operation_contract_tson_v1(
        &mut self,
        provider_route_contract: CapabilityContractIdentity,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        let request = self
            .provider_hosts
            .admit_typed_request_for_provider_host_set_operation_contract_tson_v1(
                provider_route_contract,
                contract,
                input,
            )?;
        let context =
            ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )?;
        self.provider_hosts
            .invoke_admitted_rust_sdk_request(request, context)
    }

    pub fn admit_provider_call_request_for_rust_sdk_contract_v1(
        &self,
        contract: &CapabilityContractIdentity,
        call_input: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.provider_hosts
            .admit_provider_call_request_for_rust_sdk_contract_v1(contract, call_input)
    }

    pub fn invoke_admitted_rust_sdk_request(
        &mut self,
        request: HostAdmittedTypedProviderRequest,
        context: ProviderHostContext,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        self.provider_hosts
            .invoke_admitted_rust_sdk_request(request, context)
    }

    pub fn invoke_selected_provider_boundary_request_for_direct_run_owner_v1(
        &mut self,
        request: SelectedProviderBoundaryHostRequest,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        let provider_id = request.provider_id().to_owned();
        match request.into_route_for_provider_host_set_owner_v1() {
            SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1::RustSdk(request) => {
                self.provider_hosts
                    .invoke_selected_provider_boundary_request_for_direct_run_owner_v1(request)
            }
            SelectedProviderBoundaryRequestRouteForProviderHostSetOwnerV1::ManifestResolvedExternal(
                invocation,
            ) => self
                .external_provider_session
                .as_mut()
                .ok_or_else(|| CapabilitySdkError::NoExternalCapabilityProviderHost {
                    provider_id,
                    provider_execution_domain:
                        swarm_capability_model::EXTERNAL_TRANSPORT_PROVIDER_DOMAIN.to_owned(),
                })?
                .invoke_manifest_resolved_call_for_provider_host_set_owner_v1(invocation),
        }
    }
}

impl Drop for ProviderHostExecutionSession {
    fn drop(&mut self) {
        if let Some(session) = self.external_provider_session.as_mut() {
            let _ = session.shutdown_for_provider_host_set_owner_v1();
        }
    }
}

#[cfg(test)]
mod provider_import_execution_start_admission_tests {
    use super::*;

    const _: fn() = || {
        trait AmbiguousIfClone<A> {
            fn probe() {}
        }
        impl<T: ?Sized> AmbiguousIfClone<()> for T {}
        impl<T: ?Sized + Clone> AmbiguousIfClone<u8> for T {}
        let _ = <ProviderHostExecutionSession as AmbiguousIfClone<_>>::probe;
        let _ = <ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 as AmbiguousIfClone<_>>::probe;
    };

    #[test]
    fn admission_is_consuming_and_refusal_keeps_session_with_the_complete_incoming_set() {
        fn exercise_type_contract(
            session: ProviderHostExecutionSession,
            first: PreparedRuntimeProviderImportExecutionStartAdmissionSet,
            second: PreparedRuntimeProviderImportExecutionStartAdmissionSet,
        ) {
            let admitted = match session
                .admit_prepared_runtime_provider_import_execution_start_for_direct_run_owner_v1(
                    first,
                ) {
                Ok(admitted) => admitted,
                Err(refusal) => {
                    let ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 {
                        session,
                        incoming,
                    } = refusal;
                    let _retained_custody = (session, incoming);
                    return;
                }
            };
            let refusal = match admitted
                .admit_prepared_runtime_provider_import_execution_start_for_direct_run_owner_v1(
                    second,
                ) {
                Ok(_) => panic!("a second execution-start admission must be refused"),
                Err(refusal) => refusal,
            };
            let ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1 {
                session,
                incoming,
            } = refusal;
            let _retained_custody = (session, incoming);
        }

        let _typed_contract: fn(
            ProviderHostExecutionSession,
            PreparedRuntimeProviderImportExecutionStartAdmissionSet,
            PreparedRuntimeProviderImportExecutionStartAdmissionSet,
        ) = exercise_type_contract;
    }
}

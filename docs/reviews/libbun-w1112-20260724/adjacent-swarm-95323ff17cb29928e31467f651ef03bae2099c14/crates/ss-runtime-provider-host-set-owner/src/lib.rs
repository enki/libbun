#![forbid(unsafe_code)]

use swarm_provider_host_set::ProviderHostSet;
use swarm_rust_sdk_static_provider_host::RustSdkStaticManifestProviderBridgeForPackageGraphOwner;

#[derive(Debug)]
pub struct SsRuntimeProviderHostSet {
    provider_hosts: ProviderHostSet,
    static_manifest_provider_bridge:
        Option<RustSdkStaticManifestProviderBridgeForPackageGraphOwner>,
}

impl SsRuntimeProviderHostSet {
    pub fn from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
        rust_sdk: swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostSet,
    ) -> swarm_capability_model::CapabilitySdkResult<Self> {
        let provider_hosts =
            ProviderHostSet::from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
                rust_sdk,
            )?;
        Ok(Self {
            provider_hosts,
            static_manifest_provider_bridge: None,
        })
    }

    /// Product-environment composition keeps package-graph contract authority
    /// distinct from installed execution-host inventory. Contract-family host
    /// admissions intentionally do not retain prepared Contract-TSON, so the
    /// source compiler must consume the separately owner-minted manifest
    /// bridge rather than trying to reconstruct one from execution bindings.
    pub fn from_rust_sdk_static_provider_host_set_and_manifest_bridge_for_ss_product_environment_owner_v1(
        rust_sdk: swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostSet,
        static_manifest_provider_bridge: RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
    ) -> swarm_capability_model::CapabilitySdkResult<Self> {
        let provider_hosts =
            ProviderHostSet::from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
                rust_sdk,
            )?;
        Ok(Self {
            provider_hosts,
            static_manifest_provider_bridge: Some(static_manifest_provider_bridge),
        })
    }

    pub fn duplicate_for_source_test_bridge_owner_v1(&self) -> Self {
        Self {
            provider_hosts: self
                .provider_hosts
                .duplicate_for_ss_runtime_provider_host_set_owner_v1(),
            static_manifest_provider_bridge: self
                .static_manifest_provider_bridge
                .as_ref()
                .map(
                    RustSdkStaticManifestProviderBridgeForPackageGraphOwner::duplicate_for_package_graph_owner_v1,
                ),
        }
    }

    pub fn duplicate_for_source_entrypoint_execution_owner_v1(&self) -> Self {
        Self {
            provider_hosts: self
                .provider_hosts
                .duplicate_for_ss_runtime_provider_host_set_owner_v1(),
            static_manifest_provider_bridge: self
                .static_manifest_provider_bridge
                .as_ref()
                .map(
                    RustSdkStaticManifestProviderBridgeForPackageGraphOwner::duplicate_for_package_graph_owner_v1,
                ),
        }
    }

    pub fn duplicate_for_source_compiler_owner_v1(&self) -> Self {
        Self {
            provider_hosts: self
                .provider_hosts
                .duplicate_for_ss_runtime_provider_host_set_owner_v1(),
            static_manifest_provider_bridge: self
                .static_manifest_provider_bridge
                .as_ref()
                .map(
                    RustSdkStaticManifestProviderBridgeForPackageGraphOwner::duplicate_for_package_graph_owner_v1,
                ),
        }
    }

    pub fn duplicate_for_direct_run_native_harness_owner_v1(&self) -> Self {
        Self {
            provider_hosts: self
                .provider_hosts
                .duplicate_for_ss_runtime_provider_host_set_owner_v1(),
            static_manifest_provider_bridge: self
                .static_manifest_provider_bridge
                .as_ref()
                .map(
                    RustSdkStaticManifestProviderBridgeForPackageGraphOwner::duplicate_for_package_graph_owner_v1,
                ),
        }
    }

    pub fn static_manifest_provider_bridge_for_source_compiler_owner_v1(
        &self,
    ) -> Option<RustSdkStaticManifestProviderBridgeForPackageGraphOwner> {
        self.static_manifest_provider_bridge
            .as_ref()
            .map(
                RustSdkStaticManifestProviderBridgeForPackageGraphOwner::duplicate_for_package_graph_owner_v1,
            )
    }

    pub fn into_provider_host_set_for_source_runtime_owner_v1(self) -> ProviderHostSet {
        self.provider_hosts
    }

    pub fn into_provider_host_set_for_direct_run_native_harness_owner_v1(self) -> ProviderHostSet {
        self.provider_hosts
    }

    pub fn with_external_transport_capability_provider_for_ss_external_provider_owner_v1(
        mut self,
        host: swarm_provider_host_set::ExternalTransportCapabilityProviderHost,
    ) -> swarm_capability_model::CapabilitySdkResult<Self> {
        self.provider_hosts = self
            .provider_hosts
            .with_external_transport_capability_provider_host(host)?;
        Ok(self)
    }
}

pub fn admit_static_provider_host_admissions_for_ss_runtime_provider_host_set_owner_v1(
    admissions: swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostAdmissionSet,
) -> swarm_capability_model::CapabilitySdkResult<SsRuntimeProviderHostSet> {
    let rust_sdk =
        swarm_rust_sdk_static_provider_host::RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            admissions,
        )?;
    SsRuntimeProviderHostSet::from_rust_sdk_static_provider_host_set_for_ss_runtime_provider_host_set_owner_v1(
        rust_sdk,
    )
}

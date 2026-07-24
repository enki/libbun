use std::fmt;
use std::sync::Arc;

use swarm_capability_model::{CapabilitySdkError, CapabilitySdkResult, ProviderExecutionDomain};
use swarm_rust_sdk_static_provider_host::{
    DurableExternalProviderInvocationAuthority,
    SelectedProviderBoundaryExecutionResultForProviderHostOwner,
};

use crate::json_fields::require_trimmed_nonblank;
use crate::provider_host_set::ProviderHostSetObservation;

pub struct ExternalTransportCapabilityProviderHost {
    host_id: String,
    host_kind: String,
    factory: Arc<dyn DurableExternalCapabilityProviderFactory>,
}

pub trait DurableExternalCapabilityProvider: fmt::Debug {
    fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
        &mut self,
        invocation: DurableExternalProviderInvocationAuthority,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner>;

    fn shutdown_for_provider_host_set_owner_v1(&mut self) -> CapabilitySdkResult<()>;
}

pub trait DurableExternalCapabilityProviderFactory: fmt::Debug + Send + Sync {
    fn open_for_provider_host_set_owner_v1(
        &self,
    ) -> CapabilitySdkResult<Box<dyn DurableExternalCapabilityProvider>>;
}

pub(crate) struct ExternalTransportCapabilityProviderExecutionSession {
    factory: Arc<dyn DurableExternalCapabilityProviderFactory>,
    provider: Option<Box<dyn DurableExternalCapabilityProvider>>,
}

impl fmt::Debug for ExternalTransportCapabilityProviderHost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalTransportCapabilityProviderHost")
            .field("host_id", &self.host_id)
            .field("host_kind", &self.host_kind)
            .field("hidden_external_provider_host_authority", &"redacted")
            .finish()
    }
}

impl ExternalTransportCapabilityProviderHost {
    fn new(
        host_id: impl Into<String>,
        host_kind: impl Into<String>,
        factory: Arc<dyn DurableExternalCapabilityProviderFactory>,
    ) -> CapabilitySdkResult<Self> {
        let host_id = host_id.into();
        require_trimmed_nonblank(&host_id)
            .map_err(|_| CapabilitySdkError::InvalidProviderHostId(host_id.clone()))?;
        let host_kind = host_kind.into();
        require_trimmed_nonblank(&host_kind)
            .map_err(|_| CapabilitySdkError::InvalidProviderHostId(host_kind.clone()))?;
        Ok(Self {
            host_id,
            host_kind,
            factory,
        })
    }

    pub fn libbun_for_ss_external_capability_provider_owner_v1(
        factory: Arc<dyn DurableExternalCapabilityProviderFactory>,
    ) -> CapabilitySdkResult<Self> {
        Self::new("libbun", "libbun", factory)
    }

    pub fn host_id(&self) -> &str {
        &self.host_id
    }

    pub fn host_kind(&self) -> &str {
        &self.host_kind
    }

    pub(crate) fn duplicate_for_provider_host_owner_v1(&self) -> Self {
        Self {
            host_id: self.host_id.clone(),
            host_kind: self.host_kind.clone(),
            factory: Arc::clone(&self.factory),
        }
    }

    pub(crate) fn begin_execution_session_for_provider_host_owner_v1(
        &self,
    ) -> ExternalTransportCapabilityProviderExecutionSession {
        ExternalTransportCapabilityProviderExecutionSession {
            factory: Arc::clone(&self.factory),
            provider: None,
        }
    }

    pub fn observation(&self) -> ProviderHostSetObservation {
        let observation = ProviderHostSetObservation::external_transport_for_provider_host_owner_v1(
            self.host_id().to_owned(),
            self.host_kind().to_owned(),
        );
        debug_assert_eq!(observation.host_id(), self.host_id());
        debug_assert_eq!(observation.host_kind(), self.host_kind());
        debug_assert_eq!(
            observation.provider_execution_domain(),
            ProviderExecutionDomain::ExternalTransportCapabilityProvider.as_str()
        );
        observation
    }
}

impl ExternalTransportCapabilityProviderExecutionSession {
    pub(crate) fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
        &mut self,
        invocation: DurableExternalProviderInvocationAuthority,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        if self.provider.is_none() {
            self.provider = Some(self.factory.open_for_provider_host_set_owner_v1()?);
        }
        self.provider
            .as_mut()
            .expect("external provider is installed after lazy open")
            .invoke_manifest_resolved_call_for_provider_host_set_owner_v1(invocation)
    }

    pub(crate) fn shutdown_for_provider_host_set_owner_v1(&mut self) -> CapabilitySdkResult<()> {
        if let Some(provider) = self.provider.as_mut() {
            provider.shutdown_for_provider_host_set_owner_v1()?;
        }
        self.provider = None;
        Ok(())
    }
}

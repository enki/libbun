#![forbid(unsafe_code)]

// compiler-custody-scope: status=complete reviewer=package-root-compiler-kernel-hardcut-20260722 justification="all Rust source in this assigned owner was reviewed; every lexical custody candidate is classified adjacent to its item"

mod external_transport;
mod json_fields;
mod loaded_native;
mod manifest;
mod provider_host_set;
mod removed_contracts;

pub use external_transport::{
    DurableExternalCapabilityProvider, DurableExternalCapabilityProviderFactory,
    ExternalTransportCapabilityProviderHost,
};
pub use manifest::AdmittedCompiledSwarmBinaryManifest;
pub use provider_host_set::{
    LoadedNativeProviderKindAdmission, NativeProviderLinkHostSetAdmission,
    ProviderHostExecutionSession,
    ProviderHostExecutionSessionProviderImportExecutionStartAdmissionRefusalV1, ProviderHostSet,
    ProviderHostSetObservation,
};

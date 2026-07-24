#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use swarm_capability_linker_core::{
    ProviderValue, provider_value_from_canonical_json_v1, provider_value_to_canonical_json_v1,
};
use swarm_capability_model::CapabilitySdkError;
use swarm_provider_host_set::{
    DurableExternalCapabilityProvider, DurableExternalCapabilityProviderFactory,
    ExternalTransportCapabilityProviderHost,
};
use swarm_rust_sdk_static_provider_host::{
    DurableExternalProviderInvocationAuthority,
    SelectedProviderBoundaryExecutionResultForProviderHostOwner,
};
use thiserror::Error;

pub use ss_runtime_provider_host_set_owner::SsRuntimeProviderHostSet;

#[derive(Debug, Error)]
pub enum SsExternalCapabilityProviderError {
    #[error(transparent)]
    CapabilitySdk(#[from] CapabilitySdkError),
    #[error(transparent)]
    Libbun(#[from] libbun::LibbunError),
    #[error(transparent)]
    ProviderListing(#[from] ss_runtime_provider_listing_owner::SsProviderListingError),
}

pub type SsExternalCapabilityProviderResult<T> = Result<T, SsExternalCapabilityProviderError>;

type LibbunExternalCapabilityProviderBackend =
    libbun::BunProviderBackend<libbun::dynamic::DynamicBunRuntime>;

const LIBBUN_PROVIDER_ADAPTER_EXPORT: &str = "__swarmInvokeSelectedProvider";

static NEXT_LIBBUN_INVOCATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
struct SsLibbunExternalCapabilityProviderFactory {
    working_directory: PathBuf,
}

pub struct SsExternalCapabilityProviderHost {
    working_directory: PathBuf,
    backend: LibbunExternalCapabilityProviderBackend,
}

impl std::fmt::Debug for SsExternalCapabilityProviderHost {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SsExternalCapabilityProviderHost")
            .field("working_directory", &self.working_directory)
            .finish()
    }
}

impl SsExternalCapabilityProviderHost {
    pub fn new_for_source_compiler_owner_v1(
        working_directory: &Path,
    ) -> SsExternalCapabilityProviderResult<Self> {
        Self::new_with_dropped_process_output(working_directory)
    }

    pub fn new_for_ss_test_runtime_provider_owner_v1(
        working_directory: &Path,
    ) -> SsExternalCapabilityProviderResult<Self> {
        Self::new_with_dropped_process_output(working_directory)
    }

    fn new_with_dropped_process_output(
        working_directory: &Path,
    ) -> SsExternalCapabilityProviderResult<Self> {
        let mut config = libbun::BunRuntimeConfig::new("libbun", working_directory);
        config.stdout = libbun::SinkPolicy::Drop;
        config.stderr = libbun::SinkPolicy::Drop;
        config.log = libbun::SinkPolicy::Drop;
        let backend = LibbunExternalCapabilityProviderBackend::open(config)?;
        Ok(Self {
            working_directory: working_directory.to_path_buf(),
            backend,
        })
    }

    pub fn shutdown(&mut self) -> SsExternalCapabilityProviderResult<()> {
        self.backend.shutdown()?;
        Ok(())
    }
}

pub fn install_libbun_external_capability_provider_for_ss_runtime_owner_v1(
    provider_host_set: SsRuntimeProviderHostSet,
    working_directory: &Path,
) -> SsExternalCapabilityProviderResult<SsRuntimeProviderHostSet> {
    let factory = Arc::new(SsLibbunExternalCapabilityProviderFactory {
        working_directory: working_directory.to_path_buf(),
    });
    let host = ExternalTransportCapabilityProviderHost::libbun_for_ss_external_capability_provider_owner_v1(
        factory,
    )?;
    provider_host_set
        .with_external_transport_capability_provider_for_ss_external_provider_owner_v1(host)
        .map_err(Into::into)
}

impl DurableExternalCapabilityProviderFactory for SsLibbunExternalCapabilityProviderFactory {
    fn open_for_provider_host_set_owner_v1(
        &self,
    ) -> Result<Box<dyn DurableExternalCapabilityProvider>, CapabilitySdkError> {
        SsExternalCapabilityProviderHost::new_with_dropped_process_output(&self.working_directory)
            .map(|provider| Box::new(provider) as Box<dyn DurableExternalCapabilityProvider>)
            .map_err(external_provider_fault)
    }
}

impl DurableExternalCapabilityProvider for SsExternalCapabilityProviderHost {
    fn invoke_manifest_resolved_call_for_provider_host_set_owner_v1(
        &mut self,
        invocation: DurableExternalProviderInvocationAuthority,
    ) -> Result<SelectedProviderBoundaryExecutionResultForProviderHostOwner, CapabilitySdkError>
    {
        let (call_authority, provider_input, output_settlement) = invocation
            .into_call_input_and_output_settlement_for_durable_external_provider_owner_v1();
        let (contract, provider_module_import_path, provider_module_export) =
            call_authority.into_contract_and_module_for_durable_external_provider_owner_v1();
        if !matches!(&provider_input, ProviderValue::Array(_)) {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                "manifest-resolved external provider call must carry its sealed positional argument vector"
                    .to_owned(),
            ));
        }
        let input_json = provider_value_to_canonical_json_v1(&provider_input).map_err(|error| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "manifest-resolved external provider input canonicalization failed: {error}"
            ))
        })?;
        let input = serde_json::from_str(&input_json).map_err(|error| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "manifest-resolved external provider input JSON decode failed: {error}"
            ))
        })?;
        let fingerprint = contract
            .fingerprint()
            .ok_or_else(|| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    "manifest-resolved external provider contract requires an exact fingerprint"
                        .to_owned(),
                )
            })?
            .as_str()
            .to_owned();
        let invocation_id = format!(
            "ss-libbun-external-provider-{}",
            NEXT_LIBBUN_INVOCATION_ID.fetch_add(1, Ordering::Relaxed)
        );
        let provider_adapter_source = libbun_provider_adapter_source_for_selected_route(
            &provider_module_import_path,
            &provider_module_export,
            contract.package_specifier(),
            contract.export_name(),
        )?;
        let request = libbun::ProviderRequest {
            contract: libbun::ProviderContractIdentity {
                package: contract.package_specifier().to_owned(),
                capability: contract.export_name().to_owned(),
                contract_fingerprint: fingerprint,
            },
            domain: libbun::ProviderDomainClass::JavaScriptExternalTransport,
            module: libbun::BunModuleSpec::Source {
                module_id: format!("{invocation_id}-adapter"),
                source: provider_adapter_source,
            },
            export: LIBBUN_PROVIDER_ADAPTER_EXPORT.to_owned(),
            input: libbun::StructuralValue(input),
        };
        let descriptor = libbun::ProviderInvocationDescriptor::new(invocation_id.clone())
            .with_output_policy(libbun::InvocationOutputPolicy::Drop);
        let finished = self
            .backend
            .begin_invocation(descriptor)
            .and_then(|lease| {
                lease.settle_provider(
                    request,
                    libbun::ProviderSettleOptions::new(libbun::ProviderDeadline::after(
                        Duration::from_secs(30),
                    ))
                    .with_call_id(invocation_id),
                )
            })
            .and_then(|settled| settled.finish())
            .map_err(external_provider_fault)?;
        let output = match finished.receipt {
            libbun::SettledProviderReceipt::Ready {
                result: libbun::ProviderCallResult::Ok(output),
                ..
            } => output.0,
            libbun::SettledProviderReceipt::Ready {
                result: libbun::ProviderCallResult::Err(error),
                ..
            } => {
                return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    format!(
                        "manifest-resolved external provider rejected the call ({}): {}",
                        error.code, error.message
                    ),
                ));
            }
            libbun::SettledProviderReceipt::Failed(failure) => {
                return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    format!("manifest-resolved external provider execution failed: {failure:?}"),
                ));
            }
        };
        let output_json = serde_json::to_string(&output).map_err(|error| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "manifest-resolved external provider output JSON encode failed: {error}"
            ))
        })?;
        let output = provider_value_from_canonical_json_v1(&output_json).map_err(|error| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "manifest-resolved external provider output admission failed: {error}"
            ))
        })?;
        output_settlement.settle_ready_for_durable_external_provider_owner_v1(output)
    }

    fn shutdown_for_provider_host_set_owner_v1(&mut self) -> Result<(), CapabilitySdkError> {
        self.shutdown().map_err(external_provider_fault)
    }
}

/// Builds the transport-only adapter for one already-selected external route.
///
/// `libbun` invokes one module export with one structural input. Swarm provider
/// modules instead expose a zero-argument, branded factory whose result is the
/// callable behavior. The adapter preserves those distinct stages: Rust embeds
/// the upstream-selected module path and export, the factory receives no host
/// input, and only the resolved behavior receives the sealed positional vector.
fn libbun_provider_adapter_source_for_selected_route(
    provider_module_import_path: &str,
    provider_module_export: &str,
    contract_package_specifier: &str,
    contract_export_name: &str,
) -> Result<String, CapabilitySdkError> {
    let provider_module_url =
        file_url_for_selected_provider_path(Path::new(provider_module_import_path))?;
    let provider_module_url = adapter_string_literal("provider module URL", &provider_module_url)?;
    let provider_module_export =
        adapter_string_literal("provider module export", provider_module_export)?;
    let contract_package_specifier =
        adapter_string_literal("contract package specifier", contract_package_specifier)?;
    let contract_export_name =
        adapter_string_literal("contract export name", contract_export_name)?;

    Ok(format!(
        r#"const providerModuleImportPath = {provider_module_url};
const providerModuleExport = {provider_module_export};
const contractPackageSpecifier = {contract_package_specifier};
const contractExportName = {contract_export_name};
const providerFactoryKind = "swarm.provider.factory.v1";

function requireProviderFactory(value) {{
  if (typeof value !== "function" || value.kind !== providerFactoryKind) {{
    throw new TypeError(
      `capability '${{contractPackageSpecifier}}:${{contractExportName}}' must resolve to the selected @swarm/provider factory`,
    );
  }}
  return value;
}}

function requireCallableProvider(value) {{
  if (typeof value !== "function") {{
    throw new TypeError(
      `provider '${{contractPackageSpecifier}}:${{contractExportName}}' did not resolve to a callable behavior`,
    );
  }}
  return value;
}}

function requireSealedPositionalArguments(value) {{
  if (!Array.isArray(value)) {{
    throw new TypeError(
      `provider '${{contractPackageSpecifier}}:${{contractExportName}}' requires a sealed positional argument vector`,
    );
  }}
  return value;
}}

export async function {LIBBUN_PROVIDER_ADAPTER_EXPORT}(positionalArguments) {{
  const providerModule = await import(providerModuleImportPath);
  const factory = requireProviderFactory(providerModule?.[providerModuleExport]);
  const provider = requireCallableProvider(await factory());
  return await provider(...requireSealedPositionalArguments(positionalArguments));
}}
"#
    ))
}

fn adapter_string_literal(field: &'static str, value: &str) -> Result<String, CapabilitySdkError> {
    serde_json::to_string(value).map_err(|error| {
        CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
            "manifest-resolved external provider adapter {field} projection failed: {error}"
        ))
    })
}

fn file_url_for_selected_provider_path(path: &Path) -> Result<String, CapabilitySdkError> {
    let path = path.to_str().ok_or_else(|| {
        CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
            "manifest-resolved external provider module path is not valid UTF-8: {}",
            path.display()
        ))
    })?;
    let mut url = String::from("file://");
    for byte in path.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                url.push(char::from(*byte));
            }
            _ => url.push_str(&format!("%{byte:02X}")),
        }
    }
    Ok(url)
}

fn external_provider_fault(error: impl ToString) -> CapabilitySdkError {
    CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
        "retained libbun external provider failed: {}",
        error.to_string()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_route_adapter_separates_factory_resolution_from_positional_invocation() {
        let source = libbun_provider_adapter_source_for_selected_route(
            "/tmp/provider path/swarm.ts",
            "create",
            "@fixture/resource-contract",
            "create",
        )
        .expect("selected provider adapter source");

        assert!(source.contains(
            r#"const providerModuleImportPath = "file:///tmp/provider%20path/swarm.ts";"#
        ));
        assert!(source.contains(r#"const providerModuleExport = "create";"#));
        assert!(source.contains("providerModule?.[providerModuleExport]"));
        assert!(source.contains("value.kind !== providerFactoryKind"));
        assert!(source.contains("await factory()"));
        assert!(!source.contains("factory(positionalArguments)"));
        assert!(
            source.contains("provider(...requireSealedPositionalArguments(positionalArguments))")
        );
    }
}

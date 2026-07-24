use crate::{SsError, SsExternalCapabilityProviderHost, SsResult};
use serde_json::json;
use std::path::PathBuf;

#[derive(Default)]
pub(super) struct ExternalCapabilityProviderPool {
    active: Option<(PathBuf, SsExternalCapabilityProviderHost)>,
}

pub(super) struct ExternalCapabilityProviderCheckout<'a> {
    provider: &'a mut SsExternalCapabilityProviderHost,
    pub(super) initialized: bool,
}

impl ExternalCapabilityProviderPool {
    pub(super) fn checkout(
        &mut self,
        working_directory: PathBuf,
    ) -> SsResult<ExternalCapabilityProviderCheckout<'_>> {
        let should_replace = self
            .active
            .as_ref()
            .map(|(active_working_directory, _)| active_working_directory != &working_directory)
            .unwrap_or(true);
        if should_replace {
            if let Some((_, active_provider)) = self.active.as_mut() {
                active_provider.shutdown()?;
            }
            self.active = Some((
                working_directory.clone(),
                SsExternalCapabilityProviderHost::new_for_ss_test_runtime_provider_owner_v1(
                    &working_directory,
                )?,
            ));
        }
        let Some((_, provider)) = self.active.as_mut() else {
            return Err(SsError::Cli(
                json!({
                    "schema": "swarm.ss.test.execution_fault.v1",
                    "code": "ss_test_external_capability_provider_pool_checkout_missing",
                    "reason": "ss test provider-host pool failed to retain the checked-out external capability provider backend",
                    "working_directory": working_directory.display().to_string(),
                })
                .to_string(),
            ));
        };
        Ok(ExternalCapabilityProviderCheckout {
            provider,
            initialized: should_replace,
        })
    }

    pub(super) fn shutdown(&mut self) -> SsResult<()> {
        if let Some((_, provider)) = self.active.as_mut() {
            provider.shutdown()?;
        }
        self.active = None;
        Ok(())
    }
}

impl ExternalCapabilityProviderCheckout<'_> {
    pub(super) fn provider_mut(&mut self) -> &mut SsExternalCapabilityProviderHost {
        self.provider
    }
}

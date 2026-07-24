impl RustSdkStaticProviderHostSet {
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn host_resource_release_commit_fixture_for_test_support_v1() -> CapabilitySdkResult<(
        Self,
        SelectedProviderHostResourceReleaseV1,
        StaticProviderHostResourceReleaseCommitObservationForTestSupportV1,
    )> {
        let (selected, observation) =
            mint_static_provider_host_resource_release_commit_fixture_for_test_support_v1()?;
        Ok((
            Self::empty_for_provider_host_set_owner_v1()?,
            selected,
            observation,
        ))
    }

    pub(crate) fn from_admission_set_v1(
        admissions: RustSdkStaticProviderHostAdmissionSet,
    ) -> CapabilitySdkResult<Self> {
        let mut exact_providers = Vec::new();
        let mut contract_families = Vec::new();
        for admission in admissions.into_admissions() {
            match admission.kind {
                RustSdkStaticProviderHostAdmissionKind::Exact { binding, executor } => {
                    exact_providers.push(RustSdkInstalledStaticProvider { binding, executor });
                }
                RustSdkStaticProviderHostAdmissionKind::ContractFamily { family, executor } => {
                    contract_families
                        .push(RustSdkInstalledStaticProviderContractFamily { family, executor });
                }
            }
        }
        Ok(Self {
            host_id: RUST_SDK_PROVIDER_HOST_ID.to_owned(),
            exact_providers,
            contract_families,
        })
    }

    pub fn from_provider_host_set_owner_admission_set_v1(
        admissions: RustSdkStaticProviderHostAdmissionSet,
    ) -> CapabilitySdkResult<Self> {
        Self::from_admission_set_v1(admissions)
    }

    pub fn empty_for_provider_host_set_owner_v1() -> CapabilitySdkResult<Self> {
        Self::from_admission_set_v1(RustSdkStaticProviderHostAdmissionSet {
            admissions: Vec::new(),
        })
    }

    pub fn commit_selected_host_resource_release_for_swarm_provider_host_set_owner_v1(
        &mut self,
        selected: SelectedProviderHostResourceReleaseV1,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseRefusalV1> {
        selected.commit_release_for_static_provider_host_set_owner_v1()
    }

    pub fn commit_selected_host_resource_release_borrowed_for_swarm_provider_host_set_owner_v1(
        &mut self,
        selected: &mut SelectedProviderHostResourceReleaseV1,
    ) -> Result<ProviderHostResourceReleaseReceiptV1, ProviderHostResourceReleaseFaultV1> {
        selected.commit_release_borrowed_for_static_provider_host_set_owner_v1()
    }

    pub fn admit_body_local_process_output_observations_for_swarm_provider_host_set_owner_v1(
        &mut self,
        observations: RustSdkStaticProviderBodyLocalProcessOutputObservationSetForProviderHostOwnerV1,
    ) -> CapabilitySdkResult<RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1>
    {
        let Some(test_family) = self
            .contract_families
            .iter_mut()
            .find(|family| family.family.package_specifier() == SWARM_TEST_MODULE_SPECIFIER)
        else {
            return Ok(
                RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1::BodyLocalStaticTestExecutorAbsent,
            );
        };
        test_family
            .executor
            .admit_body_local_process_output_observations_for_provider_host_owner_v1(
                observations,
            )?;
        Ok(
            RustSdkBodyLocalProcessOutputObservationAdmissionForProviderHostOwnerV1::ObservedByBodyLocalStaticTestExecutor,
        )
    }

    pub fn from_admission_set_for_direct_run_owner_v1(
        admissions: RustSdkStaticProviderHostAdmissionSet,
    ) -> CapabilitySdkResult<Self> {
        Self::from_admission_set_v1(admissions)
    }

    pub(crate) fn with_admission_set_v1(
        mut self,
        admissions: RustSdkStaticProviderHostAdmissionSet,
    ) -> CapabilitySdkResult<Self> {
        let additional = Self::from_admission_set_v1(admissions)?;
        for provider in additional.exact_providers {
            self.reject_exact_provider_conflict(&provider.binding)?;
            self.exact_providers.push(provider);
        }
        for family in additional.contract_families {
            self.reject_contract_family_conflict(&family.family)?;
            self.contract_families.push(family);
        }
        Ok(self)
    }

    pub fn with_provider_host_set_owner_admission_set_v1(
        self,
        admissions: RustSdkStaticProviderHostAdmissionSet,
    ) -> CapabilitySdkResult<Self> {
        self.with_admission_set_v1(admissions)
    }

    fn reject_exact_provider_conflict(
        &self,
        binding: &RustSdkStaticProviderBinding,
    ) -> CapabilitySdkResult<()> {
        let provider_id = binding.provider_id();
        if self
            .exact_providers
            .iter()
            .any(|provider| provider.binding.provider_id() == provider_id)
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                provider_id,
            ));
        }
        if self
            .contract_families
            .iter()
            .any(|family| family.family.package_specifier() == binding.package_specifier())
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(
                provider_id,
            ));
        }
        Ok(())
    }

    fn reject_contract_family_conflict(
        &self,
        family: &RustSdkStaticProviderContractFamily,
    ) -> CapabilitySdkResult<()> {
        let package = family.package_specifier().to_owned();
        if self
            .contract_families
            .iter()
            .any(|existing| existing.family.package_specifier() == package)
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(package));
        }
        let provider_prefix = format!("{package}:");
        if self
            .exact_providers
            .iter()
            .any(|provider| provider.binding.provider_id().starts_with(&provider_prefix))
        {
            return Err(CapabilitySdkError::DuplicateProviderHostProvider(package));
        }
        Ok(())
    }

    pub fn host_request_admission_for_provider_host_owner_v1(
        &self,
    ) -> CapabilitySdkResult<ProviderHostRequestAdmission> {
        ProviderHostRequestAdmission::rust_sdk_static_provider_host_for_static_provider_host_owner_v1(
            self.provider_count(),
        )
    }

    pub fn host_id(&self) -> &str {
        self.host_id.as_str()
    }

    pub fn provider_count(&self) -> usize {
        self.exact_providers.len()
            + self
                .contract_families
                .iter()
                .map(|family| family.family.provider_count())
                .sum::<usize>()
    }

    pub fn admits_package_specifier_v1(&self, package_specifier: &str) -> bool {
        self.exact_providers
            .iter()
            .any(|provider| provider.binding.package_specifier() == package_specifier)
            || self
                .contract_families
                .iter()
                .any(|family| family.family.package_specifier() == package_specifier)
    }

    pub(crate) fn admit_executable_provider_target_for_direct_run_v1(
        &self,
        contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Option<AdmittedRustSdkExecutableProviderTargetForDirectRun>> {
        let _ = self;
        match contract {}
    }

    pub fn admit_executable_provider_target_for_libswarm_runtime_owner_v1(
        &self,
        contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Option<AdmittedRustSdkExecutableProviderTargetForDirectRun>> {
        self.admit_executable_provider_target_for_direct_run_v1(contract)
    }

    pub(crate) fn admit_executable_provider_target_for_direct_run_provider_id_v1(
        &self,
        provider_id: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Option<AdmittedRustSdkExecutableProviderTargetForDirectRun>> {
        let _ = self;
        match provider_id {}
    }

    pub fn admit_executable_provider_target_for_provider_id_for_libswarm_runtime_owner_v1(
        &self,
        provider_id: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
    ) -> CapabilitySdkResult<Option<AdmittedRustSdkExecutableProviderTargetForDirectRun>> {
        self.admit_executable_provider_target_for_direct_run_provider_id_v1(provider_id)
    }

    pub fn provider_listing_for_swarm_provider_host_set_owner_v1(
        &self,
    ) -> RustSdkStaticProviderListing {
        RustSdkStaticProviderListing::from_static_provider_host_inventory_for_static_provider_host_owner_v1(
            self.exact_providers
                .iter()
                .map(|provider| {
                    provider
                        .binding
                        .duplicate_for_static_provider_host_owner_v1()
                })
                .collect(),
            self.contract_families
                .iter()
                .map(|family| family.family.duplicate_for_static_provider_host_owner_v1())
                .collect(),
        )
    }

    pub fn installed_static_provider_inventory_for_static_provider_host_owner_v1(
        &self,
    ) -> CapabilitySdkResult<Option<RustSdkInstalledStaticProviderInventoryForPackageGraphOwner>>
    {
        if self.exact_providers.is_empty() && self.contract_families.is_empty() {
            return Ok(None);
        }
        RustSdkInstalledStaticProviderInventoryForPackageGraphOwner::admit_from_static_provider_host_set_owner_v1(
            self.host_id.clone(),
            self.exact_providers.iter().map(|provider| {
                provider
                    .binding
                    .duplicate_for_static_provider_host_owner_v1()
            }),
            self.contract_families
                .iter()
                .map(|family| family.family.duplicate_for_static_provider_host_owner_v1()),
        )
        .map(Some)
    }

    pub(crate) fn duplicate_for_provider_host_owner_v1(&self) -> Self {
        Self {
            host_id: self.host_id.clone(),
            exact_providers: self
                .exact_providers
                .iter()
                .map(|provider| RustSdkInstalledStaticProvider {
                    binding: provider
                        .binding
                        .duplicate_for_static_provider_host_owner_v1(),
                    executor: provider
                        .executor
                        .as_ref()
                        .map(|executor| executor.duplicate_for_provider_host_owner_v1()),
                })
                .collect(),
            contract_families: self
                .contract_families
                .iter()
                .map(|family| RustSdkInstalledStaticProviderContractFamily {
                    family: family.family.duplicate_for_static_provider_host_owner_v1(),
                    executor: family.executor.duplicate_for_provider_host_owner_v1(),
                })
                .collect(),
        }
    }

    pub fn duplicate_for_swarm_provider_host_set_owner_v1(&self) -> Self {
        self.duplicate_for_provider_host_owner_v1()
    }

    pub fn require_exact_contract_v1(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<()> {
        let provider_id = contract.provider_id();
        let Some(binding) = self.provider_binding_for_contract(contract)? else {
            return Err(CapabilitySdkError::NoRustSdkProvider { provider_id });
        };
        if !binding.matches_identity_for_static_provider_host_owner_v1(contract) {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                format!(
                    "Rust SDK provider host contract for {provider_id} must match the exact admitted contract identity"
                ),
            ));
        }
        Ok(())
    }

    pub fn reject_installed_native_provider_overlap_for_provider_host_set_owner_v1<A>(
        &self,
        installed_native: &A,
    ) -> CapabilitySdkResult<()>
    where
        A: RustSdkStaticProviderInstalledNativeHostAdmission + ?Sized,
    {
        for provider in &self.exact_providers {
            installed_native
                .reject_package_export_conflict_for_static_provider_host_owner_v1(
                    provider.binding.package_specifier(),
                    provider.binding.export_name(),
                )?;
        }
        for family in &self.contract_families {
            installed_native
                .reject_package_specifier_conflict_for_static_provider_host_owner_v1(
                    family.family.package_specifier(),
                )?;
        }
        Ok(())
    }

    pub(crate) fn admit_typed_request_for_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let contract_identity = contract.identity().duplicate_for_capability_model_owner();
        self.admit_typed_request_for_operation_contract_tson_v1(contract_identity, contract, input)
    }

    pub fn admit_typed_request_for_operation_contract_tson_v1(
        &self,
        provider_route_contract: CapabilityContractIdentity,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let schema_contract_identity = contract.identity().duplicate_for_capability_model_owner();
        if provider_route_contract.package_specifier()
            != schema_contract_identity.package_specifier()
        {
            return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                format!(
                    "provider route contract {} cannot consume Contract-TSON schema authority from {}",
                    provider_route_contract.provider_id(),
                    schema_contract_identity.provider_id(),
                ),
            ));
        }
        let operation_export = contract
            .operation_name_for_provider_host_owner_v1()
            .map(str::to_owned);
        let (operation, output_type_contract) = contract
            .into_operation_and_output_type_contract_authority_for_provider_host_owner_v1()
            .map_err(|source| {
                CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                    "static provider Contract-TSON operation admission failed: {source}"
                ))
            })?;
        let provider_id = provider_route_contract.provider_id();
        let Some(binding) = self.provider_binding_for_contract(&provider_route_contract)? else {
            return Err(CapabilitySdkError::NoRustSdkProvider { provider_id });
        };
        swarm_capability_linker_core::validate_provider_user_payload_value(
            &input,
            binding.provider_id().as_str(),
        )
        .map_err(|source| {
            CapabilitySdkError::InvalidDirectRunProviderRequirement(format!(
                "static provider request input admission failed for {provider_id}: {source}"
            ))
        })?;
        let invocation = admit_provider_operation_invocation_for_static_provider_host_owner_v1(
            &provider_route_contract,
            operation_export.as_deref(),
            &input,
        )?;
        Ok(HostAdmittedTypedProviderRequest {
            host: self.host_request_admission_for_provider_host_owner_v1()?,
            binding,
            request: TypedProviderRequest {
                provider_id,
                contract: provider_route_contract,
                operation,
                output_type_contract,
                input,
                invocation,
            },
        })
    }

    pub fn admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
        &self,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.admit_typed_request_for_contract_tson_v1(contract, input)
    }

    pub fn admit_typed_request_for_provider_host_set_operation_contract_tson_v1(
        &self,
        provider_route_contract: CapabilityContractIdentity,
        contract: AdmittedCapabilityContractTson,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        self.admit_typed_request_for_operation_contract_tson_v1(
            provider_route_contract,
            contract,
            input,
        )
    }

    pub fn admit_typed_request_for_provider_host_set_owner_contract_v1(
        &self,
        contract: swarm_substrate_invariant::ProjectionCargoForbiddenAtAuthorityBoundary,
        input: ProviderValue,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let _ = (self, input);
        match contract {}
    }

    pub fn admit_typed_request_for_contract_for_direct_run_owner_v1(
        &self,
        contract: &CapabilityContractIdentity,
        input: swarm_substrate_invariant::RawTransportValueForbiddenAtSemanticBoundary,
    ) -> CapabilitySdkResult<HostAdmittedTypedProviderRequest> {
        let _ = contract;
        match input {}
    }

    pub(crate) fn invoke_admitted_request(
        &mut self,
        request: HostAdmittedTypedProviderRequest,
        _context: ProviderHostContext,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        for provider in &mut self.exact_providers {
            if provider
                .binding
                .matches_identity_for_static_provider_host_owner_v1(request.contract())
            {
                if let Some(executor) = &mut provider.executor {
                    let mut adapter = executor
                        .prepare_exact_adapter_operation_for_provider_host_owner_v1(
                            request.contract(),
                            request.request.operation_for_static_provider_host_owner_v1(),
                        )?;
                    let implementation_output = adapter
                        .invoke_for_static_provider_host_owner_v1(
                            RustSdkProviderAdapterInvocationInput::new_for_static_provider_host_owner_v1(
                                request.provider_input(),
                            ),
                        )?;
                    return request
                        .into_execution_result_from_implementation_output_for_static_provider_host_owner_v1(
                            implementation_output,
                        );
                }
                if swarm_event_provider_requires_product_session_boundary(
                    &provider.binding.provider_id(),
                ) {
                    return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                        format!(
                            "Rust SDK provider '{}' execution authority is the direct-run kernel-owned product-session event route (backend session authority EventJournalProductSession, public host route direct_run.provider_execution_authority); the static provider host map cannot execute it, and reaching this wall means the caller bypassed the kernel route for contract {}:{}",
                            provider.binding.provider_id(),
                            request.contract().package_specifier(),
                            request.contract().export_name(),
                        ),
                    ));
                }
                return Err(CapabilitySdkError::InvalidDirectRunProviderRequirement(
                    format!(
                        "Rust SDK provider '{}' is admitted, but no Rust-owned ProviderHost execution authority is installed for contract {}:{}",
                        provider.binding.provider_id(),
                        request.contract().package_specifier(),
                        request.contract().export_name(),
                    ),
                ));
            }
        }
        for family in &mut self.contract_families {
            if family.family.admits_contract(request.contract())? {
                let mut adapter = family
                    .executor
                    .prepare_exact_adapter_operation_for_provider_host_owner_v1(
                        request.contract(),
                        request.request.operation_for_static_provider_host_owner_v1(),
                    )?;
                let implementation_output = adapter
                    .invoke_for_static_provider_host_owner_v1(
                        RustSdkProviderAdapterInvocationInput::new_for_static_provider_host_owner_v1(
                            request.provider_input(),
                        ),
                    )?;
                return request
                    .into_execution_result_from_implementation_output_for_static_provider_host_owner_v1(
                        implementation_output,
                    );
            }
        }
        Err(CapabilitySdkError::NoRustSdkProvider {
            provider_id: request.provider_id().to_owned(),
        })
    }

    pub fn invoke_admitted_request_for_swarm_provider_host_set_owner_v1(
        &mut self,
        request: HostAdmittedTypedProviderRequest,
        context: ProviderHostContext,
    ) -> CapabilitySdkResult<RustSdkStaticProviderExecutionResultForProviderHostOwner> {
        self.invoke_admitted_request(request, context)
    }

    pub fn invoke_selected_provider_boundary_request_for_swarm_provider_host_set_owner_v1(
        &mut self,
        request: SelectedProviderBoundaryHostRequest,
    ) -> CapabilitySdkResult<SelectedProviderBoundaryExecutionResultForProviderHostOwner> {
        let (request, selected_output_authority) = request
            .into_request_and_selected_output_authority_for_static_provider_host_owner_v1()?;
        let context =
            ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )?;
        let result = self.invoke_admitted_request(request, context)?;
        Ok(
            SelectedProviderBoundaryExecutionResultForProviderHostOwner::from_static_provider_host_owner_v1(
                result,
                selected_output_authority,
            ),
        )
    }

    fn provider_binding_for_contract(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<Option<RustSdkStaticProviderBinding>> {
        if let Some(provider) = self.exact_providers.iter().find(|provider| {
            provider
                .binding
                .matches_identity_for_static_provider_host_owner_v1(contract)
        }) {
            return Ok(Some(
                provider
                    .binding
                    .duplicate_for_static_provider_host_owner_v1(),
            ));
        }
        self.provider_for_contract_family(contract)
    }

    fn provider_for_contract_family(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> CapabilitySdkResult<Option<RustSdkStaticProviderBinding>> {
        for family in &self.contract_families {
            if family.family.admits_contract(contract)? {
                return Ok(Some(
                    RustSdkStaticProviderBinding::from_contract_identity_for_static_provider_host_owner_v1(contract)?,
                ));
            }
        }
        Ok(None)
    }

    pub fn installed_contract_for_package_export(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> Option<CapabilityContractIdentity> {
        self.exact_providers.iter().find_map(|provider| {
            let installed = provider
                .binding
                .sealed_identity_for_static_provider_host_owner_v1();
            if let Some(installed) = installed
                && installed.package_specifier() == contract.package_specifier()
                && installed.export_name() == contract.export_name()
            {
                Some(installed)
            } else {
                None
            }
        })
    }

    pub fn installed_contract_for_provider_host_set_owner_projection_v1(
        &self,
        projection: &CapabilityContractProjection,
    ) -> Option<CapabilityContractIdentity> {
        self.exact_providers.iter().find_map(|provider| {
            let installed = provider
                .binding
                .sealed_identity_for_static_provider_host_owner_v1()?;
            if contract_identity_matches_projection(&installed, projection) {
                Some(installed)
            } else {
                None
            }
        })
    }

    pub fn contract_projection_fingerprint_mismatch_for_provider_host_set_owner_v1(
        &self,
        projection: &CapabilityContractProjection,
    ) -> Option<CapabilitySdkError> {
        self.exact_providers.iter().find_map(|provider| {
            let installed = provider
                .binding
                .sealed_identity_for_static_provider_host_owner_v1()?;
            if installed.package_specifier() == projection.package_specifier()
                && installed.export_name() == projection.export_name()
                && !contract_identity_matches_projection(&installed, projection)
            {
                Some(CapabilitySdkError::ContractFingerprintMismatch {
                    package_specifier: projection.package_specifier().to_owned(),
                    export_name: projection.export_name().to_owned(),
                    expected: installed
                        .fingerprint()
                        .map(CapabilityContractFingerprint::as_str)
                        .unwrap_or("<absent>")
                        .to_owned(),
                    observed: projection.fingerprint().unwrap_or("<absent>").to_owned(),
                })
            } else {
                None
            }
        })
    }

    pub fn contract_fingerprint_mismatch(
        &self,
        contract: &CapabilityContractIdentity,
    ) -> Option<CapabilitySdkError> {
        let installed = self.installed_contract_for_package_export(contract)?;
        if installed.fingerprint() == contract.fingerprint() {
            return None;
        }
        Some(
            CapabilitySdkError::NativeBinaryManifestContractFingerprintMismatch {
                package_specifier: contract.package_specifier().to_owned(),
                export_name: contract.export_name().to_owned(),
                expected: installed
                    .fingerprint()
                    .map(CapabilityContractFingerprint::as_str)
                    .unwrap_or("<absent>")
                    .to_owned(),
                observed: contract
                    .fingerprint()
                    .map(CapabilityContractFingerprint::as_str)
                    .unwrap_or("<absent>")
                    .to_owned(),
            },
        )
    }
}

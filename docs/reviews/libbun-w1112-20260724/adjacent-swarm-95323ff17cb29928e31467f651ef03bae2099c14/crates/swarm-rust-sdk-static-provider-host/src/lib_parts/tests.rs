#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        sync::{Arc, Mutex},
    };

    use libswarm_package_graph_contract_source_admission::admit_package_graph_contract_source_from_admitted_source_for_contract_source_owner_v1;
    use libswarm_package_graph_source_model::{
        PackageGraphPackageUniverseAdmission, PackageGraphSessionSourceAdmission,
    };
    use swarm_capability_contract_tson::{
        AdmittedCapabilityContractOperationRegistrationForProviderHostOwnerV1,
        AdmittedCapabilityContractTson,
    };
    use swarm_capability_linker_core::ProviderValue;
    use swarm_rust_sdk_static_provider_listing::RustSdkStaticProviderBinding;

    use super::{
        HostAdmittedTypedProviderRequest, ProviderHostContext, RustSdkBuiltinProviderCatalogue,
        RustSdkProviderAdapterInvocationInput, RustSdkProviderAdapterOperation,
        RustSdkStatelessProviderExecutor,
        RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner,
        RustSdkStaticProviderHostAdmission,
        RustSdkStaticProviderHostAdmissionKind, RustSdkStaticProviderHostAdmissionSet,
        RustSdkStaticProviderHostOwner, RustSdkStaticProviderHostSet,
    };

    const EXACT_OPERATION_DISPATCH_FIXTURE_TSON: &str = r#"
      import { command, defineContract, jsonValue, object, ref } from "@swarm/contract";

      const fixtureCommand = command({
        input: jsonValue(),
        accepted: ref("FixtureAccepted"),
      });

      export const fixture = defineContract({
        schema: "swarm.contract.v1",
        types: {
          FixtureAccepted: object({ value: jsonValue() }),
        },
        objects: {
          Fixture: object({
            fields: {},
            methods: {
              first: fixtureCommand,
              second: fixtureCommand,
            },
          }),
        },
      });
    "#;

    #[derive(Debug)]
    struct RecordingExactOperationExecutor {
        label: &'static str,
        calls: Arc<Mutex<Vec<&'static str>>>,
    }

    impl RustSdkProviderAdapterOperation for RecordingExactOperationExecutor {
        fn duplicate_for_provider_host_owner_v1(&self) -> Box<dyn RustSdkProviderAdapterOperation> {
            Box::new(Self {
                label: self.label,
                calls: Arc::clone(&self.calls),
            })
        }

        fn invoke_adapter_for_provider_host_owner_v1(
            &mut self,
            _input: RustSdkProviderAdapterInvocationInput<'_>,
        ) -> super::CapabilitySdkResult<
            super::RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1,
        > {
            self.calls
                .lock()
                .expect("exact-operation recording lock")
                .push(self.label);
            Ok(HostAdmittedTypedProviderRequest::implementation_output_for_rust_sdk_static_provider_executor_owner_v1(
                ProviderValue::Object(BTreeMap::from([(
                    "value".to_owned(),
                    ProviderValue::String(self.label.to_owned()),
                )]).into()),
            ))
        }
    }

    #[test]
    fn exact_operation_registration_dispatches_by_sealed_contract_operation() {
        let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("static provider host owner");
        let calls = Arc::new(Mutex::new(Vec::new()));
        let registration = owner
            .admit_exact_operation_native_provider_registration_for_product_environment_owner_v1(
                vec![
                    (
                        exact_operation_dispatch_registration("first"),
                        Box::new(RecordingExactOperationExecutor {
                            label: "first",
                            calls: Arc::clone(&calls),
                        }),
                    ),
                    (
                        exact_operation_dispatch_registration("second"),
                        Box::new(RecordingExactOperationExecutor {
                            label: "second",
                            calls: Arc::clone(&calls),
                        }),
                    ),
                ],
            )
            .expect("two exact operation executors should bind to one exact contract");
        let admissions = owner
            .admit_admission_set_for_provider_host_set_owner_v1(vec![
                registration
                    .into_static_provider_host_admission_for_native_provider_composition_owner_v1(),
            ])
            .expect("exact operation admission set");
        let mut host_set =
            RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(admissions)
                .expect("exact operation host set");

        for operation in ["second", "first"] {
            let request = host_set
                .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
                    exact_operation_dispatch_contract(operation),
                    ProviderValue::Null,
                )
                .expect("exact operation request");
            let context = ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )
            .expect("exact operation provider context");
            host_set
                .invoke_admitted_request_for_swarm_provider_host_set_owner_v1(request, context)
                .expect("exact operation executor invocation");
        }

        assert_eq!(
            calls
                .lock()
                .expect("exact-operation recording lock")
                .as_slice(),
            &["second", "first"]
        );
    }

    #[test]
    fn exact_operation_registration_refuses_an_empty_operation_set() {
        let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("static provider host owner");

        match owner
            .admit_exact_operation_native_provider_registration_for_product_environment_owner_v1(
                Vec::new(),
            ) {
            Err(super::CapabilitySdkError::InvalidDirectRunProviderRequirement(_)) => {}
            Err(other) => panic!(
                "empty exact-operation registration must settle as a typed provider-requirement refusal, observed {other:?}"
            ),
            Ok(_) => panic!("empty exact-operation registration must not mint host authority"),
        }
    }

    #[test]
    fn exact_operation_registration_refuses_mixed_contract_identities() {
        let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("static provider host owner");
        let calls = Arc::new(Mutex::new(Vec::new()));
        let first_contract_operation = exact_operation_dispatch_registration("first");
        let second_contract_operation = exact_operation_dispatch_registration_for_package(
            "/workspace/other-exact-operation-dispatch-fixture",
            "@fixture/other-exact-operation-dispatch",
            "second",
        );

        match owner
            .admit_exact_operation_native_provider_registration_for_product_environment_owner_v1(
                vec![
                    (
                        first_contract_operation,
                        Box::new(RecordingExactOperationExecutor {
                            label: "first",
                            calls: Arc::clone(&calls),
                        }) as Box<dyn RustSdkProviderAdapterOperation>,
                    ),
                    (
                        second_contract_operation,
                        Box::new(RecordingExactOperationExecutor {
                            label: "second",
                            calls,
                        }) as Box<dyn RustSdkProviderAdapterOperation>,
                    ),
                ],
            ) {
            Err(super::CapabilitySdkError::InvalidDirectRunProviderRequirement(_)) => {}
            Err(other) => panic!(
                "mixed exact contract identities must settle as a typed provider-requirement refusal, observed {other:?}"
            ),
            Ok(_) => panic!("mixed exact contract identities must not mint host authority"),
        }
    }

    #[test]
    fn exact_operation_registration_refuses_a_duplicate_admitted_operation() {
        let owner = RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("static provider host owner");
        let calls = Arc::new(Mutex::new(Vec::new()));

        match owner
            .admit_exact_operation_native_provider_registration_for_product_environment_owner_v1(
                vec![
                    (
                        exact_operation_dispatch_registration("first"),
                        Box::new(RecordingExactOperationExecutor {
                            label: "first-a",
                            calls: Arc::clone(&calls),
                        }) as Box<dyn RustSdkProviderAdapterOperation>,
                    ),
                    (
                        exact_operation_dispatch_registration("first"),
                        Box::new(RecordingExactOperationExecutor {
                            label: "first-b",
                            calls,
                        }) as Box<dyn RustSdkProviderAdapterOperation>,
                    ),
                ],
            ) {
            Err(super::CapabilitySdkError::InvalidDirectRunProviderRequirement(_)) => {}
            Err(other) => panic!(
                "duplicate exact operation must settle as a typed provider-requirement refusal, observed {other:?}"
            ),
            Ok(_) => panic!("duplicate exact operation must not mint host authority"),
        }
    }

    fn exact_operation_dispatch_registration(
        operation: &str,
    ) -> AdmittedCapabilityContractOperationRegistrationForProviderHostOwnerV1 {
        exact_operation_dispatch_registration_for_package(
            "/workspace/exact-operation-dispatch-fixture",
            "@fixture/exact-operation-dispatch",
            operation,
        )
    }

    fn exact_operation_dispatch_registration_for_package(
        package_root: &str,
        module_specifier: &str,
        operation: &str,
    ) -> AdmittedCapabilityContractOperationRegistrationForProviderHostOwnerV1 {
        let contract_source =
            exact_operation_dispatch_contract_source_for_package(package_root, module_specifier);
        let command = contract_source
            .admit_command_operation_for_package_graph_provider_requirements_owner_v1(operation)
            .expect("fixture exact command operation");
        AdmittedCapabilityContractTson::admit_exact_command_operation_registration_from_package_graph_contract_source_for_provider_host_owner_v1(
            contract_source,
            command,
        )
        .expect("fixture exact operation registration")
    }

    fn exact_operation_dispatch_contract(operation: &str) -> AdmittedCapabilityContractTson {
        let contract_source = exact_operation_dispatch_contract_source();
        let contract_tson = contract_source.contract_tson_for_contract_tson_owner_v1();
        AdmittedCapabilityContractTson::admit_command_accepted_package_contract_tson_for_contract_tson_owner_v1(
            contract_source.package_identity_for_capability_linker_owner_v1(),
            &contract_tson,
            "Fixture",
            operation,
        )
        .expect("fixture exact operation Contract-TSON")
    }

    fn exact_operation_dispatch_contract_source()
    -> libswarm_package_graph_contract_source_admission::AdmittedPackageGraphContractSource {
        exact_operation_dispatch_contract_source_for_package(
            "/workspace/exact-operation-dispatch-fixture",
            "@fixture/exact-operation-dispatch",
        )
    }

    fn exact_operation_dispatch_contract_source_for_package(
        package_root: &str,
        module_specifier: &str,
    ) -> libswarm_package_graph_contract_source_admission::AdmittedPackageGraphContractSource {
        let package_universe =
            PackageGraphPackageUniverseAdmission::admit_for_package_graph_owner_v1(vec![
                package_root.to_owned(),
            ])
            .expect("exact-operation fixture package universe");
        let source_admission =
            PackageGraphSessionSourceAdmission::admit_for_package_graph_owner_v1(
                format!("{package_root}/src/fixture.tson.ts"),
                package_root,
                &package_universe,
            )
            .expect("exact-operation fixture source admission");
        admit_package_graph_contract_source_from_admitted_source_for_contract_source_owner_v1(
            &source_admission,
            module_specifier,
            EXACT_OPERATION_DISPATCH_FIXTURE_TSON,
        )
        .expect("exact-operation fixture contract source")
    }

    // #131 rung-4 L2 CALIBRATION: the 56 first-party builtins resolve IDENTICALLY
    // through the dangling refusal — every executor-less builtin (event
    // product-session ops, @swarm/actors kernel session-internal ops, and the
    // @swarm/mesh/* families in the baseline coverage set) is exempt, so the full
    // builtin, product-binary (mesh + process families), and ss-test-lane
    // admission sets construct WITHOUT firing the refusal. If any of the 56
    // unexpectedly fired, one of these `new`-backed constructions would Err.
    #[test]
    fn all_56_builtins_pass_the_dangling_refusal_at_admission() {
        let owner = super::RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("product binary static provider host owner");
        // Baseline builtin admission set (mesh/event/actors executor-less Exact
        // coverage bindings present) — new() runs the dangling refusal.
        owner
            .admit_builtin_static_provider_host_admissions_for_provider_host_set_owner_v1()
            .expect("the 56 builtins must all be executor-backed or session-internal exempt");
        // ss-test lane baseline.
        owner
            .admit_test_mode_static_provider_host_admissions_for_provider_host_set_owner_v1()
            .expect("the ss-test lane builtins must all pass the dangling refusal");
        // Product-binary composition with the mesh + process family executors.
        let mesh_executor = || -> Box<dyn super::RustSdkProviderAdapterOperation> {
            Box::new(RustSdkStatelessProviderExecutor::new_v1(
                provider_result_body_fixture_executor,
            ))
        };
        let mut native = owner
            .admit_mesh_control_contract_family_executions_for_mesh_capability_host_owner_v1(
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
            )
            .expect("mesh contract family execution admissions");
        native.extend(
            owner
                .admit_process_run_contract_family_execution_for_direct_run_kernel_owner_v1(
                    mesh_executor(),
                )
                .expect("process run contract family execution admission"),
        );
        owner
            .admit_builtin_static_provider_host_admissions_with_installed_native_host_admissions_for_product_binary_owner_v1(
                native,
            )
            .expect("the composed product-binary admission set must pass the dangling refusal");
    }

    #[test]
    fn builtin_catalogue_debug_redacts_provider_binding_authority() {
        let catalogue =
            RustSdkBuiltinProviderCatalogue::builtin_for_static_provider_host_owner_v1()
                .expect("builtin catalogue");
        let debug = format!("{catalogue:?}");

        assert!(debug.contains("RustSdkBuiltinProviderCatalogue"));
        assert!(debug.contains("provider_count"));
        assert!(debug.contains("hidden_provider_binding_authority"));
        assert!(!debug.contains("@swarm/event"));
        assert!(!debug.contains("publishEvent"));
        assert!(!debug.contains("sha256:"));
    }

    #[test]
    fn provider_execution_result_body_fixture_consumes_admitted_request() {
        let contract = admitted_contract_tson_fixture_for_provider_host_owner();
        let provider_id = contract.identity().provider_id();
        let mut host_set = host_set_with_result_body_fixture_executor(&contract);
        let request = host_set
            .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
                contract,
                ProviderValue::Object(BTreeMap::from([(
                    "prompt".to_owned(),
                    ProviderValue::String("hello".to_owned()),
                )]).into()),
            )
            .expect("provider-host typed request should admit from Contract-TSON authority");
        let context =
            ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )
            .expect("provider-host context");

        let result = host_set
            .invoke_admitted_request_for_swarm_provider_host_set_owner_v1(request, context)
            .expect("provider-host executor should consume admitted request");
        let (ready_output, host_resource_releases) =
            result.into_ready_output_for_static_provider_host_owner_v1();
        host_resource_releases
            .finish_for_session_execution_kernel_owner_v1()
            .expect("result-body fixture must not retain host-resource release authority");
        let closed_sum = ready_output
            .into_contract_output_for_provider_drive_result_owner_v1()
            .into_closed_sum_contract_output_for_provider_host_owner_v1()
            .expect("provider-host result body should carry closed-sum contract authority");
        assert_eq!(closed_sum.provider_id(), provider_id);

        let RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
            provider_id: _,
            contract: _,
            closed_sum_output_type,
            output_body,
        } = closed_sum;
        let variant = output_body
            .into_closed_sum_variant_for_provider_host_owner_v1(
                closed_sum_output_type,
                "FixtureResult",
            )
            .expect("closed-sum body should consume explicit carrier truth");
        assert_eq!(variant.variant, "FixtureOk");
        assert!(matches!(
            variant.payload,
            Some(ProviderValue::Object(fields))
                if matches!(
                    fields.get("text"),
                    Some(ProviderValue::String(value)) if value == "fixture ready"
                )
        ));
    }

    #[test]
    fn exact_contract_tson_result_ok_settles_only_the_accepted_payload() {
        let output =
            exact_std_result_closed_sum_contract_output(ProviderValue::Object(BTreeMap::from([
                ("kind".to_owned(), ProviderValue::String("ok".to_owned())),
                (
                    "value".to_owned(),
                    ProviderValue::String("accepted payload".to_owned()),
                ),
            ]).into()));
        let (pending, selected) =
            swarm_capability_model::mint_provider_boundary_output_correspondence_v1();
        let ready = super::RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_closed_sum_for_provider_drive_result_owner_v1(output)
            .into_provider_ready_boundary_output_for_selected_boundary_owner_v1(selected)
            .expect("exact Contract-TSON Result ok must settle at the provider-host boundary");
        let correlated = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("provider-boundary halves must correspond");
        let settlement = correlated
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect("exact Contract-TSON Result must retain typed settlement authority");
        assert_eq!(
            settlement
                .into_accepted_payload_for_session_runtime_owner_v1()
                .expect("ok must be an accepted settlement"),
            ProviderValue::String("accepted payload".to_owned()),
        );
    }

    #[test]
    fn exact_contract_tson_result_err_settles_only_the_rejected_payload() {
        let output =
            exact_std_result_closed_sum_contract_output(ProviderValue::Object(BTreeMap::from([
                (
                    "error".to_owned(),
                    ProviderValue::String("rejected payload".to_owned()),
                ),
                ("kind".to_owned(), ProviderValue::String("err".to_owned())),
            ]).into()));
        let (pending, selected) =
            swarm_capability_model::mint_provider_boundary_output_correspondence_v1();
        let ready = super::RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_closed_sum_for_provider_drive_result_owner_v1(output)
            .into_provider_ready_boundary_output_for_selected_boundary_owner_v1(selected)
            .expect("exact Contract-TSON Result err must settle at the provider-host boundary");
        let correlated = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("provider-boundary halves must correspond");
        let settlement = correlated
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect("exact Contract-TSON Result must retain typed settlement authority");
        assert_eq!(
            settlement
                .into_accepted_payload_for_session_runtime_owner_v1()
                .expect_err("err must be a rejected settlement")
                .into_payload_for_session_runtime_owner_v1(),
            ProviderValue::String("rejected payload".to_owned()),
        );
    }

    #[test]
    fn non_result_closed_sum_with_result_shaped_object_remains_authored_cargo() {
        let contract = admitted_contract_tson_fixture_for_provider_host_owner();
        let output = closed_sum_contract_output_from_admitted_contract(
            contract,
            ProviderValue::Object(BTreeMap::from([
                ("kind".to_owned(), ProviderValue::String("ok".to_owned())),
                (
                    "value".to_owned(),
                    ProviderValue::String("authored cargo".to_owned()),
                ),
            ]).into()),
        );
        let (pending, selected) =
            swarm_capability_model::mint_provider_boundary_output_correspondence_v1();
        let ready = super::RustSdkStaticProviderOutputForSessionRuntimeOwnerV1::from_closed_sum_for_provider_drive_result_owner_v1(output)
            .into_provider_ready_boundary_output_for_selected_boundary_owner_v1(selected)
            .expect("non-Result closed sums remain authored provider cargo");
        let authored = pending
            .consume_corresponded_ready_output_for_provider_boundary_owner_v1(ready)
            .expect("provider-boundary halves must correspond")
            .into_result_settlement_for_session_runtime_owner_v1()
            .expect_err("a raw tag-shaped object cannot mint Result settlement authority")
            .into_payload_for_session_runtime_owner_v1();
        assert!(matches!(
            authored,
            ProviderValue::Object(fields)
                if matches!(fields.get("kind"), Some(ProviderValue::String(kind)) if kind == "ok")
                    && matches!(fields.get("value"), Some(ProviderValue::String(value)) if value == "authored cargo")
        ));
    }

    #[test]
    fn mesh_provider_settlement_source_yields_one_sealed_item_then_done() {
        let contract = admitted_contract_tson_fixture_for_provider_host_owner();
        let mut host_set = host_set_with_result_body_fixture_executor(&contract);
        let request = host_set
            .admit_typed_request_for_provider_host_set_owner_contract_tson_v1(
                contract,
                ProviderValue::Object(BTreeMap::from([(
                    "prompt".to_owned(),
                    ProviderValue::String("hello".to_owned()),
                )]).into()),
            )
            .expect("provider-host typed request");
        let context =
            ProviderHostContext::from_admitted_request_for_swarm_rust_sdk_static_provider_host_owner_v1(
                &request,
            )
            .expect("provider-host context");
        let result = host_set
            .invoke_admitted_request_for_swarm_provider_host_set_owner_v1(request, context)
            .expect("provider-host executor result");
        let (initial_observation, mut source) = result
            .into_ready_output_observation_and_mesh_provider_settlement_stream_source_for_mesh_capability_host_owner_v1();

        let item = match source.next_for_mesh_capability_host_owner_v1() {
            super::MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1::Item(item) => item,
            other => panic!("first source advance must yield Item, observed {other:?}"),
        };
        assert_eq!(
            item.output_observation_for_mesh_capability_host_owner_v1()
                .output_fingerprint_for_mesh_capability_host_owner_v1(),
            initial_observation.output_fingerprint_for_mesh_capability_host_owner_v1(),
        );
        assert!(matches!(
            source.next_for_mesh_capability_host_owner_v1(),
            super::MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1::Done(_),
        ));
        assert!(matches!(
            source.next_for_mesh_capability_host_owner_v1(),
            super::MeshProviderSettlementStreamStepForMeshCapabilityHostOwnerV1::Done(_),
        ));
    }

    fn admitted_contract_tson_fixture_for_provider_host_owner() -> AdmittedCapabilityContractTson {
        let package_root = "/workspace/r8219-provider-fixture";
        let contract_specifier = "@r8219/provider-fixture";
        let export_name = "fixtureProvider";
        let source = format!(
            r#"
              import {{ command, defineContract, object, string, union, ref }} from "@swarm/contract";

              export const {export_name} = defineContract({{
                schema: "swarm.contract.v1",
                types: {{
                  FixtureOk: object({{ text: string() }}),
                  FixtureErr: object({{ message: string() }}),
                  FixtureResult: union([ref("FixtureOk"), ref("FixtureErr")]),
                }},
                objects: {{
                  {export_name}: object({{
                    fields: {{}},
                    methods: {{
                      run: command({{
                        input: object({{ prompt: string() }}),
                        accepted: ref("FixtureResult"),
                      }}),
                    }},
                  }}),
                }},
              }});
            "#
        );
        let package_universe =
            PackageGraphPackageUniverseAdmission::admit_for_package_graph_owner_v1(vec![
                package_root.to_owned(),
            ])
            .expect("provider fixture package universe admission");
        let source_admission =
            PackageGraphSessionSourceAdmission::admit_for_package_graph_owner_v1(
                format!("{package_root}/src/provider.tson.ts"),
                package_root,
                &package_universe,
            )
            .expect("provider fixture package source admission");
        let contract_source =
            admit_package_graph_contract_source_from_admitted_source_for_contract_source_owner_v1(
                &source_admission,
                contract_specifier,
                &source,
            )
            .expect("provider fixture package contract source admission");
        let contract_tson = contract_source.contract_tson_for_contract_tson_owner_v1();
        let package_identity = contract_source.package_identity_for_capability_linker_owner_v1();
        AdmittedCapabilityContractTson::admit_command_accepted_package_contract_tson_for_contract_tson_owner_v1(
            package_identity,
            &contract_tson,
            export_name,
            "run",
        )
        .expect("provider fixture admitted capability Contract-TSON")
    }

    fn admitted_exact_std_result_contract_tson_for_provider_host_owner()
    -> AdmittedCapabilityContractTson {
        let package_root = "/workspace/exact-std-result-provider-fixture";
        let contract_specifier = "@fixture/exact-std-result-provider";
        let export_name = "resultProvider";
        let source = format!(
            r#"
              import {{ command, defineContract, literal, object, ref, string, union }} from "@swarm/contract";

              export const {export_name} = defineContract({{
                schema: "swarm.contract.v1",
                types: {{
                  ok: object({{ kind: literal("ok"), value: string() }}),
                  err: object({{ kind: literal("err"), error: string() }}),
                  FixtureResult: union([ref("ok"), ref("err")]),
                }},
                objects: {{
                  {export_name}: object({{
                    fields: {{}},
                    methods: {{
                      run: command({{
                        input: object({{ prompt: string() }}),
                        accepted: ref("FixtureResult"),
                      }}),
                    }},
                  }}),
                }},
              }});
            "#
        );
        let package_universe =
            PackageGraphPackageUniverseAdmission::admit_for_package_graph_owner_v1(vec![
                package_root.to_owned(),
            ])
            .expect("exact Result provider fixture package universe admission");
        let source_admission =
            PackageGraphSessionSourceAdmission::admit_for_package_graph_owner_v1(
                format!("{package_root}/src/provider.tson.ts"),
                package_root,
                &package_universe,
            )
            .expect("exact Result provider fixture package source admission");
        let contract_source =
            admit_package_graph_contract_source_from_admitted_source_for_contract_source_owner_v1(
                &source_admission,
                contract_specifier,
                &source,
            )
            .expect("exact Result provider fixture contract source admission");
        let contract_tson = contract_source.contract_tson_for_contract_tson_owner_v1();
        let package_identity = contract_source.package_identity_for_capability_linker_owner_v1();
        AdmittedCapabilityContractTson::admit_command_accepted_package_contract_tson_for_contract_tson_owner_v1(
            package_identity,
            &contract_tson,
            export_name,
            "run",
        )
        .expect("exact Result provider fixture admitted capability Contract-TSON")
    }

    fn exact_std_result_closed_sum_contract_output(
        output: ProviderValue,
    ) -> RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
        closed_sum_contract_output_from_admitted_contract(
            admitted_exact_std_result_contract_tson_for_provider_host_owner(),
            output,
        )
    }

    fn closed_sum_contract_output_from_admitted_contract(
        contract: AdmittedCapabilityContractTson,
        output: ProviderValue,
    ) -> RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
        let provider_id = contract.identity().provider_id();
        let contract_identity = contract.identity().duplicate_for_capability_model_owner();
        let (_operation, output_type_contract) = contract
            .into_operation_and_output_type_contract_authority_for_provider_host_owner_v1()
            .expect("fixture operation output contract authority");
        let closed_sum_output_type = output_type_contract
            .into_closed_sum_output_type_for_provider_host_owner_v1()
            .expect("fixture output must be a closed sum");
        let output_body =
            super::RustSdkStaticProviderClosedSumOutputBodyForProviderHostOwner::from_provider_host_owner_contract_output_v1(
                provider_id.clone(),
                contract_identity.duplicate_for_capability_model_owner(),
                "owner-test-output-fingerprint".to_owned(),
                output,
            );
        RustSdkStaticProviderClosedSumContractOutputForProviderHostOwner {
            provider_id,
            contract: contract_identity,
            closed_sum_output_type,
            output_body,
        }
    }

    fn host_set_with_result_body_fixture_executor(
        contract: &AdmittedCapabilityContractTson,
    ) -> RustSdkStaticProviderHostSet {
        let binding =
            RustSdkStaticProviderBinding::from_contract_identity_for_static_provider_host_owner_v1(
                contract.identity(),
            )
            .expect("provider fixture binding");
        let admissions =
            RustSdkStaticProviderHostAdmissionSet::new(vec![RustSdkStaticProviderHostAdmission {
                owner_namespace: super::SS_PRODUCT_BINARY_STATIC_PROVIDER_HOST_OWNER_NAMESPACE
                    .to_owned(),
                kind: RustSdkStaticProviderHostAdmissionKind::Exact {
                    binding,
                    executor: Some(Box::new(RustSdkStatelessProviderExecutor::new_v1(
                        provider_result_body_fixture_executor,
                    ))),
                },
            }])
            .expect("provider fixture admission set");
        RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(admissions)
            .expect("provider fixture host set")
    }

    fn provider_result_body_fixture_executor(
        _operation: &str,
        _input: RustSdkProviderAdapterInvocationInput<'_>,
    ) -> super::CapabilitySdkResult<super::RustSdkStaticProviderImplementationOutputForProviderHostOwnerV1>
    {
        Ok(HostAdmittedTypedProviderRequest::implementation_output_for_rust_sdk_static_provider_executor_owner_v1(
            closed_sum_fixture_provider_output(),
        ))
    }

    fn closed_sum_fixture_provider_output() -> ProviderValue {
        ProviderValue::Object(BTreeMap::from([
            (
                swarmscript_types::CLOSED_SUM_CARRIER_SYMBOL_FIELD.to_owned(),
                ProviderValue::String("FixtureResult".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_VARIANT_FIELD.to_owned(),
                ProviderValue::String("FixtureOk".to_owned()),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_HAS_PAYLOAD_FIELD.to_owned(),
                ProviderValue::Bool(true),
            ),
            (
                swarmscript_types::CLOSED_SUM_CARRIER_PAYLOAD_FIELD.to_owned(),
                ProviderValue::Object(BTreeMap::from([(
                    "text".to_owned(),
                    ProviderValue::String("fixture ready".to_owned()),
                )]).into()),
            ),
        ]).into())
    }

    // #107: the five mesh contract families in the static-provider-inventory
    // owner must correspond exactly to the sealed builtin catalogue mesh
    // definitions: every catalogue mesh binding is admitted by exactly one
    // family, and no family admits an export the catalogue does not define.
    #[test]
    fn mesh_contract_families_correspond_to_builtin_catalogue_definitions() {
        let families = [
            super::swarm_mesh_identity_contract_family_for_static_provider_host_owner_v1()
                .expect("mesh identity family"),
            super::swarm_mesh_connection_contract_family_for_static_provider_host_owner_v1()
                .expect("mesh connection family"),
            super::swarm_mesh_provider_contract_family_for_static_provider_host_owner_v1()
                .expect("mesh provider family"),
            super::swarm_mesh_operation_contract_family_for_static_provider_host_owner_v1()
                .expect("mesh operation family"),
            super::swarm_mesh_actor_contract_family_for_static_provider_host_owner_v1()
                .expect("mesh actor family"),
        ];
        let catalogue_mesh_bindings =
            RustSdkBuiltinProviderCatalogue::builtin_for_static_provider_host_owner_v1()
                .expect("builtin catalogue")
                .into_provider_bindings()
                .into_iter()
                .filter(|binding| binding.package_specifier().starts_with("@swarm/mesh/"))
                .collect::<Vec<_>>();
        assert!(
            !catalogue_mesh_bindings.is_empty(),
            "the builtin catalogue must define the mesh contract families"
        );
        let mut family_export_totals = 0usize;
        for family in &families {
            family_export_totals += family
                .export_names_for_static_provider_host_owner_v1()
                .expect("mesh families are finite export families")
                .len();
        }
        assert_eq!(
            family_export_totals,
            catalogue_mesh_bindings.len(),
            "mesh family export sets must cover exactly the catalogue mesh definitions"
        );
        for binding in &catalogue_mesh_bindings {
            let admitting_families = families
                .iter()
                .filter(|family| {
                    family.package_specifier() == binding.package_specifier()
                        && family
                            .admits_export_for_static_provider_host_owner_v1(binding.export_name())
                })
                .count();
            assert_eq!(
                admitting_families,
                1,
                "catalogue mesh binding {}:{} must be admitted by exactly one mesh family",
                binding.package_specifier(),
                binding.export_name(),
            );
        }
    }

    // #107: the builtin-plus-native assembly supersedes exactly the
    // executor-less Exact coverage bindings of the incoming contract-family
    // packages; the resulting set still admits the mesh packages (now with
    // execution authority) and every other builtin package is untouched.
    #[test]
    fn builtin_admissions_with_mesh_native_families_supersede_executorless_exact_bindings() {
        let owner = super::RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("product binary static provider host owner");
        let mesh_executor = || -> Box<dyn super::RustSdkProviderAdapterOperation> {
            Box::new(RustSdkStatelessProviderExecutor::new_v1(
                provider_result_body_fixture_executor,
            ))
        };
        let native_admissions = owner
            .admit_mesh_control_contract_family_executions_for_mesh_capability_host_owner_v1(
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
                mesh_executor(),
            )
            .expect("mesh contract family execution admissions");
        assert_eq!(native_admissions.len(), 5);

        let baseline = RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            owner
                .admit_builtin_static_provider_host_admissions_for_provider_host_set_owner_v1()
                .expect("builtin admissions"),
        )
        .expect("builtin host set");
        let composed = RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            owner
                .admit_builtin_static_provider_host_admissions_with_installed_native_host_admissions_for_product_binary_owner_v1(
                    native_admissions,
                )
                .expect("builtin admissions with mesh native families"),
        )
        .expect("composed host set");

        for package in [
            "@swarm/mesh/identity",
            "@swarm/mesh/connection",
            "@swarm/mesh/provider",
            "@swarm/mesh/operation",
            "@swarm/mesh/actor",
        ] {
            assert!(
                composed.admits_package_specifier_v1(package),
                "composed host set must admit {package}"
            );
        }
        assert!(baseline.admits_package_specifier_v1("@swarm/mesh/connection"));
        assert_eq!(
            baseline.provider_count(),
            composed.provider_count(),
            "the mesh families replace exactly the superseded executor-less exact bindings"
        );
    }

    // #132 phase-1 (audit R7, rides R6): `@swarm/bindings:addSource` and
    // `@swarm/capabilities:acquire` are declared in the builtin catalogue (they
    // feed import/contract/manifest resolution through the package-graph
    // bridges) but must NOT be admitted as externally-executable static
    // providers. Their execution owner is the (dormant) mesh-capability-host
    // resource-capability owner (#132 phase-2, blocked on #124/R-B), so
    // admitting them executor-less would surface a silent
    // `InvalidDirectRunProviderRequirement` no-authority wall. Poisoning host
    // admission instead settles residual host-execution demand as the typed
    // `NoRustSdkProvider` fault (the require_exact_contract / admit_typed_request
    // binding lookup returns NoRustSdkProvider for an unadmitted provider before
    // invoke is ever reached). This asserts the filter fires: both providers are
    // present in the raw catalogue but absent from the admitted host set.
    #[test]
    fn capability_owner_bindings_are_filtered_from_static_provider_host_admission() {
        let owner = super::RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("product binary static provider host owner");
        let host_set = RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            owner
                .admit_builtin_static_provider_host_admissions_for_provider_host_set_owner_v1()
                .expect("builtin admissions"),
        )
        .expect("builtin host set");

        // Present in the raw catalogue (declaration is untouched — filtered, not
        // absent): each module carries exactly its single provider export.
        let catalogue_bindings =
            RustSdkBuiltinProviderCatalogue::builtin_for_static_provider_host_owner_v1()
                .expect("builtin catalogue")
                .into_provider_bindings();
        for (package, export) in [
            ("@swarm/bindings", "addSource"),
            ("@swarm/capabilities", "acquire"),
        ] {
            assert!(
                catalogue_bindings.iter().any(|binding| {
                    binding.package_specifier() == package && binding.export_name() == export
                }),
                "the builtin catalogue must still declare {package}:{export}"
            );
        }

        // Absent from host-set execution admission: the doctrine filter poisons
        // both, so any residual host-execution demand settles as
        // NoRustSdkProvider (not the admitted-but-inexecutable
        // InvalidDirectRunProviderRequirement wall).
        assert!(
            !host_set.admits_package_specifier_v1("@swarm/bindings"),
            "@swarm/bindings:addSource must be filtered from static provider host admission (#132 phase-1)"
        );
        assert!(
            !host_set.admits_package_specifier_v1("@swarm/capabilities"),
            "@swarm/capabilities:acquire must be filtered from static provider host admission (#132 phase-1)"
        );
    }

    // #118: the @swarm/process:run contract family must correspond exactly to
    // the sealed builtin catalogue definition of @swarm/process:run, and must
    // NOT admit any other @swarm/process operation: those stay
    // session-internal direct-run kernel targets under the R41021 doctrine.
    #[test]
    fn process_run_contract_family_corresponds_to_builtin_catalogue_run_definition() {
        let family = super::swarm_process_run_contract_family_for_static_provider_host_owner_v1()
            .expect("process run family");
        assert_eq!(family.package_specifier(), "@swarm/process");
        assert_eq!(
            family
                .export_names_for_static_provider_host_owner_v1()
                .expect("process run family is a finite export family"),
            vec!["run".to_owned()],
            "the process family admits exactly the run export"
        );

        let catalogue_process_bindings =
            RustSdkBuiltinProviderCatalogue::builtin_for_static_provider_host_owner_v1()
                .expect("builtin catalogue")
                .into_provider_bindings()
                .into_iter()
                .filter(|binding| binding.package_specifier() == "@swarm/process")
                .collect::<Vec<_>>();
        let run_definitions = catalogue_process_bindings
            .iter()
            .filter(|binding| binding.export_name() == "run")
            .count();
        assert_eq!(
            run_definitions, 1,
            "the builtin catalogue must define @swarm/process:run exactly once"
        );
        for binding in &catalogue_process_bindings {
            let admitted =
                family.admits_export_for_static_provider_host_owner_v1(binding.export_name());
            assert_eq!(
                admitted,
                binding.export_name() == "run",
                "process family must admit run and only run; {} violated the R41021 boundary",
                binding.export_name(),
            );
        }
    }

    // #118: unlike the mesh packages, the R41021 filter keeps every
    // @swarm/process catalogue binding out of host-set execution admission,
    // so the process:run family supersedes NOTHING: it is the only
    // @swarm/process host-set entry and adds exactly one provider to both
    // the builtin and the test-mode (ss-test lane) assemblies.
    #[test]
    fn builtin_and_test_mode_admissions_with_process_run_native_family_install_execution_over_r41021_filtered_package()
     {
        let owner = super::RustSdkStaticProviderHostOwner::admit_for_ss_product_binary_owner_v1()
            .expect("product binary static provider host owner");
        let process_executor = || -> Box<dyn super::RustSdkProviderAdapterOperation> {
            Box::new(RustSdkStatelessProviderExecutor::new_v1(
                provider_result_body_fixture_executor,
            ))
        };
        let native_admissions = owner
            .admit_process_run_contract_family_execution_for_direct_run_kernel_owner_v1(
                process_executor(),
            )
            .expect("process run contract family execution admission");
        assert_eq!(native_admissions.len(), 1);

        let baseline = RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            owner
                .admit_builtin_static_provider_host_admissions_for_provider_host_set_owner_v1()
                .expect("builtin admissions"),
        )
        .expect("builtin host set");
        assert!(
            !baseline.admits_package_specifier_v1("@swarm/process"),
            "R41021 keeps @swarm/process out of builtin host-set execution admission"
        );
        let composed = RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
            owner
                .admit_builtin_static_provider_host_admissions_with_installed_native_host_admissions_for_product_binary_owner_v1(
                    native_admissions,
                )
                .expect("builtin admissions with process run native family"),
        )
        .expect("composed host set");
        assert!(
            composed.admits_package_specifier_v1("@swarm/process"),
            "composed host set must admit @swarm/process"
        );
        assert_eq!(
            baseline.provider_count() + 1,
            composed.provider_count(),
            "the process run family supersedes no exact binding; it adds exactly the run provider"
        );

        let test_mode_baseline =
            RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
                owner
                    .admit_test_mode_static_provider_host_admissions_for_provider_host_set_owner_v1(
                    )
                    .expect("test-mode admissions"),
            )
            .expect("test-mode host set");
        assert!(!test_mode_baseline.admits_package_specifier_v1("@swarm/process"));
        let test_mode_composed =
            RustSdkStaticProviderHostSet::from_provider_host_set_owner_admission_set_v1(
                owner
                    .admit_test_mode_static_provider_host_admissions_with_installed_native_host_admissions_for_product_binary_owner_v1(
                        owner
                            .admit_process_run_contract_family_execution_for_direct_run_kernel_owner_v1(
                                process_executor(),
                            )
                            .expect("process run contract family execution admission"),
                    )
                    .expect("test-mode admissions with process run native family"),
            )
            .expect("test-mode composed host set");
        assert!(
            test_mode_composed.admits_package_specifier_v1("@swarm/process"),
            "the ss-test lane assembly must admit @swarm/process once the family is installed"
        );
        assert!(test_mode_composed.admits_package_specifier_v1("@swarm/test"));
        assert_eq!(
            test_mode_baseline.provider_count() + 1,
            test_mode_composed.provider_count(),
        );
    }
}

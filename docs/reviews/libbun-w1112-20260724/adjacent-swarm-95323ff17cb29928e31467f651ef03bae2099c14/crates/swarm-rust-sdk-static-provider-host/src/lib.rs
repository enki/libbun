#![forbid(unsafe_code)]

// compiler-custody-scope: status=complete reviewer=package-root-compiler-kernel-hardcut-20260722 justification="all Rust source in this assigned owner was reviewed; every lexical custody candidate is classified adjacent to its item"

use std::{
    collections::BTreeMap,
    fmt,
    io::{self, BufRead, Write},
};

use prepared_runtime_image_manifest_model::{
    ManifestResolvedExternalProviderCallAdmission, ManifestResolvedExternalProviderCallAuthority,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swarm_capability_contract_tson::{
    AdmittedCapabilityContractOperationDescriptorForProviderRouteOwnerV1,
    AdmittedCapabilityContractOperationRegistrationForProviderHostOwnerV1,
    AdmittedCapabilityContractTson,
};
use swarm_capability_linker_core::{
    AuthoredResourceValue, CapabilityContractClosedSumOutputTypeForProviderHostOwner,
    CapabilityContractOutputTypeContractAuthorityProduct, CapabilityTypeContractError,
    ProviderValue, SwarmInteger, provider_value_to_canonical_json_v1,
    provider_value_to_canonical_output_observation_json_v1,
};
pub use swarm_capability_model::RUST_SDK_PROVIDER_HOST_ID;
pub(crate) use swarm_capability_model::{
    CapabilityContractFingerprint, CapabilityContractIdentity, CapabilityContractProjection,
    CapabilitySdkError, CapabilitySdkResult, LOADED_NATIVE_PROVIDER_HOST_KIND,
    RUST_SDK_PROVIDER_DOMAIN, RUST_SDK_PROVIDER_HOST_KIND,
    RustSdkStaticProviderInstalledNativeHostAdmission,
};
use swarm_capability_model::{
    ProviderReadyBoundaryOutput, SelectedProviderBoundaryOutputAuthority,
};
use swarm_provider_value_model::ProviderValueObject;
pub use swarm_rust_sdk_static_provider_listing::swarm_event_provider_requires_product_session_boundary;
use swarm_rust_sdk_static_provider_listing::{
    RustSdkBuiltinProviderCatalogue, RustSdkInstalledStaticProviderInventoryForPackageGraphOwner,
    RustSdkStaticProviderBinding, RustSdkStaticProviderContractFamily,
    RustSdkStaticProviderContractFamilyDefinition, RustSdkStaticProviderListing,
    SWARM_ACTOR_MODULE_ID, SWARM_ACTOR_START_EXPORT, SWARM_BINDINGS_ADD_SOURCE_EXPORT,
    SWARM_BINDINGS_ADD_SOURCE_PROVIDER_ID, SWARM_BINDINGS_MODULE_ID,
    SWARM_CAPABILITIES_ACQUIRE_EXPORT, SWARM_CAPABILITIES_ACQUIRE_PROVIDER_ID,
    SWARM_CAPABILITIES_MODULE_ID, SWARM_EVENT_MINT_OBJECT_SOURCE_REF_EXPORT, SWARM_EVENT_MODULE_ID,
    SWARM_HOST_MATERIALIZE_PROGRAM_PROVIDER_ID, SWARM_IO_IO_EXPORT, SWARM_IO_MODULE_ID,
    SWARM_LIQUID_MODULE_ID, SWARM_LIQUID_RENDER_EXPORT, SWARM_PROCESS_MODULE_ID,
    SWARM_TEST_EXPECT_CONTAINS_EVENT_PROVIDER_ID, SWARM_TEST_EXPECT_CONTAINS_PROVIDER_ID,
    SWARM_TEST_EXPECT_EQUAL_PROVIDER_ID, SWARM_TEST_MODULE_SPECIFIER,
    SWARM_TEST_PROCESS_OUTPUT_PROVIDER_ID, SWARM_TEST_READ_OUTPUT_PROVIDER_ID,
    SWARM_TEST_READ_SCOPED_RESOURCE_RELEASE_PROVIDER_ID, SWARM_TEST_READ_TEXT_FILE_PROVIDER_ID,
    SWARM_TEST_SKIP_PROVIDER_ID, SWARM_TEST_TEMP_DURABILITY_ROOT_PROVIDER_ID,
    SWARM_TEST_TEMP_PACKAGE_HOME_PROVIDER_ID, SWARM_TEST_TEMP_PROCESS_SCOPED_RESOURCE_PROVIDER_ID,
    SWARM_TEST_TEMP_ROOT_PROVIDER_ID, SWARM_TEST_TEMP_SCOPED_RESOURCE_PROVIDER_ID,
    SWARM_TEST_TEST_PROVIDER_ID, SWARM_TEST_TODO_PROVIDER_ID,
    SWARM_TEST_WRITE_EXECUTABLE_FILE_PROVIDER_ID, SWARM_TEST_WRITE_TEXT_FILE_PROVIDER_ID,
    swarm_mesh_actor_contract_family_for_static_provider_host_owner_v1,
    swarm_mesh_connection_contract_family_for_static_provider_host_owner_v1,
    swarm_mesh_identity_contract_family_for_static_provider_host_owner_v1,
    swarm_mesh_operation_contract_family_for_static_provider_host_owner_v1,
    swarm_mesh_provider_contract_family_for_static_provider_host_owner_v1,
    swarm_process_run_contract_family_for_static_provider_host_owner_v1,
};
pub use swarm_rust_sdk_static_provider_package_graph::{
    RustSdkStaticManifestProviderBridgeForPackageGraphOwner,
    RustSdkStaticProviderRequirementForPackageGraphOwner,
    RustSdkStaticTestManifestProviderBridgeForPackageGraphOwner,
    RustSdkStaticTestProviderRequirementForPackageGraphOwner,
};
mod direct_run_provider_target;
pub use direct_run_provider_target::{
    AdmittedRustSdkExecutableProviderOperationForDirectRun,
    AdmittedRustSdkExecutableProviderTargetForDirectRun,
    DirectRunRustSdkExecutableProviderOperationSelectionForDirectRun,
    admit_direct_run_rust_sdk_executable_provider_operation_selection_v1,
    admit_rust_sdk_executable_provider_operation_for_direct_run_v1,
    admit_rust_sdk_executable_provider_target_for_direct_run_module_export_v1,
    admit_rust_sdk_executable_provider_target_for_direct_run_v1,
    admit_rust_sdk_executable_provider_target_from_static_provider_binding_for_direct_run_v1,
    admit_rust_sdk_static_provider_host_set_for_direct_run_v1,
    rust_sdk_static_provider_package_is_admitted_for_direct_run_v1,
};
const SS_PRODUCT_BINARY_STATIC_PROVIDER_HOST_OWNER_NAMESPACE: &str =
    "swarm.ss.product_binary.static_provider_host";

const SWARM_DATASTORE_MODULE_ID_FOR_PRODUCT_BINARY_TEST_MANIFEST_COVERAGE: &str =
    "@swarm/datastore";
const SWARM_DATASTORE_DATASTORE_EXPORT_FOR_PRODUCT_BINARY_TEST_MANIFEST_COVERAGE: &str =
    "datastore";
const SWARM_DATASTORE_CONTRACT_TSON_FOR_PRODUCT_BINARY_TEST_MANIFEST_COVERAGE: &str =
    include_str!("../../../packages/datastore/src/datastore.tson.ts");
const SESSION_WORK_RUNTIME_STD_RESULT_CLOSED_SUM_SYMBOL: &str = "std.Result";
const SESSION_WORK_RUNTIME_STD_RESULT_OK_VARIANT: &str = "Ok";
const SESSION_WORK_RUNTIME_STD_RESULT_ERR_VARIANT: &str = "Err";
const EXTERNAL_PROVIDER_STD_RESULT_OK_VARIANT: &str = "ok";
const EXTERNAL_PROVIDER_STD_RESULT_ERR_VARIANT: &str = "err";

include!("lib_parts/request_and_output.rs");
include!("lib_parts/native_request_and_executor.rs");
include!("builtin_operation_admission.rs");
include!("lib_parts/admission_model.rs");
include!("lib_parts/host_set.rs");
include!("lib_parts/host_owner.rs");
include!("lib_parts/tests.rs");
include!("lib_parts/datastore_bridge_tests.rs");

pub(crate) fn direct_run_public_aperture_prepared_runtime_process_start_admission_input_v1(
    command: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartCommand,
    provider_execution_session: swarm_provider_host_set::ProviderHostExecutionSession,
) -> DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1 {
    DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1::new_for_direct_run_public_aperture_owner_v1(
        command,
        provider_execution_session,
    )
}

pub(crate) fn admit_direct_run_public_aperture_prepared_runtime_process_start_v1(
    input: DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionInputV1,
) -> Result<
    AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1,
    DirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartAdmissionRefusalV1,
> {
    input.admit_for_direct_run_public_aperture_owner_v1()
}

pub(crate) fn drive_direct_run_public_aperture_prepared_runtime_process_start_command_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1(
    admitted: AdmittedDirectSwarmScriptRunPublicAperturePreparedRuntimeProcessStartV1,
) -> Result<
    DirectRunProcessSessionPublicApertureOutputEmissionProductV1,
    DirectRunProcessSessionDriveFaultV1,
> {
    admitted.drive_until_terminal_with_runtime_terminal_observation_for_ss_test_owner_v1()
}

impl WorkRuntimeStores {
    pub(crate) fn new_for_swarmvm_session_runtime_open_owner_v1() -> Self {
        Self {
            prepared: PreparedRuntimeStore::default(),
            payloads: PayloadStore::default(),
            work: WorkStore::default(),
            effects: EffectLedger::default(),
            scheduler: SchedulerQueues::default(),
        }
    }

    pub(crate) fn new_for_scheduler_execution_profile_owner_v1(
        profile: SchedulerExecutionProfileSelection,
        transfer_receiver_capacity: usize,
    ) -> Self {
        Self {
            prepared: PreparedRuntimeStore::default(),
            payloads: PayloadStore::default(),
            work: WorkStore::default(),
            effects: EffectLedger::default(),
            scheduler: SchedulerQueues::from_profile_for_scheduler_owner_v1(
                profile,
                transfer_receiver_capacity,
            ),
        }
    }

    pub(crate) fn restore_from_checkpoint_manifest_for_swarmvm_session_runtime_owner_v1(
        manifest: CheckpointManifest,
    ) -> Result<Self, String> {
        let restored = Self {
            prepared: manifest.prepared,
            payloads: PayloadStore::from_checkpoint_records(manifest.payloads)?,
            work: manifest.work,
            effects: manifest.effects,
            scheduler: manifest.scheduler,
        };
        restored.validate_store_owned_checkpoint_records()?;
        Ok(restored)
    }

    fn validate_store_owned_checkpoint_records(&self) -> Result<(), String> {
        for (work_id, record) in &self.work.records {
            if &record.handle.id != work_id {
                return Err(serde_json::json!({
                    "kind": "checkpoint_work_record_key_mismatch_forbidden",
                    "reason": "ADR-2027 checkpoint restore must rebuild WorkStore from store-owned WorkRecords whose key and WorkHandle match exactly",
                    "record_key": work_id.as_str(),
                    "work_handle": record.handle.id.as_str(),
                    "work_generation": record.handle.generation(),
                })
                .to_string());
            }
            if let Some(prepared) = &record.prepared {
                if !self.prepared.records.contains_key(&prepared.id) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_work_record_prepared_runtime_missing_forbidden",
                        "reason": "ADR-2027 WorkRecord restore may reference only prepared runtime records present in the same store-owned checkpoint manifest",
                        "work_id": record.handle.id.as_str(),
                        "prepared_runtime_id": prepared.id.0.as_str(),
                        "prepared_generation": prepared.generation.value(),
                    })
                    .to_string());
                }
            }
            self.validate_checkpoint_work_frame(record)?;
        }

        self.effects
            .validate_checkpoint_records_for_session_work_runtime_owner_v1(
                |payload| self.payloads.contains_handle(payload),
                |work| self.work.contains_handle(work),
            )?;

        self.validate_checkpoint_scheduler_queues()
    }

    fn validate_checkpoint_work_frame(&self, record: &WorkRecord) -> Result<(), String> {
        self.validate_checkpoint_work_frame_kind(record)?;
        match &record.frame {
            WorkFrame::SessionStart(frame) => {
                if !self
                    .prepared
                    .records
                    .contains_key(&frame.segmented_live_session.prepared.id)
                {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_session_start_prepared_runtime_missing_forbidden",
                        "reason": "ADR-2027 SessionStart restore requires a named PreparedRuntimeRecord, not a checkpoint DTO reconstruction",
                        "work_id": record.handle.id.as_str(),
                        "prepared_runtime_id": frame.segmented_live_session.prepared.id.0.as_str(),
                    })
                    .to_string());
                }
                if !self.payloads.contains_handle(&frame.root_input) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_session_start_root_input_payload_missing_forbidden",
                        "reason": "ADR-2027 SessionStart restore requires the root input PayloadHandle to resolve in PayloadStore",
                        "work_id": record.handle.id.as_str(),
                        "root_input_payload": frame
                            .root_input
                            .id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ActorTurn(frame) => {
                if !self.payloads.contains_handle(&frame.actor_turn) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_actor_turn_payload_missing_forbidden",
                        "reason": "ADR-2027 ActorTurn restore requires the actor-turn PayloadHandle to resolve in PayloadStore",
                        "work_id": record.handle.id.as_str(),
                        "actor_turn_payload": frame
                            .actor_turn
                            .id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ActorCheckpointBodyWork(frame) => {
                if record.handle.kind != WorkKind::ActorCheckpointBodyWork {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_actor_checkpoint_body_work_kind_mismatch_forbidden",
                        "reason": "ADR-2027 actor-checkpoint body/work restore records must use ActorCheckpointBodyWork WorkKind",
                        "work_id": record.handle.id.as_str(),
                        "work_kind": &record.handle.kind,
                    })
                    .to_string());
                }
                if !self.payloads.contains_handle(&frame.payload) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_actor_checkpoint_body_work_payload_missing_forbidden",
                        "reason": "ADR-2027 actor-checkpoint body/work restore requires a store-owned PayloadRecord; checkpoint projection body flags cannot mint bodies",
                        "work_id": record.handle.id.as_str(),
                        "payload": frame.payload.id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ActorRequestReadyOkResult(frame) => {
                if !self.payloads.contains_handle(&frame.result_payload) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_actor_request_ready_ok_payload_missing_forbidden",
                        "reason": "ADR-2204 actor-request ready ok restore requires the private ok PayloadHandle; descriptors cannot restore result bodies",
                        "work_id": record.handle.id.as_str(),
                        "request_id": frame.request_id.as_str(),
                        "result_payload": frame
                            .result_payload
                            .id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ActorRequestReadyErrResult(frame) => {
                if !self.payloads.contains_handle(&frame.result_payload) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_actor_request_ready_err_payload_missing_forbidden",
                        "reason": "ADR-2204 actor-request ready err restore requires the private err PayloadHandle; descriptors cannot restore result bodies",
                        "work_id": record.handle.id.as_str(),
                        "request_id": frame.request_id.as_str(),
                        "result_payload": frame
                            .result_payload
                            .id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ProviderResume(frame) => {
                if record.handle.kind != WorkKind::ProviderResume {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_provider_resume_work_kind_mismatch_forbidden",
                        "reason": "ADR-2193 ProviderResume restore records must use ProviderResume WorkKind; checkpoint activity frames cannot mint provider-resume authority from another work domain",
                        "work_id": record.handle.id.as_str(),
                        "work_kind": &record.handle.kind,
                    })
                    .to_string());
                }
                if !self.effects.contains_handle(&frame.effect) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_provider_resume_effect_record_missing_forbidden",
                        "reason": "ADR-2027 ProviderResume restore requires the matching EffectRecord; checkpoint activity frames cannot mint provider-resume work",
                        "work_id": record.handle.id.as_str(),
                        "effect": "session_work_runtime_owned",
                    })
                    .to_string());
                }
                if matches!(
                    &frame.selected_authority_custody,
                    ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1::Consumed
                ) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_provider_resume_selected_authority_consumed_forbidden",
                        "reason": "ProviderResume checkpoint restore requires pending selected contract, provider-output, and exact-use custody; consumed in-flight host authority cannot be reconstructed from checkpoint observations",
                        "work_id": record.handle.id.as_str(),
                    })
                    .to_string());
                }
            }
            WorkFrame::EventAppend(frame) => {
                if record.handle.kind != WorkKind::EventAppend {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_event_append_work_kind_mismatch_forbidden",
                        "reason": "ADR-2179 event append restore records must use EventAppend WorkKind; publishEvent may not restore as provider-resume authority",
                        "work_id": record.handle.id.as_str(),
                        "work_kind": &record.handle.kind,
                    })
                    .to_string());
                }
                if !self.effects.contains_handle(&frame.effect) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_event_append_effect_record_missing_forbidden",
                        "reason": "ADR-2179 EventAppend restore requires the matching EffectRecord; checkpoint activity frames cannot mint event append work from descriptors",
                        "work_id": record.handle.id.as_str(),
                            "effect": "session_work_runtime_owned",
                    })
                    .to_string());
                }
            }
            WorkFrame::Projection(frame) => {
                if !self.work.contains_handle(&frame.source) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_projection_source_work_missing_forbidden",
                        "reason": "ADR-2027 projection restore requires the source WorkHandle to resolve in WorkStore",
                        "work_id": record.handle.id.as_str(),
                        "source_work": frame.source.id_str(),
                    })
                    .to_string());
                }
            }
            WorkFrame::ExternalIngress(frame) => {
                if !self.payloads.contains_handle(&frame.ingress_payload) {
                    return Err(serde_json::json!({
                        "kind": "checkpoint_external_ingress_payload_missing_forbidden",
                        "reason": "ADR-2027 ExternalIngress restore requires the ingress PayloadHandle to resolve in PayloadStore",
                        "work_id": record.handle.id.as_str(),
                        "ingress_payload": frame
                            .ingress_payload
                            .id_str_for_session_work_runtime_owner_v1(),
                    })
                    .to_string());
                }
            }
            WorkFrame::SchedulerReawaken(_)
            | WorkFrame::SchedulerRunnable
            | WorkFrame::InstructionContinuation(_)
            | WorkFrame::EventWaitProducer(_)
            | WorkFrame::TimerWake(_) => {}
        }
        Ok(())
    }

    fn validate_checkpoint_work_frame_kind(&self, record: &WorkRecord) -> Result<(), String> {
        let (expected_kind, error_kind, reason) = match &record.frame {
            WorkFrame::SessionStart(_) => (
                WorkKind::SessionStart,
                "checkpoint_session_start_work_kind_mismatch_forbidden",
                "ADR-2196 SessionStart restore records must use SessionStart WorkKind; checkpoint records cannot rehydrate session-start authority from another work domain",
            ),
            WorkFrame::SchedulerReawaken(_) => (
                WorkKind::SchedulerReawaken,
                "checkpoint_scheduler_reawaken_work_kind_mismatch_forbidden",
                "ADR-2196 SchedulerReawaken restore records must use SchedulerReawaken WorkKind; checkpoint records cannot rehydrate scheduler wake authority from another work domain",
            ),
            WorkFrame::SchedulerRunnable => (
                WorkKind::SchedulerRunnable,
                "checkpoint_scheduler_runnable_work_kind_mismatch_forbidden",
                "scheduler runnable restore records must use SchedulerRunnable WorkKind; checkpoint records cannot rehydrate scheduler execution authority from another work domain",
            ),
            WorkFrame::InstructionContinuation(_) => (
                WorkKind::InstructionContinuation,
                "checkpoint_instruction_continuation_work_kind_mismatch_forbidden",
                "ADR-2196 InstructionContinuation restore records must use InstructionContinuation WorkKind; checkpoint records cannot rehydrate instruction-continuation authority from another work domain",
            ),
            WorkFrame::ActorTurn(_) => (
                WorkKind::ActorTurn,
                "checkpoint_actor_turn_work_kind_mismatch_forbidden",
                "ADR-2196 ActorTurn restore records must use ActorTurn WorkKind; checkpoint records cannot rehydrate actor-turn authority from another work domain",
            ),
            WorkFrame::ActorCheckpointBodyWork(_) => (
                WorkKind::ActorCheckpointBodyWork,
                "checkpoint_actor_checkpoint_body_work_kind_mismatch_forbidden",
                "ADR-2027 actor-checkpoint body/work restore records must use ActorCheckpointBodyWork WorkKind",
            ),
            WorkFrame::ActorRequestReadyOkResult(_) => (
                WorkKind::ActorRequestReadyOkResult,
                "checkpoint_actor_request_ready_ok_work_kind_mismatch_forbidden",
                "ADR-2204 ActorRequestReadyOkResult restore records must use ActorRequestReadyOkResult WorkKind; checkpoint records cannot rehydrate actor-request ok result authority from another work domain",
            ),
            WorkFrame::ActorRequestReadyErrResult(_) => (
                WorkKind::ActorRequestReadyErrResult,
                "checkpoint_actor_request_ready_err_work_kind_mismatch_forbidden",
                "ADR-2204 ActorRequestReadyErrResult restore records must use ActorRequestReadyErrResult WorkKind; checkpoint records cannot rehydrate actor-request err result authority from another work domain",
            ),
            WorkFrame::EventWaitProducer(_) => (
                WorkKind::EventWaitProducer,
                "checkpoint_event_wait_producer_work_kind_mismatch_forbidden",
                "ADR-2196 EventWaitProducer restore records must use EventWaitProducer WorkKind; checkpoint records cannot rehydrate event-wait authority from another work domain",
            ),
            WorkFrame::ProviderResume(_) => (
                WorkKind::ProviderResume,
                "checkpoint_provider_resume_work_kind_mismatch_forbidden",
                "ADR-2193 ProviderResume restore records must use ProviderResume WorkKind; checkpoint activity frames cannot mint provider-resume authority from another work domain",
            ),
            WorkFrame::EventAppend(_) => (
                WorkKind::EventAppend,
                "checkpoint_event_append_work_kind_mismatch_forbidden",
                "ADR-2179 event append restore records must use EventAppend WorkKind; publishEvent may not restore as provider-resume authority",
            ),
            WorkFrame::Projection(_) => (
                WorkKind::Projection,
                "checkpoint_projection_work_kind_mismatch_forbidden",
                "ADR-2196 Projection restore records must use Projection WorkKind; checkpoint records cannot rehydrate projection authority from another work domain",
            ),
            WorkFrame::ExternalIngress(_) => (
                WorkKind::ExternalIngress,
                "checkpoint_external_ingress_work_kind_mismatch_forbidden",
                "ADR-2196 ExternalIngress restore records must use ExternalIngress WorkKind; checkpoint records cannot rehydrate ingress authority from another work domain",
            ),
            WorkFrame::TimerWake(_) => (
                WorkKind::TimerWake,
                "checkpoint_timer_wake_work_kind_mismatch_forbidden",
                "ADR-2196 TimerWake restore records must use TimerWake WorkKind; checkpoint records cannot rehydrate timer-wake authority from another work domain",
            ),
        };
        if record.handle.kind == expected_kind {
            return Ok(());
        }
        Err(serde_json::json!({
            "kind": error_kind,
            "reason": reason,
            "work_id": record.handle.id.as_str(),
            "work_kind": &record.handle.kind,
            "expected_work_kind": expected_kind,
        })
        .to_string())
    }

    fn validate_checkpoint_scheduler_queues(&self) -> Result<(), String> {
        for (work, queue) in self.scheduler.queued_entries_for_checkpoint_validation() {
            if !self.work.contains_handle(work) {
                return Err(serde_json::json!({
                    "kind": "checkpoint_scheduler_queue_work_missing_forbidden",
                    "reason": "ADR-2027 scheduler queue restore requires every queued WorkHandle to resolve in WorkStore",
                    "queue": queue,
                    "work_id": work.id_str(),
                })
                .to_string());
            }
        }
        for (effect_id, work) in &self.scheduler.waiting_effects {
            let effect_present = self
                .effects
                .contains_effect_id_for_session_work_runtime_owner_v1(effect_id);
            if !effect_present || !self.work.contains_handle(work) {
                return Err(serde_json::json!({
                    "kind": "checkpoint_scheduler_waiting_effect_record_missing_forbidden",
                    "reason": "ADR-2027 waiting-effect queue restore requires both the EffectRecord and waiting WorkRecord",
                    "effect_id": effect_id.as_str(),
                    "work_id": work.id_str(),
                    "effect_present": effect_present,
                    "work_present": self.work.contains_handle(work),
                })
                .to_string());
            }
        }
        for (deadline_id, work) in &self.scheduler.waiting_timers {
            if !self.work.contains_handle(work) {
                return Err(serde_json::json!({
                    "kind": "checkpoint_scheduler_waiting_timer_work_missing_forbidden",
                    "reason": "ADR-2027 waiting-timer queue restore requires the waiting WorkRecord",
                    "deadline_id": deadline_id.observation_for_session_work_runtime_owner_v1(),
                    "work_id": work.id_str(),
                })
                .to_string());
            }
        }
        for (delivery_id, work) in &self.scheduler.waiting_actor_delivery {
            if !self.work.contains_handle(work) {
                return Err(serde_json::json!({
                    "kind": "checkpoint_scheduler_waiting_actor_delivery_work_missing_forbidden",
                    "reason": "ADR-2027 waiting-actor-delivery queue restore requires the waiting WorkRecord",
                    "delivery_id": delivery_id.observation_for_session_work_runtime_owner_v1(),
                    "work_id": work.id_str(),
                })
                .to_string());
            }
        }
        Ok(())
    }

    pub(crate) fn effect_ref_uses_event_append_work_frame(
        &self,
        effect_ref: &EffectRef,
    ) -> Result<bool, String> {
        let record = self
            .effects
            .require_record_for_ref(effect_ref, "effect_ref_work_frame_classification")?;
        let work_record = self
            .work
            .records
            .get(&record.resume_work_handle_for_session_work_runtime_owner_v1().id)
            .ok_or_else(|| {
                serde_json::json!({
                    "kind": "effect_ref_work_frame_classification_work_record_missing",
                    "reason": "ADR-2179/ADR-2181 host activity frontier classification must use the store-owned WorkRecord named by the EffectRecord; descriptors cannot classify authority",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": record.resume_work_handle_for_session_work_runtime_owner_v1().id.as_str(),
                })
                .to_string()
            })?;
        let resume_work_handle = record.resume_work_handle_for_session_work_runtime_owner_v1();
        if !work_record
            .handle
            .matches_session_work_runtime_owner_v1(&resume_work_handle)
        {
            return Err(serde_json::json!({
                "kind": "effect_ref_work_frame_classification_handle_mismatch",
                "reason": "ADR-2179/ADR-2181 host activity frontier classification requires EffectRecord resume_work to match the stored WorkHandle exactly",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work_handle.id.as_str(),
                "stored_generation": work_record.handle.generation(),
                "expected_generation": resume_work_handle.generation(),
            })
            .to_string());
        }
        Ok(matches!(work_record.frame, WorkFrame::EventAppend(_)))
    }

    fn require_event_append_parts_in_state(
        &self,
        effect_ref: &EffectRef,
        context: &'static str,
        expected_state: EffectState,
        invalid_state_kind: &'static str,
        invalid_state_reason: &'static str,
    ) -> Result<
        (
            EffectDescriptor,
            PayloadHandle,
            EffectResumeFrame,
            WorkHandle,
            WorkRef,
        ),
        String,
    > {
        let record = self.effects.require_record_for_ref(effect_ref, context)?;
        if !record.is_host_activity_for_session_work_runtime_owner_v1() {
            return Err(serde_json::json!({
                "kind": "event_append_effect_kind_mismatch",
                "reason": "ADR-2179 event append authority is defined only for HostActivity EffectRecords",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "effect_kind": "non_host_activity",
                "context": context,
            })
            .to_string());
        }
        if !record.state_matches_for_session_work_runtime_owner_v1(&expected_state) {
            return Err(serde_json::json!({
                "kind": invalid_state_kind,
                "reason": invalid_state_reason,
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "effect_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                "effect_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
                "effect_state": "not_expected",
                "expected_state": &expected_state,
                "context": context,
            })
            .to_string());
        }

        let descriptor = record.descriptor_for_session_work_runtime_owner_v1();
        let input_handle = record.input_handle_for_session_work_runtime_owner_v1();
        let resume = record.resume_frame_for_session_work_runtime_owner_v1();
        let resume_work_handle = record.resume_work_handle_for_session_work_runtime_owner_v1();
        let resume_work_ref = record.resume_work_ref_for_session_work_runtime_owner_v1();
        let work_record = self
            .work
            .records
            .get(&resume_work_handle.id)
            .ok_or_else(|| {
                serde_json::json!({
                    "kind": "event_append_work_record_missing",
                    "reason": "ADR-2179 event append authority must prove the EventAppend WorkRecord named by the EffectRecord before publication or await-result application",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work_handle.id.as_str(),
                    "context": context,
                })
                .to_string()
            })?;
        if !work_record
            .handle
            .matches_session_work_runtime_owner_v1(&resume_work_handle)
        {
            return Err(serde_json::json!({
                "kind": "event_append_work_handle_mismatch",
                "reason": "ADR-2179 event append authority requires the stored WorkHandle to match the EffectRecord resume_work handle exactly",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work_handle.id.as_str(),
                "stored_generation": work_record.handle.generation(),
                "expected_generation": resume_work_handle.generation(),
                "context": context,
            })
            .to_string());
        }
        if work_record.status != WorkStatus::WaitingOnEffect {
            return Err(serde_json::json!({
                "kind": "event_append_work_not_waiting_on_effect",
                "reason": "ADR-2179 event append authority may only target EventAppend work that is waiting on the effect result",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work_handle.id.as_str(),
                "work_status": &work_record.status,
                "context": context,
            })
            .to_string());
        }
        let WorkFrame::EventAppend(frame) = &work_record.frame else {
            return Err(serde_json::json!({
                "kind": "event_append_work_frame_kind_mismatch",
                "reason": "ADR-2179 event append authority requires an EventAppend WorkFrame; ProviderResume is forbidden for @swarm/event.publishEvent",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work_handle.id.as_str(),
                "context": context,
            })
            .to_string());
        };
        if !effect_ref_matches_handle(&effect_ref, &frame.effect) {
            return Err(serde_json::json!({
                "kind": "event_append_work_effect_mismatch",
                "reason": "ADR-2179 event append authority requires the EventAppend frame to name the same effect as the requested EffectRef",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "requested_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                "frame_effect": "session_work_runtime_owned",
                "requested_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
                "context": context,
            })
            .to_string());
        }
        Ok((
            descriptor,
            input_handle,
            resume,
            resume_work_handle,
            resume_work_ref,
        ))
    }

    pub(crate) fn take_selected_event_append_publication_work_for_direct_run_owner_v1(
        &mut self,
        effect_ref: &EffectRef,
    ) -> Result<SelectedEventAppendPublicationWorkForDirectRunOwnerV1, String> {
        let effect_input = self
            .effects
            .take_event_append_publication_input_for_effect_ref(
                effect_ref,
                "selected_event_append_publication_work",
            )?;
        self.admit_event_append_publication_effect_input(effect_ref, effect_input)
    }

    pub(crate) fn take_selected_provider_resume_host_input_for_direct_run_owner_v1(
        &mut self,
        effect_ref: &EffectRef,
    ) -> Result<SelectedProviderResumeHostInputForDirectRunOwnerV1, String> {
        let preflighted_effect = self
            .effects
            .preflight_provider_resume_host_input_for_effect_ref(
                effect_ref,
                "selected_provider_resume_host_input",
            )?;
        let preflighted_work =
            self.preflight_provider_resume_host_input_work_frame(effect_ref, &preflighted_effect)?;
        let preflighted_payload = self
            .payloads
            .preflight_activity_input_payload_for_provider_resume_host_input(
                &preflighted_effect.input,
                "selected_provider_resume_host_input",
            )?;

        let invocation_authority = self
            .effects
            .consume_preflighted_provider_resume_host_input_for_session_work_runtime_owner_v1(
                preflighted_effect,
                "selected_provider_resume_host_input_commit",
            )?;
        let work_record = self
            .work
            .records
            .get_mut(&preflighted_work.work_id)
            .expect("provider-resume work preflight proved the exact record exists");
        let WorkFrame::ProviderResume(frame) = &mut work_record.frame else {
            unreachable!("provider-resume work preflight proved the exact frame kind")
        };
        let ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1::Pending {
            selected_contract,
            selected_output_authority,
            exact_static_child_use,
        } = std::mem::replace(
            &mut frame.selected_authority_custody,
            ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1::Consumed,
        )
        else {
            unreachable!("provider-resume work preflight proved pending selected custody")
        };
        let provider_input = self
            .payloads
            .take_preflighted_activity_input_payload_for_provider_resume_host_input(
                preflighted_payload,
            );
        Ok(
            SelectedProviderResumeHostInputForDirectRunOwnerV1::from_session_work_runtime_owner_v1(
                provider_input,
                selected_contract,
                selected_output_authority,
                invocation_authority,
                exact_static_child_use,
            ),
        )
    }

    fn preflight_provider_resume_host_input_work_frame(
        &self,
        effect_ref: &EffectRef,
        effect_input: &PreflightedProviderResumeHostInputEffectRecordForSessionWorkRuntimeOwnerV1,
    ) -> Result<PreflightedProviderResumeHostInputWorkFrameForSessionWorkRuntimeOwnerV1, String>
    {
        let work_record = self
            .work
            .records
            .get(&effect_input.resume_work_handle.id)
            .ok_or_else(|| {
                serde_json::json!({
                    "kind": "selected_provider_resume_host_input_work_record_missing",
                    "reason": "selected ProviderResume host input requires the exact ProviderResume WorkRecord named by the EffectRecord",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": effect_input.resume_work_handle.id.as_str(),
                })
                .to_string()
            })?;
        if !work_record
            .handle
            .matches_session_work_runtime_owner_v1(&effect_input.resume_work_handle)
        {
            return Err(serde_json::json!({
                "kind": "selected_provider_resume_host_input_work_handle_mismatch",
                "reason": "selected ProviderResume host input requires the stored WorkHandle to match the EffectRecord resume_work handle exactly",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
                "stored_generation": work_record.handle.generation(),
                "expected_generation": effect_input.resume_work_handle.generation(),
            })
            .to_string());
        }
        if work_record.status != WorkStatus::WaitingOnEffect {
            return Err(serde_json::json!({
                "kind": "selected_provider_resume_host_input_work_not_waiting_on_effect",
                "reason": "selected ProviderResume host input may only target ProviderResume work waiting on the effect result",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
                "work_status": &work_record.status,
            })
            .to_string());
        }
        let WorkFrame::ProviderResume(frame) = &work_record.frame else {
            return Err(serde_json::json!({
                "kind": "selected_provider_resume_host_input_work_frame_kind_mismatch",
                "reason": "selected ProviderResume host input requires a ProviderResume WorkFrame",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
            })
            .to_string());
        };
        if !effect_ref_matches_handle(effect_ref, &frame.effect) {
            return Err(serde_json::json!({
                "kind": "selected_provider_resume_host_input_work_effect_mismatch",
                "reason": "selected ProviderResume host input requires the ProviderResume frame to name the same effect as the requested EffectRef",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "requested_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                "frame_effect": "session_work_runtime_owned",
                "requested_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
            })
            .to_string());
        }
        if !matches!(
            &frame.selected_authority_custody,
            ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1::Pending { .. }
        ) {
            return Err(serde_json::json!({
                "kind": "selected_provider_resume_host_input_authority_missing",
                "reason": "selected ProviderResume host input must retain its selected contract, paired provider-output authority, and exact static-child use until one direct-run route consumes them",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
            })
            .to_string());
        }
        Ok(
            PreflightedProviderResumeHostInputWorkFrameForSessionWorkRuntimeOwnerV1 {
                work_id: effect_input.resume_work_handle.id.clone(),
            },
        )
    }

    fn admit_event_append_publication_effect_input(
        &mut self,
        effect_ref: &EffectRef,
        effect_input: EventAppendPublicationEffectRecordInput,
    ) -> Result<SelectedEventAppendPublicationWorkForDirectRunOwnerV1, String> {
        let work_record = self
            .work
            .records
            .get(&effect_input.resume_work_handle.id)
            .ok_or_else(|| {
                serde_json::json!({
                    "kind": "selected_event_append_publication_work_record_missing",
                    "reason": "selected EventAppend publication work requires the exact EventAppend WorkRecord named by the EffectRecord",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": effect_input.resume_work_handle.id.as_str(),
                })
                .to_string()
            })?;
        if !work_record
            .handle
            .matches_session_work_runtime_owner_v1(&effect_input.resume_work_handle)
        {
            return Err(serde_json::json!({
                "kind": "selected_event_append_publication_work_handle_mismatch",
                "reason": "selected EventAppend publication work requires the stored WorkHandle to match the EffectRecord resume_work handle exactly",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
                "stored_generation": work_record.handle.generation(),
                "expected_generation": effect_input.resume_work_handle.generation(),
            })
            .to_string());
        }
        if work_record.status != WorkStatus::WaitingOnEffect {
            return Err(serde_json::json!({
                "kind": "selected_event_append_publication_work_not_waiting_on_effect",
                "reason": "selected EventAppend publication work may only target EventAppend work that is waiting on the effect result",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
                "work_status": &work_record.status,
            })
            .to_string());
        }
        let WorkFrame::EventAppend(frame) = &work_record.frame else {
            return Err(serde_json::json!({
                "kind": "selected_event_append_publication_work_frame_kind_mismatch",
                "reason": "selected EventAppend publication work requires an EventAppend WorkFrame; ProviderResume is forbidden for @swarm/event.publishEvent",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": effect_input.resume_work_handle.id.as_str(),
            })
            .to_string());
        };
        if !effect_ref_matches_handle(effect_ref, &frame.effect) {
            return Err(serde_json::json!({
                "kind": "selected_event_append_publication_work_effect_mismatch",
                "reason": "selected EventAppend publication work requires the EventAppend frame to name the same effect as the requested EffectRef",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "requested_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                "frame_effect": "session_work_runtime_owned",
                "requested_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
            })
            .to_string());
        }
        let payload = self
            .payloads
            .take_event_append_publication_payload_from_effect_record(
                &effect_input.input,
                "selected_event_append_publication_work",
            )?
            .into_materialized_swarm_event_publish_payload_for_direct_run_owner_v1();
        let completion_ticket =
            EventAppendApplicationCompletionTicket::from_selected_event_append_work_for_session_work_runtime_owner_v1(
                effect_ref.duplicate_for_session_work_runtime_owner_v1(),
                effect_input.resume_work_handle,
            );
        Ok(
            SelectedEventAppendPublicationWorkForDirectRunOwnerV1::from_session_work_runtime_owner_v1(
                completion_ticket,
                payload,
            ),
        )
    }

    pub fn checkpoint_manifest(self) -> Result<CheckpointManifest, String> {
        let Self {
            prepared,
            payloads,
            work,
            effects,
            scheduler,
        } = self;
        let payloads = payloads.into_checkpoint_records()?;
        Ok(CheckpointManifest {
            prepared,
            payloads,
            work,
            effects,
            scheduler,
        })
    }

    pub(crate) fn admit_scheduler_runnable_work_for_scheduler_owner_v1(
        &mut self,
        class: SchedulerWorkAdmissionClass,
    ) -> Result<(), SchedulerWorkAdmissionFault> {
        let sequence = self
            .work
            .next_scheduler_runnable_sequence
            .checked_add(1)
            .ok_or(SchedulerWorkAdmissionFault::WorkIdentityExhausted)?;
        let scheduler_handle = WorkHandle::scheduler_runnable(sequence);
        let work_id = scheduler_handle.id.clone();
        let record_handle = scheduler_handle.duplicate_for_session_work_runtime_owner_v1();
        self.work.records.insert(
            work_id,
            WorkRecord {
                handle: record_handle,
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "scheduler_runnable_work".to_owned(),
                },
                frame: WorkFrame::SchedulerRunnable,
                retention: WorkRetentionClass::CheckpointRestorable,
                status: WorkStatus::Ready,
                created_by: WorkCreationCause {
                    cause_kind: "scheduler_owner_admission".to_owned(),
                },
            },
        );
        self.work.next_scheduler_runnable_sequence = sequence;
        self.scheduler
            .work_distribution
            .admit_work_for_scheduler_owner_v1(class, scheduler_handle);
        Ok(())
    }

    pub(crate) fn execute_admitted_scheduler_work_for_profile_owner_v1(
        &mut self,
        completion_inputs: Vec<SchedulerWorkTerminalInput>,
        schedule: SchedulerIncidentalExecutionSchedule,
    ) -> Result<(), SchedulerWorkExecutionFault> {
        self.validate_admitted_scheduler_work_for_profile_owner_v1(completion_inputs.len())?;
        self.scheduler
            .work_distribution
            .prepare_incidental_distribution_schedule_for_scheduler_owner_v1(
                schedule.distribution_seed_for_scheduler_owner_v1(),
            );
        self.scheduler
            .work_distribution
            .rebalance_for_profile_execution_owner_v1()?;
        let mut selected = self
            .scheduler
            .work_distribution
            .select_all_runnable_work_for_profile_execution_owner_v1();
        selected.sort_by_key(|work| work.commit_order_for_scheduler_owner_v1());

        let mut executing = Vec::with_capacity(selected.len());
        for (selected, terminal) in selected.into_iter().zip(completion_inputs) {
            let (work_id, mut record) = self
                .work
                .records
                .remove_entry(&selected.work_handle_for_session_work_runtime_owner_v1().id)
                .expect("prevalidated scheduler work record must remain store-owned");
            record.status = WorkStatus::Running;
            executing.push(SchedulerExecutingStoreWork {
                work_id,
                record,
                selected,
                terminal,
            });
        }

        let completion_seed = self
            .scheduler
            .work_distribution
            .completion_seed_for_profile_execution_owner_v1(
                schedule.completion_seed_for_scheduler_owner_v1(),
            );
        deterministic_shuffle_scheduler_completions(&mut executing, completion_seed);
        let mut completed_by_owner_order = std::collections::BTreeMap::new();
        for executing_work in executing {
            let completed =
                Self::execute_selected_scheduler_work_for_scheduler_owner_v1(executing_work);
            let replaced = completed_by_owner_order.insert(
                completed.selected.commit_order_for_scheduler_owner_v1(),
                completed,
            );
            assert!(
                replaced.is_none(),
                "scheduler owner admission order must be unique"
            );
        }
        for (_, completed) in completed_by_owner_order {
            self.settle_completed_scheduler_work_for_scheduler_owner_v1(completed);
        }
        Ok(())
    }

    fn validate_admitted_scheduler_work_for_profile_owner_v1(
        &self,
        completion_input_count: usize,
    ) -> Result<(), SchedulerWorkExecutionFault> {
        let queued: Vec<_> = self
            .scheduler
            .work_distribution
            .queued_entries_for_session_work_runtime_owner_v1()
            .map(|(work, _)| work)
            .collect();
        if queued.len() != completion_input_count {
            return Err(SchedulerWorkExecutionFault::CompletionInputCardinalityMismatch);
        }
        for (index, work) in queued.iter().enumerate() {
            if queued[..index]
                .iter()
                .any(|prior| prior.matches_session_work_runtime_owner_v1(work))
            {
                return Err(SchedulerWorkExecutionFault::DuplicateRunnableWork);
            }
            let record = self
                .work
                .records
                .get(&work.id)
                .ok_or(SchedulerWorkExecutionFault::StoreWorkMissing)?;
            if !record.handle.matches_session_work_runtime_owner_v1(work) {
                return Err(SchedulerWorkExecutionFault::StoreWorkHandleMismatch);
            }
            if record.handle.kind != WorkKind::SchedulerRunnable
                || !matches!(record.frame, WorkFrame::SchedulerRunnable)
            {
                return Err(SchedulerWorkExecutionFault::StoreWorkFrameMismatch);
            }
            if record.status != WorkStatus::Ready {
                return Err(SchedulerWorkExecutionFault::StoreWorkNotReady);
            }
        }
        Ok(())
    }

    fn execute_selected_scheduler_work_for_scheduler_owner_v1(
        executing: SchedulerExecutingStoreWork,
    ) -> SchedulerCompletedStoreWork {
        let SchedulerExecutingStoreWork {
            work_id,
            record,
            selected,
            terminal,
        } = executing;
        SchedulerCompletedStoreWork {
            work_id,
            record,
            selected,
            terminal,
        }
    }

    fn settle_completed_scheduler_work_for_scheduler_owner_v1(
        &mut self,
        mut completed: SchedulerCompletedStoreWork,
    ) {
        completed.record.status = match completed.terminal {
            SchedulerWorkTerminalInput::Commit => WorkStatus::Completed,
            SchedulerWorkTerminalInput::Fault(_) => WorkStatus::Faulted,
        };
        let replaced = self
            .work
            .records
            .insert(completed.work_id, completed.record);
        assert!(
            replaced.is_none(),
            "running scheduler work must settle into its vacant store slot"
        );
        self.scheduler
            .work_distribution
            .commit_completed_work_for_scheduler_owner_v1(completed.selected, completed.terminal);
    }

    pub(crate) fn committed_scheduler_results_match_store_for_scheduler_owner_v1(&self) -> bool {
        let committed = self
            .scheduler
            .work_distribution
            .committed_results_for_scheduler_owner_v1();
        for (index, result) in committed.iter().enumerate() {
            if committed[..index].iter().any(|prior| {
                prior
                    .work_handle_for_session_work_runtime_owner_v1()
                    .matches_session_work_runtime_owner_v1(
                        result.work_handle_for_session_work_runtime_owner_v1(),
                    )
            }) {
                return false;
            }
            let handle = result.work_handle_for_session_work_runtime_owner_v1();
            let Some(record) = self.work.records.get(&handle.id) else {
                return false;
            };
            if !record.handle.matches_session_work_runtime_owner_v1(handle) {
                return false;
            }
            let expected_status = match result.terminal_input_for_session_work_runtime_owner_v1() {
                SchedulerWorkTerminalInput::Commit => WorkStatus::Completed,
                SchedulerWorkTerminalInput::Fault(_) => WorkStatus::Faulted,
            };
            if record.status != expected_status {
                return false;
            }
        }
        self.work
            .records
            .values()
            .filter(|record| record.handle.kind == WorkKind::SchedulerRunnable)
            .all(|record| {
                committed.iter().any(|result| {
                    result
                        .work_handle_for_session_work_runtime_owner_v1()
                        .matches_session_work_runtime_owner_v1(&record.handle)
                })
            })
    }

    pub(crate) fn commit_actor_turn_work_for_session_work_runtime_owner_v1(
        &mut self,
        payload: ActorTurnPayloadProduct,
        handler_entry: super::descriptors::InstructionCursor,
    ) -> Result<WorkHandle, String> {
        let (payload_handle, actor_descriptor) =
            self.payloads.insert_actor_turn_payload(payload)?;
        let work_handle = WorkHandle::actor_turn(&actor_descriptor.actor_turn_id);
        let work_id = work_handle.id.clone();
        if self.work.records.contains_key(&work_id) {
            return Err(serde_json::json!({
                "kind": "actor_turn_work_duplicate_forbidden",
                "reason": "actor-turn work may be committed only once for the store-owned started actor turn payload",
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        self.work.records.insert(
            work_id,
            WorkRecord {
                handle: work_handle.duplicate_for_session_work_runtime_owner_v1(),
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "actor_turn_work".to_owned(),
                },
                frame: WorkFrame::ActorTurn(ActorTurnWorkFrame {
                    actor_turn: payload_handle,
                    actor_descriptor,
                    handler_entry,
                }),
                retention: WorkRetentionClass::PublicContinuation,
                status: WorkStatus::Ready,
                created_by: WorkCreationCause {
                    cause_kind: "started_actor_turn_payload".to_owned(),
                },
            },
        );
        Ok(work_handle)
    }

    pub(crate) fn admit_actor_turn_payload_handle_for_swarmvm_session_runtime_owner_v1(
        &self,
        handle: &PayloadHandle,
        context: &'static str,
    ) -> Result<(), String> {
        self.payloads
            .admit_actor_turn_payload_handle_for_session_runtime_owner_v1(handle, context)
    }

    pub(crate) fn take_actor_turn_whole_payload_for_swarmvm_session_runtime_owner_v1(
        &mut self,
        handle: &PayloadHandle,
        context: &'static str,
    ) -> Result<VmBoundaryValue, String> {
        self.payloads
            .take_actor_turn_whole_payload_for_session_runtime_owner_v1(handle, context)
    }

    pub(in crate::session) fn try_store_checkpoint_actor_restore_body_work_for_session_runtime_owner_v1(
        &mut self,
        selected: crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreForSessionRuntimeOwnerV1,
        input: crate::privileged_hostcalls::actor_store::PendingCheckpointActorRestoreBodyWorkInputForSessionRuntimeOwnerV1,
    ) -> Result<
        crate::privileged_hostcalls::actor_store::CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
        crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkAdmissionRefusal,
    > {
        use crate::privileged_hostcalls::actor_store::{
            CheckpointActorRestoreBodyWorkAdmissionRefusal,
            CheckpointActorRestoreBodyWorkFault,
            CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
        };
        if !selected
            .correlation
            .corresponds_to_checkpoint_actor_restore_candidate_owner_v1(&input.correlation)
        {
            return Err(CheckpointActorRestoreBodyWorkAdmissionRefusal {
                selected,
                input,
                fault: CheckpointActorRestoreBodyWorkFault::CorrelationMismatch,
            });
        }
        if let Err(fault) =
            self.preflight_checkpoint_actor_restore_body_work_batch_for_session_runtime_owner_v1(1)
        {
            return Err(CheckpointActorRestoreBodyWorkAdmissionRefusal {
                selected,
                input,
                fault,
            });
        }
        Ok(self.commit_preflighted_checkpoint_actor_restore_body_work_for_session_runtime_owner_v1(
            selected, input,
        ))
    }

    pub(crate) fn preflight_checkpoint_actor_restore_body_work_batch_for_session_runtime_owner_v1(
        &self,
        count: usize,
    ) -> Result<(), crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkFault>
    {
        use crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkFault;
        for offset in 1..=count {
            let offset = u64::try_from(offset)
                .map_err(|_| CheckpointActorRestoreBodyWorkFault::IdentityExhausted)?;
            let sequence = self
                .work
                .next_checkpoint_actor_restore_sequence
                .checked_add(offset)
                .ok_or(CheckpointActorRestoreBodyWorkFault::IdentityExhausted)?;
            let work_handle = WorkHandle::actor_checkpoint_body_work(sequence);
            let payload_handle = PayloadHandle::actor_checkpoint_body_work(
                sequence,
                PayloadRetentionClass::CheckpointRestorable,
            );
            if self.work.records.contains_key(&work_handle.id)
                || self.payloads.contains_handle(&payload_handle)
            {
                return Err(CheckpointActorRestoreBodyWorkFault::WorkHandleMismatch);
            }
        }
        Ok(())
    }

    pub(crate) fn commit_preflighted_checkpoint_actor_restore_body_work_for_session_runtime_owner_v1(
        &mut self,
        selected: crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreForSessionRuntimeOwnerV1,
        input: crate::privileged_hostcalls::actor_store::PendingCheckpointActorRestoreBodyWorkInputForSessionRuntimeOwnerV1,
    ) -> crate::privileged_hostcalls::actor_store::CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1
    {
        use crate::privileged_hostcalls::actor_store::{
            CheckpointActorRestoreBodyWorkReferenceForSessionRuntimeOwnerV1,
            CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
        };
        debug_assert!(selected
            .correlation
            .corresponds_to_checkpoint_actor_restore_candidate_owner_v1(&input.correlation));
        let sequence = self
            .work
            .next_checkpoint_actor_restore_sequence
            .checked_add(1)
            .expect("preflighted checkpoint actor body-work identity remains available");
        let work_handle = WorkHandle::actor_checkpoint_body_work(sequence);
        let payload_handle = PayloadHandle::actor_checkpoint_body_work(
            sequence,
            PayloadRetentionClass::CheckpointRestorable,
        );
        debug_assert!(!self.work.records.contains_key(&work_handle.id));
        debug_assert!(!self.payloads.contains_handle(&payload_handle));
        let frame_correlation =
            input.correlation.duplicate_for_checkpoint_actor_restore_owner_v1();
        let reference_correlation =
            input.correlation.duplicate_for_checkpoint_actor_restore_owner_v1();
        let inserted_payload = self
            .payloads
            .insert_actor_checkpoint_body_work_payload(sequence, input.body)
            .expect("preflighted checkpoint actor body payload insertion is infallible");
        debug_assert!(inserted_payload.matches_session_work_runtime_owner_v1(&payload_handle));
        self.work.records.insert(
            work_handle.id.clone(),
            WorkRecord {
                handle: work_handle.duplicate_for_session_work_runtime_owner_v1(),
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "checkpoint_actor_restore_body_work".to_owned(),
                },
                frame: WorkFrame::ActorCheckpointBodyWork(ActorCheckpointBodyWorkFrame {
                    correlation: frame_correlation,
                    payload: inserted_payload,
                }),
                retention: WorkRetentionClass::CheckpointRestorable,
                status: WorkStatus::Ready,
                created_by: WorkCreationCause {
                    cause_kind: "selected_checkpoint_actor_restore".to_owned(),
                },
            },
        );
        self.work.next_checkpoint_actor_restore_sequence = sequence;
        CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1 {
            selected,
            body_work: CheckpointActorRestoreBodyWorkReferenceForSessionRuntimeOwnerV1 {
                handle: work_handle,
                correlation: reference_correlation,
            },
        }
    }

    pub(in crate::session) fn try_select_checkpoint_actor_restore_body_work_for_session_runtime_owner_v1(
        &mut self,
        selection: crate::privileged_hostcalls::actor_store::CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
    ) -> Result<
        (
            crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreForSessionRuntimeOwnerV1,
            crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreBodyWorkForSessionRuntimeOwnerV1,
        ),
        crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkSelectionRefusal,
    > {
        use crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkSelectionRefusal;
        let refuse = |selection, fault| CheckpointActorRestoreBodyWorkSelectionRefusal {
            selection,
            fault,
        };
        if let Err(fault) = self
            .preflight_checkpoint_actor_restore_body_work_selection_for_session_runtime_owner_v1(
                &selection,
            )
        {
            return Err(refuse(selection, fault));
        }
        Ok(self.commit_preflighted_checkpoint_actor_restore_body_work_selection_for_session_runtime_owner_v1(selection))
    }

    pub(crate) fn preflight_checkpoint_actor_restore_body_work_selection_for_session_runtime_owner_v1(
        &self,
        selection: &crate::privileged_hostcalls::actor_store::CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
    ) -> Result<(), crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkFault>
    {
        use crate::privileged_hostcalls::actor_store::CheckpointActorRestoreBodyWorkFault;
        if !selection
            .selected
            .correlation
            .corresponds_to_checkpoint_actor_restore_candidate_owner_v1(
                &selection.body_work.correlation,
            )
        {
            return Err(CheckpointActorRestoreBodyWorkFault::CorrelationMismatch);
        }
        let Some(record) = self.work.records.get(&selection.body_work.handle.id) else {
            return Err(CheckpointActorRestoreBodyWorkFault::WorkRecordMissing);
        };
        let fault = if !record
            .handle
            .matches_session_work_runtime_owner_v1(&selection.body_work.handle)
        {
            Some(CheckpointActorRestoreBodyWorkFault::WorkHandleMismatch)
        } else if record.handle.kind != WorkKind::ActorCheckpointBodyWork {
            Some(CheckpointActorRestoreBodyWorkFault::WorkKindMismatch)
        } else if record.retention != WorkRetentionClass::CheckpointRestorable {
            Some(CheckpointActorRestoreBodyWorkFault::WorkRetentionMismatch)
        } else if record.status != WorkStatus::Ready {
            Some(CheckpointActorRestoreBodyWorkFault::WorkStatusMismatch)
        } else {
            match &record.frame {
                WorkFrame::ActorCheckpointBodyWork(frame)
                    if frame
                        .correlation
                        .corresponds_to_checkpoint_actor_restore_candidate_owner_v1(
                            &selection.body_work.correlation,
                        ) && self
                        .payloads
                        .contains_exact_actor_checkpoint_body_work_payload(&frame.payload) => None,
                WorkFrame::ActorCheckpointBodyWork(_) => {
                    Some(CheckpointActorRestoreBodyWorkFault::PayloadMismatch)
                }
                _ => Some(CheckpointActorRestoreBodyWorkFault::WorkKindMismatch),
            }
        };
        if let Some(fault) = fault {
            return Err(fault);
        }
        Ok(())
    }

    pub(crate) fn commit_preflighted_checkpoint_actor_restore_body_work_selection_for_session_runtime_owner_v1(
        &mut self,
        selection: crate::privileged_hostcalls::actor_store::CheckpointActorRestoreProjectionSelectionForSessionRuntimeOwnerV1,
    ) -> (
        crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreForSessionRuntimeOwnerV1,
        crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreBodyWorkForSessionRuntimeOwnerV1,
    ) {
        use crate::privileged_hostcalls::actor_store::SelectedCheckpointActorRestoreBodyWorkForSessionRuntimeOwnerV1;
        debug_assert!(self
            .preflight_checkpoint_actor_restore_body_work_selection_for_session_runtime_owner_v1(
                &selection,
            )
            .is_ok());
        let record = self
            .work
            .records
            .remove(&selection.body_work.handle.id)
            .expect("preflighted checkpoint actor body work must remain present");
        let WorkFrame::ActorCheckpointBodyWork(frame) = record.frame else {
            unreachable!("preflighted checkpoint actor body work frame must match");
        };
        let payload = self
            .payloads
            .take_actor_checkpoint_body_work_payload_product(
                &frame.payload,
                "checkpoint_actor_restore_body_selection",
            )
            .expect("preflighted checkpoint actor body payload commit is infallible");
        (
            selection.selected,
            SelectedCheckpointActorRestoreBodyWorkForSessionRuntimeOwnerV1::from_exact_work_runtime_selection_for_session_runtime_owner_v1(
                payload,
                frame.correlation,
            ),
        )
    }

    pub(crate) fn insert_activity_input_payload(
        &mut self,
        activity_attempt_id: &ActivityAttemptId,
        payload: crate::session::execution_kernel::executable_value::SessionRuntimeMaterializedActivityInputPayloadProduct,
    ) -> Result<PayloadHandle, String> {
        let payload =
            ActivityInputPayloadProduct::from_session_runtime_materialized_activity_input_payload_for_session_work_runtime_owner_v1(
                payload,
            )?;
        self.payloads
            .insert_activity_input_payload(activity_attempt_id, payload)
    }

    pub fn commit_selected_host_boundary_pending_activity_for_swarmvm_session_runtime_owner_v1(
        &mut self,
        activity_attempt_id: ActivityAttemptId,
        provider_host_contract:
            crate::session::execution_kernel::executable_image::CompilerExactProviderMaterializedCommandContractForSessionRuntimeOwnerV1,
        payload: SessionRuntimeMaterializedActivityInputPayloadProduct,
        exact_static_child_use: crate::session::execution_kernel::executable_image::SelectedProviderBoundaryExactStaticChildUseForSessionWorkRuntimeOwnerV1,
    ) -> Result<
        (
            PendingActivityEffectFrame,
            PendingActivityEffectFrame,
            swarm_capability_model::PendingProviderBoundaryOutputCommitAuthority,
        ),
        String,
    > {
        let effect_handle =
            EffectHandle::host_activity_for_session_work_runtime_owner_v1(&activity_attempt_id);
        if self.effects.contains_handle(&effect_handle) {
            return Err(serde_json::json!({
                "kind": "selected_host_boundary_effect_duplicate_forbidden",
                "reason": "selected host-boundary admission may mint exactly one HostActivity effect record per activity attempt",
                "effect": "session_work_runtime_owned",
            })
            .to_string());
        }
        let effect_ref = effect_ref_from_handle(&effect_handle);
        let resume_work_handle = WorkHandle::provider_resume(&activity_attempt_id);
        let work_id = resume_work_handle.id.clone();
        if self.work.records.contains_key(&work_id) {
            return Err(serde_json::json!({
                "kind": "selected_host_boundary_provider_resume_work_duplicate_forbidden",
                "reason": "selected host-boundary admission may mint exactly one ProviderResume work record per activity attempt",
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        let (pending_output_authority, selected_output_authority) =
            swarm_capability_model::mint_provider_boundary_output_correspondence_v1();
        let input_handle = self.insert_activity_input_payload(&activity_attempt_id, payload)?;
        let resume_frame = EffectResumeFrame {
            activity_attempt_id: activity_attempt_id.duplicate_for_isa_owner(),
            owner_actor_request_id: None,
            owner_actor_delivery_id: None,
            owner_actor_delivery_sequence: None,
            owner_actor_request_context: None,
            owner_actor_turn_id: None,
        };
        let descriptor = EffectDescriptor {
            effect_ref: effect_ref.duplicate_for_session_work_runtime_owner_v1(),
            kind: EffectKind::HostActivity,
            operation_id: "session_work_runtime_owned".to_owned(),
            site_id: "session_work_runtime_owned".to_owned(),
            input: PayloadDescriptor::new(
                input_handle.duplicate_for_session_work_runtime_owner_v1(),
                PayloadSizeClass::ForbiddenInPublicAperture,
                None,
            ),
            resume_work: WorkRef::from_handle(&resume_work_handle),
            deadline: None,
        };
        let record = EffectRecord::host_activity_for_session_work_runtime_owner_v1(
            effect_handle.duplicate_for_session_work_runtime_owner_v1(),
            input_handle,
            resume_work_handle.duplicate_for_session_work_runtime_owner_v1(),
            resume_frame,
            descriptor,
            None,
            None,
        );
        self.effects.insert_record(record)?;
        self.work.records.insert(
            work_id,
            WorkRecord {
                handle: resume_work_handle.duplicate_for_session_work_runtime_owner_v1(),
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "selected_host_boundary_provider_resume".to_owned(),
                },
                frame: WorkFrame::ProviderResume(ProviderResumeWorkFrame {
                    effect: effect_handle.duplicate_for_session_work_runtime_owner_v1(),
                    selected_authority_custody:
                        ProviderResumeSelectedAuthorityCustodyForSessionWorkRuntimeOwnerV1::Pending {
                            selected_contract: provider_host_contract,
                            selected_output_authority,
                            exact_static_child_use,
                        },
                }),
                retention: WorkRetentionClass::EffectResume,
                status: WorkStatus::WaitingOnEffect,
                created_by: WorkCreationCause {
                    cause_kind: "selected_host_boundary_pending_activity".to_owned(),
                },
            },
        );
        self.scheduler
            .blocking_provider
            .enqueue_provider_admission_for_session_work_runtime_owner_v1(
                resume_work_handle.duplicate_for_session_work_runtime_owner_v1(),
            );
        let pending_activity = self.effects.require_pending_activity_frame(
            &effect_ref,
            "selected_host_boundary_pending_activity_admission",
        )?;
        let pending_effect = pending_activity.duplicate_for_session_work_runtime_owner_v1();
        Ok((pending_activity, pending_effect, pending_output_authority))
    }

    pub(crate) fn insert_event_append_publication_payload(
        &mut self,
        activity_attempt_id: &ActivityAttemptId,
        payload: SessionRuntimeMaterializedSwarmEventPublishPayloadProduct,
    ) -> Result<PayloadHandle, String> {
        let payload =
            EventAppendPublicationPayloadProduct::from_session_runtime_materialized_swarm_event_publish_payload_for_session_work_runtime_owner_v1(
                payload,
            );
        self.payloads
            .insert_event_append_publication_payload(activity_attempt_id, payload)
    }

    pub(crate) fn insert_actor_request_ready_ok_payload(
        &mut self,
        request_id: &ActorRequestId,
        payload: String,
    ) -> Result<(PayloadHandle, ActorRequestReadyOkWorkHandle), String> {
        let work_handle = WorkHandle::actor_request_ready_ok_result(request_id);
        let work_id = work_handle.id.clone();
        if self
            .work
            .consumed_actor_request_ready_result_work
            .contains_key(&work_id)
        {
            return Err(serde_json::json!({
                "kind": "actor_request_ready_ok_work_remint_after_consume_forbidden",
                "reason": "ADR-2204 actor-request ready ok work is a one-shot result authority and cannot be reminted after consumption",
                "request_id": request_id.as_str(),
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        if self.work.records.contains_key(&work_id) {
            return Err(serde_json::json!({
                "kind": "actor_request_ready_ok_work_duplicate_forbidden",
                "reason": "ADR-2204 actor-request ready ok work must be minted once for each actor request",
                "request_id": request_id.as_str(),
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        let payload_handle = self.payloads.insert_actor_request_ready_ok_payload(
            request_id,
            ActorRequestReadyOkPayloadProduct::from_session_work_runtime_owner_v1(payload),
        )?;
        self.work.records.insert(
            work_id,
            WorkRecord {
                handle: work_handle.duplicate_for_session_work_runtime_owner_v1(),
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "actor_request_ready_ok_result_work".to_owned(),
                },
                frame: WorkFrame::ActorRequestReadyOkResult(ActorRequestReadyOkWorkFrame {
                    request_id: request_id.duplicate_for_isa_owner(),
                    result_payload: payload_handle.duplicate_for_session_work_runtime_owner_v1(),
                }),
                retention: WorkRetentionClass::PublicContinuation,
                status: WorkStatus::Ready,
                created_by: WorkCreationCause {
                    cause_kind: "actor_request_ready_ok_result_payload".to_owned(),
                },
            },
        );
        Ok((
            payload_handle,
            ActorRequestReadyOkWorkHandle::from_work_handle(work_handle),
        ))
    }

    pub(crate) fn insert_actor_request_ready_err_payload(
        &mut self,
        request_id: &ActorRequestId,
        payload: String,
    ) -> Result<(PayloadHandle, ActorRequestReadyErrWorkHandle), String> {
        let work_handle = WorkHandle::actor_request_ready_err_result(request_id);
        let work_id = work_handle.id.clone();
        if self.work.records.contains_key(&work_id) {
            return Err(serde_json::json!({
                "kind": "actor_request_ready_err_work_duplicate_forbidden",
                "reason": "ADR-2204 actor-request ready err result work may be minted only once per request id",
                "request_id": request_id.as_str(),
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        if self
            .work
            .consumed_actor_request_ready_result_work
            .contains_key(&work_id)
        {
            return Err(serde_json::json!({
                "kind": "actor_request_ready_err_work_remint_after_consume_forbidden",
                "reason": "ADR-2204 consumed actor-request ready err result work cannot be reminted from a later payload",
                "request_id": request_id.as_str(),
                "work_id": work_id.as_str(),
            })
            .to_string());
        }
        let payload_handle = self.payloads.insert_actor_request_ready_err_payload(
            request_id,
            ActorRequestReadyErrPayloadProduct::from_session_work_runtime_owner_v1(payload),
        )?;
        let result_work_handle = ActorRequestReadyErrWorkHandle::from_work_handle(
            work_handle.duplicate_for_session_work_runtime_owner_v1(),
        );
        self.work.records.insert(
            work_id,
            WorkRecord {
                handle: work_handle,
                prepared: None,
                authority: WorkAuthority {
                    authority_kind: "actor_request_ready_err_result".to_owned(),
                },
                frame: WorkFrame::ActorRequestReadyErrResult(ActorRequestReadyErrWorkFrame {
                    request_id: request_id.duplicate_for_isa_owner(),
                    result_payload: payload_handle.duplicate_for_session_work_runtime_owner_v1(),
                }),
                retention: WorkRetentionClass::PublicContinuation,
                status: WorkStatus::Ready,
                created_by: WorkCreationCause {
                    cause_kind: "actor_request_ready_err_result_payload".to_owned(),
                },
            },
        );
        Ok((payload_handle, result_work_handle))
    }

    pub(crate) fn take_actor_request_ready_ok_for_store_owned_work(
        &mut self,
        result_work_handle: ActorRequestReadyOkWorkHandle,
        expected_payload_handle: PayloadHandle,
    ) -> Result<(), StoreOwnedWorkSettlementFault> {
        let (payload_handle, payload_body) = (|| -> Result<_, String> {
            let work_handle = result_work_handle.into_store_owned_work_handle();
            let Some(work_record) = self.work.records.get(&work_handle.id) else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_handle_missing",
                    "reason": "ADR-2204 actor-request ready ok driver must consume the exact store-owned WorkHandle once",
                    "work_id": work_handle.id.as_str(),
                    "work_generation": work_handle.generation(),
                })
                .to_string());
            };
            if !work_record
                .handle
                .matches_session_work_runtime_owner_v1(&work_handle)
            {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_handle_mismatch",
                    "reason": "ADR-2204 actor-request ready ok driver requires the stored WorkRecord to match the typed ok WorkHandle exactly",
                    "work_id": work_handle.id.as_str(),
                    "stored_generation": work_record.handle.generation(),
                    "expected_generation": work_handle.generation(),
                })
                .to_string());
            }
            if work_record.status != WorkStatus::Ready {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_not_ready",
                    "reason": "ADR-2204 actor-request ready ok WorkHandle may be consumed only after the actor request result is Ready",
                    "work_id": work_handle.id.as_str(),
                    "work_status": &work_record.status,
                })
                .to_string());
            }
            let WorkFrame::ActorRequestReadyOkResult(frame) = &work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_frame_kind_mismatch",
                    "reason": "ADR-2204 actor-request ready ok driver can consume only ActorRequestReadyOkResult work frames",
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            };
            if !frame
                .result_payload
                .matches_session_work_runtime_owner_v1(&expected_payload_handle)
            {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_authority_payload_mismatch",
                    "reason": "ADR-2204 actor-request ok work authority must match the consumed store-owned payload handle exactly",
                    "payload_handle_match": false,
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            }
            let Some(work_record) = self.work.records.remove(&work_handle.id) else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_handle_missing_after_validation",
                    "reason": "ADR-2204 actor-request ready ok work disappeared after exact authority validation",
                    "work_id": work_handle.id.as_str(),
                    "work_generation": work_handle.generation(),
                })
                .to_string());
            };
            let WorkFrame::ActorRequestReadyOkResult(frame) = work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_ok_work_frame_kind_mismatch_after_validation",
                    "reason": "ADR-2204 actor-request ready ok work frame changed after exact authority validation",
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            };
            self.work.consumed_actor_request_ready_result_work.insert(
                work_handle.id.clone(),
                work_handle
                    .generation
                    .duplicate_for_session_work_runtime_owner_v1(),
            );
            let payload_handle = frame.result_payload;
            let payload_body = self.payloads.take_actor_request_ready_ok_payload(
                &payload_handle,
                "actor_request_ready_ok_work_driver",
            )?;
            Ok((payload_handle, payload_body))
        })()
        .map_err(StoreOwnedWorkSettlementFault::from)?;
        Err(
            StoreOwnedWorkSettlementFault::ActorRequestReadyOkStoreOwnedSettlementRequired {
                payload_handle,
                payload_body,
            },
        )
    }

    pub(crate) fn take_actor_request_ready_err_for_store_owned_work(
        &mut self,
        result_work_handle: ActorRequestReadyErrWorkHandle,
        expected_payload_handle: PayloadHandle,
    ) -> Result<(), StoreOwnedWorkSettlementFault> {
        let (payload_handle, payload_body) = (|| -> Result<_, String> {
            let work_handle = result_work_handle.into_store_owned_work_handle();
            let Some(work_record) = self.work.records.get(&work_handle.id) else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_handle_missing",
                    "reason": "ADR-2204 actor-request ready err driver must consume the exact store-owned WorkHandle once",
                    "work_id": work_handle.id.as_str(),
                    "work_generation": work_handle.generation(),
                })
                .to_string());
            };
            if !work_record
                .handle
                .matches_session_work_runtime_owner_v1(&work_handle)
            {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_handle_mismatch",
                    "reason": "ADR-2204 actor-request ready err driver requires the stored WorkRecord to match the typed err WorkHandle exactly",
                    "work_id": work_handle.id.as_str(),
                    "stored_generation": work_record.handle.generation(),
                    "expected_generation": work_handle.generation(),
                })
                .to_string());
            }
            if work_record.status != WorkStatus::Ready {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_not_ready",
                    "reason": "ADR-2204 actor-request ready err WorkHandle may be consumed only after the actor request result is Ready",
                    "work_id": work_handle.id.as_str(),
                    "work_status": &work_record.status,
                })
                .to_string());
            }
            let WorkFrame::ActorRequestReadyErrResult(frame) = &work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_frame_kind_mismatch",
                    "reason": "ADR-2204 actor-request ready err driver can consume only ActorRequestReadyErrResult work frames",
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            };
            if !frame
                .result_payload
                .matches_session_work_runtime_owner_v1(&expected_payload_handle)
            {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_authority_payload_mismatch",
                    "reason": "ADR-2204 actor-request err work authority must match the consumed store-owned payload handle exactly",
                    "payload_handle_match": false,
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            }
            let Some(work_record) = self.work.records.remove(&work_handle.id) else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_handle_missing_after_validation",
                    "reason": "ADR-2204 actor-request ready err work disappeared after exact authority validation",
                    "work_id": work_handle.id.as_str(),
                    "work_generation": work_handle.generation(),
                })
                .to_string());
            };
            let WorkFrame::ActorRequestReadyErrResult(frame) = work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "actor_request_ready_err_work_frame_kind_mismatch_after_validation",
                    "reason": "ADR-2204 actor-request ready err work frame changed after exact authority validation",
                    "work_id": work_handle.id.as_str(),
                })
                .to_string());
            };
            self.work.consumed_actor_request_ready_result_work.insert(
                work_handle.id.clone(),
                work_handle
                    .generation
                    .duplicate_for_session_work_runtime_owner_v1(),
            );
            let payload_handle = frame.result_payload;
            let payload_body = self.payloads.take_actor_request_ready_err_payload(
                &payload_handle,
                "actor_request_ready_err_work_driver",
            )?;
            Ok((payload_handle, payload_body))
        })()
        .map_err(StoreOwnedWorkSettlementFault::from)?;
        Err(
            StoreOwnedWorkSettlementFault::ActorRequestReadyErrStoreOwnedSettlementRequired {
                payload_handle,
                payload_body,
            },
        )
    }

    pub(crate) fn complete_provider_resume_application_by_effect_and_work(
        &mut self,
        effect_ref: &EffectRef,
        resume_work: &WorkHandle,
    ) -> Result<(), String> {
        {
            let work_record = self.work.records.get(&resume_work.id).ok_or_else(|| {
                serde_json::json!({
                    "kind": "provider_resume_completion_work_missing",
                    "reason": "ProviderResume completion requires the exact ProviderResume resume work selected by the completion ticket",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                })
                .to_string()
            })?;
            if !work_record
                .handle
                .matches_session_work_runtime_owner_v1(resume_work)
            {
                return Err(serde_json::json!({
                    "kind": "provider_resume_completion_work_handle_mismatch",
                    "reason": "ProviderResume completion requires the stored WorkHandle to match the selected completion ticket exactly",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                    "stored_generation": work_record.handle.generation(),
                    "expected_generation": resume_work.generation(),
                })
                .to_string());
            }
            if work_record.status != WorkStatus::WaitingOnEffect {
                return Err(serde_json::json!({
                    "kind": "provider_resume_completion_work_not_waiting_on_effect",
                    "reason": "ProviderResume completion may only ready work that is still waiting on the selected effect",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                    "work_status": &work_record.status,
                })
                .to_string());
            }
            let WorkFrame::ProviderResume(frame) = &work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "provider_resume_completion_frame_kind_mismatch",
                    "reason": "ProviderResume completion cannot apply a completion ticket to non-ProviderResume resume work",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                })
                .to_string());
            };
            if !effect_ref_matches_handle(effect_ref, &frame.effect) {
                return Err(serde_json::json!({
                    "kind": "provider_resume_completion_effect_mismatch",
                    "reason": "ProviderResume completion requires the ProviderResume frame to name the same effect as the selected completion ticket",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "requested_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                    "frame_effect": "session_work_runtime_owned",
                    "requested_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
                })
                .to_string());
            }
        }

        self.effects.mark_consumed_by_resume_work(
            effect_ref,
            resume_work,
            "provider_resume_application_completion",
        )?;
        let work_record = self.work.records.get_mut(&resume_work.id).ok_or_else(|| {
            serde_json::json!({
                "kind": "provider_resume_completion_work_missing_after_effect_consume",
                "reason": "ProviderResume completion consumed the effect but could not ready the exact selected work",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work.id.as_str(),
            })
            .to_string()
        })?;
        work_record.status = WorkStatus::Ready;
        self.scheduler
            .blocking_provider
            .enqueue_provider_completion_for_session_work_runtime_owner_v1(
                resume_work.duplicate_for_session_work_runtime_owner_v1(),
            );
        Ok(())
    }

    pub(crate) fn complete_event_append_publication_for_completion_ticket(
        &mut self,
        ticket: EventAppendApplicationCompletionTicket,
    ) -> Result<(), String> {
        let (effect_ref, resume_work) = ticket.into_parts_for_session_work_runtime_owner_v1();
        {
            let work_record = self.work.records.get(&resume_work.id).ok_or_else(|| {
                serde_json::json!({
                    "kind": "event_append_publication_completion_work_missing",
                    "reason": "EventAppend publication completion requires the exact EventAppend resume work selected by the publication ticket",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                })
                .to_string()
            })?;
            if !work_record
                .handle
                .matches_session_work_runtime_owner_v1(&resume_work)
            {
                return Err(serde_json::json!({
                    "kind": "event_append_publication_completion_work_handle_mismatch",
                    "reason": "EventAppend publication completion requires the stored WorkHandle to match the selected publication ticket exactly",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                    "stored_generation": work_record.handle.generation(),
                    "expected_generation": resume_work.generation(),
                })
                .to_string());
            }
            if work_record.status != WorkStatus::WaitingOnEffect {
                return Err(serde_json::json!({
                    "kind": "event_append_publication_completion_work_not_waiting_on_effect",
                    "reason": "EventAppend publication completion may only ready work that is still waiting on the selected effect",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                    "work_status": &work_record.status,
                })
                .to_string());
            }
            let WorkFrame::EventAppend(frame) = &work_record.frame else {
                return Err(serde_json::json!({
                    "kind": "event_append_publication_completion_frame_kind_mismatch",
                    "reason": "EventAppend publication completion cannot apply a publication ticket to non-EventAppend resume work",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "work_id": resume_work.id.as_str(),
                })
                .to_string());
            };
            if !effect_ref_matches_handle(&effect_ref, &frame.effect) {
                return Err(serde_json::json!({
                    "kind": "event_append_publication_completion_effect_mismatch",
                    "reason": "EventAppend publication completion requires the EventAppend frame to name the same effect as the selected publication ticket",
                    "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                    "requested_kind": effect_ref.kind_for_session_work_runtime_owner_v1(),
                    "frame_effect": "session_work_runtime_owned",
                    "requested_generation": effect_ref.generation_for_session_work_runtime_owner_v1(),
                })
                .to_string());
            }
        }

        self.effects.mark_consumed_by_resume_work(
            &effect_ref,
            &resume_work,
            "event_append_publication_completion",
        )?;
        let work_record = self.work.records.get_mut(&resume_work.id).ok_or_else(|| {
            serde_json::json!({
                "kind": "event_append_publication_completion_work_missing_after_effect_consume",
                "reason": "EventAppend publication completion consumed the effect but could not ready the exact selected work",
                "effect_id": effect_ref.id_str_for_session_work_runtime_owner_v1(),
                "work_id": resume_work.id.as_str(),
            })
            .to_string()
        })?;
        work_record.status = WorkStatus::Ready;
        Ok(())
    }
}

fn deterministic_shuffle_scheduler_completions<T>(values: &mut [T], mut state: u64) {
    if state == 0 || values.len() < 2 {
        return;
    }
    for upper in (1..values.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let selected = (state as usize) % (upper + 1);
        values.swap(upper, selected);
    }
}

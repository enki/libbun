# Adjacent Swarm retained-runtime final close, atomic deletion, and hostile-test owner source bundle (correction 6)

Exact source SHA: 95323ff17cb29928e31467f651ef03bae2099c14

Exact source tree: 43b47bbd49a6053d270b3e15cc141cb1b1bb86da

The exact repository-wide final-close discovery binds the retained provider pool and settlement lane, the separately defined shutdown helper, the consuming live-feed close, both outer close carriers, their module/privacy boundary, and the sole final success/fault consumer. The final consumer first shuts down the runtime-file pool, consumes graph settlements and the live feed, then either consumes the runtime-file session into successful settlement or restores it while marking closeout failed. A correction must reconcile the already-consumed retained runtime on failure, perform exact-once final shutdown, forbid retry from a consumed backend, delete stale plugin/callback/raw compatibility atomically, and require hostile refusal/retry/cancel/unwind/Drop/shutdown tests.

Every compact excerpt names the complete owning item span selected from the exact file, plus the full-file blob/SHA-256/byte identity and an excerpt SHA-256. Small bounded files are included completely. The repository-wide discovery gate runs before this fixed closure is rendered.

## Bound source inventory

| Path | Git blob | Full-file SHA-256 | Bytes | Included source |
| --- | --- | --- | ---: | --- |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs | f6e17b88112ce2155d59e73720fe6f4d768ef6be | 55cdd5dd24536523d35ae11ef15192d9d3b72c79c6d65f5ced4f172b631e370f | 21973 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs | 83a214b7502debaf2a9c4f16baadd249e0dabd4b | 963f60cde7af4217c253ed6ce9ffd03e4eb9bdbd80792e31dcf14567382fc473 | 20265 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs | 92ceabd5eda889f6d1763082c90d5932b78a5086 | 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de | 2269 | complete file |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs | 1caaed34441eb4de28053e6ea0acd0212981cbf8 | 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323 | 3030 | complete file |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs | b5f25593df8f095bc4c51b573be8f85a757b9f15 | 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5 | 85748 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs | 7379a6c2a2a8fcf9db5d882f3d314f7a7e930bf9 | 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012 | 6025 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs | 14d0aff964240b651d64b6cbc02622554dba61df | 54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760 | 29621 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs | c8676f5f9854111b3d4a928ada3a23ba991b8196 | 137bb8d5d0f3536f970d742c708d6f6ec7da840456e723f1ee473ac9b4168833 | 55502 | complete owning items |
| crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs | 32a948f725a24809c6a1d24b68a71d1c07a159ec | bc3b73f90cf21d8ae48b409c1c1158863c268907ba2be0063de669c0a5dde10f | 80170 | complete owning items |

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session.rs:295-311

- Full-file Git blob: f6e17b88112ce2155d59e73720fe6f4d768ef6be
- Full-file SHA-256: 55cdd5dd24536523d35ae11ef15192d9d3b72c79c6d65f5ced4f172b631e370f
- Full-file bytes: 21973
- Excerpt line span: 295-311
- Excerpt SHA-256: 97a8f301d1e175a26d8b5042a18ee8a8c54aa30d7628551568e8e03dfc5f96bc

   295      pub(in crate::test_runner) fn close_for_execution_graph_owner(
   296          self,
   297          session: &mut SsTestRunnerSession,
   298          graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
   299      ) -> SsResult<SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner> {
   300          let SsTestLiveSourceWorkSetRuntimePlanEmissionSession {
   301              live_feed_session,
   302              authored_file_order,
   303              ..
   304          } = self.emission;
   305          let result = live_feed_session.close_for_execution_graph_owner(
   306              session,
   307              authored_file_order,
   308              graph_settlements,
   309          )?;
   310          Ok(SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner::admit(result))
   311      }

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner.rs:1-85

- Full-file Git blob: 83a214b7502debaf2a9c4f16baadd249e0dabd4b
- Full-file SHA-256: 963f60cde7af4217c253ed6ce9ffd03e4eb9bdbd80792e31dcf14567382fc473
- Full-file bytes: 20265
- Excerpt line span: 1-85
- Excerpt SHA-256: a5cb570f2700de8381e19c51314407047fec4121155403431e7a14278b8ef708

     1  #[path = "runtime_plan_owner/body_authority_registry.rs"]
     2  mod body_authority_registry;
     3  #[path = "runtime_plan_owner/child_local_execution_custody.rs"]
     4  mod child_local_execution_custody;
     5  #[path = "runtime_plan_owner/ready_file_case_outcome.rs"]
     6  mod ready_file_case_outcome;
     7  use self::body_authority_registry::SsTestArtifactExecutionState;
     8  use self::ready_file_case_outcome::{
     9      SsReadyFileCaseExecution, SsReadyFileExecutionSettlementOwner,
    10  };
    11  #[path = "runtime_plan_owner/pool_worker_child.rs"]
    12  mod pool_worker_child;
    13  #[path = "runtime_plan_owner/pool_worker_transport_credential.rs"]
    14  mod pool_worker_transport_credential;
    15  #[path = "runtime_plan_owner/runtime_file_worker_execution_lease_registry.rs"]
    16  mod runtime_file_worker_execution_lease_registry;
    17  pub(in crate::test_runner) use pool_worker_child::{
    18      SsTestCompilerWorkerPhaseObservation, SsTestPoolWorkerParentObservedFrame,
    19      read_child_frame_for_pool_worker_parent_v1,
    20  };
    21  pub(crate) use pool_worker_child::{
    22      encode_run_frame_for_pool_harness_observation_v1,
    23      encode_shutdown_frame_for_pool_harness_observation_v1,
    24      read_child_frame_for_pool_harness_observation_v1,
    25      run_pool_worker_child_session_for_pool_worker_child_owner_v1,
    26  };
    27  pub(in crate::test_runner) use pool_worker_transport_credential::{
    28      SsTestPoolWorkerParentPreparedSettlementCargo, SsTestPoolWorkerRuntimeRefusalKind,
    29  };
    30  #[path = "runtime_plan_owner/pool_worker_parent.rs"]
    31  mod pool_worker_parent;
    32  pub(in crate::test_runner) use pool_worker_parent::SsTestPoolWorkerParentPool;
    33  #[path = "runtime_plan_owner/exact_terminal_observation_carriage.rs"]
    34  mod exact_terminal_observation_carriage;
    35  #[path = "runtime_plan_owner/phase_trace_projection.rs"]
    36  mod phase_trace_projection;
    37  #[path = "runtime_plan_owner/runtime_execution_domain.rs"]
    38  mod runtime_execution_domain;
    39  #[path = "runtime_plan_owner/source_work_set_artifact_dag.rs"]
    40  mod source_work_set_artifact_dag;
    41  #[path = "runtime_plan_owner/source_work_set_worker_execution.rs"]
    42  mod source_work_set_worker_execution;
    43  use super::super::preparation_failure::ss_collected_file_from_preparation_failure;
    44  use super::super::{
    45      SsCollectedTestCase, SsCollectedTestFile, SsPoolDispatchedSelectedSourceTestFile,
    46      SsSelectedSourcePoolDispatchRefusal, SsSelectedSourceTestFile,
    47      SsTestFileFailureCollectionReceipt, SsTestFileFailureReceipt, SsTestFileWorkStage,
    48      SsTestResultStatus,
    49  };
    50  use crate::test_runner::state::{
    51      SsTestExecutionGraphRuntimeFileExecutionSession,
    52      SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
    53      SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement,
    54      SsTestExecutionGraphRuntimeFileFailureFeedAdmission,
    55      SsTestExecutionGraphRuntimeFileReadyWorkAdmission, SsTestProfileSpan,
    56  };
    57  use crate::test_runner::{
    58      SsTestProfilePhase, SsTestProfileSpanContext, SsTestRunnerSession,
    59      SsTestRuntimePlanOwnerSession,
    60  };
    61  use crate::test_runner::{
    62      SsTestRunEvent, SsTestRunSummary, SsTestTarget, duration_nanos_u64, scheduler_width_projection,
    63      ss_test_no_tests_matched_diagnostic,
    64  };
    65  use crate::{SsError, SsResult};
    66  use serde_json::{Value, json};
    67  pub(in crate::test_runner::artifact_session) use source_work_set_artifact_dag::SsTestSourceWorkSetArtifactDagNodeId;
    68  use source_work_set_artifact_dag::{
    69      SsTestArtifactDagAuthority, SsTestSourceWorkSetArtifactDag,
    70      SsTestSourceWorkSetArtifactDagSnapshot, map_artifact_dag_result,
    71  };
    72  use source_work_set_worker_execution::SourceWorkSetRuntimePlanExecutionSession;
    73  pub(in crate::test_runner) use source_work_set_worker_execution::SourceWorkSetRuntimePlanWorkerExecutionAuthorities;
    74  use ss_runtime_source_compiler_owner::SsTestSourceWorkSetReceipt;
    75  use std::num::NonZeroUsize;
    76  use std::path::Path;
    77  use std::path::PathBuf;
    78  use std::time::{Duration, Instant};
    79
    80  use runtime_execution_domain::SsRuntimeExecutionDomainOwner;
    81  pub(in crate::test_runner) use runtime_execution_domain::{
    82      SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    83      SsRuntimeExecutionDomainCommittedFileCandidate,
    84      SsRuntimeExecutionDomainReadyFileGraphSettlement,
    85  };

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/external_capability_provider_pool.rs: complete file

- Full-file Git blob: 92ceabd5eda889f6d1763082c90d5932b78a5086
- Full-file SHA-256: 3eed047f11c5ac8bb8aee51dc4c5f7e96520af3b8ed41874d6736f643018c9de
- Full-file bytes: 2269
- Complete file line span: 1-66

     1  use crate::{SsError, SsExternalCapabilityProviderHost, SsResult};
     2  use serde_json::json;
     3  use std::path::PathBuf;
     4
     5  #[derive(Default)]
     6  pub(super) struct ExternalCapabilityProviderPool {
     7      active: Option<(PathBuf, SsExternalCapabilityProviderHost)>,
     8  }
     9
    10  pub(super) struct ExternalCapabilityProviderCheckout<'a> {
    11      provider: &'a mut SsExternalCapabilityProviderHost,
    12      pub(super) initialized: bool,
    13  }
    14
    15  impl ExternalCapabilityProviderPool {
    16      pub(super) fn checkout(
    17          &mut self,
    18          working_directory: PathBuf,
    19      ) -> SsResult<ExternalCapabilityProviderCheckout<'_>> {
    20          let should_replace = self
    21              .active
    22              .as_ref()
    23              .map(|(active_working_directory, _)| active_working_directory != &working_directory)
    24              .unwrap_or(true);
    25          if should_replace {
    26              if let Some((_, active_provider)) = self.active.as_mut() {
    27                  active_provider.shutdown()?;
    28              }
    29              self.active = Some((
    30                  working_directory.clone(),
    31                  SsExternalCapabilityProviderHost::new_for_ss_test_runtime_provider_owner_v1(
    32                      &working_directory,
    33                  )?,
    34              ));
    35          }
    36          let Some((_, provider)) = self.active.as_mut() else {
    37              return Err(SsError::Cli(
    38                  json!({
    39                      "schema": "swarm.ss.test.execution_fault.v1",
    40                      "code": "ss_test_external_capability_provider_pool_checkout_missing",
    41                      "reason": "ss test provider-host pool failed to retain the checked-out external capability provider backend",
    42                      "working_directory": working_directory.display().to_string(),
    43                  })
    44                  .to_string(),
    45              ));
    46          };
    47          Ok(ExternalCapabilityProviderCheckout {
    48              provider,
    49              initialized: should_replace,
    50          })
    51      }
    52
    53      pub(super) fn shutdown(&mut self) -> SsResult<()> {
    54          if let Some((_, provider)) = self.active.as_mut() {
    55              provider.shutdown()?;
    56          }
    57          self.active = None;
    58          Ok(())
    59      }
    60  }
    61
    62  impl ExternalCapabilityProviderCheckout<'_> {
    63      pub(super) fn provider_mut(&mut self) -> &mut SsExternalCapabilityProviderHost {
    64          self.provider
    65      }
    66  }

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/provider_settlement_lane.rs: complete file

- Full-file Git blob: 1caaed34441eb4de28053e6ea0acd0212981cbf8
- Full-file SHA-256: 48b30f77c809f06b7dfb130b11e190cff8e742809140840d14a3a4b5fb9a4323
- Full-file bytes: 3030
- Complete file line span: 1-86

     1  use super::{SsTestBodyWorkerLaunchAuthority, SsTestReadyFileBodyDispatchSettlement};
     2  use crate::SsResult;
     3  use crate::test_runner::artifact_session::SsCollectedTestCase;
     4
     5  #[path = "external_capability_provider_pool.rs"]
     6  mod external_capability_provider_pool;
     7
     8  use self::external_capability_provider_pool::ExternalCapabilityProviderPool;
     9
    10  pub(in super::super) struct SsTestProviderSettlementPool {
    11      provider_pool: ExternalCapabilityProviderPool,
    12  }
    13
    14  struct SsTestOwnerLibbunProviderSettlementLane<'a> {
    15      provider_pool: &'a mut ExternalCapabilityProviderPool,
    16  }
    17
    18  pub(in super::super) struct SsTestReadyFileBodyDispatchOwner<'a> {
    19      owner_libbun_lane: SsTestOwnerLibbunProviderSettlementLane<'a>,
    20  }
    21
    22  impl SsTestProviderSettlementPool {
    23      pub(in super::super) fn new() -> Self {
    24          Self {
    25              provider_pool: ExternalCapabilityProviderPool::default(),
    26          }
    27      }
    28
    29      pub(in super::super) fn admit_ready_file_body_dispatch_owner(
    30          &mut self,
    31      ) -> SsTestReadyFileBodyDispatchOwner<'_> {
    32          SsTestReadyFileBodyDispatchOwner {
    33              owner_libbun_lane: SsTestOwnerLibbunProviderSettlementLane {
    34                  provider_pool: &mut self.provider_pool,
    35              },
    36          }
    37      }
    38
    39      pub(in super::super) fn shutdown(&mut self) -> SsResult<()> {
    40          self.provider_pool.shutdown()
    41      }
    42  }
    43
    44  impl SsTestReadyFileBodyDispatchOwner<'_> {
    45      fn dispatch_ready_file_non_provider_body(
    46          &mut self,
    47          test: &SsCollectedTestCase,
    48          launch_authority: SsTestBodyWorkerLaunchAuthority,
    49      ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
    50          let _ = (self, test);
    51          launch_authority.execute_for_ready_file_body_dispatch_owner_v1(None, None)
    52      }
    53
    54      fn settle_ready_file_provider_affine_body(
    55          &mut self,
    56          test: &SsCollectedTestCase,
    57          launch_authority: SsTestBodyWorkerLaunchAuthority,
    58      ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
    59          let _ = test;
    60          let working_directory =
    61              launch_authority.libbun_working_directory_for_ready_file_body_dispatch_owner_v1();
    62          let mut provider_checkout = self
    63              .owner_libbun_lane
    64              .provider_pool
    65              .checkout(working_directory)?;
    66          let provider_checkout_initialized = Some(provider_checkout.initialized);
    67          launch_authority.execute_for_ready_file_body_dispatch_owner_v1(
    68              Some(provider_checkout.provider_mut()),
    69              provider_checkout_initialized,
    70          )
    71      }
    72  }
    73
    74  impl SsTestReadyFileBodyDispatchOwner<'_> {
    75      pub(in super::super) fn dispatch_ready_file_body(
    76          &mut self,
    77          test: &SsCollectedTestCase,
    78          launch_authority: SsTestBodyWorkerLaunchAuthority,
    79      ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
    80          if launch_authority.external_capability_provider_enabled() {
    81              self.settle_ready_file_provider_affine_body(test, launch_authority)
    82          } else {
    83              self.dispatch_ready_file_non_provider_body(test, launch_authority)
    84          }
    85      }
    86  }

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/runtime_execution_domain.rs:47-176

- Full-file Git blob: b5f25593df8f095bc4c51b573be8f85a757b9f15
- Full-file SHA-256: 5519fbbf653aa19cc883dc881eaaf4ff2587dc13bb2b7ab4d3ac2d9575a3b8d5
- Full-file bytes: 85748
- Excerpt line span: 47-176
- Excerpt SHA-256: 301c2a698efc3fed1faa2bc3d1094152a91cbb1774351cd7363a2d01e3a1bc7a

    47  pub(super) struct SsRuntimeExecutionDomainOwner {
    48      provider_settlement_pool: SsTestProviderSettlementPool,
    49  }
    50
    51  pub(super) struct SsRuntimeExecutionDomainState {
    52      deferred_owner_lane_ready_files: SsRuntimeExecutionDomainOwnerLaneQueue,
    53  }
    54
    55  pub(super) struct SsReadyFileExecutionFileAdmission {
    56      _private: (),
    57  }
    58
    59  pub(in crate::test_runner) struct SsRuntimeExecutionDomainReadyFileGraphSettlement {
    60      kind: SsRuntimeExecutionDomainReadyFileGraphSettlementKind,
    61  }
    62
    63  /// One-shot parent settlement minted by consuming the exact dispatched
    64  /// selected-source readiness through authenticated Running-slot staging.
    65  pub(in crate::test_runner) struct SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
    66      dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    67      preflighted: SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement,
    68  }
    69
    70  struct SsRuntimeExecutionDomainPreflightedPoolWorkerSettlement {
    71      outcome: SsTestReadyFileNodeOutcome,
    72      prepared_settlement: SsRuntimeExecutionDomainPreparedWorkerFileSettlement,
    73  }
    74
    75  pub(in crate::test_runner) struct SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal {
    76      pub(in crate::test_runner) dispatched_source: super::SsPoolDispatchedSelectedSourceTestFile,
    77      pub(in crate::test_runner) error: SsError,
    78  }
    79
    80  impl std::fmt::Debug for SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal {
    81      fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    82          formatter
    83              .debug_struct("SsRuntimeExecutionDomainPoolWorkerSettlementAdmissionRefusal")
    84              .field("error", &self.error.to_string())
    85              .finish_non_exhaustive()
    86      }
    87  }
    88
    89  impl SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement {
    90      pub(super) fn cancel_for_parent_owner_v1(
    91          self,
    92      ) -> super::SsPoolDispatchedSelectedSourceTestFile {
    93          self.dispatched_source
    94      }
    95  }
    96
    97  /// Move-only normalized runtime-file cargo minted by consuming a graph
    98  /// settlement only after the graph has committed that file.
    99  pub(in crate::test_runner) struct SsRuntimeExecutionDomainCommittedFileCandidate {
   100      projected_result: super::SsReadyFileProjectedResult,
   101      exact_terminal_observations:
   102          super::exact_terminal_observation_carriage::SsTestCommittedFileTerminalObservations,
   103      projection_dependencies: SsTestReadyFileResultProjectionDependencies,
   104  }
   105
   106  enum SsRuntimeExecutionDomainReadyFileGraphSettlementKind {
   107      OwnerSettlement {
   108          settlement: SsRuntimeExecutionDomainWorkerFileSettlement,
   109      },
   110      AdjudicatedCommittedFile {
   111          candidate: SsRuntimeExecutionDomainCommittedFileCandidate,
   112      },
   113  }
   114
   115  pub(super) struct SsRuntimeExecutionDomainBodyAuthorityOwner<'a> {
   116      artifact_execution_state: &'a mut super::SsTestArtifactExecutionState,
   117  }
   118
   119  struct SsRuntimeExecutionDomainOwnerLaneQueue {
   120      deferred: VecDeque<SsTestOwnerLaneReadyFileWorkItem>,
   121  }
   122
   123  impl SsRuntimeExecutionDomainOwner {
   124      pub(super) fn new() -> Self {
   125          Self {
   126              provider_settlement_pool: SsTestProviderSettlementPool::new(),
   127          }
   128      }
   129
   130      fn execute_owner_lane_ready_file(
   131          &mut self,
   132          work_item: SsTestOwnerLaneReadyFileWorkItem,
   133          artifact_execution_state: &mut super::SsTestArtifactExecutionState,
   134          session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
   135          spawned_worker_child_liveness_deadline: Duration,
   136      ) -> crate::SsResult<SsRuntimeExecutionDomainWorkerFileSettlement> {
   137          let mut body_authority_owner =
   138              SsRuntimeExecutionDomainBodyAuthorityOwner::new(artifact_execution_state);
   139          self.execute_owner_lane_ready_file_with_body_authority(
   140              work_item,
   141              &mut body_authority_owner,
   142              session,
   143              spawned_worker_child_liveness_deadline,
   144          )
   145      }
   146
   147      fn execute_owner_lane_ready_file_with_body_authority(
   148          &mut self,
   149          work_item: SsTestOwnerLaneReadyFileWorkItem,
   150          body_authority_owner: &mut SsRuntimeExecutionDomainBodyAuthorityOwner<'_>,
   151          session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
   152          spawned_worker_child_liveness_deadline: Duration,
   153      ) -> crate::SsResult<SsRuntimeExecutionDomainWorkerFileSettlement> {
   154          let _ = spawned_worker_child_liveness_deadline;
   155          work_item.execute(
   156              body_authority_owner,
   157              &mut self.provider_settlement_pool,
   158              session,
   159          )
   160      }
   161
   162      pub(super) fn shutdown(
   163          &mut self,
   164          session: &mut crate::test_runner::SsTestRuntimePlanOwnerSession,
   165      ) -> crate::SsResult<()> {
   166          let started = Instant::now();
   167          let shutdown_result = self.provider_settlement_pool.shutdown();
   168          session.record_profile_span(
   169              SsTestProfilePhase::ProviderHostPoolShutdown,
   170              started.elapsed(),
   171              SsTestProfileSpanContext::counters(json!({
   172                  "status": if shutdown_result.is_ok() { "shutdown" } else { "failed" },
   173              })),
   174          );
   175          shutdown_result
   176      }

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_live_feed_session.rs:1-140

- Full-file Git blob: 7379a6c2a2a8fcf9db5d882f3d314f7a7e930bf9
- Full-file SHA-256: 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012
- Full-file bytes: 6025
- Excerpt line span: 1-140
- Excerpt SHA-256: 3be90814fe8071aed3c48778c307a63266d3fc10aa689581e73f7a4e6bf83012

     1  pub(in crate::test_runner::artifact_session) struct SsTestSourceWorkSetRuntimePlanLiveFeedSession {
     2      runtime_plan_execution_session: SourceWorkSetRuntimePlanExecutionSession,
     3      runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
     4      source_work_set_receipt: SsTestSourceWorkSetReceipt,
     5      runtime_execution_domain_owner: SsRuntimeExecutionDomainOwner,
     6  }
     7
     8  impl SsTestSourceWorkSetRuntimePlanLiveFeedSession {
     9      pub(in crate::test_runner::artifact_session) fn open(
    10          source_work_set_generation_id: &str,
    11          _package_graph_session_fingerprint: &str,
    12          _package_graph_manifest_fingerprint: Option<&str>,
    13          total_admission_file_count: usize,
    14          worker_limit: Option<NonZeroUsize>,
    15          runtime_plan_owner_session: SsTestRuntimePlanOwnerSession,
    16          source_work_set_receipt: SsTestSourceWorkSetReceipt,
    17          runtime_plan_background_liveness_deadline: Duration,
    18          spawned_worker_child_liveness_deadline: Duration,
    19      ) -> SsResult<Self> {
    20          let artifact_session_authority =
    21              SsTestArtifactSessionAuthority::from_source_work_set_receipt(
    22                  source_work_set_generation_id.to_owned(),
    23                  &source_work_set_receipt,
    24              );
    25          let runtime_plan_execution_session = SourceWorkSetRuntimePlanExecutionSession::new(
    26              artifact_session_authority,
    27              total_admission_file_count,
    28              source_work_set_generation_id.to_owned(),
    29              worker_limit,
    30              runtime_plan_background_liveness_deadline,
    31              spawned_worker_child_liveness_deadline,
    32          )?;
    33          Ok(Self {
    34              runtime_plan_execution_session,
    35              runtime_plan_owner_session,
    36              source_work_set_receipt,
    37              runtime_execution_domain_owner: SsRuntimeExecutionDomainOwner::new(),
    38          })
    39      }
    40
    41      pub(in crate::test_runner::artifact_session) fn admit_live_file_product_emission(
    42          &mut self,
    43          file: SsTestFileFailureReceipt,
    44      ) -> SsResult<()> {
    45          self.runtime_plan_execution_session
    46              .admit_live_file_product_emission(file)
    47      }
    48
    49      pub(in crate::test_runner::artifact_session) fn close_file_failure_feed_for_execution_graph_owner(
    50          &mut self,
    51          session: &mut SsTestRunnerSession,
    52      ) -> SsResult<()> {
    53          self.runtime_plan_execution_session
    54              .close_file_failure_feed_for_execution_graph_owner(
    55                  &mut self.runtime_plan_owner_session,
    56              )?;
    57          let _ = session;
    58          Ok(())
    59      }
    60
    61      pub(in crate::test_runner::artifact_session) fn admit_next_runtime_file_ready_work_for_execution_graph_owner(
    62          &mut self,
    63          runtime_file_execution_session: &mut SsTestExecutionGraphRuntimeFileExecutionSession,
    64      ) -> SsResult<SsTestExecutionGraphRuntimeFileReadyWorkAdmission> {
    65          self.runtime_plan_execution_session
    66              .admit_next_runtime_file_ready_work_for_execution_graph_owner(
    67                  runtime_file_execution_session,
    68              )
    69      }
    70
    71      pub(in crate::test_runner::artifact_session) fn admit_next_file_failure_to_live_source_for_execution_graph_owner(
    72          &mut self,
    73          session: &mut SsTestRunnerSession,
    74      ) -> SsResult<SsTestExecutionGraphRuntimeFileFailureFeedAdmission> {
    75          let admission = self
    76              .runtime_plan_execution_session
    77              .admit_next_file_failure_to_live_source_for_execution_graph_owner(
    78                  &mut self.runtime_plan_owner_session,
    79              )?;
    80          let _ = session;
    81          Ok(admission)
    82      }
    83
    84      pub(in crate::test_runner::artifact_session) fn commit_admitted_pool_worker_settlement_for_execution_graph_owner(
    85          &mut self,
    86          admitted: runtime_execution_domain::SsRuntimeExecutionDomainAdmittedPoolWorkerSettlement,
    87      ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
    88          runtime_execution_domain::commit_admitted_pool_worker_settlement_for_execution_graph_owner_v1(
    89              admitted,
    90          )
    91      }
    92
    93      pub(in crate::test_runner::artifact_session) fn settle_pool_worker_loss_for_execution_graph_owner(
    94          &mut self,
    95          dispatched_source: SsPoolDispatchedSelectedSourceTestFile,
    96          worker_loss_fault: &serde_json::Value,
    97      ) -> SsResult<SsRuntimeExecutionDomainReadyFileGraphSettlement> {
    98          runtime_execution_domain::settle_pool_worker_loss_for_execution_graph_owner_v1(
    99              dispatched_source,
   100              worker_loss_fault,
   101              &self.runtime_plan_owner_session,
   102          )
   103      }
   104
   105      pub(in crate::test_runner::artifact_session) fn execute_runtime_file_worker_input_for_execution_graph_owner(
   106          &mut self,
   107          session: &mut SsTestRunnerSession,
   108          worker_input: SsTestExecutionGraphRuntimeFileExecutionWorkerInput,
   109      ) -> SsResult<SsTestExecutionGraphRuntimeFileExecutionWorkerSettlement> {
   110          let worker_settlement = self
   111              .runtime_plan_execution_session
   112              .execute_runtime_file_worker_input_for_execution_graph_owner(
   113                  &mut self.runtime_plan_owner_session,
   114                  &mut self.runtime_execution_domain_owner,
   115                  worker_input,
   116              );
   117          let _ = session;
   118          worker_settlement
   119      }
   120
   121      pub(in crate::test_runner::artifact_session) fn close_for_execution_graph_owner(
   122          self,
   123          session: &mut SsTestRunnerSession,
   124          authored_file_order: Vec<String>,
   125          graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
   126      ) -> SsResult<Value> {
   127          let mut runtime_plan_owner_session = self.runtime_plan_owner_session;
   128          let result = self
   129              .runtime_plan_execution_session
   130              .close_runtime_plan_feed_for_execution_graph_owner(
   131                  &mut runtime_plan_owner_session,
   132                  self.source_work_set_receipt,
   133                  authored_file_order,
   134                  graph_settlements,
   135              );
   136          session
   137              .append_runtime_plan_owner_observations(runtime_plan_owner_session.into_observations());
   138          result
   139      }
   140  }

## crates/ss-runtime-test-execution-owner/src/test_runner/artifact_session/runtime_plan_owner/source_work_set_worker_execution.rs:700-736

- Full-file Git blob: 14d0aff964240b651d64b6cbc02622554dba61df
- Full-file SHA-256: 54c9aaee5abca13fb791eeffff19e8abd3b02134db567bec5135d439bc691760
- Full-file bytes: 29621
- Excerpt line span: 700-736
- Excerpt SHA-256: 8a75b25a58856894a1b3714b47e708670843b9cbb46f1069a42ec151b5b2ab30

   700                      events,
   701                      test_count,
   702                  )
   703          } else {
   704              self.owner_session.require_ready_for_projection()?;
   705              let events = closeout_projection.into_execution_projection();
   706              if authored_file_order.is_empty() && graph_settlements.is_empty() {
   707                  Ok(self
   708                      .artifact_dag_owner
   709                      .executor
   710                      .project_empty_source_work_set_run(
   711                          session,
   712                          None,
   713                          source_work_set_receipt,
   714                          events,
   715                      ))
   716              } else {
   717                  self.owner_session.project_settled_ready_file_node_outcomes(
   718                      &mut self.artifact_dag_owner.executor,
   719                      session,
   720                      None,
   721                      source_work_set_receipt,
   722                      events,
   723                      authored_file_order,
   724                      graph_settlements,
   725                  )
   726              }
   727          }
   728      }
   729  }
   730
   731  fn shutdown_runtime_execution_domain_owner(
   732      runtime_execution_domain_owner: &mut SsRuntimeExecutionDomainOwner,
   733      session: &mut SsTestRuntimePlanOwnerSession,
   734  ) -> SsResult<()> {
   735      runtime_execution_domain_owner.shutdown(session)
   736  }

## crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner.rs:1424-1435

- Full-file Git blob: c8676f5f9854111b3d4a928ada3a23ba991b8196
- Full-file SHA-256: 137bb8d5d0f3536f970d742c708d6f6ec7da840456e723f1ee473ac9b4168833
- Full-file bytes: 55502
- Excerpt line span: 1424-1435
- Excerpt SHA-256: 739af8e2311dd4981d9fad6183f2998f30eb36b31c26323a2fc3c51e1fdb3c5c

  1424      fn close_for_execution_graph_owner(
  1425          self,
  1426          session: &mut SsTestRunnerSession,
  1427          graph_settlements: Vec<SsRuntimeExecutionDomainReadyFileGraphSettlement>,
  1428      ) -> SsResult<SsTestSourceWorkSetRuntimePlanTerminalSummaryForExecutionGraphOwner> {
  1429          let summary = self
  1430              .live_runtime_plan_emission_session
  1431              .close_for_execution_graph_owner(session, graph_settlements)?;
  1432          self.feed_emission_obligations
  1433              .close_check_for_feed_close()?;
  1434          Ok(summary)
  1435      }

## crates/ss-runtime-test-execution-owner/src/test_runner/state/source_work_set_execution_graph_owner/source_work_set_runtime_dispatch.rs:1560-1612

- Full-file Git blob: 32a948f725a24809c6a1d24b68a71d1c07a159ec
- Full-file SHA-256: bc3b73f90cf21d8ae48b409c1c1158863c268907ba2be0063de669c0a5dde10f
- Full-file bytes: 80170
- Excerpt line span: 1560-1612
- Excerpt SHA-256: cac9395ff570b49e8a2b836029f536bcaa2d249fc20ff3c423c83c5fcc1f7a24

  1560      pub(super) fn produce_graph_close_receipt_for_execution_graph_owner(
  1561          &mut self,
  1562          session: &mut SsTestRunnerSession,
  1563          graph_coordinator_context: &mut SsTestExecutionGraphCoordinatorContext,
  1564      ) -> SsResult<SsTestExecutionGraphCoordinatorStepReceipt> {
  1565          let mut runtime_file_execution_session = graph_coordinator_context
  1566              .runtime_file_execution_session
  1567              .take()
  1568              .ok_or_else(|| {
  1569                  SsError::Cli(
  1570                      json!({
  1571                          "schema": "swarm.ss.test.execution_graph_kernel_fault.v1",
  1572                          "code": "ss_test_execution_graph_runtime_file_session_consumed",
  1573                          "reason": "runtime-file graph session may be consumed exactly once by graph-owned closeout",
  1574                      })
  1575                      .to_string(),
  1576                  )
  1577              })?;
  1578          if !runtime_file_execution_session.is_terminal_runtime_file_work_for_execution_graph_owner()
  1579          {
  1580              let blocked_receipt =
  1581                  runtime_file_execution_session.runtime_file_close_blocked_receipt();
  1582              graph_coordinator_context.runtime_file_execution_session =
  1583                  Some(runtime_file_execution_session);
  1584              return Ok(blocked_receipt);
  1585          }
  1586          graph_coordinator_context
  1587              .runtime_file_pool
  1588              .shutdown_and_reap_for_execution_graph_owner_v1()?;
  1589          let graph_settlements = runtime_file_execution_session
  1590              .consume_ready_file_graph_settlements_for_graph_closeout_projection()?;
  1591          let runtime_plan_closeout_ticket = self.graph_session.admit_runtime_plan_closeout()?;
  1592          let live_runtime_plan_feed =
  1593              graph_coordinator_context.consume_live_runtime_plan_feed_for_execution_graph_owner()?;
  1594          match live_runtime_plan_feed.close_for_execution_graph_owner(session, graph_settlements) {
  1595              Ok(terminal_summary) => {
  1596                  self.graph_session
  1597                      .consume_runtime_file_execution_session(runtime_file_execution_session)?;
  1598                  let finished = self.graph_session.settle_runtime_plan_closeout_succeeded(
  1599                      runtime_plan_closeout_ticket,
  1600                      terminal_summary,
  1601                  )?;
  1602                  Ok(SsTestExecutionGraphCoordinatorStepReceipt::GraphClosed { finished })
  1603              }
  1604              Err(error) => {
  1605                  graph_coordinator_context.runtime_file_execution_session =
  1606                      Some(runtime_file_execution_session);
  1607                  self.graph_session
  1608                      .settle_runtime_plan_closeout_failed(runtime_plan_closeout_ticket)?;
  1609                  Err(error)
  1610              }
  1611          }
  1612      }

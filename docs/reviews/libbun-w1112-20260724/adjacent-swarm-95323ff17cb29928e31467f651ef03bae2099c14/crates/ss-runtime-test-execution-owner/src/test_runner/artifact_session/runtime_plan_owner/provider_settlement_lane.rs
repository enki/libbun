use super::{SsTestBodyWorkerLaunchAuthority, SsTestReadyFileBodyDispatchSettlement};
use crate::SsResult;
use crate::test_runner::artifact_session::SsCollectedTestCase;

#[path = "external_capability_provider_pool.rs"]
mod external_capability_provider_pool;

use self::external_capability_provider_pool::ExternalCapabilityProviderPool;

pub(in super::super) struct SsTestProviderSettlementPool {
    provider_pool: ExternalCapabilityProviderPool,
}

struct SsTestOwnerLibbunProviderSettlementLane<'a> {
    provider_pool: &'a mut ExternalCapabilityProviderPool,
}

pub(in super::super) struct SsTestReadyFileBodyDispatchOwner<'a> {
    owner_libbun_lane: SsTestOwnerLibbunProviderSettlementLane<'a>,
}

impl SsTestProviderSettlementPool {
    pub(in super::super) fn new() -> Self {
        Self {
            provider_pool: ExternalCapabilityProviderPool::default(),
        }
    }

    pub(in super::super) fn admit_ready_file_body_dispatch_owner(
        &mut self,
    ) -> SsTestReadyFileBodyDispatchOwner<'_> {
        SsTestReadyFileBodyDispatchOwner {
            owner_libbun_lane: SsTestOwnerLibbunProviderSettlementLane {
                provider_pool: &mut self.provider_pool,
            },
        }
    }

    pub(in super::super) fn shutdown(&mut self) -> SsResult<()> {
        self.provider_pool.shutdown()
    }
}

impl SsTestReadyFileBodyDispatchOwner<'_> {
    fn dispatch_ready_file_non_provider_body(
        &mut self,
        test: &SsCollectedTestCase,
        launch_authority: SsTestBodyWorkerLaunchAuthority,
    ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
        let _ = (self, test);
        launch_authority.execute_for_ready_file_body_dispatch_owner_v1(None, None)
    }

    fn settle_ready_file_provider_affine_body(
        &mut self,
        test: &SsCollectedTestCase,
        launch_authority: SsTestBodyWorkerLaunchAuthority,
    ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
        let _ = test;
        let working_directory =
            launch_authority.libbun_working_directory_for_ready_file_body_dispatch_owner_v1();
        let mut provider_checkout = self
            .owner_libbun_lane
            .provider_pool
            .checkout(working_directory)?;
        let provider_checkout_initialized = Some(provider_checkout.initialized);
        launch_authority.execute_for_ready_file_body_dispatch_owner_v1(
            Some(provider_checkout.provider_mut()),
            provider_checkout_initialized,
        )
    }
}

impl SsTestReadyFileBodyDispatchOwner<'_> {
    pub(in super::super) fn dispatch_ready_file_body(
        &mut self,
        test: &SsCollectedTestCase,
        launch_authority: SsTestBodyWorkerLaunchAuthority,
    ) -> SsResult<SsTestReadyFileBodyDispatchSettlement> {
        if launch_authority.external_capability_provider_enabled() {
            self.settle_ready_file_provider_affine_body(test, launch_authority)
        } else {
            self.dispatch_ready_file_non_provider_body(test, launch_authority)
        }
    }
}

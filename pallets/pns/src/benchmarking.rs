//! Benchmarking setup for pallet-power
#![cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use super::*;
#[allow(unused)]
use crate::Pallet as Miner;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn change_worker_address() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn change_peer_id() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn confirm_update_worker_key() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn change_owner_address() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        create(RawOrigin::Signed(caller));
    }

    impl_benchmark_test_suite!(Miner, crate::mock::new_test_ext(), crate::mock::Test,);
}

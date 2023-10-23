//! Benchmarking setup for pallet-miner
#![cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

use super::*;
#[allow(unused)]
use crate::Pallet as Miner;

#[benchmarks]
mod benchmarks {
    use pallet_pns_common::{address::Address, registered_proof::RegisteredPoStProof};

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
        change_worker_address(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn change_peer_id() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        change_peer_id(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn confirm_update_worker_key() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        confirm_update_worker_key(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn change_owner_address() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        change_owner_address(RawOrigin::Signed(caller));
    }

    impl_benchmark_test_suite!(Miner, crate::mock::new_test_ext(), crate::mock::Test,);
}

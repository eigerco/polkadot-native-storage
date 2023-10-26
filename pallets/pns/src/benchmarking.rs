//! Benchmarking setup for pallet-pns
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
    fn store_file() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        store_file(RawOrigin::Signed(caller));
    }

    #[benchmark]
    fn retrieve_file() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        retrieve_file(RawOrigin::Signed(caller));
    }


    impl_benchmark_test_suite!(PnsModule, crate::mock::new_test_ext(), crate::mock::Test,);
}

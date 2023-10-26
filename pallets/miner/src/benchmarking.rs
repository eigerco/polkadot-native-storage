//! Benchmarking setup for pallet-miner
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
    fn check_events() {
        let caller: T::AccountId = whitelisted_caller();

        #[extrinsic_call]
        check_events(RawOrigin::Signed(caller.clone()));
    }

    impl_benchmark_test_suite!(Miner, crate::mock::new_test_ext(), crate::mock::Test,);
}

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

use frame_support::dispatch::Vec;
pub use pallet::{Claims, Config, MinerCount, Pallet, TotalRawBytesPower};
use pallet_pns_common::{address::Address, registered_proof::RegisteredPoStProof, Claim, Power};

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Libp2p Peer Identifier, usually array of bytes
        type PeerId: Parameter + Member + AsRef<[u8]> + Clone + Send + 'static;
        /// Unit used for recoding raw bytes and quality adjusted power
        type StoragePower: Parameter + Member + Clone + Default;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info] // Allows to define storage items without fixed size
    pub struct Pallet<T>(PhantomData<T>);

    /// Miners address mapped to their Claims on storage power
    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub type Claims<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Claim<T::StoragePower>>;

    /// Total Miner registered in the system
    #[pallet::storage]
    #[pallet::getter(fn miner_count)]
    pub type MinerCount<T: Config> = StorageValue<_, u64>;

    /// Total Power in Raw bytes declared in the system
    #[pallet::storage]
    #[pallet::getter(fn total_raw_bytes_power)]
    pub type TotalRawBytesPower<T: Config> = StorageValue<_, u64>;

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        Unknown,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Empty - there may be some extrinsics in the future
    }
}

impl<T: Config> Power for Pallet<T> {
    type AccountId = T::AccountId;
    type StoragePower = T::StoragePower;
    type PeerId = T::PeerId;

    /// Sample creating a miner with a new worker address.
    /// It returns new claim of storage power for the miner
    fn create_miner(
        miner: T::AccountId,
        _owner: T::AccountId,
        _worker: Address,
        _window_post_proof_type: RegisteredPoStProof,
        _peer: T::PeerId,
        _multiaddrs: Vec<Vec<u8>>,
    ) -> Option<Claim<Self::StoragePower>> {
        let miner_count = MinerCount::<T>::get().unwrap_or_default();
        if let Some(new_miner_count) = miner_count.checked_add(1) {
            let claim = Claim::default();
            Claims::<T>::insert(miner, claim.clone());
            MinerCount::<T>::put(new_miner_count);
            Some(claim)
        } else {
            None
        }
    }

    /// As claims can be updated, this function is used to update the claim of a miner.
    fn update_claimed_power(
        _miner: <T as frame_system::Config>::AccountId,
        _raw_bytes_delta: Self::StoragePower,
        _quality_adjusted_delta: Self::StoragePower,
    ) -> Option<Claim<Self::StoragePower>> {
        unimplemented!()
    }
}

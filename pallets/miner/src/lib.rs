#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Decode, Encode};
use frame_support::dispatch::Vec;
use pallet_pns_common::address::Address;
use scale_info::TypeInfo;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use pallet_pns_common::{
        address::Address, registered_proof::RegisteredPoStProof, AccountIdConversion, MinerId,
        Power,
    };

    use super::{MinerInfo, Vec};
    use crate::{MinerControllers, WorkerKeyChange};
    pub type MinerAccountId<T> = <<T as Config>::Power as Power>::AccountId;
    pub type PeerId<T> = <<T as Config>::Power as Power>::PeerId;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Power: Power;
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type BlockDelay: Get<BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info] // Allows to define storage items without fixed size
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn miners)]
    pub type Miners<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        MinerAccountId<T>,
        MinerInfo<T::AccountId, BlockNumberFor<T>, PeerId<T>>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn miner_index)]
    pub type MinerIndex<T: Config> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emits new miner address
        MinerCreated { miner_account_id: T::AccountId },

        /// Emits when worker address change is requested
        WorkerChangeRequested {
            miner_account_id: MinerAccountId<T>,
            new_worker: Address,
            new_controllers: MinerControllers<T::AccountId>,
        },

        /// Emits when peer id is changed
        PeerIdChanged {
            miner_account_id: MinerAccountId<T>,
            new_peer_id: PeerId<T>,
        },

        /// Emits when worker address is changed
        WorkerChanged {
            miner_account_id: MinerAccountId<T>,
            new_worker: Address,
        },

        /// Emits miner address and new owner address to update to
        OwnerChangeRequested {
            miner_account_id: MinerAccountId<T>,
            new_owner: T::AccountId,
        },

        /// Emits miner address and new owner address
        OwnerChanged {
            miner_account_id: MinerAccountId<T>,
            new_owner: T::AccountId,
        },

        TestEvent {
            emiter: T::AccountId
        }
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        Overflow,
        ClaimsNotSet,
        NoSuchMiner,
        InvalidSigner,
        NoRequest,
        IneffectiveRequest,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            owner: T::AccountId,
            worker: Address,
            peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let _who = ensure_signed(origin)?;

            let mut miner_index = MinerIndex::<T>::get().unwrap_or_default();
            miner_index = miner_index.checked_add(1).ok_or(Error::<T>::Overflow)?;
            let miner: MinerAccountId<T> = MinerId(miner_index).into_account();
            MinerIndex::<T>::put(miner_index);

            T::Power::create_miner(
                miner.clone(),
                miner.clone(), // For now miner is his own owner
                worker,
                RegisteredPoStProof::StackedDRGWindow64GiBV1P1,
                peer_id.clone(),
                Vec::new(),
            )
            .ok_or(Error::<T>::ClaimsNotSet)?;

            let miner_info = MinerInfo {
                owner: owner.clone(),
                worker,
                controllers: Vec::new(),
                peer_id,
                pending_worker: None,
                pending_owner: None,
            };

            Miners::<T>::insert(miner.clone(), miner_info);
            Self::deposit_event(Event::MinerCreated {
                miner_account_id: owner,
            });

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn change_worker_address(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_worker: Address,
            new_controllers: MinerControllers<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            let mut miner_info =
                Miners::<T>::try_get(&miner).map_err(|_| Error::<T>::NoSuchMiner)?;

            // Ensure that the caller is the owner of the miner to make any updates
            ensure!(who == miner_info.owner, Error::<T>::InvalidSigner);

            // This is different from filecoin miner_actor impl where ChangeWorkerAddress will ALWAYS overwrite the existing control addresses
            // with the control addresses passed in the params. Instead we match MinerControllers
            // Variant here
            if let MinerControllers::Override(controllers) = new_controllers.clone() {
                miner_info.controllers = controllers;
            }

            // A worker change will be scheduled if the worker passed in the params is different from the existing worker.
            if miner_info.worker != new_worker {
                miner_info.pending_worker = Some(WorkerKeyChange {
                    new_worker: new_worker.clone(),
                    effective_at: <frame_system::Pallet<T>>::block_number() + T::BlockDelay::get(),
                });
            } else {
                miner_info.pending_worker = None;
            }

            Miners::<T>::insert(&miner, miner_info);
            Self::deposit_event(Event::WorkerChangeRequested {
                miner_account_id: miner,
                new_worker,
                new_controllers,
            });

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn change_peer_id(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_peer_id: PeerId<T>,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            Miners::<T>::try_mutate(&miner, |maybe_miner_info| -> DispatchResultWithPostInfo {
                let miner_info = maybe_miner_info.as_mut().ok_or(Error::<T>::NoSuchMiner)?;
                ensure!(
                    who == miner_info.owner
                        || miner_info.controllers.iter().any(|account| account == &who),
                    Error::<T>::InvalidSigner
                );
                miner_info.peer_id = new_peer_id.clone();
                Self::deposit_event(Event::PeerIdChanged {
                    miner_account_id: miner.clone(),
                    new_peer_id,
                });
                Ok(().into())
            })
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn confirm_update_worker_key(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let _who = ensure_signed(origin)?;

            Miners::<T>::try_mutate(&miner, |maybe_miner_info| -> DispatchResultWithPostInfo {
                let miner_info = maybe_miner_info.as_mut().ok_or(Error::<T>::NoSuchMiner)?;
                if let Some(key_change) = &miner_info.pending_worker {
                    // Can only change to new_worker addr after effective_at block number
                    if key_change.effective_at <= <frame_system::Pallet<T>>::block_number() {
                        let new_worker = key_change.new_worker.clone();
                        miner_info.worker = new_worker.clone();
                        miner_info.pending_worker = None;
                        Self::deposit_event(Event::WorkerChanged {
                            miner_account_id: miner.clone(),
                            new_worker,
                        });
                        Ok(().into())
                    } else {
                        Err(Error::<T>::IneffectiveRequest.into())
                    }
                } else {
                    Err(Error::<T>::NoRequest.into())
                }
            })
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn change_owner_address(
            origin: OriginFor<T>,
            miner: MinerAccountId<T>,
            new_owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let signer = ensure_signed(origin)?;

            let mut miner_info =
                Miners::<T>::try_get(&miner).map_err(|_| Error::<T>::NoSuchMiner)?;

            match miner_info.pending_owner {
                Some(proposed_owner) if new_owner == proposed_owner && signer == proposed_owner => {
                    // New owner confirms proposed
                    miner_info.owner = signer;
                    miner_info.pending_owner = None;
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChanged {
                        miner_account_id: miner,
                        new_owner,
                    });
                }
                Some(_) if signer == miner_info.owner && signer == new_owner => {
                    // Existing owner cancels the ownership change
                    miner_info.pending_owner = None;
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested {
                        miner_account_id: miner,
                        new_owner,
                    });
                }
                Some(_) if signer == miner_info.owner => {
                    // Override existing proposal
                    miner_info.pending_owner = Some(new_owner.clone());
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested {
                        miner_account_id: miner,
                        new_owner,
                    });
                }
                None if signer == miner_info.owner && new_owner == miner_info.owner => {
                    // Attempted to change ownership to themselves
                    return Err(Error::<T>::IneffectiveRequest.into());
                }
                None if signer == miner_info.owner => {
                    // Initiate ownership transfer of the miner
                    miner_info.pending_owner = Some(new_owner.clone());
                    Miners::<T>::insert(miner.clone(), miner_info);
                    Self::deposit_event(Event::<T>::OwnerChangeRequested {
                        miner_account_id: miner,
                        new_owner,
                    });
                }
                Some(_) | None => {
                    let is_current_owner = signer == miner_info.owner;
                    let is_proposed_owner = miner_info
                        .pending_owner
                        .map(|po| po == signer)
                        .unwrap_or(false);

                    assert!(!is_current_owner && !is_proposed_owner);
                    return Err(Error::<T>::InvalidSigner.into());
                }
            }

            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn check_events(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;

            Self::deposit_event(Event::TestEvent { emiter: who });

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }
    }
}

/// Miner information stored in the storage
#[derive(Encode, Decode, TypeInfo)]
pub struct MinerInfo<
    AccountId: Encode + Decode + Eq + PartialEq,
    BlockNumber: Encode + Decode + Eq + PartialEq,
    PeerId: Encode + Decode + Eq + PartialEq,
> {
    /// Owner of this Miner
    owner: AccountId,
    /// Worker of this Miner
    /// Used to sign messages (and in the future blocks) on behalf of the miner
    /// Worker address can be changed by the owner and can be outside of Substrate ecosystem
    worker: Address,
    /// Other addresses that can sign messages on behalf of the miner,
    /// a limit for max number of controllers to be added
    controllers: Vec<AccountId>,
    /// Miner's libp2p PeerId
    peer_id: PeerId,
    /// Update to this worker address to at defined time
    pending_worker: Option<WorkerKeyChange<BlockNumber>>,
    /// Update to this owner address when it confirms
    pending_owner: Option<AccountId>,
}

/// Helper structure for updating worker address
#[derive(Encode, Decode, Debug, TypeInfo)]
pub struct WorkerKeyChange<BlockNumber: Encode + Decode + Eq + PartialEq> {
    /// New Worker Address to be updated
    new_worker: Address,
    /// Time after which confirm_update_worker_key will trigger updates to MinerInfo
    effective_at: BlockNumber,
}

/// Miner controllers structure
#[derive(Encode, Decode, Debug, PartialEq, Clone, TypeInfo)]
pub enum MinerControllers<AccountId> {
    /// Miner controller accounts should be set as follows
    Override(Vec<AccountId>),
    /// Miner controller accounts should not change
    NoChange,
}

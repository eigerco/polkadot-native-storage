#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod address;
pub mod registered_proof;

use frame_support::{dispatch::Vec, Parameter};
use sp_runtime::RuntimeDebug;
use sp_runtime::traits::Member;

use codec::{Encode, Decode};
use scale_info::TypeInfo;

use crate::{address::Address, registered_proof::RegisteredPoStProof};

use serde::{Deserialize, Serialize};

use alloc::string::String;

/// Identifier for Actors, includes builtin and initialized actors
pub type ActorID = u64;

/// Identifier for a CID
pub type Cid = String;

/// Data reference structure
#[derive(Clone, Encode, Decode, Default, TypeInfo, Serialize, Deserialize)]
pub struct DataRef
{
    pub piece_cid: Cid,
    pub piece_size: u32,
    pub root: Cid,
    pub transfer_type: String
}

/// Deal information structure
#[derive(Clone, Encode, Decode, Default, TypeInfo, Serialize, Deserialize)]
pub struct DealInfo
{
    pub creation_time: u64,
    pub data_ref: DataRef,
    pub deal_id: u32,
    pub duration: u32,
    pub message: String,
    pub piece_cid: Cid,
    pub price_per_epoch: String,
    pub proposal_cid: Cid,
    pub provider: String,
    pub size: u32,
    pub state: u32,
    pub verified: bool
}

/// Strcuture describing a message
#[derive(Clone, Encode, Decode, Default, TypeInfo, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    pub gas_fee_cap: String,
    pub gas_limit: u32,
    pub gas_premium: String,
    pub method: u32,
    pub nonce: u32,
    pub params: String,
    pub to: String,
    pub value: String,
    pub version: u32
}

pub trait Power {
    /// AccountId type for miner
    type AccountId: Parameter + Member + Ord;
    /// Unit of Storage Power of a miner
    type StoragePower: Parameter + Member + Clone;
    /// Libp2p PeerId
    type PeerId: Parameter + Member + AsRef<[u8]> + Clone + Send + 'static;

    /// Register a miner - used by miner
    fn create_miner(
        miner: Self::AccountId,
        owner: Self::AccountId,
        worker: Address,
        window_post_proof_type: RegisteredPoStProof,
        peer: Self::PeerId,
        multiaddrs: Vec<Vec<u8>>
    ) -> Option<Claim<Self::StoragePower>>;

    /// Updates the claimed power for a miner, requested by miners
    /// Example: Worker recovers faulty sector and adds power back
    fn update_claimed_power(
        miner: Self::AccountId,
        raw_bytes_delta: Self::StoragePower,
        quality_adjusted_delta: Self::StoragePower
    ) -> Option<Claim<Self::StoragePower>>;

}

/// Struct that stores the claimed storage from a miner, used when submitting PoRep to ensure miner has claims
/// Claims are updated by miners as they update their storage and deals to update their Storage Power
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo)]
pub struct Claim<StoragePower> {
    /// Raw Bytes Stored by the miner
    raw_bytes_power: StoragePower,
    /// Quality Adjusted Power
    /// This is the raw bytes * Sector Quality Multiplier (when committing storage)
    /// It is equal to raw_bytes_power for now
    quality_adjusted_power: StoragePower,
}


#[derive(Encode, Decode, Default)]
pub struct MinerId(pub u32);

// Code from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
/// This type can be converted into and possibly from an AccountId (which itself is generic).
pub trait AccountIdConversion<AccountId>: Sized {
    /// Convert into an account ID. This is infallible.
    fn into_account(&self) -> AccountId;

    /// Try to convert an account ID into this type. Might not succeed.
    fn try_from_account(a: &AccountId) -> Option<Self>;
}

// Code from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
// This will be moved to own crate and can remove
struct TrailingZeroInput<'a>(&'a [u8]);
impl<'a> codec::Input for TrailingZeroInput<'a> {
    fn remaining_len(&mut self) -> Result<Option<usize>, codec::Error> {
        Ok(None)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<(), codec::Error> {
        let len = into.len().min(self.0.len());
        into[..len].copy_from_slice(&self.0[..len]);
        for i in &mut into[len..] {
            *i = 0;
        }
        self.0 = &self.0[len..];
        Ok(())
    }
}

// Code modified from https://github.com/paritytech/polkadot/blob/rococo-v1/parachain/src/primitives.rs
/// Format is b"miner" ++ encode(minerId) ++ 00.... where 00... is indefinite trailing
/// zeroes to fill AccountId.
impl<T: Encode + Decode> AccountIdConversion<T> for MinerId {
    fn into_account(&self) -> T {
        (b"miner", self)
            .using_encoded(|b| T::decode(&mut TrailingZeroInput(b)))
            .unwrap()
    }

    fn try_from_account(x: &T) -> Option<Self> {
        x.using_encoded(|d| {
            if &d[0..5] != b"miner" {
                return None;
            }
            let mut cursor = &d[5..];
            let result = Decode::decode(&mut cursor).ok()?;
            if cursor.iter().all(|x| *x == 0) {
                Some(result)
            } else {
                None
            }
        })
    }
}
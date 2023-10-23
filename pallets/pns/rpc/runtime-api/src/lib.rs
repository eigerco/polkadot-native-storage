#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;

use frame_support::dispatch::Vec;
use pallet_pns_common::{Cid, DealInfo, Message};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs` of the node)
sp_api::decl_runtime_apis! {
    pub trait PnsApi<AccountId> where
        AccountId: codec::Codec,
    {
        /// List deals for a client
        fn client_list_deals(client_id: AccountId) -> Vec<DealInfo>;

        /// Get deal status for a client
        fn client_get_deal_status(client_id: AccountId, deal: u32) -> String;

        /// Get block messages
        fn chain_get_block_messages(cid: Cid) -> Vec<u8>;

        /// Get full block
        fn chain_get_block(cid: Cid) -> Vec<u8>;

        /// Get message
        fn chain_get_message(cid: Cid) -> Message;
    }
}

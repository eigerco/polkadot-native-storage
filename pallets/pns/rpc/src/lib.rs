use std::sync::Arc;

use codec::Codec;
use frame_support::dispatch::Vec;
use jsonrpsee::{
    core::{Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
use pallet_pns_common::{Cid, DealInfo, Message};
pub use pallet_pns_runtime_api::PnsApi as PnsRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;

/// Public RPC API of the PNS pallet.
#[rpc(server)]
pub trait PnsApi<BlockHash, AccountId> {
    /// List deals for a client
    #[method(name = "pns_clientListDeals")]
    fn client_list_deals(
        &self,
        client_id: AccountId,
        at: Option<BlockHash>,
    ) -> RpcResult<Vec<DealInfo>>;

    /// Get deal status for a client
    #[method(name = "pns_clientGetDealStatus")]
    fn client_get_deal_status(
        &self,
        client_id: AccountId,
        deal: u32,
        at: Option<BlockHash>,
    ) -> RpcResult<String>;

    /// Get block messages
    #[method(name = "pns_chainGetBlockMessages")]
    fn chain_get_block_messages(&self, cid: Cid, at: Option<BlockHash>) -> RpcResult<Vec<u8>>;

    /// Get full block
    #[method(name = "pns_chainGetBlock")]
    fn chain_get_block(&self, cid: Cid, at: Option<BlockHash>) -> RpcResult<Vec<u8>>;

    /// Get message
    #[method(name = "pns_chainGetMessage")]
    fn chain_get_message(&self, cid: Cid, at: Option<BlockHash>) -> RpcResult<Message>;
}

/// A struct that implements the `PnsApi`.
pub struct PnsPallet<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> PnsPallet<C, Block> {
    /// Create new `PnsPallet` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId> PnsApiServer<<Block as BlockT>::Hash, AccountId> for PnsPallet<C, Block>
where
    Block: BlockT,
    AccountId: Clone + std::fmt::Display + Codec,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PnsRuntimeApi<Block, AccountId>,
{
    /// List deals for a client
    fn client_list_deals(
        &self,
        client_id: AccountId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Vec<DealInfo>> {
        let api = self.client.runtime_api();
        let res = api.client_list_deals(
            at.unwrap_or_else(|| self.client.info().best_hash),
            client_id,
        );

        res.map_err(runtime_error_into_rpc_err)
    }

    /// Get deal status for a client
    fn client_get_deal_status(
        &self,
        client_id: AccountId,
        deal: u32,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<String> {
        let api = self.client.runtime_api();
        let res = api.client_get_deal_status(
            at.unwrap_or_else(|| self.client.info().best_hash),
            client_id,
            deal,
        );

        res.map_err(runtime_error_into_rpc_err)
    }

    /// Get block messages
    fn chain_get_block_messages(
        &self,
        cid: Cid,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Vec<u8>> {
        let api = self.client.runtime_api();
        let res =
            api.chain_get_block_messages(at.unwrap_or_else(|| self.client.info().best_hash), cid);

        res.map_err(runtime_error_into_rpc_err)
    }

    /// Get full block
    fn chain_get_block(&self, cid: Cid, at: Option<<Block as BlockT>::Hash>) -> RpcResult<Vec<u8>> {
        let api = self.client.runtime_api();
        let res = api.chain_get_block(at.unwrap_or_else(|| self.client.info().best_hash), cid);

        res.map_err(runtime_error_into_rpc_err)
    }

    /// Get message
    fn chain_get_message(
        &self,
        cid: Cid,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Message> {
        let api = self.client.runtime_api();
        let res = api.chain_get_message(at.unwrap_or_else(|| self.client.info().best_hash), cid);

        res.map_err(runtime_error_into_rpc_err)
    }
}

const RUNTIME_ERROR: i32 = 1;

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> JsonRpseeError {
    CallError::Custom(ErrorObject::owned(
        RUNTIME_ERROR,
        "Runtime error",
        Some(format!("{:?}", err)),
    ))
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_error_into_rpc_err_test_str() {
        let err_str = "test";
        let err_str_tst = "\"\\\"test\\\"\"";
        let res = runtime_error_into_rpc_err(err_str);
        match res {
            JsonRpseeError::Call(err) => match err {
                CallError::Custom(err) => {
                    assert_eq!(err.code(), RUNTIME_ERROR);
                    assert_eq!(err.message(), "Runtime error");
                    assert_eq!(err.data().unwrap().get(), err_str_tst)
                }
                _ => panic!("Wrong error type"),
            },
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn runtime_error_into_rpc_err_empty_str() {
        let err_str = "";
        let err_str_tst = "\"\\\"\\\"\"";
        let res = runtime_error_into_rpc_err(err_str);
        match res {
            JsonRpseeError::Call(err) => match err {
                CallError::Custom(err) => {
                    assert_eq!(err.code(), RUNTIME_ERROR);
                    assert_eq!(err.message(), "Runtime error");
                    assert_eq!(err.data().unwrap().get(), err_str_tst)
                }
                _ => panic!("Wrong error type"),
            },
            _ => panic!("Wrong error type"),
        }
    }
}

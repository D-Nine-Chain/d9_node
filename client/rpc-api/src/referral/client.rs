// use super::ReferralRpcServer;
use crate::referral::ReferralApi;
use codec::{ Codec, MaxEncodedLen };
use jsonrpsee::{ core::RpcResult, types::{ error::CallError, ErrorObject } };
use runtime_api::referral::ReferralRuntimeAPI;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_std::fmt::Debug;
use std::sync::Arc;
pub struct Referral<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Referral<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

// ReferralRpcServer is define by the macro #rpc[(client, server, namespace = "referral")]
impl<C, Block, AccountId> ReferralApiServer<AccountId>
	for Referral<C, Block>
	where
		Block: BlockT,
		C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
		C::Api: ReferralRuntimeAPI<Block, AccountId>,
		AccountId: Codec + MaxEncodedLen + Eq + Debug + Clone
{
	fn get_parent(&self, account: AccountId) -> RpcResult<Option<AccountId>> {
		let api = self.client.runtime_api();
		let at_block = self.client.info().best_hash as <Block as BlockT>::Hash;
		let maybe_parent = api
			.get_parent(at_block, account)
			.map_err(|e| {
				CallError::Custom(
					ErrorObject::owned(0, "Unable to get parent", Some(format!("{e:?}")))
				)
			})?;
		Ok(maybe_parent)
	}

	fn get_ancestors(&self, account: AccountId) -> RpcResult<Option<Vec<AccountId>>> {
		let api = self.client.runtime_api();
		let at_block = self.client.info().best_hash as <Block as BlockT>::Hash;
		let maybe_ancestor = api
			.get_ancestors(at_block, account)
			.map_err(|e| {
				CallError::Custom(
					ErrorObject::owned(0, "Unable to get ancestors", Some(format!("{e:?}")))
				)
			})?;
		Ok(maybe_ancestor)
	}
}

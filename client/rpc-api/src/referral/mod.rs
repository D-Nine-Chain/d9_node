use std::sync::Arc;

use codec::{ Codec, MaxEncodedLen };

use jsonrpsee::{ core::RpcResult, proc_macros::rpc, types::error::{ CallError, ErrorObject } };
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_std::fmt::Debug;
pub use runtime_api::ReferralRuntimeApi;

#[rpc(client, server, namespace = "referral")]
pub trait ReferralApi<BlockHash, AccountId> {
	#[method(name = "getParent")]
	fn get_parent(&self, account: AccountId, at: Option<BlockHash>) -> RpcResult<Option<AccountId>>;

	#[method(name = "getAncestors")]
	fn get_ancestors(
		&self,
		account: AccountId,
		at: Option<BlockHash>
	) -> RpcResult<Option<Vec<AccountId>>>;
}

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

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 2,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block, AccountId> ReferralApiServer<<Block as BlockT>::Hash, AccountId>
	for Referral<C, Block>
	where
		Block: BlockT,
		C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
		C::Api: ReferralRuntimeApi<Block, AccountId>,
		AccountId: Codec + MaxEncodedLen + Eq + Debug + Clone
{
	fn get_parent(
		&self,
		account: AccountId,
		at: Option<<Block as BlockT>::Hash>
	) -> RpcResult<Option<AccountId>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		let maybe_parent = api
			.get_parent(at, account)
			.map_err(|e| map_err(e, "Unable to query dispatch info."))?;

		Ok(maybe_parent)
	}

	fn get_ancestors(
		&self,
		account: AccountId,
		at: Option<<Block as BlockT>::Hash>
	) -> RpcResult<Option<Vec<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		let maybe_ancestors = api
			.get_ancestors(at, account)
			.map_err(|e| map_err(e, "Unable to get ancestors"))?;
		Ok(maybe_ancestors)
	}
}

fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
}
// pub fn owned<S: Serialize>(
//     code: i32,
//     message: impl Into<String>,
//     data: Option<S>
// ) -> ErrorObject<'static>

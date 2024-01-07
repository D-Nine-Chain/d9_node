#![cfg_attr(not(feature = "std"), no_std)]
use std::sync::Arc;

use codec::{ Codec, MaxEncodedLen };

use jsonrpsee::{ core::RpcResult, proc_macros::rpc, types::error::{ CallError, ErrorObject } };
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_std::fmt::Debug;
pub use runtime_api::referral::ReferralApi as ReferralRuntimeApi;

#[rpc(client, server)]
pub trait ReferralApi<BlockHash> {
	#[method(name = "referral_getParent")]
	fn get_parent(&self) -> RpcResult<u8>;

	#[method(name = "referral_getParent")]
	fn get_direct_referral_count(&self, account: AccountId) -> RpcResult<u32>;

	// #[method(name = "referral_getAncestors")]
	// fn get_ancestors(
	// 	&self,
	// 	account: AccountId,
	// 	at: Option<BlockHash>
	// ) -> RpcResult<Option<Vec<AccountId>>>;
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
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

impl<C, Block> ReferralApiServer<<Block as BlockT>::Hash>
	for Referral<C, Block>
	where
		Block: BlockT,
		C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
		C::Api: ReferralRuntimeApi<Block>
{
	fn get_parent(&self) -> RpcResult<u8> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;
		let maybe_parent = api.get_parent(at).map_err(|e| map_err(e, "Unable to get "))?;

		Ok(maybe_parent)
	}

	fn get_direct_referral_count(&self, account: AccountId) -> RpcResult<u32> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;
		api.get_direct_referral_count(at, account)
	}
	// fn get_ancestors(
	// 	&self,
	// 	account: AccountId,
	// 	at: Option<Block::Hash>
	// ) -> RpcResult<Option<Vec<AccountId>>> {
	// 	let api = self.client.runtime_api();
	// 	let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

	// 	let maybe_ancestors = api
	// 		.get_ancestors(at_hash, account)
	// 		.map_err(|e| map_err(e, "Unable to get ancestors"))?;
	// 	Ok(maybe_ancestors)
	// }
}

fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
}

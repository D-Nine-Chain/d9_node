use std::sync::Arc;

use codec::{ Codec, MaxEncodedLen };

use jsonrpsee::{ core::RpcResult, proc_macros::rpc, types::error::{ CallError, ErrorObject } };
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use sp_std::fmt::Debug;
pub use runtime_api::NodeVotingRuntimeApi;

#[rpc(client, server, namespace = "voting")]
pub trait VotingApi<BlockHash, AccountId> {
	#[method(name = "getSortedCandidates")]
	fn get_sorted_candidates(&self, at: Option<BlockHash>) -> RpcResult<Vec<(AccountId, u64)>>;
}

pub struct Voting<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Voting<C, P> {
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

impl<C, Block, AccountId> VotingApiServer<<Block as BlockT>::Hash, AccountId>
	for Voting<C, Block>
	where
		Block: BlockT,
		C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
		C::Api: NodeVotingRuntimeApi<Block, AccountId>,
		AccountId: Codec + MaxEncodedLen + Eq + Debug + Clone
{
	fn get_sorted_candidates(
		&self,
		at: Option<<Block as BlockT>::Hash>
	) -> RpcResult<Vec<(AccountId, u64)>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		let maybe_candidates = api
			.get_sorted_candidates_with_votes(at)
			.map_err(|e| map_err(e, "Unable to query dispatch info."))?;

		Ok(maybe_candidates)
	}
}

fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
}

//! Node-specific RPC methods for interaction with contracts.

use std::sync::Arc;

use codec::Codec;
// use jsonrpc_core::{Error, ErrorCode, Result};
// use jsonrpc_derive::rpc;
use pallet_contracts_primitives::RentProjection;
use jsonrpsee::{ core::RpcResult, proc_macros::rpc, types::error::{ CallError, ErrorObject } };
// use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{ Bytes, H256 };
// use sp_rpc::number;
use sp_runtime::{ generic::BlockId, traits::{ Block as BlockT, Header as HeaderT }, DispatchError };
use std::convert::{ TryFrom, TryInto };
use pallet_contracts_primitives::ContractExecResult;
pub use pallet_contracts::ContractsApi as ContractsRuntimeApi;
const RUNTIME_ERROR: i64 = 1;
const CONTRACT_DOESNT_EXIST: i64 = 2;
const CONTRACT_IS_A_TOMBSTONE: i64 = 3;

/// A rough estimate of how much gas a decent hardware consumes per second,
/// using native execution.
/// This value is used to set the upper bound for maximal contract calls to
/// prevent blocking the RPC for too long.
///
/// As 1 gas is equal to 1 weight we base this on the conducted benchmarks which
/// determined runtime weights:
/// https://github.com/paritytech/substrate/pull/5446
const GAS_PER_SECOND: u64 = 1_000_000_000_000;

pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	InputError,
	GasLimitDoesntFit,
	GasLimitTooHigh,
	/// The call to runtime failed.
	RuntimeError,
	ContractError,
}
/// A private newtype for converting `ContractAccessError` into an RPC error.
struct ContractAccessError(pallet_contracts_primitives::ContractAccessError);
impl From<ContractAccessError> for ErrorObject {
	fn from(e: ContractAccessError) -> ErrorObject {
		use pallet_contracts_primitives::ContractAccessError::*;
		match e.0 {
			DoesntExist =>
				ErrorObject::owned(
					Error::RuntimeError.into(),
					"The specified contract doesn't exist.".into(),
					Some("ContractDoesntExist".into())
				),
			IsTombstone =>
				ErrorObject::owned(
					Error::RuntimeError.into(),
					"ContractIsATombstone".into(),
					Some("ContractIsATombstone".into())
				),
		}
	}
}

/// A struct that encodes RPC parameters required for a call to a smart-contract.
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
pub struct CallRequest<AccountId> {
	origin: AccountId,
	dest: AccountId,
	value: number::NumberOrHex,
	gas_limit: number::NumberOrHex,
	input_data: Bytes,
}

// #[derive(Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// #[serde(rename_all = "camelCase")]
struct RpcContractExecSuccess {
	/// The return flags. See `pallet_contracts_primitives::ReturnFlags`.
	flags: u32,
	/// Data as returned by the contract.
	data: Bytes,
}

/// An RPC serializable result of contract execution
// #[derive(Serialize, Deserialize)]
// #[serde(deny_unknown_fields)]
// #[serde(rename_all = "camelCase")]
pub struct RpcContractExecResult {
	/// How much gas was consumed by the call. In case of an error this is the amount
	/// that was used up until the error occurred.
	gas_consumed: u64,
	/// Additional dynamic human readable error information for debugging. An empty string
	/// indicates that no additional information is available.
	debug_message: String,
	/// Indicates whether the contract execution was successful or not.
	result: std::result::Result<RpcContractExecSuccess, DispatchError>,
}

impl From<ContractExecResult> for RpcContractExecResult {
	fn from(r: ContractExecResult) -> Self {
		match r.exec_result {
			Ok(val) =>
				RpcContractExecResult {
					gas_consumed: r.gas_consumed,
					debug_message: String::new(),
					result: Ok(RpcContractExecSuccess {
						flags: val.flags.bits(),
						data: val.data.into(),
					}),
				},
			Err(err) =>
				RpcContractExecResult {
					gas_consumed: r.gas_consumed,
					debug_message: String::new(),
					result: Err(err.error),
				},
		}
	}
}

/// Contracts RPC methods.
#[rpc(client, server, namespace = "contracts")]
pub trait ContractsApi<BlockHash, BlockNumber, AccountId, Balance> {
	/// Executes a call to a contract.
	///
	/// This call is performed locally without submitting any transactions. Thus executing this
	/// won't change any state. Nonetheless, the calling state-changing contracts is still possible.
	///
	/// This method is useful for calling getter-like methods on contracts.
	#[method(name = "call")]
	fn call(
		&self,
		call_request: CallRequest<AccountId>,
		at: Option<BlockHash>
	) -> RpcResult<RpcContractExecResult>;

	/// Returns the value under a specified storage `key` in a contract given by `address` param,
	/// or `None` if it is not set.
	#[method(name = "getStorage")]
	fn get_storage(
		&self,
		address: AccountId,
		key: H256,
		at: Option<BlockHash>
	) -> RpcResult<Option<Bytes>>;

	/// Returns the projected time a given contract will be able to sustain paying its rent.
	///
	/// The returned projection is relevant for the given block, i.e. it is as if the contract was
	/// accessed at the beginning of that block.
	///
	/// Returns `None` if the contract is exempted from rent.
	#[method(name = "rentProjection")]
	fn rent_projection(
		&self,
		address: AccountId,
		at: Option<BlockHash>
	) -> RpcResult<Option<BlockNumber>>;
}

/// An implementation of contract specific RPC methods.
pub struct Contracts<C, B> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<B>,
}

impl<C, B> Contracts<C, B> {
	/// Create new `Contracts` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Contracts {
			client,
			_marker: Default::default(),
		}
	}
}
impl<C, Block, AccountId, Balance> ContractsApi<
	<Block as BlockT>::Hash,
	<<Block as BlockT>::Header as HeaderT>::Number,
	AccountId,
	Balance
>
	for Contracts<C, Block>
	where
		Block: BlockT,
		C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
		C::Api: ContractsRuntimeApi<
			Block,
			AccountId,
			Balance,
			<<Block as BlockT>::Header as HeaderT>::Number
		>,
		AccountId: Codec,
		Balance: Codec + TryFrom<number::NumberOrHex>
{
	fn call(
		&self,
		call_request: CallRequest<AccountId>,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<RpcContractExecResult> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(
			at.unwrap_or_else(
				||
					// If the block hash is not supplied assume the best block.
					self.client.info().best_hash
			)
		);

		let CallRequest { origin, dest, value, gas_limit, input_data } = call_request;

		// Make sure that value fits into the balance type.
		let value: Balance = value
			.try_into()
			.map_err(|_|
				CallError::Custom(
					ErrorObject::owned(
						Error::InputError.into(),
						"Value doesn't fit into the balance type".into(),
						Some("ValueDoesntFit".into())
					)
				)
			)?;

		let gas_limit: u64 = value
			.try_into()
			.map_err(|_|
				CallError::Custom(
					ErrorObject::owned(
						Error::GasLimitDoesntFit.into(),
						format!("{:?} doesn't fit in 64 bit unsigned value", gas_limit).into(),
						Some("GasLimitDoesntFit".into())
					)
				)
			)?;

		// Make sure that gas_limit fits into 64 bits.
		// let gas_limit: u64 = gas_limit.try_into().map_err(|_| Error {
		// 	code: ErrorCode::InvalidParams,
		// 	message: format!("{:?} doesn't fit in 64 bit unsigned value", gas_limit),
		// 	data: None,
		// })?;

		let max_gas_limit = 5 * GAS_PER_SECOND;
		if gas_limit > max_gas_limit {
			// return Err(Error {
			// 	code: ErrorCode::InvalidParams,
			// 	message: format!(
			// 		"Requested gas limit is greater than maximum allowed: {} > {}",
			// 		gas_limit,
			// 		max_gas_limit
			// 	),
			// 	data: None,
			// });
			return Err(
				CallError::Custom(
					ErrorObject::owned(
						Error::GasLimitTooHigh.into(),
						format!(
							"Requested gas limit is greater than maximum allowed: {} > {}",
							gas_limit,
							max_gas_limit
						).into(),
						Some("GasLimitTooHigh".into())
					)
				)
			);
		}

		let exec_result = api
			.call(&at, origin, dest, value, gas_limit, input_data.to_vec())
			.map_err(|e| map_err(e, "ContractExecutionError"))?;

		Ok(exec_result.into())
	}

	fn get_storage(
		&self,
		address: AccountId,
		key: H256,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<Option<Bytes>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(
			at.unwrap_or_else(
				||
					// If the block hash is not supplied assume the best block.
					self.client.info().best_hash
			)
		);

		let result = api
			.get_storage(&at, address, key.into())
			.map_err(|e| map_err(e, "StorageError"))
			.map(Bytes);

		Ok(result)
	}

	fn instantiate(
		origin: AccountId,
		value: Balance,
		gas_limit: Option<Weight>,
		storage_deposit_limit: Option<Balance>,
		code: pallet_contracts_primitives::Code<Hash>,
		data: Vec<u8>,
		salt: Vec<u8>
	) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance> {
		let api = self.client.runtime_api();
		let gas_limit = gas_limit.unwrap_or(BlockWeights::get().max_block);
		api.bare_instantiate(
			origin,
			value,
			gas_limit,
			storage_deposit_limit,
			code,
			data,
			salt,
			true
		).map_err(|e| map_err(e, "InstatiationError"))
	}

	fn upload_code(
		origin: AccountId,
		code: Vec<u8>,
		storage_deposit_limit: Option<Balance>,
		determinism: pallet_contracts::Determinism
	) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance> {
		let api = self.client.runtime_api();
		api.bare_upload_code(origin, code, storage_deposit_limit, determinism).map_err(|e|
			map_err(e, "UploadError")
		)
	}
}

/// Converts a runtime trap into an RPC error.
fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::U256;

	#[test]
	fn call_request_should_serialize_deserialize_properly() {
		type Req = CallRequest<String>;
		let req: Req = serde_json
			::from_str(
				r#"
		{
			"origin": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
			"dest": "5DRakbLVnjVrW6niwLfHGW24EeCEvDAFGEXrtaYS5M4ynoom",
			"value": "0x112210f4B16c1cb1",
			"gasLimit": 1000000000000,
			"inputData": "0x8c97db39"
		}
		"#
			)
			.unwrap();
		assert_eq!(req.gas_limit.into_u256(), U256::from(0xe8d4a51000u64));
		assert_eq!(req.value.into_u256(), U256::from(1234567890987654321u128));
	}

	#[test]
	fn result_should_serialize_deserialize_properly() {
		fn test(expected: &str) {
			let res: RpcContractExecResult = serde_json::from_str(expected).unwrap();
			let actual = serde_json::to_string(&res).unwrap();
			assert_eq!(actual, expected);
		}
		test(
			r#"{"gasConsumed":5000,"debugMessage":"helpOk","result":{"Ok":{"flags":5,"data":"0x1234"}}}"#
		);
		test(r#"{"gasConsumed":3400,"debugMessage":"helpErr","result":{"Err":"BadOrigin"}}"#);
	}
}

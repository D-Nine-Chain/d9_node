#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait ReferralRuntimeApi<AccountId> where AccountId: Codec {
		fn get_parent(account: AccountId) -> Option<AccountId>;
		fn get_ancestors(account: AccountId) -> Option<Vec<AccountId>>;
	}
   
   pub trait SimplifiedContractApi< AccountId, Balance, BlockNumber, Hash> where AccountId: Codec, Balance: Codec, BlockNumber: Codec, Hash: Codec {
      fn call(
         origin: AccountId,
         dest: AccountId,
         value: Balance,
         gas_limit: u64,
         input_data: Vec<u8>,
         at: Option<BlockNumber>
      ) -> Result<(Vec<u8>, u64), Hash>;
   }
}

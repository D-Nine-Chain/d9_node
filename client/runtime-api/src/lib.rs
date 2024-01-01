#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait ReferralRuntimeApi<AccountId> where AccountId: Codec {
		fn get_parent(account: AccountId) -> Option<AccountId>;
		fn get_ancestors(account: AccountId) -> Option<Vec<AccountId>>;
	}

	pub trait NodeVotingRuntimeApi<AccountId> where AccountId: Codec {
		fn get_sorted_candidates_with_votes() -> Vec<(AccountId, u64)>;
	}
}

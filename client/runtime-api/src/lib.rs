#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
sp_api::decl_runtime_apis! {
	pub trait ReferralRuntimeApi<AccountId> where AccountId: Codec {
		fn get_parent(account: AccountId) -> Option<AccountId>;
		fn get_ancestors(account: AccountId) -> Option<Vec<AccountId>>;
		fn get_direct_referral_count(account: AccountId) -> u32;
	}

	pub trait NodeVotingRuntimeApi<AccountId> where AccountId: Codec {
		fn get_sorted_candidates() -> Vec<(AccountId, u64)>;
	}
}

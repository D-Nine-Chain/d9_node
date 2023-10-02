use ink::{ env::Environment, prelude::vec::Vec };
use codec::{ Encode, Decode };
// use pallet_contracts::chain_extension::{ ChainExtension };
// use sp_runtime::DispatchError;
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
	const MAX_EVENT_TOPICS: usize = <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

	type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
	type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
	type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
	type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
	type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;
	type ChainExtension = D9ChainExtension;
}

pub struct D9ChainContractsExtension;
#[ink::chain_extension]
pub trait D9ChainExtension {
	type ErrorCode = ErrorCode;
	/// Retrieves the parent referrer account for a given referree account.
	///
	/// This function allows for querying the parent (or direct referrer) of a
	/// specified referree account in a referral system. If the referree has no
	/// parent referrer, the function returns `None`.
	///
	/// # Parameters
	///
	/// - `referree`: The `AccountId` of the referree whose parent referrer is to be retrieved.
	///
	/// # Returns
	///
	/// - `Option<AccountId>`: An `Option` containing the `AccountId` of the parent referrer.
	///   Returns `None` if the referree has no parent referrer.
	///
	/// # Example
	///
	/// ```rust
	/// let parent = my_contract.get_referree_parent(referree_account);
	/// ```
	#[ink(extension = 0)]
	fn get_referree_parent(
		referree: <CustomEnvironment as Environment>::AccountId
	) -> Option<<CustomEnvironment as Environment>::AccountId>;

	/// Retrieves the list of ancestor accounts for a given referree account up to a maximum depth.
	///
	/// This function provides a way to fetch all the ancestor referrers (i.e., the referral chain)
	/// for a specific referree account. The returned list starts with the direct parent referrer
	/// and goes upwards, tracing the referral lineage.
	///
	/// The maximum depth of the returned vector is determined by the `MaxReferralDepth` set in the
	/// runtime. This limit is initially set during the deployment of the runtime and can only be
	/// modified by a call originating from the `SetMaxReferralDepthOrigin`.
	///
	/// # Parameters
	///
	/// - `referree`: The `AccountId` of the referree whose ancestor referrers are to be retrieved.
	///
	/// # Returns
	///
	/// - `Result<Vec<AccountId>>`: A `Result` containing a vector of `AccountId`s representing the
	///   ancestor referrers in the order from the closest parent to the furthest. If an error occurs
	///   during retrieval, an appropriate error is returned.
	///
	/// # Example
	///
	/// ```rust
	/// let ancestors = my_contract.get_ancestors(referree_account).unwrap_or_default();
	/// ```
	#[ink(extension = 1)]
	fn get_ancestors(
		referree: <CustomEnvironment as Environment>::AccountId
	) -> Result<Vec<<CustomEnvironment as Environment>::AccountId>, ErrorCode>;
}
#[derive(Encode, Decode, scale_info::TypeInfo)]
pub enum ErrorCode {
	/// Indicates that no referral account record was found.
	///
	/// This error is returned when an operation expects a referral account
	/// to exist, but no such record is present in the system. This could occur
	/// due to a missing or incorrect account identifier, or if the referral account
	/// was never registered.
	NoReferralAccountRecord,
}

impl ink_env::chain_extension::FromStatusCode for ErrorCode {
	fn from_status_code(status_code: u32) -> Result<(), Self> {
		match status_code {
			0 => Ok(()),
			1 => Err(Self::NoReferralAccountRecord),
			_ => panic!("encountered unknown status code"),
		}
	}
}

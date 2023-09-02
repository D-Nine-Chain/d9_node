//! # D9 Referral Program Pallet
//!
//! The D9 Referral Program Pallet: A pallet designed to manage the D9 referral account program.
//!
//! Run `cargo doc --package pallet-d9-referral-program --open` to view this module's
//! documentation.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! This pallet is designed to manage the D9 referral account program.
//! In the D9 network, an account that endows another account (i.e., provides it with its first balance)
//! creates a parent-child relationship between the two accounts. This relationship is initially used in the D9
//! network's burn mining apparatus, where an account that receives mining rewards pays a percentage of
//! those rewards to its direct parent and an equal, smaller percentage to its other ancestors, up to a
//! depth of `MaxReferralDepth`.
//!
//! For example, if an account `A` endows an account `B`, `A` becomes the parent of `B`. If `B` then
//! receives mining rewards, it will pay a percentage of those rewards to `A`, and if `A` has a parent,
//! a smaller yet equal percentage to that account, and so on, up to a depth of `MaxReferralDepth`.
//!
//! this pallet is meant to maintain the referral tree, and provide functions to query the tree, dividende calculation or
//! any other functionality is handled in an ink! contract, ro some other pallet..
//!
//! ## Implementation
//!
//! The pallet provides a `ReferralAccount` struct that contains the following fields:
//! - `parent`: an `Option<AccountId>` representing the parent of the account. This is `None` for the
//! root of the tree.
//! - `children`: a `Vec<AccountId>` representing the children of the account.
//! - `alpha_account`: an `AccountId` representing the "alpha" account of the account.
//! - `depth`: a `u32` representing the depth of the account in the referral tree.
//!
//! The pallet also provides a `ReferralAccounts` storage map that maps `AccountId`s to
//! `ReferralAccount`s.
//!
//! ## Usage
//!
//! To use this pallet in your runtime, add it to your runtime's `construct_runtime!` macro and
//! configure it in your runtime's `impl` block.
//!
//! Here is an example of how to use this pallet in your runtime:
//!
//! ```rust
//! use pallet_d9_referral_program::Config;
//!
//! impl pallet_d9_referral_program::Config for Runtime {
//!     type Event = Event;
//! }
//!
//! construct_runtime!(
//!     pub enum Runtime where
//!         Block = Block,
//!         NodeBlock = opaque::Block,
//!         UncheckedExtrinsic = UncheckedExtrinsic
//!     {
//!         // ... other pallets
//!         D9ReferralProgram: pallet_d9_referral_program::{Pallet, Call, Storage, Event<T>},
//!     }
//! );
//! ```
//!
//! ## API
//!
//! The pallet provides the following callable functions:
//!
//! - `create_referral_account(origin, parent)`: Creates a referral account with the specified parent.
//! - `add_child(origin, child)`: Adds a child to the caller's referral account.
//! - ... other functions
#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod tests;
// pub mod weights;

// use pallet_treasury::Config as TreasuryConfig;
// pub use weights::WeightInfo;
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::{ *, OptionQuery },
		traits::{ EnsureOrigin, OnUnbalanced, Currency, Imbalance },
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{ traits::AtLeast32BitUnsigned, FixedPointOperand };
	use sp_std::fmt::Debug;
	use codec::Codec;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		type RuntimeEvent: From<Event<Self, I>> +
			IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The maximum depth of the referral tree.
		///
		/// This variable determines how deep the referral tree can be,
		/// meaning the maximum number of levels of referrals that are allowed.
		/// For example, if the maximum depth is 19, then there can be up to 19 levels
		/// of referrals, starting from the initial referral as level 0.
		#[pallet::constant]
		#[pallet::no_default]
		type MaxReferralDepth: Get<u32>;
	}

	/// The current storage version.
	const STORAGE_VERSION: frame_support::traits::StorageVersion = frame_support::traits::StorageVersion::new(
		1
	);

	pub struct ReferralAccount<AccountId> {
		/// if parent is None consider this the root of a referral tree.
		/// the account endowed (the account that receives its first balance)
		///  is the root of a new referral treee
		/// if the endower (sender) is at the MaxReferralDepth, of its own referral tree.
		parent: Option<AccountId>,
		/// all the accounts endowed by this account
		children: Vec<AccountId>,
		/// this account's depth in its referral tree
		depth: u32,
	}

	/// A mapping from an account ID to its corresponding referral account information.
	#[pallet::storage]
	#[pallet::getter(fn referral_account)]
	pub type ReferralAccounts<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		ReferralAccount<AccountId>,
		ValueQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		NewTreasurer(T::AccountId),
	}

	/// Error for the treasury pallet.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// there is no records for this account
		NoReferralAccountRecord,
		ParentAndChildMustDiffer,
		ChildReferralAccountAlreadyExists,
	}
}

#[pallet::hooks]
impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
	fn offchain_worker(block_number: BlockNumberFor<T>) {}
}

#[pallet::call]
impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Create a referral relationship between two accounts.
	///
	/// This function is used to establish a referral relationship where one account
	/// refers another account. The account being referred is identified as the `child`,
	/// and the account doing the referring is identified as the `parent`.
	///
	/// # Parameters
	/// - `origin`: The origin of the call. This function only accepts calls from the root origin.
	/// - `parent`: The account ID of the parent account.
	/// - `child`: The account ID of the child account.
	///
	/// # Requirements
	/// - The `origin` must be the root origin.
	/// - The `parent` and `child` accounts must be different.
	/// - The `child` account must not already have a referral relationship (i.e., it must not already
	///   be a child or a parent in the referral tree).
	///
	/// # Storage Changes
	/// - The `ReferralAccounts` storage map will be updated to include the `parent` and `child` accounts
	///   and their associated `ReferralAccount` structures.
	/// - If the `parent` account is not already in the `ReferralAccounts` storage map, it will be added
	///   with a `parent` field of `None`, a `children` field containing only the `child` account, and a
	///   `depth` field of 0.
	/// - The `child` account will be added with a `parent` field of `Some(parent)`, an empty `children`
	///   field, and a `depth` field that is one greater than the `depth` of the `parent` account.
	///
	/// # Errors
	/// - Will return `ParentAndChildMustDiffer` if the `parent` and `child` accounts are the same.
	/// - Will return `ChildReferralAccountAlreadyExists` if the `child` account is already in the
	///   `ReferralAccounts` storage map.
	///
	/// # Weight
	/// The weight of this function is equivalent to 2 reads and 2 writes from/to the `ReferralAccounts`
	/// storage.
	#[pallet::call_index(0)]
	#[pallet::weight(T::DbWeight::get().reads_writes(2, 2))]
	pub fn create_referral_relationship(
		origin: OriginFor<T>,
		parent: T::AccountId,
		child: T::AccountId
	) -> DispatchResult {
		ensure_root(origin)?;
		ensure!(parent != child, Error < T, I > ::ParentAndChildMustDiffer);
		let child_referral_account = ReferralAccounts::<T, I>::get(child);
		//we have this `ensure!` because the balances pallet clears the storage
		// of accounts that fall below existential deposit
		// so an account that was Endowed before, and is cleared
		// can be endowed again since there is no record
		// of its existence after 'dust' accounts are cleared.
		// we still want to maintain the record so one account
		// cant be the child of two different accounts.
		ensure!(
			child_referral_account.is_none(),
			Error < T,
			I > ::ChildReferralAccountAlreadyExists
		);
		let parent_referral_account = ReferralAccounts::<T, I>::get(parent);
		if parent_referral_account.is_none() {
			ReferralAccounts::<T, I>::insert(parent, ReferralAccount {
				parent: None,
				children: vec![child],
				depth: 0,
			});
			ReferralAccounts::<T, I>::insert(child, ReferralAccount {
				parent: Some(parent),
				children: Vec::new(),
				depth: 1,
			});
		} else {
			let mut parent_referral_account = parent_referral_account.unwrap();
			if parent_referral_account.depth + 1 == MaxReferralDepth::get() {
				ReferralAccounts::<T, I>::insert(child, ReferralAccount {
					parent: None,
					children: Vec::new(),
					depth: 0,
				});
			} else {
				parent_referral_account.children.push(child);
				ReferralAccounts::<T, I>::insert(parent, parent_referral_account);
				ReferralAccounts::<T, I>::insert(child, ReferralAccount {
					parent: Some(parent),
					children: Vec::new(),
					depth: parent_referral_account.depth + 1,
				});
			}
		}
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Returns the parent account of the given account.
	///
	/// This function retrieves the parent account of a given account from the
	/// `ReferralAccounts` storage map. If the given account does not exist in
	/// the `ReferralAccounts` storage map, it will return an error of
	/// `NoReferralAccountRecord`.
	///
	/// # Arguments
	///
	/// * `account` - The account ID whose parent account ID needs to be retrieved.
	///
	/// # Returns
	///
	/// * An `Option<T::AccountId>` containing the parent account ID if it exists,
	///   or an error of `NoReferralAccountRecord` if the given account does not
	///   exist in the `ReferralAccounts` storage map.
	///
	/// # Context
	///
	/// In a referral system, users are connected in a hierarchical structure,
	/// where each user has a parent (referrer) who referred them to the system,
	/// except for the root node which does not have a parent. This function is
	/// essential for navigating this hierarchical structure, as it allows the
	/// system to retrieve the parent of any given account, which is necessary
	/// for distributing rewards, calculating commissions, and other operations
	/// that depend on the referral structure.
	pub fn get_parent(account: T::AccountId) -> Option<T::AccountId> {
		let referral_account = ReferralAccounts::<T, I>::get(account);
		ensure!(referral_account.is_some(), Error < T, I > ::NoReferralAccountRecord);
		referral_account.parent
	}

	/// Returns a vector of ancestor accounts of a given account.
	///
	/// This function retrieves all ancestor accounts (i.e., parent, grandparent, etc.)
	/// of a given account, up to and including the `alpha_account` , which is assumed
	/// to be the root node of a particular referral tree.
	///
	/// # Arguments
	///
	/// * `account` - The account ID whose ancestor accounts need to be retrieved.
	///
	/// # Returns
	///
	/// * An `Option<Vec<T::AccountId>>` containing all ancestor accounts of the
	///   given account, including the `alpha_account`, or an error of
	///   `NoReferralAccountRecord` if the given account does not exist in the
	///   `ReferralAccounts` storage map.
	///
	/// # Context
	///

	pub fn get_ancestors(account: T::AccountId) -> Option<Vec<T::AccountId>> {
		let referral_account = ReferralAccounts::<T, I>::get(account);
		ensure!(referral_account.is_some(), Error < T, I > ::NoReferralAccountRecord);
		let mut ancestors: Vec<T::AccountId> = Vec::new();
		let mut parent = referral_account.parent;
		while parent.is_some() {
			ancestors.push(parent.unwrap());
			parent = Self::get_parent(parent.unwrap());
		}
		ancestors.push(referral_account.alpha_account);
		ancestors
	}

	/// Returns a vector of children accounts of a given account.
	///
	/// This function retrieves all children accounts (i.e., direct referrals) of a
	/// given account from the `ReferralAccounts` storage map.
	///
	/// # Arguments
	///
	/// * `account` - The account ID whose children accounts need to be retrieved.
	///
	/// # Returns
	///
	/// * An `Option<Vec<T::AccountId>>` containing all children accounts of the
	///   given account, or an error of `NoReferralAccountRecord` if the given
	///   account does not exist in the `ReferralAccounts` storage map.
	pub fn get_children(account: T::AccountId) -> Option<Vec<T::AccountId>> {
		let referral_account = ReferralAccounts::<T, I>::get(account);
		ensure!(referral_account.is_some(), Error < T, I > ::NoReferralAccountRecord);
		ReferralAccounts::<T, I>::get(account).children
	}

	/// Returns the `alpha_account` (root node) associated with a given account.
	///
	/// each referral account keeps on record its root node so as to potentially keep any extrinsic function's
	/// weight down.
	///
	/// # Arguments
	///
	/// * `account` - The account ID whose `alpha_account` needs to be retrieved.
	///
	/// # Returns
	///
	/// * An `Option<T::AccountId>` containing the `alpha_account` associated with the
	///   given account, or an error of `NoReferralAccountRecord` if the given account
	///   does not exist in the `ReferralAccounts` storage map.
	pub fn get_alpha_account(account: T::AccountId) -> Option<T::AccountId> {
		let referral_account = ReferralAccounts::<T, I>::get(account);
		ensure!(referral_account.is_some(), Error < T, I > ::NoReferralAccountRecord);
		referral_account.alpha_account
	}

	/// get the referral account record for a given account. returns None if no record exists.
	pub fn get_referral_account(account: T::AccountId) -> Option<ReferralAccount<T::AccountId>> {
		ReferralAccounts::<T, I>::get(account)
	}

	sp_api::decl_runtime_apis! {
		pub trait ReferralApi {
			fn get_parent(account: T::AccountId) -> Option<T::AccountId>;
			fn get_children(account: T::AccountId) -> Option<Vec<T::AccountId>>;
			fn get_alpha_account(account: T::AccountId) -> Option<T::AccountId>;
			fn get_referral_account(account: T::AccountId) -> Option<ReferralAccount<T::AccountId>>;
		}
	}
}

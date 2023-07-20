//doing[epic=staking] creating composite treasury pallet
#![cfg_attr(not(feature = "std"), no_std)]
mod benchmarking;
mod impl_currency;
mod impl_fungible;
pub mod migration;
mod tests;
mod types;
pub mod weights;

use codec::{ Codec, MaxEncodedLen };
use frame_support::{ ensure, pallet_prelude::DispatchResult };
use frame_system;
use sp_std::{ cmp, fmt::Debug, mem, prelude::*, result };
pub use types::{
	AccountData,
	BalanceLock,
	DustCleaner,
	ExtraFlags,
	IdAmount,
	Reasons,
	ReserveData,
};
pub use weights::WeightInfo;
pub use pallet::*;
const LOG_TARGET: &str = "runtime::d9_treasury";
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

#[pallet:pallet]
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{ pallet_prelude::* };
	const STORAGE_VERSION: frame_support::traits::StorageVersion = frame_support::traits::StorageVersion::new(
		1
	);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + pallet_treasury::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//todo[epic=treasury,seq=345] write tests for d9_treasury pallet
	//todo[epic=treasury,seq=345] write events
	//todo[epic=treasury,seq=345] write errors
	#[pallet::pallet]
	#[pallet:storage_version(STORAGE_VERSION)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	//the controller of the treasury account
	#[pallet::storage]
	#[pallet::getter(fn treasury_controller)]
	pub type TreasuryController<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config, I: 'static = ()> {
		NewTreasuryController(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T, I: 'static = ()> {
		OnlyTreasuryControllerCanDoThis,
		//todo[epic=treasury,seq=346] add more Errors if necessary
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::set_treasury_controller())]
		pub fn set_treasury_controller(
			origin: OriginFor<T>,
			controller: T::AccountId
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(
				who == Self::treasury_controller(),
				Error::<T, I>::OnlyTreasuryControllerCanDoThis
			);
			TreasuryController::<T>::put(controller);
			Ok(().into())
		}
	}
	///EnsureOrigin impl for pallet
	///
	/// this is a frame provided trait meant to validate the source of an extrinsic
	impl<
		O: Into<Result<RawOrigin<AccountId, I>, O>> + From<RawOrigin<AccountId, I>>,
		I,
		AccountId: Decode
	> EnsureOrigin<O> for Pallet<T, I> {
		type Success = AccountId;
		fn try_origin(o: O) -> Result<Self::Success, O> {
			let origin = o.into();
			match origin {
				Ok(RawOrigin::Signed(account_id)) => {
					if account_id != Self::treasury_controller() {
						return Err(origin);
					}
					Ok(account_id)
				}
				Ok(RawOrigin::None) => Err(origin),
				_ => Err(origin),
			}
		}
	}

	//todo check to see which hooks to impl
	//todo [epic=staking,seq=345] write tests for d9_treasury pallet
	//todo[epic=staking,seq=345] research what benchmarking does and implement into this
}

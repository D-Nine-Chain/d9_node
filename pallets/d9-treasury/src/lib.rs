#![cfg_attr(not(feature = "std"), no_std)]

// #[cfg(test)]
// mod tests;
pub mod weights;

use pallet_treasury::Config as TreasuryConfig;
pub use weights::WeightInfo;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{ pallet_prelude::{ *, OptionQuery } };
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + TreasuryConfig<I> {
		type RuntimeEvent: From<Event<Self, I>> +
			IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn treasurer)]
	pub(crate) type Treasurer<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		Option<T::AccountId>,
		OptionQuery
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub treasurer: Option<T::AccountId>,
		_marker: PhantomData<I>,
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			if let Some(ref treasurer_account) = self.treasurer {
				Treasurer::<T, I>::put(Some(treasurer_account));
			}
		}
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			// Default values for your genesis config
			Self {
				treasurer: Default::default(),
				_marker: Default::default(),
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		NewTreasurer(T::AccountId),
	}

	/// Error for the treasury pallet.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		OnlyTreasurerCanDoThis,
	}

	// #[derive(Default)]
	// struct SpendContext<Balance> {
	// 	spend_in_context: BTreeMap<Balance, Balance>,
	// }

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		// 	#[pallet::call_index(0)]
		// 	#[pallet::weight(T::WeightInfo::change_treasurer())]
		// 	pub fn new_treasurer(
		// 		origin: OriginFor<T>,
		// 		#[pallet::compact] new_treasurer: T::AccountId
		// 	) -> DispatchResult {
		// 		let current_treasurer = ensure_signed(origin)?;

		// 		ensure!(current_treasurer == Self::treasurer(), Error::<T, I>::OnlyTreasurerCanDoThis);

		// 		Self::deposit_event(Event::NewTreasurer(new_treasurer));
		// 		Ok(())
		// 	}
	}
}
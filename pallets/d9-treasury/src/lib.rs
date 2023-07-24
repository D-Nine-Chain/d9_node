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
	use frame_support::{ pallet_prelude::* };
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
		T::AccountId,
		ValueQuery
	>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub treasurer: T::AccountId,
		_marker: PhantomData<I>,
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {
			Treasurer::<T, I>::put(&self.treasurer);
		}
	}

	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			// Default values for your genesis configt
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
		/// Put forward a suggestion for spending. A deposit proportional to the value
		/// is reserved and slashed if the proposal is rejected. It is returned once the
		/// proposal is awarded.
		///
		/// ## Complexity
		/// - O(1)
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::change_treasurer())]
		pub fn new_treasurer(
			origin: OriginFor<T>,
			#[pallet::compact] new_treasurer: T::AccountId
		) -> DispatchResult {
			let current_treasurer = ensure_signed(origin)?;

			ensure!(current_treasurer == Self::treasurer(), Error::<T, I>::OnlyTreasurerCanDoThis);

			Self::deposit_event(Event::NewTreasurer(new_treasurer));
			Ok(())
		}
	}
}

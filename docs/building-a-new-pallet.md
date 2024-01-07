Creating a composite pallet in Substrate involves creating a new pallet that uses existing pallets as part of its implementation. Here's a basic example of how you could create a new pallet that uses the treasury pallet and adds additional storage:

Firstly, create your new pallet just like any other. Here's an example with a placeholder for the custom data you'd like to store:

```rust
#[frame_support::pallet]
pub mod my_treasury_pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_treasury::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn my_data)]
    pub type MyData<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MyDataChanged(T::Balance),
    }

    // rest of your pallet implementation...
}
```

In this example, the new `my_treasury_pallet` includes a storage item `MyData` of type `T::Balance`, along with an event that's emitted when the data changes.

Next, when constructing your runtime, you include both the original treasury pallet and your new pallet:

```rust
use pallet_treasury as Treasury;
use my_treasury_pallet as MyTreasuryPallet;

impl pallet_treasury::Config for Runtime {
    // configuration for the treasury pallet...
}

impl my_treasury_pallet::Config for Runtime {
    type Event = Event;
    // any additional configuration...
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>},
        MyTreasuryPallet: my_treasury_pallet::{Pallet, Call, Storage, Event<T>},
        // rest of your runtime...
    }
);
```

In your new pallet's implementation, you can call into the treasury pallet as necessary to perform any of its functions. You can also interact with your own additional storage.

Remember that this is a very basic example and your actual implementation would likely be more complex. Be sure to check for any potential security issues and thoroughly test any new code you add to ensure it behaves as expected.

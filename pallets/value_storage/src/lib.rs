#![cfg_attr(not(feature = "std"), no_std)]
// 1. Imports and Dependencies
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // 2. Declaration of the Pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 3. Runtime Configuration Trait
    // All types and constants go here.
    // Use #[pallet::constant] and #[pallet::extra_constants]
    // to pass in values to metadata.
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    // 4. Runtime Storage
    // Use to declare storage items.
    #[pallet::storage]
    #[pallet::getter(fn get_value)]
    pub type Value<T> = StorageValue<Value=u32>;

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T:Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn set_value(
			_origin: OriginFor<T>,
			value: u32
		) -> DispatchResult {
			<Value<T>>::put(value);
			Ok(())
		}

	}

}

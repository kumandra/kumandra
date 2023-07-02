use frame_support::{
    pallet_prelude::*,
    traits::{Currency, Get},
};
use frame_system::pallet_prelude::*;

#[pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type MaxCapacity: Get<u64>;

        #[pallet::constant]
        type InitialReliability: Get<u64>;

        #[pallet::constant]
        type InitialReputation: Get<u64>;

        type Balance: Currency<Self::AccountId>;
    }

    #[derive(Encode, Decode, Default, Clone, PartialEq)]
    pub struct ProviderInfo<T: Config> {
        pub capacity: u64,
        pub reliability: u64,
        pub reputation: u64,
        pub rewards: T::Balance,
        pub commissions: T::Balance,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn storage_providers)]
    pub type StorageProviders<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, ProviderInfo<T>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProviderRegistered(T::AccountId),
        ProviderUpdated(T::AccountId),
        ProviderRemoved(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProviderAlreadyRegistered,
        ProviderNotRegistered,
        CapacityOverflow,
        InvalidReliabilityValue,
        InvalidReputationValue,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn register_provider(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let provider = ensure_signed(origin)?;

            ensure!(
                !<StorageProviders<T>>::contains_key(&provider),
                Error::<T>::ProviderAlreadyRegistered
            );

            let provider_info = ProviderInfo {
                capacity: T::MaxCapacity::get(),
                reliability: T::InitialReliability::get(),
                reputation: T::InitialReputation::get(),
                rewards: T::Balance::zero(),
                commissions: T::Balance::zero(),
            };

            <StorageProviders<T>>::insert(&provider, provider_info);
            Self::deposit_event(Event::ProviderRegistered(provider));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn update_provider(
            origin: OriginFor<T>,
            capacity: u64,
            reliability: u64,
            reputation: u64,
        ) -> DispatchResultWithPostInfo {
            let provider = ensure_signed(origin)?;

            let mut provider_info = Self::storage_providers(&provider)
                .ok_or(Error::<T>::ProviderNotRegistered)?;

            provider_info.capacity = capacity;
            provider_info.reliability = reliability;
            provider_info.reputation = reputation;

            <StorageProviders<T>>::insert(&provider, provider_info.clone());
            Self::deposit_event(Event::ProviderUpdated(provider));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn remove_provider
        origin: OriginFor<T>,
    ) -> DispatchResultWithPostInfo {
        let provider = ensure_signed(origin)?;

        ensure!(
            <StorageProviders<T>>::contains_key(&provider),
            Error::<T>::ProviderNotRegistered
        );

        <StorageProviders<T>>::remove(&provider);
        Self::deposit_event(Event::ProviderRemoved(provider));
        Ok(().into())
    }
}

impl<T: Config> Pallet<T> {
    fn validate_reliability(reliability: u64) -> Result<(), Error<T>> {
        // Perform your reliability validation logic here
        // For example, check if the value is within a valid range
        if reliability > 100 {
            Err(Error::<T>::InvalidReliabilityValue)
        } else {
            Ok(())
        }
    }

    fn validate_reputation(reputation: u64) -> Result<(), Error<T>> {
        // Perform your reputation validation logic here
        // For example, check if the value is within a valid range
        if reputation > 100 {
            Err(Error::<T>::InvalidReputationValue)
        } else {
            Ok(())
        }
    }
}


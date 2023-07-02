use frame_support::{pallet, ensure, decl_event, decl_error, decl_module};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[pallet]
pub mod payment_gateway {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn payments)]
    pub(super) type Payments<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn commissions)]
    pub(super) type Commissions<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u64>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PaymentMade(T::AccountId, u64),
        CommissionDistributed(T::AccountId, u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidPayment,
        InsufficientCommissions,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn make_payment(
            origin: OriginFor<T>,
            payment_id: Vec<u8>,
            amount: u64,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(amount > 0, Error::<T>::InvalidPayment);

            Payments::<T>::insert(&payment_id, &sender);

            Self::deposit_event(Event::PaymentMade(sender.clone(), amount));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn distribute_commission(
            origin: OriginFor<T>,
            provider: T::AccountId,
            amount: u64,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(amount > 0, Error::<T>::InvalidPayment);
            ensure!(Commissions::<T>::contains_key(&provider), Error::<T>::InsufficientCommissions);
            let commission = Commissions::<T>::get(&provider);

            ensure!(commission >= amount, Error::<T>::InsufficientCommissions);

            Commissions::<T>::mutate(&provider, |commission| {
                *commission -= amount;
            });

            Self::deposit_event(Event::CommissionDistributed(provider.clone(), amount));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn get_payment_history(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> Vec<Vec<u8>> {
            let _ = ensure_signed(origin)?;

            Payments::<T>::iter()
                .filter(|(_, sender)| *sender == account)
                .map(|(payment_id, _)| payment_id.clone())
                .collect::<Vec<Vec<u8>>>()
        }

        #[pallet::weight(10_000)]
        pub fn calculate_commission(
            origin: OriginFor<T>,
            provider: T::AccountId,
        ) -> Option<u64> {
            let _ = ensure_signed(origin)?;

            Commissions::<T>::get(&provider)
        }
    }
}

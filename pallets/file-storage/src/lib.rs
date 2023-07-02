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
        type MaxFileSize: Get<u64>;

        type Balance: Currency<Self::AccountId>;
    }

    #[derive(Encode, Decode, Default, Clone, PartialEq)]
    pub struct FileInfo<T: Config> {
        pub name: Vec<u8>,
        pub size: u64,
        pub timestamp: u64,
        pub uploader: T::AccountId,
        pub content_address: Vec<u8>,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn files)]
    pub type Files<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, FileInfo<T>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        FileUploaded(T::AccountId, Vec<u8>),
        FileDownloaded(T::AccountId, Vec<u8>),
        FileDeleted(Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        FileNotFound,
        FileSizeExceeded,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn upload_file(
            origin: OriginFor<T>,
            name: Vec<u8>,
            size: u64,
            content_address: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let uploader = ensure_signed(origin)?;

            ensure!(
                size <= T::MaxFileSize::get(),
                Error::<T>::FileSizeExceeded
            );

            let timestamp = <frame_system::Pallet<T>>::block_number().into();

            let file_info = FileInfo {
                name,
                size,
                timestamp,
                uploader: uploader.clone(),
                content_address: content_address.clone(),
            };

            <Files<T>>::insert(&content_address, file_info);
            Self::deposit_event(Event::FileUploaded(uploader, content_address));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn download_file(
            origin: OriginFor<T>,
            content_address: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let downloader = ensure_signed(origin)?;

            let file_info = Self::files(&content_address)
                .ok_or(Error::<T>::FileNotFound)?;

            Self::deposit_event(Event::FileDownloaded(downloader, content_address));
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn delete_file(
            origin: OriginFor<T>,
            content_address: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(
                <Files<T>>::contains_key(&content_address),
                Error::<T>::FileNotFound
            );

            <Files<T>>::remove(&content_address);
            Self::deposit_event(Event::FileDeleted(content_address));
            Ok(().into())
        }
    }
}

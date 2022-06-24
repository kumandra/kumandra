use crate::Config;
use frame_support::{
    log, storage_alias,
    traits::{Get, PalletInfoAccess, StorageVersion},
    weights::Weight,
};
use sp_std::vec::Vec;

#[storage_alias]
type SessionForValidatorsChange = StorageValue<Kumandra, u32>;

#[storage_alias]
type Validators<T> = StorageValue<Kumandra, Vec<<T as frame_system::Config>::AccountId>>;

pub fn migrate<T: Config, P: PalletInfoAccess>() -> Weight {
    log::info!(target: "pallet_kumandra", "Running migration from STORAGE_VERSION 0 to 1");

    let mut writes = 0;

    match SessionForValidatorsChange::translate(|old: Option<Option<u32>>| -> Option<u32> {
        log::info!(target: "pallet_kumandra", "Current storage value for SessionForValidatorsChange {:?}", old);
        match old {
            Some(Some(x)) => Some(x),
            _ => None,
        }
    }) {
        Ok(_) => {
            writes += 1;
            log::info!(target: "pallet_kumandra", "Succesfully migrated storage for SessionForValidatorsChange");
        }
        Err(why) => {
            log::error!(target: "pallet_kumandra", "Something went wrong during the migration of SessionForValidatorsChange {:?}", why);
        }
    };

    match Validators::<T>::translate(
        |old: Option<Option<Vec<T::AccountId>>>| -> Option<Vec<T::AccountId>> {
            log::info!(target: "pallet_kumandra", "Current storage value for Validators {:?}", old);
            match old {
                Some(Some(x)) => Some(x),
                _ => None,
            }
        },
    ) {
        Ok(_) => {
            writes += 1;
            log::info!(target: "pallet_kumandra", "Succesfully migrated storage for Validators");
        }
        Err(why) => {
            log::error!(target: "pallet_kumandra", "Something went wrong during the migration of Validators storage {:?}", why);
        }
    };

    // store new version
    StorageVersion::new(1).put::<P>();
    writes += 1;

    T::DbWeight::get().reads(2) + T::DbWeight::get().writes(writes)
}

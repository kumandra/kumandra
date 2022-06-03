use crate::Config;
use frame_support::{
    generate_storage_alias, log,
    traits::{Get, PalletInfoAccess, StorageVersion},
    weights::Weight,
};

generate_storage_alias!(Kumandra, SessionForValidatorsChange => Value<()>);
generate_storage_alias!(Kumandra, MillisecsPerBlock => Value<()>);
generate_storage_alias!(Kumandra, SessionPeriod => Value<()>);
generate_storage_alias!(Kumandra, Validators => Value<()>);

pub fn migrate<T: Config, P: PalletInfoAccess>() -> Weight {
    let mut writes = 0;
    let mut reads = 0;
    log::info!(target: "pallet_kumandra", "Running migration from STORAGE_VERSION 1 to 2");

    if !SessionForValidatorsChange::exists() {
        log::info!(target: "pallet_kumandra", "Storage item SessionForValidatorsChange does not exist!");
    } else {
        writes += 1;
    }
    SessionForValidatorsChange::kill();
    reads += 1;

    if !MillisecsPerBlock::exists() {
        log::info!(target: "pallet_kumandra", "Storage item MillisecsPerBlock does not exist!");
    } else {
        writes += 1;
    }
    MillisecsPerBlock::kill();
    reads += 1;

    if !SessionPeriod::exists() {
        log::info!(target: "pallet_kumandra", "Storage item SessionPeriod does not exist!");
    } else {
        writes += 1;
    }
    SessionPeriod::kill();
    reads += 1;

    if !Validators::exists() {
        log::info!(target: "pallet_kumandra", "Storage item Validators does not exist!");
    } else {
        writes += 1;
    }
    Validators::kill();
    reads += 1;

    // store new version
    StorageVersion::new(2).put::<P>();
    writes += 1;

    T::DbWeight::get().reads(reads) + T::DbWeight::get().writes(writes)
}

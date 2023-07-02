// Import the necessary pallet dependencies
use pallet_storage_provider as storage_provider;

// Include the pallets in the configuration trait
parameter_types! {
    pub const MaxCapacity: u64 = 100_000;  // Define the maximum storage capacity
    pub const InitialReliability: u64 = 80;  // Define the initial reliability value
    pub const InitialReputation: u64 = 90;  // Define the initial reputation value
}

impl storage_provider::Config for Runtime {
    type Event = Event;
    type MaxCapacity = MaxCapacity;
    type InitialReliability = InitialReliability;
    type InitialReputation = InitialReputation;
    type Balance = Balance;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Include other modules and pallets of your runtime configuration
        
        // Add the Storage Provider Pallet
        StorageProvider: storage_provider::{Pallet, Call, Storage, Event<T>},
    }
);
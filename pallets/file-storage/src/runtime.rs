// Import the necessary pallet dependencies
use pallet_file_storage as file_storage;

// Include the pallets in the configuration trait
parameter_types! {
    pub const MaxFileSize: u64 = 10_000_000;  // Define the maximum file size (in bytes)
}

impl file_storage::Config for Runtime {
    type Event = Event;
    type MaxFileSize = MaxFileSize;
    type Balance = Balance;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Include other modules and pallets of your runtime configuration
        
        // Add the File Storage Pallet
        FileStorage: file_storage::{Pallet, Call, Storage, Event<T>},
    }
);

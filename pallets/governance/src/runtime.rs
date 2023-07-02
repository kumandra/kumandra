use frame_support::{construct_runtime, parameter_types};
use pallet_governance as governance;

parameter_types! {
    pub const MinVotesThreshold: u32 = 10; // Set your desired minimum votes threshold
}

impl governance::Config for Runtime {
    type Event = Event;
    type MinVotesThreshold = MinVotesThreshold;
}

construct_runtime! {
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Other modules and pallets...

        // Add the governance pallet
        Governance: governance::{Module, Call, Storage, Event<T>, Config<T>},
    }
}

// Configure the pallet-specific events
impl pallet_governance::Config for Runtime {
    type Event = Event;
    type MinVotesThreshold = MinVotesThreshold;
}

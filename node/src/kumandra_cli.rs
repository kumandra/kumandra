use kumandra_primitives::DEFAULT_UNIT_CREATION_DELAY;
use clap::Parser;
use finality_kumandra::UnitCreationDelay;

#[derive(Debug, Parser, Clone)]
pub struct KumandraCli {
    #[clap(long)]
    unit_creation_delay: Option<u64>,
}

impl KumandraCli {
    pub fn unit_creation_delay(&self) -> UnitCreationDelay {
        UnitCreationDelay(
            self.unit_creation_delay
                .unwrap_or(DEFAULT_UNIT_CREATION_DELAY),
        )
    }
}

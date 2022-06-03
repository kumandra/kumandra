use crate::data_io::{KumandraData, UnvalidatedKumandraProposal};
use sp_runtime::traits::Block as BlockT;
use substrate_test_runtime_client::runtime::{Block, Header};

pub fn unvalidated_proposal_from_headers(headers: Vec<Header>) -> UnvalidatedKumandraProposal<Block> {
    let num = headers.last().unwrap().number;
    let hashes = headers.into_iter().map(|header| header.hash()).collect();
    UnvalidatedKumandraProposal::new(hashes, num)
}

pub fn kumandra_data_from_blocks(blocks: Vec<Block>) -> KumandraData<Block> {
    let headers = blocks.into_iter().map(|b| b.header().clone()).collect();
    kumandra_data_from_headers(headers)
}

pub fn kumandra_data_from_headers(headers: Vec<Header>) -> KumandraData<Block> {
    if headers.is_empty() {
        KumandraData::Empty
    } else {
        KumandraData::HeadProposal(unvalidated_proposal_from_headers(headers))
    }
}

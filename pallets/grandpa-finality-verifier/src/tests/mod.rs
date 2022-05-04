mod justification;
mod keyring;
mod mock;

use crate::chain::OpaqueExtrinsic;
use crate::{
    chain::Chain,
    grandpa::{verify_justification, AuthoritySet, Error, GrandpaJustification},
    initialize, validate_finalized_block, BestKnownFinalized, CurrentAuthoritySet, Error as ErrorP,
    InitializationData, LastImportedBlock,
};
use codec::{Decode, Encode};
use frame_support::{assert_err, assert_ok, dispatch::DispatchResult};
use justification::*;
use keyring::*;
use mock::{run_test, ChainId, TestRuntime};
use sp_core::Hasher as HasherT;
use sp_finality_grandpa::{ConsensusLog, ScheduledChange, GRANDPA_ENGINE_ID};
use sp_runtime::traits::{Hash, Header};
use sp_runtime::{
    generic, generic::SignedBlock, traits::BlakeTwo256, Digest, DigestItem, DispatchError,
};

type TestHeader = generic::Header<u32, BlakeTwo256>;

struct TestFeedChain;

impl Chain for TestFeedChain {
    type BlockNumber = u32;
    type Hash = <BlakeTwo256 as HasherT>::Out;
    type Header = generic::Header<u32, BlakeTwo256>;
    type Hasher = BlakeTwo256;
}

#[test]
fn valid_justification_accepted() {
    let authorities = vec![(ALICE, 1), (BOB, 1), (CHARLIE, 1), (DAVE, 1)];
    let params = JustificationGeneratorParams {
        header: test_header(1),
        round: TEST_GRANDPA_ROUND,
        set_id: TEST_GRANDPA_SET_ID,
        authorities: authorities.clone(),
        ancestors: 7,
        forks: 3,
    };

    let justification = make_justification_for_header::<TestHeader>(params.clone());
    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &justification,
        ),
        Ok(()),
    );

    assert_eq!(justification.commit.precommits.len(), authorities.len());
    assert_eq!(
        justification.votes_ancestries.len(),
        params.ancestors as usize
    );
}

#[test]
fn valid_justification_accepted_with_single_fork() {
    let params = JustificationGeneratorParams {
        header: test_header(1),
        round: TEST_GRANDPA_ROUND,
        set_id: TEST_GRANDPA_SET_ID,
        authorities: vec![(ALICE, 1), (BOB, 1), (CHARLIE, 1), (DAVE, 1), (EVE, 1)],
        ancestors: 5,
        forks: 1,
    };

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &make_justification_for_header::<TestHeader>(params)
        ),
        Ok(()),
    );
}

#[test]
fn valid_justification_accepted_with_arbitrary_number_of_authorities() {
    use finality_grandpa::voter_set::VoterSet;
    use sp_finality_grandpa::AuthorityId;

    let n = 15;
    let authorities = accounts(n).iter().map(|k| (*k, 1)).collect::<Vec<_>>();

    let params = JustificationGeneratorParams {
        header: test_header(1),
        round: TEST_GRANDPA_ROUND,
        set_id: TEST_GRANDPA_SET_ID,
        authorities: authorities.clone(),
        ancestors: n.into(),
        forks: n.into(),
    };

    let authorities = authorities
        .iter()
        .map(|(id, w)| (AuthorityId::from(*id), *w))
        .collect::<Vec<(AuthorityId, _)>>();
    let voter_set = VoterSet::new(authorities).unwrap();

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set,
            &make_justification_for_header::<TestHeader>(params)
        ),
        Ok(()),
    );
}

#[test]
fn justification_with_invalid_target_rejected() {
    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(2),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &make_default_justification::<TestHeader>(&test_header(1)),
        ),
        Err(Error::InvalidJustificationTarget),
    );
}

#[test]
fn justification_with_invalid_commit_rejected() {
    let mut justification = make_default_justification::<TestHeader>(&test_header(1));
    justification.commit.precommits.clear();

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &justification,
        ),
        Err(Error::ExtraHeadersInVotesAncestries),
    );
}

#[test]
fn justification_with_invalid_authority_signature_rejected() {
    let mut justification = make_default_justification::<TestHeader>(&test_header(1));
    justification.commit.precommits[0].signature =
        sp_core::crypto::UncheckedFrom::unchecked_from([1u8; 64]);

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &justification,
        ),
        Err(Error::InvalidAuthoritySignature),
    );
}

#[test]
fn justification_with_invalid_precommit_ancestry() {
    let mut justification = make_default_justification::<TestHeader>(&test_header(1));
    justification.votes_ancestries.push(test_header(10));

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &justification,
        ),
        Err(Error::ExtraHeadersInVotesAncestries),
    );
}

#[test]
fn justification_is_invalid_if_we_dont_meet_threshold() {
    // Need at least three authorities to sign off or else the voter set threshold can't be reached
    let authorities = vec![(ALICE, 1), (BOB, 1)];

    let params = JustificationGeneratorParams {
        header: test_header(1),
        round: TEST_GRANDPA_ROUND,
        set_id: TEST_GRANDPA_SET_ID,
        authorities: authorities.clone(),
        ancestors: 2 * authorities.len() as u32,
        forks: 2,
    };

    assert_eq!(
        verify_justification::<TestHeader>(
            header_id::<TestHeader>(1),
            TEST_GRANDPA_SET_ID,
            &voter_set(),
            &make_justification_for_header::<TestHeader>(params)
        ),
        Err(Error::TooLowCumulativeWeight),
    );
}

fn valid_digests() -> Vec<DigestItem> {
    vec![DigestItem::Consensus(
        GRANDPA_ENGINE_ID,
        ConsensusLog::ScheduledChange::<u32>(ScheduledChange {
            next_authorities: authority_list(),
            delay: 0,
        })
        .encode(),
    )]
}

fn init_with_origin(chain_id: ChainId, number: u32) -> Result<InitializationData, DispatchError> {
    let mut best_finalized = test_header::<TestHeader>(number);
    if number != 0 {
        valid_digests()
            .into_iter()
            .for_each(|digest| best_finalized.digest.push(digest));
    }
    let init_data = crate::InitializationData {
        genesis_hash: test_header::<TestHeader>(0).hash().into(),
        best_known_finalized_header: best_finalized.encode(),
        set_id: 1,
    };

    initialize::<TestRuntime, TestFeedChain>(chain_id, init_data.encode().as_slice())?;
    println!("Init data: {:?}", init_data.encode());
    Ok(init_data)
}

fn valid_extrinsics() -> Vec<OpaqueExtrinsic> {
    vec![(0..255).collect()]
}

fn invalid_extrinsics() -> Vec<OpaqueExtrinsic> {
    vec![(128..255).collect()]
}

fn valid_extrinsics_root<H: Header>() -> H::Hash {
    H::Hashing::ordered_trie_root(
        valid_extrinsics().iter().map(Encode::encode).collect(),
        sp_runtime::StateVersion::V0,
    )
}

fn submit_valid_finality_proof(chain_id: ChainId, header: u8) -> Result<TestHeader, DispatchError> {
    let header = test_header::<TestHeader>(header.into());
    let justification = make_default_justification(&header);
    submit_finality_proof(chain_id, header.clone(), justification)?;
    Ok(header)
}

fn submit_finality_proof(
    chain_id: ChainId,
    header: TestHeader,
    justification: GrandpaJustification<TestHeader>,
) -> DispatchResult {
    let block = SignedBlock {
        block: generic::Block::<TestHeader, OpaqueExtrinsic> {
            header,
            extrinsics: valid_extrinsics(),
        },
        justifications: Some((GRANDPA_ENGINE_ID, justification.encode()).into()),
    };

    println!("Hash: {:?}", block.block.header.hash());
    print!("Block: {:?}", block.encode());
    validate_finalized_block::<TestRuntime, TestFeedChain>(chain_id, block.encode().as_slice())?;
    Ok(())
}

#[test]
fn init_storage_entries_are_correctly_initialized() {
    run_test(|| {
        let chain_id: ChainId = 1;
        let InitializationData {
            genesis_hash,
            best_known_finalized_header,
            ..
        } = init_with_origin(chain_id, 0).unwrap();
        assert_eq!(
            LastImportedBlock::<TestRuntime>::get(chain_id),
            (0u64, genesis_hash)
        );
        assert_eq!(
            CurrentAuthoritySet::<TestRuntime>::get(chain_id).authorities,
            authority_list()
        );
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            best_known_finalized_header
        );
    })
}

#[test]
fn successfully_imports_header_with_valid_finality() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        assert_ok!(submit_valid_finality_proof(chain_id, 1));
        let header = test_header::<TestHeader>(1);
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            header.encode()
        );

        assert_ok!(submit_valid_finality_proof(chain_id, 2));
        let header = test_header::<TestHeader>(2);
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            header.encode()
        );
    })
}

#[test]
fn successfully_imports_parent_headers_to_best_known_finalized_header() {
    run_test(|| {
        let chain_id: ChainId = 1;
        let InitializationData {
            best_known_finalized_header,
            ..
        } = init_with_origin(chain_id, 2).unwrap();

        // import block 1
        assert_ok!(submit_valid_finality_proof(chain_id, 1));
        // best is still 2
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            best_known_finalized_header
        );

        // import block 2
        let header = TestHeader::decode(&mut best_known_finalized_header.as_slice()).unwrap();
        assert_ok!(submit_finality_proof(
            chain_id,
            header.clone(),
            make_default_justification(&header)
        ));
        // best is still 2
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            best_known_finalized_header
        );

        // import block 3
        let parent_hash = header.hash();
        let mut header = test_header::<TestHeader>(3);
        header.set_parent_hash(parent_hash);
        assert_ok!(submit_finality_proof(
            chain_id,
            header.clone(),
            make_default_justification(&header)
        ));
        // best is 3
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            header.encode()
        );
    })
}

#[test]
fn rejects_justification_that_skips_authority_set_transition() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));
        let header = test_header::<TestHeader>(1);

        let params = JustificationGeneratorParams::<TestHeader> {
            set_id: 2,
            ..Default::default()
        };
        let justification = make_justification_for_header(params);

        assert_err!(
            submit_finality_proof(chain_id, header, justification),
            <ErrorP<TestRuntime>>::InvalidJustification
        );
    })
}

#[test]
fn does_not_import_header_with_invalid_finality_proof() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        let header = test_header(1);
        let mut justification = make_default_justification(&header);
        justification.round = 42;

        assert_err!(
            submit_finality_proof(chain_id, header, justification),
            <ErrorP<TestRuntime>>::InvalidJustification
        );
    })
}

#[test]
fn does_not_import_header_with_invalid_extrinsics() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        let header = test_header::<TestHeader>(1);
        let block = SignedBlock {
            block: generic::Block::<TestHeader, OpaqueExtrinsic> {
                header: header.clone(),
                extrinsics: invalid_extrinsics(),
            },
            justifications: Some(
                (
                    GRANDPA_ENGINE_ID,
                    make_default_justification(&header).encode(),
                )
                    .into(),
            ),
        };

        assert_err!(
            validate_finalized_block::<TestRuntime, TestFeedChain>(
                chain_id,
                block.encode().as_slice(),
            ),
            <ErrorP<TestRuntime>>::InvalidBlock
        );
    })
}

#[test]
fn disallows_invalid_authority_set() {
    run_test(|| {
        let chain_id: ChainId = 1;
        let invalid_authority_list = vec![(ALICE.into(), u64::MAX), (BOB.into(), u64::MAX)];
        let mut genesis = test_header::<TestHeader>(0);
        let mut digest: Digest = Default::default();
        digest.push(DigestItem::Consensus(
            GRANDPA_ENGINE_ID,
            ConsensusLog::ScheduledChange::<u32>(ScheduledChange {
                next_authorities: invalid_authority_list,
                delay: 0,
            })
            .encode(),
        ));
        genesis.digest = digest;
        let init_data = crate::InitializationData {
            genesis_hash: genesis.hash().into(),
            best_known_finalized_header: genesis.encode(),
            set_id: 1,
        };

        assert_ok!(initialize::<TestRuntime, TestFeedChain>(
            chain_id,
            init_data.encode().as_slice()
        ));

        let mut header = test_header::<TestHeader>(1);
        header.set_parent_hash(genesis.hash());
        let justification = make_default_justification(&header);

        assert_err!(
            submit_finality_proof(chain_id, header, justification),
            <ErrorP<TestRuntime>>::InvalidAuthoritySet
        );
    })
}

#[test]
fn importing_header_ensures_that_chain_is_extended() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));
        assert_ok!(submit_valid_finality_proof(chain_id, 1));
        assert_ok!(submit_valid_finality_proof(chain_id, 2));
        assert_err!(
            submit_valid_finality_proof(chain_id, 4),
            ErrorP::<TestRuntime>::InvalidBlock
        );
        assert_ok!(submit_valid_finality_proof(chain_id, 3));
    })
}

fn change_log(delay: u32) -> Digest {
    let consensus_log =
        ConsensusLog::<u32>::ScheduledChange(sp_finality_grandpa::ScheduledChange {
            next_authorities: vec![(ALICE.into(), 1), (BOB.into(), 1)],
            delay,
        });

    Digest {
        logs: vec![DigestItem::Consensus(
            GRANDPA_ENGINE_ID,
            consensus_log.encode(),
        )],
    }
}

#[test]
fn importing_header_enacts_new_authority_set() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        let next_set_id = 2;
        let next_authorities = vec![(ALICE.into(), 1), (BOB.into(), 1)];

        // Need to update the header digest to indicate that our header signals an authority set
        // change. The change will be enacted when we import our header.
        let mut header = test_header::<TestHeader>(1);
        header.digest = change_log(0);

        // Create a valid justification for the header
        let justification = make_default_justification(&header);

        // Let's import our test header
        assert_ok!(submit_finality_proof(
            chain_id,
            header.clone(),
            justification
        ));

        // Make sure that our header is the best finalized
        assert_eq!(
            <BestKnownFinalized<TestRuntime>>::get(chain_id),
            header.encode()
        );

        // Make sure that the authority set actually changed upon importing our header
        assert_eq!(
            <CurrentAuthoritySet<TestRuntime>>::get(chain_id),
            AuthoritySet {
                authorities: next_authorities,
                set_id: next_set_id
            },
        );
    })
}

#[test]
fn importing_header_rejects_header_with_scheduled_change_delay() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        // Need to update the header digest to indicate that our header signals an authority set
        // change. However, the change doesn't happen until the next block.
        let mut header = test_header::<TestHeader>(1);
        header.digest = change_log(1);

        // Create a valid justification for the header
        let justification = make_default_justification(&header);

        // Should not be allowed to import this header
        assert_err!(
            submit_finality_proof(chain_id, header, justification),
            <ErrorP<TestRuntime>>::UnsupportedScheduledChange
        );
    })
}

fn forced_change_log(delay: u32) -> Digest {
    let consensus_log = ConsensusLog::<u32>::ForcedChange(
        delay,
        sp_finality_grandpa::ScheduledChange {
            next_authorities: vec![(ALICE.into(), 1), (BOB.into(), 1)],
            delay,
        },
    );

    Digest {
        logs: vec![DigestItem::Consensus(
            GRANDPA_ENGINE_ID,
            consensus_log.encode(),
        )],
    }
}

#[test]
fn importing_header_rejects_header_with_forced_changes() {
    run_test(|| {
        let chain_id: ChainId = 1;
        assert_ok!(init_with_origin(chain_id, 0));

        // Need to update the header digest to indicate that it signals a forced authority set
        // change.
        let mut header = test_header::<TestHeader>(1);
        header.digest = forced_change_log(0);

        // Create a valid justification for the header
        let justification = make_default_justification(&header);

        // Should not be allowed to import this header
        assert_err!(
            submit_finality_proof(chain_id, header, justification),
            <ErrorP<TestRuntime>>::UnsupportedScheduledChange
        );
    })
}

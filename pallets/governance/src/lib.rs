use frame_support::{pallet, decl_event, decl_error, decl_module};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[pallet]
pub mod governance {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub(super) type Proposals<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Proposal<T::AccountId, T::BlockNumber>>;

    #[pallet::storage]
    #[pallet::getter(fn proposal_votes)]
    pub(super) type ProposalVotes<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::Hash, Blake2_128Concat, T::AccountId, VoteStatus>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProposalSubmitted(T::AccountId, T::Hash),
        ProposalVoted(T::AccountId, T::Hash, VoteType),
        ProposalExecuted(T::AccountId, T::Hash),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidProposal,
        AlreadyVoted,
        NotEnoughVotes,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn submit_proposal(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
            proposal_data: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            let proposal = Proposal {
                proposer: proposer.clone(),
                proposal_data,
            };

            Proposals::<T>::insert(&proposal_hash, &proposal);

            Self::deposit_event(Event::ProposalSubmitted(proposer, proposal_hash));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn vote_proposal(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
            vote: VoteType,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            ensure!(Proposals::<T>::contains_key(&proposal_hash), Error::<T>::InvalidProposal);
            ensure!(
                !ProposalVotes::<T>::contains_key(&proposal_hash, &voter),
                Error::<T>::AlreadyVoted
            );

            ProposalVotes::<T>::insert(&proposal_hash, &voter, vote.into());

            Self::deposit_event(Event::ProposalVoted(voter, proposal_hash, vote));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn execute_proposal(
            origin: OriginFor<T>,
            proposal_hash: T::Hash,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(Proposals::<T>::contains_key(&proposal_hash), Error::<T>::InvalidProposal);

            let proposal = Proposals::<T>::get(&proposal_hash);
            let votes = ProposalVotes::<T>::iter_prefix(&proposal_hash)
                .map(|(_, vote)| vote)
                .collect::<Vec<VoteStatus>>();

            ensure!(
                Self::is_approved(&votes),
                Error::<T>::NotEnoughVotes
            );

            Proposals::<T>::remove(&proposal_hash);
            ProposalVotes::<T>::remove_prefix(&proposal_hash);

            Self::deposit_event(Event::ProposalExecuted(proposal.proposer, proposal_hash));

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn is_approved(votes: &[VoteStatus]) -> bool {
            // Implement your custom voting logic here
            // For example, check if the number of "Aye" votes is greater than a threshold
            let num_ayes = votes.iter().filter(|&vote| *vote == VoteType::Aye).count();
            let threshold = T::MinVotesThreshold::get();

            num_ayes >= threshold
        }
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Proposal<AccountId, BlockNumber> {
    proposer: AccountId,
    proposal_data: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum VoteType {
    Aye,
    Nay,
}

impl Into<VoteStatus> for VoteType {
    fn into(self) -> VoteStatus {
        match self {
            VoteType::Aye => VoteStatus::Aye,
            VoteType::Nay => VoteStatus::Nay,
        }
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum VoteStatus {
    Aye,
    Nay,
}

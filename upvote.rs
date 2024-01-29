//Importing necessary libraries
use frame_system::Module as System;
use frame_support::{decl_module, decl_storage, decl_event, dispatch, ensure};
use sp_runtime::DispatchResult;

decl_storage! {
    trait Store for Module<T: Trait> as Aptos {
        Wallets get(fn wallets): map hasher(blake2_128_concat) T::AccountId => u64;
        Proposals get(fn proposals): map hasher(blake2_128_concat) T::Hash => Proposal<T::AccountId>;
        CurrentRound get(fn current_round): Option<Round<T::Hash>>;
    }
}


pub struct Proposal<AccountId> {
    proposer: AccountId,
    upvotes: u64,
}


pub struct Round<Hash> {
    round_id: Hash,
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as frame_system::Trait>::Hash,
    {
        WalletCreated(AccountId, u64),
        ProposalCreated(AccountId, Hash),
        ProposalUpvoted(AccountId, Hash, u64),
        RoundFinished(Hash),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        NotEnoughTokens,
        DuplicateUpvote,
        NoOngoingRound,
        OngoingRoundExists,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        fn get_wallet(origin, amount: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Wallets::<T>::contains_key(&sender), Error::<T>::OngoingRoundExists);
            Wallets::<T>::insert(&sender, amount);
            Self::deposit_event(RawEvent::WalletCreated(sender, amount));
            Ok(())
        }
// function to create a proposal
        fn create_proposal(origin, proposal_hash: T::Hash) -> dispatch::DispatchResult {
            let proposer = ensure_signed(origin)?;
            ensure!(CurrentRound::<T>::get().is_none(), Error::<T>::OngoingRoundExists);
            ensure!(!Proposals::<T>::contains_key(&proposal_hash), Error::<T>::OngoingRoundExists);
            let proposal = Proposal {
                proposer: proposer.clone(),
                upvotes: 0,
            };
            Proposals::<T>::insert(proposal_hash, proposal);
            CurrentRound::<T>::put(Round { round_id: proposal_hash });
            Self::deposit_event(RawEvent::ProposalCreated(proposer, proposal_hash));
            Ok(())
        }
// function for upvote of a proposal
        fn upvote_proposal(origin, proposal_hash: T::Hash, amount: u64) -> dispatch::DispatchResult {
            let upvoter = ensure_signed(origin)?;
            ensure!(CurrentRound::<T>::get().is_some(), Error::<T>::NoOngoingRound);
            let mut proposal = Proposals::<T>::get(&proposal_hash).ok_or(Error::<T>::NoOngoingRound)?;
            ensure!(proposal.proposer != upvoter, Error::<T>::DuplicateUpvote);
            let num_upvotes = amount / UPVOTE_PRICE;
            proposal.upvotes += num_upvotes;
            Proposals::<T>::insert(proposal_hash, proposal);
            Self::deposit_event(RawEvent::ProposalUpvoted(upvoter, proposal_hash, num_upvotes));
            Ok(())
        }

        fn finish_round(origin) -> dispatch::DispatchResult {
            ensure_root(origin)?;
            ensure!(CurrentRound::<T>::get().is_some(), Error::<T>::NoOngoingRound);
            let round = CurrentRound::<T>::take().ok_or(Error::<T>::NoOngoingRound)?;
            let winning_proposal = Proposals::<T>::iter()
                .max_by_key(|(_, proposal)| proposal.upvotes)
                .map(|(hash, _)| hash);
            if let Some(winner) = winning_proposal {
                if let Some(winning_proposal) = Proposals::<T>::get(&winner) {
                    let winner_reward = MAX_UPVOTES * UPVOTE_PRICE;
                    let other_reward = winner_reward / 2;
                    println!("Winner: {}, Reward: {} tokens", winner, winner_reward + other_reward);
                }
            }
            Proposals::<T>::remove_all(None);
            Self::deposit_event(RawEvent::RoundFinished(round.round_id));
            Ok(())
        }
    }
}

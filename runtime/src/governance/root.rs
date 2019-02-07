#![cfg_attr(not(feature = "std"), no_std)]

use srml_support::{StorageValue, StorageMap, dispatch::Result};

use governance::{council, election::{self, TriggerElection}};

pub trait Trait: system::Trait + council::Trait + election::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type TriggerElection: election::TriggerElection<election::Seats<Self::AccountId, Self::Balance>, election::ElectionParameters<Self::BlockNumber, Self::Balance>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Root {
        ElectionParameters get(election_parameters) config(): election::ElectionParameters<T::BlockNumber, T::Balance>;
    }
}

/// Event for this module.
decl_event!(
    pub enum Event<T> where <T as system::Trait>::BlockNumber {
        // TODO add more useful info to events?
        ElectionStarted(),
        CouncilTermEnded(),
        Dummy(BlockNumber),
    }
);

impl<T: Trait> Module<T> {
    // Nothing yet
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;
    }
}

impl<T: Trait> council::CouncilTermEnded for Module<T> {
    fn council_term_ended() {
        Self::deposit_event(RawEvent::CouncilTermEnded());

        if !<election::Module<T>>::is_election_running() {
            let current_council = <council::Module<T>>::active_council();

            let params = Self::election_parameters();

            if T::TriggerElection::trigger_election(current_council, params).is_ok() {
                Self::deposit_event(RawEvent::ElectionStarted());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::governance::tests::*;

    #[test]
    fn election_is_triggerred_when_council_term_ends() {
        with_externalities(&mut initial_test_ext(), || {
            System::set_block_number(1);

            assert!(Council::is_term_ended(1));
            assert!(Election::stage().is_none());

            <Governance as council::CouncilTermEnded>::council_term_ended();

            assert!(Election::stage().is_some());
        });
    }
}
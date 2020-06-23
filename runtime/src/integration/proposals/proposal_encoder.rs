use crate::{Call, Runtime};
use common::working_group::WorkingGroup;
use proposals_codex::{ProposalDetails, ProposalDetailsOf, ProposalEncoder};
use working_group::OpeningType;

use codec::Encode;
use rstd::vec::Vec;
use srml_support::print;

/// _ProposalEncoder_ implementation. It encodes extrinsics with proposal details parameters
/// using Runtime Call and parity codec.
pub struct ExtrinsicProposalEncoder;
impl ProposalEncoder<Runtime> for ExtrinsicProposalEncoder {
    fn encode_proposal(proposal_details: ProposalDetailsOf<Runtime>) -> Vec<u8> {
        match proposal_details {
            ProposalDetails::Text(text) => {
                Call::ProposalsCodex(proposals_codex::Call::execute_text_proposal(text)).encode()
            }
            ProposalDetails::SetElectionParameters(election_parameters) => Call::CouncilElection(
                governance::election::Call::set_election_parameters(election_parameters),
            )
            .encode(),
            ProposalDetails::SetContentWorkingGroupMintCapacity(mint_balance) => {
                Call::ContentWorkingGroup(content_working_group::Call::set_mint_capacity(
                    mint_balance,
                ))
                .encode()
            }
            ProposalDetails::Spending(balance, destination) => Call::Council(
                governance::council::Call::spend_from_council_mint(balance, destination),
            )
            .encode(),
            ProposalDetails::SetLead(new_lead) => {
                Call::ContentWorkingGroup(content_working_group::Call::replace_lead(new_lead))
                    .encode()
            }
            ProposalDetails::SetValidatorCount(new_validator_count) => {
                Call::Staking(staking::Call::set_validator_count(new_validator_count)).encode()
            }
            ProposalDetails::RuntimeUpgrade(wasm_code) => Call::ProposalsCodex(
                proposals_codex::Call::execute_runtime_upgrade_proposal(wasm_code),
            )
            .encode(),
            // ********** Deprecated during the Nicaea release.
            // It is kept only for backward compatibility in the Pioneer. **********
            ProposalDetails::EvictStorageProvider(_) => {
                print("Error: Calling deprecated EvictStorageProvider encoding option.");
                Vec::new()
            }
            // ********** Deprecated during the Nicaea release.
            // It is kept only for backward compatibility in the Pioneer. **********
            ProposalDetails::SetStorageRoleParameters(_) => {
                print("Error: Calling deprecated SetStorageRoleParameters encoding option.");
                Vec::new()
            }
            ProposalDetails::AddWorkingGroupLeaderOpening(add_opening_params) => {
                let call = match add_opening_params.working_group {
                    WorkingGroup::Storage => {
                        Call::StorageWorkingGroup(working_group::Call::add_opening(
                            add_opening_params.activate_at,
                            add_opening_params.commitment,
                            add_opening_params.human_readable_text,
                            OpeningType::Leader,
                        ))
                    }
                };

                call.encode()
            }
        }
    }
}

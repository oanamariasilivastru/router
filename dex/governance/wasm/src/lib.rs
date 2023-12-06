// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           22
// Async Callback (empty):               1
// Total number of exported functions:  24

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    governance
    (
        init => init
        upgrade => upgrade
        propose => propose
        upvote => upvote
        downvote => downvote
        execute => execute
        redeem => redeem
        changeQuorum => change_quorum
        changeMinTokenBalanceForProposing => change_min_weight_for_proposal
        changeVotingDelayInBlocks => change_voting_delay_in_blocks
        changeVotingPeriodInBlocks => change_voting_period_in_blocks
        changeGovernanceTokenIds => change_governance_token_ids
        changePriceProviders => change_price_providers
        getGovernanceTokenId => governance_token_ids
        getQuorum => quorum
        getMinWeightForProposal => min_weight_for_proposal
        getVotingDelayInBlocks => voting_delay_in_blocks
        getVotingPeriodInBlocks => voting_period_in_blocks
        getProposal => proposal
        getProposalIdCounter => proposal_id_counter
        getVoteNFTId => vote_nft_id
        getMexTokenId => mex_token_id
        getProposalStatus => get_proposal_status_view
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}

#[test]
fn add_liquidity_go() {
    multiversx_sc_scenario::run_go("mandos/add_liquidity.scen.json");
}

#[test]
fn calculate_rewards_for_given_position_go() {
    multiversx_sc_scenario::run_go("mandos/calculate_rewards_for_given_position.scen.json");
}

#[test]
fn calculate_rewards_for_given_position_after_compound_go() {
    multiversx_sc_scenario::run_go(
        "mandos/calculate_rewards_for_given_position_after_compound.scen.json",
    );
}

#[test]
fn check_fee_disabled_after_swap_go() {
    multiversx_sc_scenario::run_go("mandos/check_fee_disabled_after_swap.scen.json");
}

#[test]
fn check_fee_enabled_after_swap_go() {
    multiversx_sc_scenario::run_go("mandos/check_fee_enabled_after_swap.scen.json");
}

#[test]
fn claim_rewards_go() {
    multiversx_sc_scenario::run_go("mandos/claim_rewards.scen.json");
}

#[test]
fn complete_setup_go() {
    multiversx_sc_scenario::run_go("mandos/complete_setup.scen.json");
}

#[test]
fn compound_rewards_go() {
    multiversx_sc_scenario::run_go("mandos/compound_rewards.scen.json");
}

#[test]
fn create_pair_twice_go() {
    multiversx_sc_scenario::run_go("mandos/create_pair_twice.scen.json");
}

#[test]
fn enter_farm_go() {
    multiversx_sc_scenario::run_go("mandos/enter_farm.scen.json");
}

#[test]
fn enter_farm_with_merge_tokens_go() {
    multiversx_sc_scenario::run_go("mandos/enter_farm_with_merge_tokens.scen.json");
}

#[test]
fn enter_mex_farm_go() {
    multiversx_sc_scenario::run_go("mandos/enter_mex_farm.scen.json");
}

#[test]
fn exit_farm_go() {
    multiversx_sc_scenario::run_go("mandos/exit_farm.scen.json");
}

#[test]
fn exit_farm_too_soon_go() {
    multiversx_sc_scenario::run_go("mandos/exit_farm_too_soon.scen.json");
}

#[test]
fn exit_mex_farm_go() {
    multiversx_sc_scenario::run_go("mandos/exit_mex_farm.scen.json");
}

#[test]
fn farm_reward_distr_scen_1_go() {
    multiversx_sc_scenario::run_go("mandos/farm_reward_distr_scen_1.scen.json");
}

#[test]
fn farm_reward_distr_scen_2_go() {
    multiversx_sc_scenario::run_go("mandos/farm_reward_distr_scen_2.scen.json");
}

#[test]
fn farm_reward_distr_scen_3_go() {
    multiversx_sc_scenario::run_go("mandos/farm_reward_distr_scen_3.scen.json");
}

#[test]
fn farm_reward_distr_scen_4_go() {
    multiversx_sc_scenario::run_go("mandos/farm_reward_distr_scen_4.scen.json");
}

#[test]
fn farm_with_egld_token_go() {
    multiversx_sc_scenario::run_go("mandos/farm_with_egld_token.scen.json");
}

#[test]
fn farm_wrong_lp_token_go() {
    multiversx_sc_scenario::run_go("mandos/farm_wrong_lp_token.scen.json");
}

#[test]
fn get_amounts_go() {
    multiversx_sc_scenario::run_go("mandos/get_amounts.scen.json");
}

#[test]
fn get_amounts_no_liquidity_go() {
    multiversx_sc_scenario::run_go("mandos/get_amounts_no_liquidity.scen.json");
}

#[test]
fn get_pair_non_existent_go() {
    multiversx_sc_scenario::run_go("mandos/get_pair_non_existent.scen.json");
}

#[test]
fn get_pair_views_go() {
    multiversx_sc_scenario::run_go("mandos/get_pair_views.scen.json");
}

#[test]
fn merge_tokens_go() {
    multiversx_sc_scenario::run_go("mandos/merge_tokens.scen.json");
}

#[test]
fn owner_pause_farm_go() {
    multiversx_sc_scenario::run_go("mandos/owner_pause_farm.scen.json");
}

#[test]
fn owner_resume_farm_go() {
    multiversx_sc_scenario::run_go("mandos/owner_resume_farm.scen.json");
}

#[test]
fn remove_liquidity_go() {
    multiversx_sc_scenario::run_go("mandos/remove_liquidity.scen.json");
}

#[test]
fn remove_liquidity_and_buyback_and_burn_token_go() {
    multiversx_sc_scenario::run_go("mandos/remove_liquidity_and_buyback_and_burn_token.scen.json");
}

#[test]
fn remove_liquidity_twice_go() {
    multiversx_sc_scenario::run_go("mandos/remove_liquidity_twice.scen.json");
}

#[test]
fn remove_pair_go() {
    multiversx_sc_scenario::run_go("mandos/remove_pair.scen.json");
}

#[test]
fn router_pause_self_go() {
    multiversx_sc_scenario::run_go("mandos/router_pause_self.scen.json");
}

#[test]
fn router_resume_self_go() {
    multiversx_sc_scenario::run_go("mandos/router_resume_self.scen.json");
}

#[test]
fn swap_fixed_input_go() {
    multiversx_sc_scenario::run_go("mandos/swap_fixed_input.scen.json");
}

#[test]
fn swap_fixed_input_after_removed_liquidity_go() {
    multiversx_sc_scenario::run_go("mandos/swap_fixed_input_after_removed_liquidity.scen.json");
}

#[test]
fn swap_fixed_output_go() {
    multiversx_sc_scenario::run_go("mandos/swap_fixed_output.scen.json");
}

#[test]
fn swap_same_token_go() {
    multiversx_sc_scenario::run_go("mandos/swap_same_token.scen.json");
}

#[test]
fn swap_wrong_token_go() {
    multiversx_sc_scenario::run_go("mandos/swap_wrong_token.scen.json");
}

#[test]
fn upgrade_contract_go() {
    multiversx_sc_scenario::run_go("mandos/upgrade_contract.scen.json");
}

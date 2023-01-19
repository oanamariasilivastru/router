// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           63
// Async Callback:                       1
// Total number of exported functions:  65

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    farm_staking
    (
        mergeFarmTokens
        calculateRewardsForGivenPosition
        topUpRewards
        endProduceRewards
        setPerBlockRewardAmount
        setMaxApr
        setMinUnbondEpochs
        startProduceRewards
        getAccumulatedRewards
        getRewardCapacity
        getAnnualPercentageRewards
        getMinUnbondEpochs
        getRewardPerShare
        getRewardReserve
        getFarmingTokenId
        getRewardTokenId
        getPerBlockRewardAmount
        getLastRewardBlockNonce
        getDivisionSafetyConstant
        registerFarmToken
        getFarmTokenId
        getFarmTokenSupply
        addSCAddressToWhitelist
        removeSCAddressFromWhitelist
        isSCAddressWhitelisted
        addToPauseWhitelist
        removeFromPauseWhitelist
        pause
        resume
        getState
        addAdmin
        removeAdmin
        updateOwnerOrAdmin
        getPermissions
        stakeFarmThroughProxy
        stakeFarm
        claimRewards
        claimRewardsWithNewValue
        compoundRewards
        unstakeFarm
        unstakeFarmThroughProxy
        unbondFarm
        setBoostedYieldsRewardsPercentage
        collectUndistributedBoostedRewards
        getBoostedYieldsRewardsPercentage
        getAccumulatedRewardsForWeek
        getFarmSupplyForWeek
        getRemainingBoostedRewardsToDistribute
        getUndistributedBoostedRewards
        setBoostedYieldsFactors
        getBoostedYieldsFactors
        getCurrentWeek
        getFirstWeekStartEpoch
        getLastActiveWeekForUser
        getUserEnergyForWeek
        getLastGlobalUpdateWeek
        getTotalRewardsForWeek
        getTotalEnergyForWeek
        getTotalLockedTokensForWeek
        updateEnergyForUser
        getCurrentClaimProgress
        setEnergyFactoryAddress
        getEnergyFactoryAddress
        callBack
    )
}

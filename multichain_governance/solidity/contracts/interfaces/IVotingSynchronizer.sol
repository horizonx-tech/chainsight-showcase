// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IVotingSynchronizer {
    /**
     * @dev Synchronize a vote
     * @param id The id of the proposal
     * @param support Whether to vote for or against the proposal
     * @param votingPower The voting power of the voter
     * @param chainId The chain id of the voter
     **/
    function synchronize(
        uint256 id,
        bool support,
        uint248 votingPower,
        uint256 chainId
    ) external;
}

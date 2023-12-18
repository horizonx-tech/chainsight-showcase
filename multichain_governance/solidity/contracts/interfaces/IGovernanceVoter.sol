// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IGovernanceVoter {
    event Vote(address voter, bool support, uint248 voitingPower);

    /**
     * @dev Vote on a proposal
     * @param id The id of the proposal
     * @param inSupport Whether to vote for or against the proposal
     **/
    function vote(uint256 id, bool inSupport) external;
}

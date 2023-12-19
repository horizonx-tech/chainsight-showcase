// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IProposalSynchronizer {
    event ProposalSynchronized(
        uint256 indexed id,
        address indexed proposer,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    );

    /**
     * @dev Set the proposal manager
     * @param _proposalManager The address of the proposal manager
     **/
    function setProposalManager(address _proposalManager) external;

    /**
     * @dev Synchronize a proposal from another chain
     * @param id The id of the proposal
     * @param proposer proposer of the proposal
     * @param chainId The id of the chain where the proposal is created
     * @param startTimestamp The start timestamp of voting
     * @param endTimestamp The end timestamp of voting
     **/
    function synchronize(
        uint256 id,
        address proposer,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external;
}

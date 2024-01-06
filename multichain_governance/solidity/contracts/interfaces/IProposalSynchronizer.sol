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
     * @param proposedBlock The block number of the proposal
     **/
    function synchronize(
        uint256 id,
        address proposer,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp,
        uint256 proposedBlock
    ) external;

    /**
     * @dev Batch synchronize proposals from another chain
     * @param ids The ids of the proposals
     * @param proposers proposers of the proposals
     * @param chainIds The ids of the chain where the proposals are created
     * @param startTimestamps The start timestamps of voting
     * @param endTimestamps The end timestamps of voting
     * @param proposedBlocks The block numbers of the proposals
     * @notice The length of the arrays should be the same
     **/
    function batchSynchronize(
        uint256[] calldata ids,
        address[] calldata proposers,
        uint256[] calldata chainIds,
        uint256[] calldata startTimestamps,
        uint256[] calldata endTimestamps,
        uint256[] calldata proposedBlocks
    ) external;
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IVotingSynchronizer {
    event VoteSynchronized(
        uint256 indexed id,
        address indexed voter,
        uint256 indexed chainId,
        uint256 votingPower,
        bool support
    );

    /**
     * @dev Set the proposal manager
     * @param _proposalManager The address of the proposal manager
     **/
    function setProposalManager(address _proposalManager) external;

    /**
     * @dev Synchronize a vote
     * @param id The id of the proposal
     * @param voter The address of the voter
     * @param support Whether to vote for or against the proposal
     * @param votingPower The voting power of the voter
     * @param chainId The chain id of the voter
     **/
    function synchronize(
        uint256 id,
        address voter,
        bool support,
        uint248 votingPower,
        uint256 chainId
    ) external;

    /**
     * @dev Synchronize a batch of votes
     * @param ids The ids of the proposals
     * @param voters The addresses of the voters
     * @param _supports Whether to vote for or against the proposals
     * @param votingPowers The voting powers of the voters
     * @param chainIds The chain ids of the voters
     **/
    function batchSynchronize(
        uint256[] calldata ids,
        address[] calldata voters,
        bool[] calldata _supports,
        uint248[] calldata votingPowers,
        uint256[] calldata chainIds
    ) external;
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IProposalManager {
    event Voted(
        uint256 indexed id,
        uint256 indexed chainId,
        address indexed voter,
        uint256 power,
        bool support
    );

    /**
     * get proposal information.
     * @param id The id of the proposal
     * @return creator The creator of the proposal
     * @return chainId The id of the chain where the proposal is created
     * @return startTimestamp The start timestamp of voting
     * @return endTimestamp The end timestamp of voting
     * @return forVotes The total votes for the proposal
     * @return againstVotes The total votes against the proposal
     * @return accepted Whether the proposal is accepted
     */
    function getProposal(
        uint256 id
    )
        external
        view
        returns (
            address creator,
            uint256 chainId,
            uint256 startTimestamp,
            uint256 endTimestamp,
            uint256 forVotes,
            uint256 againstVotes,
            bool accepted
        );

    /**
     * @dev synchronize a proposal from another chain
     * @param id The id of the proposal
     * @param creator The creator of the proposal
     * @param chainId The id of the chain where the proposal is created
     * @param startTimestamp The start timestamp of voting
     * @param endTimestamp The end timestamp of voting
     */
    function onProposalCreated(
        uint256 id,
        address creator,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external;

    /**
     * @dev synchronize a vote from another chain
     * @param id The id of the proposal
     * @param chainId The id of the chain where the proposal is created
     * @param voter The voter of the proposal
     * @param support Whether the voter supports the proposal
     */
    function onVoted(
        uint256 id,
        uint256 chainId,
        address voter,
        uint256 power,
        bool support
    ) external;

    /**
     * @dev vote for a proposal on the current chain
     * @param id The id of the proposal
     * @param support Whether the voter supports the proposal
     */
    function vote(uint256 id, bool support) external;
}

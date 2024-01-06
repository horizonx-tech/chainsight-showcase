// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

interface IProposalFactory {
    event ProposalCreated(
        uint256 indexed id,
        address indexed proposer,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    );

    /**
     * @dev Create a new proposal
     * @param startTimestamp The start timestamp of voting
     * @param endTimestamp The end timestamp of voting
     * @return The id of the new proposal
     **/
    function create(
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external returns (uint256);
}

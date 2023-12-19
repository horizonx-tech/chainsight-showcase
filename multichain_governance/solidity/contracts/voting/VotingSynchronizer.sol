// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "../misc/Synchronizeable.sol";
import "../interfaces/IVotingSynchronizer.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IProposalManager.sol";

contract VotingSynchronizer is IVotingSynchronizer, Synchronizeable {
    IProposalManager public proposalManager;

    // mapped by proposal id => voter address => chainids
    mapping(uint256 => mapping(address => uint256[])) public votes;

    constructor() Ownable(msg.sender) {}

    /// @inheritdoc IVotingSynchronizer
    function setProposalManager(address _proposalManager) external onlyOwner {
        require(
            _proposalManager != address(0),
            "VotingSynchronizer: INVALID_PROPOSAL_MANAGER"
        );
        proposalManager = IProposalManager(_proposalManager);
    }

    /// @inheritdoc IVotingSynchronizer
    function synchronize(
        uint256 id,
        address voter,
        bool support,
        uint248 votingPower,
        uint256 chainId
    ) external override onlySynchronizer {
        require(
            address(proposalManager) != address(0),
            "VotingSynchronizer: PROPOSAL_MANAGER_NOT_SET"
        );
        // to make this function idempotent, if the vote is already synchronized, no error should be thrown
        if (votes[id][voter][chainId] != 0) {
            return;
        }
        votes[id][voter][chainId] = votingPower;
        proposalManager.onVoted(id, chainId, voter, votingPower, support);
        emit VoteSynchronized(id, voter, chainId, votingPower, support);
    }
}

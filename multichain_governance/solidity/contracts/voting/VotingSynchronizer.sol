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
        _synchronize(id, voter, support, votingPower, chainId);
    }

    /// @inheritdoc IVotingSynchronizer
    function batchSynchronize(
        uint256[] calldata ids,
        address[] calldata voters,
        bool[] calldata _supports,
        uint248[] calldata votingPowers,
        uint256[] calldata chainIds
    ) external override onlySynchronizer {
        require(
            ids.length == voters.length &&
                ids.length == _supports.length &&
                ids.length == votingPowers.length &&
                ids.length == chainIds.length,
            "VotingSynchronizer: INVALID_INPUT"
        );
        for (uint256 i = 0; i < ids.length; i++) {
            _synchronize(
                ids[i],
                voters[i],
                _supports[i],
                votingPowers[i],
                chainIds[i]
            );
        }
    }

    function _synchronize(
        uint256 id,
        address voter,
        bool support,
        uint248 votingPower,
        uint256 chainId
    ) internal {
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

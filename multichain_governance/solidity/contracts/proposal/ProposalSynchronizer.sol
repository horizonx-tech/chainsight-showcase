// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "../interfaces/IProposalSynchronizer.sol";
import "../misc/Synchronizeable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract ProposalSynchronizer is IProposalSynchronizer, Synchronizeable {
    struct Proposal {
        address proposer;
        uint256 chainId;
        uint256 startTimestamp;
        uint256 endTimestamp;
    }

    constructor() Ownable(msg.sender) {}

    mapping(uint256 => Proposal) public proposals;

    /// @inheritdoc IProposalSynchronizer
    function synchronize(
        uint256 id,
        address proposer,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external override onlySynchronizer {
        // to make this function idempotent, if the proposal is already synchronized, no error should be thrown
        if (proposals[id].proposer != address(0)) {
            return;
        }
        proposals[id] = Proposal({
            proposer: proposer,
            chainId: chainId,
            startTimestamp: startTimestamp,
            endTimestamp: endTimestamp
        });
        emit ProposalSynchronized(
            id,
            proposer,
            chainId,
            startTimestamp,
            endTimestamp
        );
    }
}

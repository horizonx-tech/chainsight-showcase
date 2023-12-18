// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "../interfaces/IProposalFactory.sol";
import "../interfaces/IProposalManager.sol";

contract ProposalFactory is IProposalFactory {
    IProposalManager public proposalManager;

    constructor(address _proposalManager) {
        proposalManager = IProposalManager(_proposalManager);
    }

    /**
     * @dev Create a new proposal
     * @param startTimestamp The start timestamp of voting
     * @param endTimestamp The end timestamp of voting
     * @return The id of the new proposal
     **/
    function create(
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external override returns (uint256) {
        uint256 currentTimestamp = block.timestamp;
        require(
            (endTimestamp > startTimestamp) &&
                (startTimestamp > currentTimestamp),
            "ProposalFactory: INVALID_TIMESTAMP"
        );

        uint256 id = uint256(
            keccak256(
                abi.encodePacked(
                    msg.sender,
                    block.timestamp,
                    startTimestamp,
                    endTimestamp
                )
            )
        );
        proposalManager.onProposalCreated(
            id,
            msg.sender,
            block.chainid,
            startTimestamp,
            endTimestamp
        );
        emit ProposalCreated(
            id,
            msg.sender,
            block.chainid,
            startTimestamp,
            endTimestamp
        );
        return id;
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

abstract contract Synchronizeable is Ownable {
    address public synchronizer;
    event SynchronizerSet(address synchronizer);

    modifier onlySynchronizer() {
        require(
            synchronizer != address(0),
            "Synchronizeable: SYNCHRONIZER_NOT_SET"
        );
        require(
            msg.sender == synchronizer,
            "Synchronizeable: ONLY_SYNCHRONIZER"
        );
        _;
    }

    function setSynchronizer(address _synchronizer) external onlyOwner {
        synchronizer = _synchronizer;
        emit SynchronizerSet(_synchronizer);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;
import "../interfaces/IAggregatorWrapper.sol";

contract MockChainlinkWrapper is IAggregatorWrapper {
    function latestAnswer() external pure override returns (uint256) {
        return 123456789;
    }
}

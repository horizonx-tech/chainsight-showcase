// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IAggregatorWrapper {
    function latestAnswer() external view returns (uint256 answer);
}

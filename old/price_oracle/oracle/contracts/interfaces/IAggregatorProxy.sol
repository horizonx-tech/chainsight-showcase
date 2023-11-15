// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IAggregatorProxy {
    function latestAnswer() external view returns (uint128 answer);
}

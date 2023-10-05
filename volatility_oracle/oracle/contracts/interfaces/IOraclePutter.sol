// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IOraclePutter {
    function updateState(uint256 price) external;
}

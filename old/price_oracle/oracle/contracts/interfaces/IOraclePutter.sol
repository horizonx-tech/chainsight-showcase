// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

interface IOraclePutter {
    function updateState(uint128 price) external;
}

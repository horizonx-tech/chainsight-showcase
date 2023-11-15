// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "../interfaces/IOraclePutter.sol";

contract Oracle is IOraclePutter {
    mapping(address => uint256) public prices;

    function updateState(uint128 price) external override {
        prices[msg.sender] = price;
    }

    function get(address putter) external view returns (uint256) {
        return prices[putter];
    }
}

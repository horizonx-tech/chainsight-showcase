// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "../interfaces/IOraclePutter.sol";

contract Oracle is IOraclePutter {
    mapping(address => uint256) public values;

    function updateState(uint256 _values) external override {
        values[msg.sender] = _values;
    }

    function get(address putter) external view returns (uint256) {
        return values[putter];
    }
}

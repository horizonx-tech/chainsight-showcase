// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "../interfaces/IOracle.sol";

contract Oracle is IOracle {
    mapping(address => bytes) public data;

    function updateState(bytes calldata _data) external override {
        data[msg.sender] = _data;
        emit StateUpdated(msg.sender, _data);
    }

    function readAsString(
        address sender
    ) external view override returns (string memory) {
        return abi.decode(data[sender], (string));
    }

    function readAsUint256(
        address sender
    ) external view override returns (uint256) {
        return abi.decode(data[sender], (uint256));
    }

    function readAsUint128(
        address sender
    ) external view override returns (uint128) {
        return abi.decode(data[sender], (uint128));
    }

    function readAsUint64(
        address sender
    ) external view override returns (uint64) {
        return abi.decode(data[sender], (uint64));
    }
}

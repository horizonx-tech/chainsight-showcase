// SPDX-License-Identifier: agpl-3.0
pragma solidity 0.8.21;
import "./interfaces/IChainsightOracle.sol";

contract ChainsightOracle is IChainsightOracle {
    mapping(address => bytes) public data;

    function updateState(bytes calldata _data) external override {
        data[msg.sender] = _data;
        emit StateUpdated(msg.sender, _data);
    }

    function readAsString(
        address sender
    ) external view override returns (string memory) {
        bytes memory d = data[sender];
        if (d.length == 0) {
            return "";
        }
        return abi.decode(d, (string));
    }

    function readAsUint256(
        address sender
    ) external view override returns (uint256) {
        bytes memory d = data[sender];
        if (d.length == 0) {
            return 0;
        }
        return abi.decode(d, (uint256));
    }

    function readAsUint128(
        address sender
    ) external view override returns (uint128) {
        bytes memory d = data[sender];
        if (d.length == 0) {
            return 0;
        }
        return abi.decode(d, (uint128));
    }

    function readAsUint64(
        address sender
    ) external view override returns (uint64) {
        bytes memory d = data[sender];
        if (d.length == 0) {
            return 0;
        }
        return abi.decode(d, (uint64));
    }
}

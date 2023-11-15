// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import "./ChainsightOracle.sol";

contract Factory {
    function deploy(bytes32 _salt) public payable returns (address) {
        return address(new ChainsightOracle{salt: _salt}());
    }
}

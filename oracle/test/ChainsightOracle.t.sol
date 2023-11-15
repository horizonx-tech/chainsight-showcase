// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.21;

import {Test, console} from "forge-std/Test.sol";
import {ChainsightOracle} from "../src/ChainsightOracle.sol";

contract ChainsightOracleTest is Test {
    ChainsightOracle public oracle;

    function setUp() public {
        oracle = new ChainsightOracle();
        console.log(address(oracle));
    }

    function test_updateState() public {
        uint256 price = 10000;
        bytes memory data = abi.encode(price);
        oracle.updateState(data);
        uint256 returnedPrice = oracle.readAsUint256(address(this));
        assertEq(price, returnedPrice);
    }
}

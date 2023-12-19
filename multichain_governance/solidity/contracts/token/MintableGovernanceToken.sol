// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MintableGovernanceToken is ERC20 {
    constructor() ERC20("SampleGovernanceToken", "GOV") {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

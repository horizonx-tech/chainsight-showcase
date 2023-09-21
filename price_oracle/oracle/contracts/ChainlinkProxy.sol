// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;
import "./interfaces/IAggregatorProxy.sol";
import "./interfaces/IAggregatorWrapper.sol";

contract ChainlinkProxy is IAggregatorProxy {
    address public aggregator;

    constructor(address _aggregator) {
        aggregator = _aggregator;
    }

    function latestAnswer() external view override returns (uint128) {
        uint256 _answer = IAggregatorWrapper(aggregator).latestAnswer();
        return uint128(_answer);
    }
}

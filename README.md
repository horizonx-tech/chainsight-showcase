# chainsight-showcase

This repository is an implementation of the data processing components available in Chainsight.

Various showcases are available, including simple propagation patterns and patterns for indicator calculation using complex calculation logic.

Implemented components will be added to the UI as they become available.

https://beta.chainsight.network/

## Contents

- [Price Oracle](#price-oracle)
- [Market Index](#market-index)
- [Risk Indicator](#risk-indecator)

### Price Oracle

- [Price Feed](/price_feed/)
  - Price oracle with simple propagation from EVM contract and Web API with guaranteed reliability
  - ex: ETH, WBTC, OAS, USDC, USDT
  - [To learn more...](https://chainsight.network/blog/price-oracle-as-simple-data-relay)

### Market Index

- [Chainsight 20](/chainsight_20)
  - It calculates an average of the market capitalization of assets selected according to criteria such as liquidity metrics, and price data from 9 of the top DEXs and CEXs.
  - [To learn more...](https://medium.com/@Chainsight_Network/chainsight-20-crypto-index-now-live-106773283a25)
- [S&P 500](/sp500)
- [S&P Global Reit](/sp_global_reit/)
- [SPDR Gold Shares](/gold/)

### Risk Indecator

- [Realized Volatility](/volatility/volatility_eth)
  - Realized volatility (RVOL), as one method of calculating volatility, has attracted attention in the financial community as a measure of price stability. 
  - [To learn more...](https://chainsight.network/blog/realized-volatility-as-lens-with-single-data-source)
- [Implied Volatility](/implied_volatility_spx/)
  - Used primarily in options trading, it is a forecast of the future rate of change (volatility) in the price of the underlying asset. It can be calculated backwards based on the current "premium" of the option, and is mainly calculated using Black-Scholes models, etc.
  - [To learn more...](https://chainsight.network/blog/implied-volatility-as-lens-with-multiple-data-sources)
- [Volatility Index](/volatility_index_spx/)
  - The Volatility Index is the market's expectation of the strength (volatility) of the S&P 500 (SPX) over the short term. The VIX is calculated from the price of the SPX index option near its maturity date and represents the market's expectation for volatility over the next 30 days. It is considered an index that reflects market risk and investor sentiment and is called a "investor fear gauge"
  - [To learn more...](https://chainsight.network/blog/volatility-index-as-complex-calculator)
- [Crypto Volatility Index](/crypto_volatility_index/)
  - The Crypto Volatility Index (CVI) is a decentralized VIX for crypto. Calculated by cvi.finance.

### Automation

- [Multichain Governance](/multichain_governance/)
  - A multi-chain voting is a voting that takes place on multiple chains. The idea is to submit a proposal on one chain and vote on it on another chain.
  - [To learn more...](https://dev.to/hide_yoshi/creating-a-multi-chain-voting-in-30-minutes-with-chainsight-486h)

### Others

- [UniswapV3 EDPR](/uniswap_edpr/)
  - Calculate an Estimated Daily Percentage Return (EDPR) for trading pairs on Uniswap V3.
  - It was developed from the community by Chainsight's bounty project.

## Appendix

### Oracle Address

Chainsight exposes Oracle Contract as pipeline destinations for propagating data to various blockchains.

[horizonx-tech/chainsight-management-oracle: Oracles supported by Chainsight](https://github.com/horizonx-tech/chainsight-management-oracle)

These are the deployed addresses. If you have a network you would like to add, please use that repository or send us an add request.

| Network | Address |
| --- | --- |
| Sepolia (Ethereum) | [0xB5Ef491939A6dBf17287666768C903F03602c550](https://sepolia.etherscan.io/address/0xB5Ef491939A6dBf17287666768C903F03602c550) |
| Scroll Sepolia | [0xB5Ef491939A6dBf17287666768C903F03602c550](https://sepolia.scrollscan.dev/address/0xB5Ef491939A6dBf17287666768C903F03602c550) | 
| Taiko Jolnir | [0xB5Ef491939A6dBf17287666768C903F03602c550](https://explorer.jolnir.taiko.xyz/address/0xB5Ef491939A6dBf17287666768C903F03602c550) |

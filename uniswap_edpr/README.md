# EDPR Oracle for Uniswap V3
This project was built for the [Chainsight](https://chainsight.network/) platform using the [Chainsight CLI](https://github.com/horizonx-tech/chainsight-cli). It is intended to calculate an **Estimated Daily Percentage Return (EDPR)** for trading pairs on [Uniswap V3](https://info.uniswap.org/). The EDPR can be calculated at a pre-set interval by canisters on the [Internet Computer](https://internetcomputer.org/) blockchain and relayed to Chainsight's [Sepolia testnet oracle](https://sepolia.etherscan.io/address/0xB5Ef491939A6dBf17287666768C903F03602c550).

The trading pairs currently included are:
- [USDC/ETH](https://info.uniswap.org/#/pools/0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8) 0.3% fee tier
- [WBTC/ETH](https://info.uniswap.org/#/pools/0x4585fe77225b41b697c938b018e2ac67ac5a20c0) 0.05% fee tier
- [RNDR/ETH](https://info.uniswap.org/#/pools/0xe936f0073549ad8b1fa53583600d629ba9375161) 1% fee tier
- [VRA/ETH](https://info.uniswap.org/#/pools/0x98409d8ca9629fbe01ab1b914ebf304175e384c8) 0.3% fee tier

EDPR is calculated as the daily percentage return expected from a liquidity position with a price range (set by the liquidity provider) approximately matching the price range of the pair over the preceding week. The price range from the previous week is calculated by taking the minimum and maximum values of the [time-weighted average price](https://tienshaoku.medium.com/a-guide-on-uniswap-v3-twap-oracle-2aa74a4a97c5) from 28 six-hour intervals covering the past week. This approach allows for any extreme outliers in the price movement to be disregarded.

The components of the project for each pair are as follows:
- **eth_usdc_price snapshot** - Shared between pairs and takes the ETH price in USD, derived from the USDC/ETH trading pair
- **pool_fees snapshot** - Shows the 24-hour fees (received by liquidity providers) in USD
- **v3pool snapshot** - Includes current liquidity and price data
- **tick_cumul_28x6h snapshot** - On-chain data required to [calculate](https://blog.uniswap.org/uniswap-v3-math-primer) time-weighted average prices for the previous week
- **algorithm_lens** - Calculates EDPR using the above snapshots
- **relayer** - Relays the calculated EDPR to Chainsight's oracle on the Sepolia testnet

Input data are obtained directly from on-chain data using the [Uniswap Core contracts](https://docs.uniswap.org/contracts/v3/reference/overview) and from the [Oku Trade API](https://oku.trade/api).

This metric should be of use to liquidity providers on the Uniswap V3 platform. A unique feature of this metric is that it estimates returns within a defined range of liquidity, in line with the real-world use of a concentrated liquidity platform, rather than simply displaying a return based on the entire range of possible liquidity.

It is important to note that information obtained from this tool should **not be taken as financial advice** or as a prompt for any financial decisions. Several other factors are involved in the profitability of a liquidity pool position, the overall risk of this activity is very high, and the EDPR metric should serve purely as a prompt for further investigation of an asset pair.

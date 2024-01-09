# EDPR Oracle for Uniswap V3
This project was built for the [Chainsight](https://chainsight.network/) platform using the [Chainsight CLI](https://github.com/horizonx-tech/chainsight-cli). It is intended to calculate an **Estimated Daily Percentage Return (EDPR)** for trading pairs on [Uniswap V3](https://info.uniswap.org/). The EDPR can be calculated at a pre-set interval by canisters on the [Internet Computer](https://internetcomputer.org/) blockchain and relayed to Chainsight's [Sepolia testnet oracle](https://sepolia.etherscan.io/address/0xB5Ef491939A6dBf17287666768C903F03602c550).

The trading pairs currently included are:
- [wstETH/ETH](https://info.uniswap.org/#/pools/0x109830a1aaad605bbf02a9dfa7b0b92ec2fb7daa) 0.01% fee tier
- [DAI/ETH](https://info.uniswap.org/#/pools/0xc2e9f25be6257c210d7adf0d4cd6e3e881ba25f8) 0.3% fee tier
- [SHIB/ETH](https://info.uniswap.org/#/pools/0x5764a6f2212d502bc5970f9f129ffcd61e5d7563) 1% fee tier
- [RPL/ETH](https://info.uniswap.org/#/pools/0xe42318ea3b998e8355a3da364eb9d48ec725eb45) 0.3% fee tier

EDPR is calculated as the daily percentage return expected from a liquidity position with a price range (set by the liquidity provider) approximately matching the price range of the pair over the preceding week. For simplicity, the liqudity locked within the currently active price tick is taken as representative of the selected price range. The actual distribution can be visualised by searching for the pair at https://info.uniswap.org/. The price range from the previous week is calculated by taking the minimum and maximum values of the [time-weighted average price](https://tienshaoku.medium.com/a-guide-on-uniswap-v3-twap-oracle-2aa74a4a97c5) from 28 six-hour intervals covering the past week. This approach allows for any extreme outliers in the price movement to be disregarded.

The components of the project for each pair are as follows:
- **eth_usdc_price snapshot** - Shared between pairs and takes the ETH price in USD, derived from the wstETH/ETH trading pair
- **pool_fees snapshot** - Shows the 24-hour fees (received by liquidity providers) in USD
- **v3pool snapshot** - Includes current liquidity and price data
- **tick_cumul_28x6h snapshot** - On-chain data required to [calculate](https://blog.uniswap.org/uniswap-v3-math-primer) time-weighted average prices for the previous week
- **algorithm_lens** (shared) - Calculates EDPR using the above snapshots
- **relayer, indexer, indexer_lens** - Relays the calculated EDPR to Chainsight's oracle on the Sepolia testnet

Input data are obtained directly from on-chain data using the [Uniswap Core contracts](https://docs.uniswap.org/contracts/v3/reference/overview) and from the [Oku Trade API](https://oku.trade/api).

This metric should be of use to liquidity providers on the Uniswap V3 platform. A unique feature of this metric is that it estimates returns within a defined range of liquidity, in line with the real-world use of a concentrated liquidity platform, rather than simply displaying a return based on the entire range of possible liquidity.

It is important to note that information obtained from this tool should **not be taken as financial advice** or as a prompt for any financial decisions. Several other factors are involved in the profitability of a liquidity pool position, the overall risk of this activity is very high, and the EDPR metric should serve purely as a prompt for further investigation of an asset pair.

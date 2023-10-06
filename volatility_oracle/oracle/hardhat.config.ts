import { HardhatUserConfig, task } from 'hardhat/config';
import '@nomicfoundation/hardhat-toolbox';
import 'hardhat-abi-exporter';
import { config as dotenvConfig } from 'dotenv';
import { resolve } from 'path';
import '@nomiclabs/hardhat-etherscan';
const dotenvConfigPath: string = process.env.DOTENV_CONFIG_PATH || './.env';
dotenvConfig({ path: resolve(__dirname, dotenvConfigPath) });
const mnemonic: string = process.env.MNEMONIC || 'test t';
const alchemyOptimismKey: string = process.env.ALCHEMY_OPTIMISM_KEY || 'test t';
const alchemyArbitrumKey: string = process.env.ALCHEMY_ARBITRUM_KEY || 'test t';
const etherscanOptimismKey: string =
  process.env.ETHERSCAN_OPTIMISM_KEY || 'test t';
const etherscanArbitrumKey: string =
  process.env.ETHERSCAN_ARBITRUM_KEY || 'test t';

task('send_ether', 'Send ether to an address')
.addParam('address', 'The address to send ether to')
.setAction(async (taskArgs, hre) => {
  const { address } = taskArgs;
  console.log(`Sending 1 ETH to ${address}`);
  const [deployer] = await hre.ethers.getSigners();

  const tx = await deployer.sendTransaction({
    to: address,
    value: hre.ethers.utils.parseEther('1.0'),
  });
  await tx.wait();

  console.log(`Sent 1 ETH to ${address}`);
  console.log(`Transaction hash: ${tx.hash}`);
}
);
const config: HardhatUserConfig = {
  solidity: {
    compilers: [{ version: '0.8.9' }],
    settings: {
      optimizer: {
        enabled: true,
        runs: 1000,
      },
    },
  },
  etherscan: {
    apiKey: {
      optimismTest: etherscanOptimismKey,
      scrollAlpha: 'abc',
      arbitrumGoerli: etherscanArbitrumKey,
    },
    customChains: [
      {
        network: 'optimismTest',
        chainId: 420,
        urls: {
          apiURL: 'https://api-goerli-optimistic.etherscan.io/api',
          browserURL: 'https://goerli-optimism.etherscan.io',
        },
      },
      {
        network: 'scrollAlpha',
        chainId: 534353,
        urls: {
          apiURL: 'https://blockscout.scroll.io/api',
          browserURL: 'https://blockscout.scroll.io/',
        },
      },
      {
        network: "localhost",
        chainId: 31337,
        urls:{
          apiURL: "http://localhost:8545",
          browserURL: ""
        }
      }
    ],
  },
  networks: {
    optimismTest: {
      url: `https://opt-goerli.g.alchemy.com/v2/${alchemyOptimismKey}`,
      accounts: {
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    scrollAlpha: {
      url: 'https://alpha-rpc.scroll.io/l2',
      accounts: {
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    },
    arbitrumGoerli: {
      url: `https://arb-goerli.g.alchemy.com/v2/${alchemyArbitrumKey}`,
      accounts: {
        count: 10,
        mnemonic,
        path: "m/44'/60'/0'/0",
      },
    }
  },
  abiExporter: {
    path: './abi',
    format: 'json',
    runOnCompile: true,
    only: ['contracts/interfaces/*'],
    flat: true,
  },
  
  
};

export default config;

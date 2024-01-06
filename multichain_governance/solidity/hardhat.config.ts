import { HardhatUserConfig, task } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import fs from "fs";
import path from "path";
require("dotenv").config();
import "@nomicfoundation/hardhat-verify";

const SKIP_LOAD = process.env.SKIP_LOAD === "true";
const tasksPath = path.join(__dirname, "tasks");
if (!SKIP_LOAD) {
  fs.readdirSync(tasksPath)
    .filter((pth) => pth.endsWith(".ts"))
    .forEach((task) => {
      require(`${tasksPath}/${task}`);
    });
}

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },

  typechain: {
    outDir: "typechain",
    target: "ethers-v6",
    alwaysGenerateOverloads: true,
  },
  etherscan: {
    apiKey: {
      sepolia: process.env.ETHERSCAN_API_KEY || "",
      scrollSepolia: process.env.SCROLLSCAN_API_KEY || "",
    },
    customChains: [
      {
        network: "scrollSepolia",
        chainId: 534351,
        urls: {
          apiURL: "https://api-sepolia.scrollscan.com/api",
          browserURL: "https://sepolia.scrollscan.dev/",
        },
      },
    ],
  },

  networks: {
    hardhat: {
      chainId: 1337,
      accounts: {
        mnemonic: "test test test test test test test test test test test junk",
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 10,
      },
    },
    sepolia: {
      chainId: 11155111,
      url: "https://endpoints.omniatech.io/v1/eth/sepolia/public",
      accounts: {
        mnemonic: process.env.MNEMONIC,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 10,
      },
    },
    scrollSepolia: {
      chainId: 534351,
      url: "https://rpc.ankr.com/scroll_sepolia_testnet",
      accounts: {
        mnemonic: process.env.MNEMONIC,
        path: "m/44'/60'/0'/0",
        initialIndex: 0,
        count: 10,
      },
    },
  },
};

export default config;

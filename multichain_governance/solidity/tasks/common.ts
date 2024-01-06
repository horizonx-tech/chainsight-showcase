import { ContractFactory, Signer, TransactionResponse } from "ethers";
import { eContractid } from "./contracts";

export const verificationRequired = (network: string) => {
  return !isLocalNetwork(network);
};

export const isLocalNetwork = (network: string) => {
  return network === "hardhat" || network === "localhost";
};

export const verifyContract = async (
  contracts: { address: string; constructorArguments: any[] }[]
) => {
  const hre = require("hardhat");
  for (const c of contracts) {
    await hre.run("verify:verify", {
      address: c.address,
      constructorArguments: c.constructorArguments,
    });
  }
};

export const deploy = async (
  id: eContractid,
  signer: Signer,
  factory: ContractFactory,
  args: any[]
) => {
  const hre = require("hardhat");
  const network = hre.network.name;
  const contract = await factory.connect(signer).deploy(...args);
  !isLocalNetwork(network) && (await contract.waitForDeployment());
  saveContractInDB(id.valueOf(), await contract.getAddress(), network);
  return contract;
};

export const getContractAddress = (id: eContractid) => {
  const hre = require("hardhat");
  const network = hre.network.name;
  const db = getDB();
  return db.contracts[id.valueOf()][network];
};

export const waitForTx = async (tx: TransactionResponse) => {
  await tx.wait();
};

const saveContractInDB = (
  contractName: string,
  address: string,
  network: string
) => {
  const db = getDB();
  if (!db.contracts.hasOwnProperty(contractName)) {
    db.contracts[contractName] = {};
  }
  db.contracts[contractName][network] = address;
  writeDB(db);
};

const writeDB = (db: any) => {
  require("fs").writeFileSync("./contracts.json", JSON.stringify(db));
};

const getDB = () => {
  const db = readJSONDB();
  return JSON.parse(db);
};

const readJSONDB = () => {
  if (!require("fs").existsSync("./contracts.json")) {
    require("fs").writeFileSync(
      "./contracts.json",
      JSON.stringify({ contracts: {} })
    );
  }
  return require("fs").readFileSync("./contracts.json", "utf8");
};

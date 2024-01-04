import { ContractFactory, Signer, TransactionResponse } from "ethers";

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
  network: string,
  signer: Signer,
  factory: ContractFactory,
  args: any[]
) => {
  const contract = await factory.connect(signer).deploy(...args);
  !isLocalNetwork(network) && (await contract.waitForDeployment());
  return contract;
};

export const waitForTx = async (tx: TransactionResponse) => {
  await tx.wait();
};

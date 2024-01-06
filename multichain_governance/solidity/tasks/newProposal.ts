import { task } from "hardhat/config";
import { Provider, Signer } from "ethers";
import { getContractAddress } from "./common";
import { eContractid } from "./contracts";
import { ProposalFactory__factory } from "../typechain";

task("newProposal", "submit a proposal", async (_, { ethers }) => {
  const factoryAddress = getContractAddress(eContractid.ProposalFactory);
  const proposer: Signer = (await ethers.getSigners())[0];
  const factory = ProposalFactory__factory.connect(factoryAddress, proposer);
  const now = Math.floor(Date.now() / 1000);
  const tx = await factory.create(now, now + 60 * 60);
  await tx.wait();
  return tx;
});

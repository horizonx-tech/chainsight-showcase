import { task } from "hardhat/config";
import { MintableGovernanceToken__factory } from "../typechain/factories/contracts/token";
import { VotingSynchronizer__factory } from "../typechain/factories/contracts/voting";
import {
  ProposalFactory__factory,
  ProposalManager__factory,
} from "../typechain";

task("deploy", "Deploy the contracts", async (_, { ethers }) => {
  console.log("Deploying contracts...");
  const deployer = (await ethers.getSigners())[0];
  const govToken = await new MintableGovernanceToken__factory()
    .connect(deployer)
    .deploy();
  const votingSyncronizer = await new VotingSynchronizer__factory()
    .connect(deployer)
    .deploy();
  const proposalSynchronizer = await new VotingSynchronizer__factory()
    .connect(deployer)
    .deploy();
  const porposalManager = await new ProposalManager__factory()
    .connect(deployer)
    .deploy(
      await govToken.getAddress(),
      await votingSyncronizer.getAddress(),
      await proposalSynchronizer.getAddress()
    );
  const proposalFactory = await new ProposalFactory__factory()
    .connect(deployer)
    .deploy(await porposalManager.getAddress());
  await votingSyncronizer.setProposalManager(
    await porposalManager.getAddress()
  );
  await proposalSynchronizer.setProposalManager(
    await porposalManager.getAddress()
  );
  await porposalManager.setProposalFactory(await proposalFactory.getAddress());

  //[
  //  { govToken: await govToken.getAddress() },
  //  { votingSyncronizer: await votingSyncronizer.getAddress() },
  //  { proposalSynchronizer: await proposalSynchronizer.getAddress() },
  //  { porposalManager: await porposalManager.getAddress() },
  //  { proposalFactory: await proposalFactory.getAddress() },
  //].forEach((address) => {
  //  console.log(address, "\n");
  //});
  return {
    govToken: govToken,
    votingSyncronizer: votingSyncronizer,
    proposalSynchronizer: proposalSynchronizer,
    porposalManager: porposalManager,
    proposalFactory: proposalFactory,
  };
});

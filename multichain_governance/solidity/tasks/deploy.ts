import { task } from "hardhat/config";
import { MintableGovernanceToken__factory } from "../typechain/factories/contracts/token";
import { VotingSynchronizer__factory } from "../typechain/factories/contracts/voting";
import {
  MintableGovernanceToken,
  ProposalFactory,
  ProposalFactory__factory,
  ProposalManager,
  ProposalManager__factory,
  VotingSynchronizer,
} from "../typechain";
import { Provider, Signer } from "ethers";
import { deploy, isLocalNetwork, verifyContract, waitForTx } from "./common";
import { eContractid } from "./contracts";

task("deploy", "Deploy the contracts", async (_, { ethers }) => {
  // Deployment
  console.log("Deploying contracts...");
  const networkName = (await (ethers.provider as Provider).getNetwork()).name;
  const deployer: Signer = (await ethers.getSigners())[0];
  const govToken = (await deploy(
    eContractid.MintableGovernanceToken,
    deployer,
    new MintableGovernanceToken__factory(),
    []
  )) as MintableGovernanceToken;
  const votingSyncronizer = (await deploy(
    eContractid.VotingSynchronizer,
    deployer,
    new VotingSynchronizer__factory(),
    []
  )) as VotingSynchronizer;
  const proposalSynchronizer = (await deploy(
    eContractid.ProposalSynchronizer,
    deployer,
    new VotingSynchronizer__factory(),
    []
  )) as VotingSynchronizer;
  const porposalManager = (await deploy(
    eContractid.ProposalManager,
    deployer,
    new ProposalManager__factory(),
    [
      await govToken.getAddress(),
      await votingSyncronizer.getAddress(),
      await proposalSynchronizer.getAddress(),
    ]
  )) as ProposalManager;

  const proposalFactory = (await deploy(
    eContractid.ProposalFactory,
    deployer,
    new ProposalFactory__factory(),
    [await porposalManager.getAddress()]
  )) as ProposalFactory;

  // Configuration
  await waitForTx(
    await votingSyncronizer.setProposalManager(
      await porposalManager.getAddress()
    )
  );

  await waitForTx(
    await proposalSynchronizer.setProposalManager(
      await porposalManager.getAddress()
    )
  );
  await waitForTx(
    await porposalManager.setProposalFactory(await proposalFactory.getAddress())
  );

  // Verification
  !isLocalNetwork(networkName) &&
    (await verifyContract([
      { address: await govToken.getAddress(), constructorArguments: [] },
      {
        address: await votingSyncronizer.getAddress(),
        constructorArguments: [],
      },
      {
        address: await proposalSynchronizer.getAddress(),
        constructorArguments: [],
      },
      {
        address: await porposalManager.getAddress(),
        constructorArguments: [
          await govToken.getAddress(),
          await votingSyncronizer.getAddress(),
          await proposalSynchronizer.getAddress(),
        ],
      },
      {
        address: await proposalFactory.getAddress(),
        constructorArguments: [await porposalManager.getAddress()],
      },
    ]));
  !isLocalNetwork(networkName) &&
    console.log(
      JSON.stringify([
        { govToken: await govToken.getAddress() },
        { votingSyncronizer: await votingSyncronizer.getAddress() },
        { proposalSynchronizer: await proposalSynchronizer.getAddress() },
        { porposalManager: await porposalManager.getAddress() },
        { proposalFactory: await proposalFactory.getAddress() },
      ])
    );
  return {
    govToken: govToken,
    votingSyncronizer: votingSyncronizer,
    proposalSynchronizer: proposalSynchronizer,
    porposalManager: porposalManager,
    proposalFactory: proposalFactory,
  };
});

import { ProposalFactory, ProposalManager } from "../typechain";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { expect } from "chai";
const hre = require("hardhat") as HardhatRuntimeEnvironment;

describe("ProposalManager", async () => {
  let proposalFactory: ProposalFactory;
  let proposalManager: ProposalManager;
  beforeEach(async () => {
    const contracts = await hre.run("deploy");
    proposalFactory = contracts.proposalFactory;
    proposalManager = contracts.porposalManager;
  });
  describe("setProposalFactory", async () => {
    it("should set the proposal factory", async () => {
      await proposalManager.setProposalFactory(
        await (await hre.ethers.getSigners())[0].getAddress()
      );
      expect(await proposalManager.proposalFactory()).to.be.equal(
        await (await hre.ethers.getSigners())[0].getAddress()
      );
    });
    it("should not set the proposal factory if not called by the owner", async () => {
      await expect(
        proposalManager
          .connect((await hre.ethers.getSigners())[1])
          .setProposalFactory(await proposalFactory.getAddress())
      ).to.be.rejected;
    });
  });
  describe("createProposal", async () => {
    it("should create a proposal", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0);
      await proposalFactory.create(current + 100, current + 200);
    });
  });
});

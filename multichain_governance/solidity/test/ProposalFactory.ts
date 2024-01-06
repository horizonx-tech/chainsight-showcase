import { ProposalFactory, ProposalManager } from "../typechain";
import { HardhatRuntimeEnvironment } from "hardhat/types";
import { ContractTransactionReceipt } from "ethers";
import { expect } from "chai";
const hre = require("hardhat") as HardhatRuntimeEnvironment;
describe("ProposalFactory", async () => {
  let proposalFactory: ProposalFactory;
  let proposalManager: ProposalManager;
  beforeEach(async () => {
    const contracts = await hre.run("deploy");
    proposalFactory = contracts.proposalFactory;
    proposalManager = contracts.porposalManager;
  });
  describe("create", async () => {
    it("should create a proposal", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0);
      await createProposal(proposalFactory, current + 100, current + 200);
    });
    it("should not create a proposal if the start date is in the past", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0).toString();
      await expect(
        proposalFactory.create(current, current + 100)
      ).to.rejectedWith("ProposalFactory: INVALID_TIMESTAMP");
    });
    it("should not create a proposal if the end date is in the past", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0).toString();
      await expect(
        proposalFactory.create(current + 100, current)
      ).to.rejectedWith("ProposalFactory: INVALID_TIMESTAMP");
    });
    it("should be recorded in the proposal manager", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0).toString();
      const proposalId = await createProposal(
        proposalFactory,
        current + 100,
        current + 200
      );
      const proposal = await proposalManager.proposals(proposalId);
      expect(proposal).to.not.be.undefined;
      expect(proposal[0]).to.be.equal(
        await (await hre.ethers.getSigners())[0].getAddress()
      );
      expect(proposal[1]).to.be.equal(1337);
      expect(proposal[2]).to.be.equal(current + 100);
      expect(proposal[3]).to.be.equal(current + 200);
      expect(proposal[4]).to.be.equal(0);
      expect(proposal[5]).to.be.equal(0);
    });
    it("should emit a ProposalCreated event", async () => {
      const current = (new Date().getTime() / 1000).toFixed(0).toString();
      const tx = await proposalFactory.create(current + 100, current + 200);
      const receipt = await tx.wait();
      const event = receipt?.logs.filter((log) => {
        return (
          log.topics[0] ===
          proposalFactory.interface.getEvent("ProposalCreated").topicHash
        );
      })[0]!;
      expect(event).to.not.be.undefined;
      const decodedEvent = proposalFactory.interface.decodeEventLog(
        "ProposalCreated",
        event.data,
        event.topics
      );
      expect(decodedEvent[0]).to.not.be.undefined;
      expect(decodedEvent[1]).to.be.equal(
        await (await hre.ethers.getSigners())[0].getAddress()
      );
      expect(decodedEvent[2]).to.be.equal(1337);
      expect(decodedEvent[3]).to.be.equal(current + 100);
      expect(decodedEvent[4]).to.be.equal(current + 200);
    });
  });
});

const createProposal = async (
  proposalFactory: ProposalFactory,
  start: string,
  end: string
) => {
  const tx = await proposalFactory.create(start, end);
  const receipt = await tx.wait();
  const proposalId = fromReceiptToProposalId(receipt!, proposalFactory);
  return proposalId;
};

const fromReceiptToProposalId = (
  porposalCreatedReceipt: ContractTransactionReceipt,
  proposalFactory: ProposalFactory
) => {
  return porposalCreatedReceipt.logs.map((log) => {
    const topics = log.topics;
    if (
      topics[0] ===
      proposalFactory.interface.getEvent("ProposalCreated").topicHash
    ) {
      return proposalFactory.interface.decodeEventLog(
        "ProposalCreated",
        log.data,
        log.topics
      );
    }
  })[0]![0];
};

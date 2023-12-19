// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "../misc/Synchronizeable.sol";
import "../interfaces/IProposalManager.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract ProposalManager is IProposalManager, Ownable {
    IERC20 public immutable votingToken;
    address public immutable votingSynchronizer;
    address public immutable proposalSyncrhonizer;
    address public proposalFactory;
    // the voting period for the proposal on the other chain. this is required for synchronizing votes to other chains.
    uint256 public constant OTHERCHAIN_VOTING_PERIOD = 1 hours;

    struct Proposal {
        address creator;
        uint256 chainId;
        uint256 startTimestamp;
        uint256 endTimestamp;
        uint256 forVotes;
        uint256 againstVotes;
    }

    mapping(uint256 => Proposal) public proposals;
    // mapped by proposal id => voter address => chainids
    mapping(uint256 => mapping(address => uint256[])) public votes;

    constructor(
        address token,
        address _votingSynchronizer,
        address _proposalSyncrhonizer
    ) Ownable(msg.sender) {
        require(token != address(0), "ProposalManager: INVALID_VOTING_TOKEN");
        require(
            _votingSynchronizer != address(0),
            "ProposalManager: INVALID_VOTING_SYNCHRONIZER"
        );
        require(
            _proposalSyncrhonizer != address(0),
            "ProposalManager: INVALID_PROPOSAL_SYNCHRONIZER"
        );
        votingToken = IERC20(token);
        votingSynchronizer = _votingSynchronizer;
        proposalSyncrhonizer = _proposalSyncrhonizer;
    }

    /// @inheritdoc IProposalManager
    function setProposalFactory(address _proposalFactory) external onlyOwner {
        require(
            _proposalFactory != address(0),
            "ProposalManager: INVALID_PROPOSAL_FACTORY"
        );
        proposalFactory = _proposalFactory;
    }

    /// @inheritdoc IProposalManager
    function createProposal(
        uint256 id,
        address creator,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external override {
        require(
            msg.sender == proposalFactory,
            "ProposalManager: ONLY_PROPOSAL_FACTORY"
        );
        uint256 _chainId = block.chainid;
        _createProposal(id, creator, _chainId, startTimestamp, endTimestamp);
    }

    /// @inheritdoc IProposalManager
    function onProposalCreated(
        uint256 id,
        address creator,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) external override {
        require(
            msg.sender == proposalSyncrhonizer,
            "ProposalManager: ONLY_PROPOSAL_SYNCHRONIZER"
        );
        _createProposal(id, creator, chainId, startTimestamp, endTimestamp);
    }

    function _createProposal(
        uint256 id,
        address creator,
        uint256 chainId,
        uint256 startTimestamp,
        uint256 endTimestamp
    ) internal {
        require(
            proposals[id].creator == address(0),
            "ProposalManager: PROPOSAL_ALREADY_EXIST"
        );
        require(
            creator != address(0),
            "ProposalManager: INVALID_PROPOSAL_CREATOR"
        );
        require(
            startTimestamp > block.timestamp,
            "ProposalManager: INVALID_START_TIMESTAMP"
        );
        require(
            endTimestamp > startTimestamp,
            "ProposalManager: INVALID_END_TIMESTAMP"
        );
        proposals[id] = Proposal({
            creator: creator,
            chainId: chainId,
            startTimestamp: startTimestamp,
            endTimestamp: endTimestamp,
            forVotes: 0,
            againstVotes: 0
        });
    }

    /// @inheritdoc IProposalManager
    function onVoted(
        uint256 id,
        uint256 chainId,
        address voter,
        uint256 power,
        bool support
    ) external override {
        require(
            msg.sender == votingSynchronizer,
            "ProposalManager: ONLY_VOTING_SYNCHRONIZER"
        );
        _vote(id, chainId, voter, power, support);
    }

    // @inheritdoc IProposalManager
    function vote(uint256 id, bool support) external {
        uint256 power = votingToken.balanceOf(msg.sender);
        uint256 chainId = block.chainid;
        _vote(id, chainId, msg.sender, power, support);
    }

    function _vote(
        uint256 id,
        uint256 fromChainId,
        address voter,
        uint256 power,
        bool support
    ) internal {
        Proposal storage proposal = proposals[id];
        _validateVote(id, voter, power, proposal);
        votes[id][voter].push(fromChainId);
        if (support) {
            proposal.forVotes += power;
        } else {
            proposal.againstVotes += power;
        }
        emit Voted(id, fromChainId, voter, power, support);
    }

    function _validateVote(
        uint256 id,
        address voter,
        uint256 power,
        Proposal storage proposal
    ) internal view {
        // if the proposal does not exist, the vote is invalid
        require(
            proposal.creator != address(0),
            "ProposalManager: PROPOSAL_NOT_EXIST"
        );
        // if the voter has no voting power, the vote is invalid
        require(power > 0, "ProposalManager: NO_VOTING_POWER");
        // if the proposal is not in the voting period, the vote is invalid
        require(
            block.timestamp >= proposal.startTimestamp,
            "ProposalManager: NOT_IN_VOTING_PERIOD"
        );
        // if the proposal is already ended, the vote is invalid
        uint256 currentChainId = block.chainid;
        uint256 period = currentChainId == proposal.chainId
            ? proposal.endTimestamp
            : proposal.endTimestamp - OTHERCHAIN_VOTING_PERIOD;
        require(
            block.timestamp <= period,
            "ProposalManager: VOTING_PERIOD_ENDED"
        );
        // if the voter has already voted, the vote is invalid
        for (uint256 i = 0; i < votes[id][voter].length; i++) {
            if (votes[id][voter][i] == block.chainid) {
                revert("ProposalManager: ALREADY_VOTED");
            }
        }
    }

    /// @inheritdoc IProposalManager
    function getProposal(
        uint256 id
    )
        external
        view
        override
        returns (
            address creator,
            uint256 chainId,
            uint256 startTimestamp,
            uint256 endTimestamp,
            uint256 forVotes,
            uint256 againstVotes,
            bool accepted
        )
    {
        return _getProposal(id);
    }

    function _getProposal(
        uint256 id
    )
        internal
        view
        returns (
            address creator,
            uint256 chainId,
            uint256 startTimestamp,
            uint256 endTimestamp,
            uint256 forVotes,
            uint256 againstVotes,
            bool accepted
        )
    {
        Proposal storage proposal = proposals[id];
        if (proposal.creator == address(0)) {
            return (address(0), 0, 0, 0, 0, 0, false);
        }
        accepted =
            (proposal.forVotes > proposal.againstVotes) &&
            proposal.endTimestamp < block.timestamp;
        return (
            proposal.creator,
            proposal.chainId,
            proposal.startTimestamp,
            proposal.endTimestamp,
            proposal.forVotes,
            proposal.againstVotes,
            accepted
        );
    }
}

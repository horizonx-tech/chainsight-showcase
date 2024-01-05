# !bin/sh

cd $(dirname $0)/..

# read contract addresses from ./solidity/contracts.json
CONTRACTS=$(cat ./solidity/contracts.json)

export PROPOSAL_FACTORY_SEPOOLIA_ADDRESS=$(echo $CONTRACTS | jq -r '.contracts.ProposalFactory.sepolia')
export PROPOSAL_FACTORY_SCROLL_SEPOOLIA_ADDRESS=$(echo $CONTRACTS | jq -r '.contracts.ProposalFactory.scrollSepolia')
export PROPOSAL_SYNCHRONIZER_ADDRESS_SEPOLIA=$(echo $CONTRACTS | jq -r '.contracts.ProposalSynchronizer.sepolia')
export PROPOSAL_SYNCHRONIZER_ADDRESS_SCROLL_SEPOLIA=$(echo $CONTRACTS | jq -r '.contracts.ProposalSynchronizer.scrollSepolia')
export PROPOSAL_MANAGER_SEPOLIA_ADDRESS=$(echo $CONTRACTS | jq -r '.contracts.ProposalManager.sepolia')
export PROPOSAL_MANAGER_SCROLL_SEPOLIA_ADDRESS=$(echo $CONTRACTS | jq -r '.contracts.ProposalManager.scrollSepolia')
export VOTING_SYNCHRONIZER_ADDRESS_SEPOLIA=$(echo $CONTRACTS | jq -r '.contracts.VotingSynchronizer.sepolia')
export VOTING_SYNCHRONIZER_ADDRESS_SCROLL_SEPOLIA=$(echo $CONTRACTS | jq -r '.contracts.VotingSynchronizer.scrollSepolia')

# update addresses in components

cd components

yq e -i '.datasource.id= env(PROPOSAL_FACTORY_SEPOOLIA_ADDRESS)' proposal_factory_event_indexer_sepolia.yaml
yq e -i '.datasource.id= env(PROPOSAL_FACTORY_SCROLL_SEPOOLIA_ADDRESS)' proposal_factory_event_indexer_scroll_sepolia.yaml
yq e -i '.destination.oracle_address= env(PROPOSAL_SYNCHRONIZER_ADDRESS_SEPOLIA)' proposal_relayer_sepolia.yaml
yq e -i '.destination.oracle_address= env(PROPOSAL_SYNCHRONIZER_ADDRESS_SCROLL_SEPOLIA)' proposal_relayer_scroll_sepolia.yaml
yq e -i '.datasource.id= env(PROPOSAL_MANAGER_SEPOLIA_ADDRESS)' voting_event_indexer_sepolia.yaml
yq e -i '.datasource.id= env(PROPOSAL_MANAGER_SCROLL_SEPOLIA_ADDRESS)' voting_event_indexer_scroll_sepolia.yaml
yq e -i '.destination.oracle_address= env(VOTING_SYNCHRONIZER_ADDRESS_SEPOLIA)' voting_relayer_sepolia.yaml
yq e -i '.destination.oracle_address= env(VOTING_SYNCHRONIZER_ADDRESS_SCROLL_SEPOLIA)' voting_relayer_scroll_sepolia.yaml


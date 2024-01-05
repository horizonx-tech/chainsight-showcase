# !bin/sh

# Deploy contracts
cd $(dirname $0)/../solidity

yarn deploy:sepolia
yarn deploy:scrollSepolia

cd ../components
# get latest block from sepolia and scroll sepolia
SEPOLIA_LATEST_BLOCK_HEX=$(curl -s -X POST -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"eth_blockNumber", "params":[]}' https://sepolia-rpc.scroll.io | jq -r '.result')
export SEPOLIA_LATEST_BLOCK=$(printf "%d\n" $SEPOLIA_LATEST_BLOCK_HEX)
SCROLL_SEPOLIA_LATEST_BLOCK_HEX=$(curl -s -X POST -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method":"eth_blockNumber", "params":[]}' https://scroll-sepolia-rpc.scroll.io | jq -r '.result')
export SCROLL_SEPOLIA_LATEST_BLOCK=$(printf "%d\n" $SCROLL_SEPOLIA_LATEST_BLOCK_HEX)

# update latest block in components
yq e -i '.datasource.from= env(SEPOLIA_LATEST_BLOCK)' proposal_factory_event_indexer_sepolia.yaml
yq e -i '.datasource.from= env(SCROLL_SEPOLIA_LATEST_BLOCK)' proposal_factory_event_indexer_scroll_sepolia.yaml
yq e -i '.datasource.from= env(SEPOLIA_LATEST_BLOCK)' voting_event_indexer_sepolia.yaml
yq e -i '.datasource.from= env(SCROLL_SEPOLIA_LATEST_BLOCK)' voting_event_indexer_scroll_sepolia.yaml

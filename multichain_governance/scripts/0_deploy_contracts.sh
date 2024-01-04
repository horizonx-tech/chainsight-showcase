# !bin/sh

# Deploy contracts
cd $(dirname $0)/../solidity

yarn deploy:sepolia
yarn deploy:scrollSepolia


# !bin/sh

cd $(dirname $0)/../solidity

# submit a proposal on sepolia
npx hardhat run --network sepolia newProposal

# submit a proposal on scroll sepolia
npx hardhat run --network scrollSepolia newProposal


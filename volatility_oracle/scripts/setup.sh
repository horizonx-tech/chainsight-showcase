#!/bin/bash

echo "ensure required packages installed"
if ! command -v jq &> /dev/null
then
    echo "jq could not be found"
    exit
fi
if ! command -v yq &> /dev/null
then
    echo "yq could not be found"
    exit
fi


cd $(dirname $0)
echo "start hardhat node if not running"
cd ../oracle
yarn
# if port 8545 is not open, start hardhat node
if ! nc -z localhost 8545 ;
then
  npx hardhat node &
  sleep 5
fi

echo "deploy contracts"
yarn deploy
# capture the address of the deployed contract
export CONTRACT_ADDRESS=$(cat ./addresses.json | jq -r '.oracle')
echo "contract address: $CONTRACT_ADDRESS"

echo "ensure dfx is running"
if nc -z localhost 4943 ;
then
  dfx stop
fi

$(dfx start --background --clean) & 
sleep 5


echo "deploy chainsight-management-canisters"
if [ ! -d "chainsight-management-canisters" ];
then
  git clone https://github.com/horizonx-tech/chainsight-management-canisters.git
fi
cd chainsight-management-canisters
make local port=4943
cd ..


echo "create canisters"
cd ../artifacts
dfx canister create --all
cd ..

echo "export canister ids"
cd artifacts/.dfx/local
export RELAYER_MAINNET_CANISTER_ID=$(cat canister_ids.json | jq -r '.relayer_mainnet.local')
export RELAYER_POLYGON_MUMBAI_CANISTER_ID=$(cat canister_ids.json | jq -r '.relayer_polygon_mumbai.local')
export UNISWAP_SNAPSHOTTER_MAINNET_CANISTER_ID=$(cat canister_ids.json | jq -r '.uniswap_snapshotter_mainnet.local')
export UNISWAP_SNAPSHOTTER_POLYGON_MUMBAI_CANISTER_ID=$(cat canister_ids.json | jq -r '.uniswap_snapshotter_polygon_mumbai.local')
export CALCULATOR_CANISTER_ID=$(cat canister_ids.json | jq -r '.calculator.local')
cd ../../..

echo "modify manifests"
yq '.lens_targets.identifiers[0] = env(RELAYER_MAINNET_CANISTER_ID)' ./components/relayer_mainnet.yaml > ./components/relayer_mainnet.yaml.tmp && mv ./components/relayer_mainnet.yaml.tmp ./components/relayer_mainnet.yaml
yq '.lens_targets.identifiers[0] = env(RELAYER_POLYGON_MUMBAI_CANISTER_ID)' ./components/relayer_polygon_mumbai.yaml > ./components/relayer_polygon_mumbai.yaml.tmp && mv ./components/relayer_polygon_mumbai.yaml.tmp ./components/relayer_polygon_mumbai.yaml
yq '.destination.oracle_address = env(CONTRACT_ADDRESS)' ./components/relayer_mainnet.yaml > ./components/relayer_mainnet.yaml.tmp && mv ./components/relayer_mainnet.yaml.tmp ./components/relayer_mainnet.yaml
yq '.destination.oracle_address = env(CONTRACT_ADDRESS)' ./components/relayer_polygon_mumbai.yaml > ./components/relayer_polygon_mumbai.yaml.tmp && mv ./components/relayer_polygon_mumbai.yaml.tmp ./components/relayer_polygon_mumbai.yaml

echo "install canisters"
csx build
csx deploy

echo "initialize"
csx exec

echo "send ether to relayer"
cd artifacts
export RELAYER_MAINNET_ADDRESS=$(dfx canister call relayer_mainnet get_ethereum_address)
# remove '(' and ')'
export RELAYER_MAINNET_ADDRESS=${RELAYER_MAINNET_ADDRESS:1:${#RELAYER_MAINNET_ADDRESS}-2}
export RELAYER_POLYGON_MUMBAI_ADDRESS=$(dfx canister call relayer_polygon_mumbai get_ethereum_address)
# remove '(' and ')'
export RELAYER_POLYGON_MUMBAI_ADDRESS=${RELAYER_POLYGON_MUMBAI_ADDRESS:1:${#RELAYER_POLYGON_MUMBAI_ADDRESS}-2}
cd ..
cd oracle
npx hardhat send_ether --address $RELAYER_MAINNET_ADDRESS
npx hardhat send_ether --address $RELAYER_POLYGON_MUMBAI_ADDRESS


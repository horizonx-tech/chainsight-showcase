# !bin/sh

# Deploy Canisters
cd $(dirname $0)/..

csx build 
csx deploy

# !bin/sh

cd $(dirname $0)/..

# clone https://github.com/horizonx-tech/chainsight-management-canisters if not already cloned

CLONED = $(ls | grep chainsight-management-canisters)

if [ -z "$CLONED" ]; then
    git clone https://github.com/horizonx-tech/chainsight-management-canisters
fi

# get dfx port
export REPLICA_PORT=$(ps aux |grep icx-proxy| grep -oP '(?<=replica http://localhost:)\d+')


# Deploy management canisters
cd chainsight-management-canisters
make local port=$REPLICA_PORT

# !bin/sh

cd $(dirname $0)

./0_deploy_contracts.sh

./dfx.sh
